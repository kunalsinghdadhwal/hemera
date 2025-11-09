//! # Hemera
//!
//! A lightweight attribute macro for measuring function execution time with divine precision.
//!
//! ## Usage
//!
//! ```rust
//! use hemera::hemera;
//!
//! #[hemera]
//! fn calculate_fibonacci(n: u32) -> u32 {
//!     if n <= 1 {
//!         n
//!     } else {
//!         calculate_fibonacci(n - 1) + calculate_fibonacci(n - 2)
//!     }
//! }
//!
//! #[hemera(name = "CustomTimer", level = "debug")]
//! fn slow_function() {
//!     std::thread::sleep(std::time::Duration::from_millis(100));
//! }
//!
//! #[hemera(threshold = "50ms")]
//! fn maybe_slow(n: u32) {
//!     // Only logs if execution takes more than 50ms
//!     std::thread::sleep(std::time::Duration::from_millis(n as u64));
//! }
//! ```
//!
//! ## Async Support
//!
//! ```rust
//! use hemera::hemera;
//!
//! #[hemera]
//! async fn fetch_data() -> String {
//!     // Async function timing
//!     "data".to_string()
//! }
//! ```
//!
//! ## Tracing Integration
//!
//! Enable the `tracing` feature for integration with the `tracing` ecosystem:
//!
//! ```toml
//! [dependencies]
//! hemera = { version = "0.1", features = ["tracing"] }
//! ```

use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
    Expr, ItemFn, Lit, Meta, MetaNameValue, Token,
};

/// Configuration for the hemera macro
struct HemeraConfig {
    name: Option<String>,
    level: Option<String>,
    threshold: Option<String>,
}

impl Parse for HemeraConfig {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut config = HemeraConfig {
            name: None,
            level: None,
            threshold: None,
        };

        let metas = Punctuated::<Meta, Token![,]>::parse_terminated(input)?;

        for meta in metas {
            match meta {
                Meta::NameValue(MetaNameValue { path, value, .. }) => {
                    let ident = path
                        .get_ident()
                        .ok_or_else(|| syn::Error::new_spanned(&path, "Expected identifier"))?;

                    match ident.to_string().as_str() {
                        "name" => {
                            if let Expr::Lit(expr_lit) = value {
                                if let Lit::Str(lit_str) = &expr_lit.lit {
                                    config.name = Some(lit_str.value());
                                }
                            }
                        }
                        "level" => {
                            if let Expr::Lit(expr_lit) = value {
                                if let Lit::Str(lit_str) = &expr_lit.lit {
                                    let level = lit_str.value();
                                    if level != "debug" && level != "info" {
                                        return Err(syn::Error::new_spanned(
                                            lit_str,
                                            "level must be either \"debug\" or \"info\"",
                                        ));
                                    }
                                    config.level = Some(level);
                                }
                            }
                        }
                        "threshold" => {
                            if let Expr::Lit(expr_lit) = value {
                                if let Lit::Str(lit_str) = &expr_lit.lit {
                                    config.threshold = Some(lit_str.value());
                                }
                            }
                        }
                        _ => {
                            return Err(syn::Error::new_spanned(
                                ident,
                                format!("Unknown attribute: {}", ident),
                            ));
                        }
                    }
                }
                _ => {
                    return Err(syn::Error::new_spanned(meta, "Expected name-value pair"));
                }
            }
        }

        Ok(config)
    }
}

/// Parse threshold string like "10ms", "1s", "500us" into Duration expression
fn parse_threshold(threshold_str: &str) -> syn::Result<proc_macro2::TokenStream> {
    let threshold_str = threshold_str.trim();

    let (value_str, unit) = if let Some(stripped) = threshold_str.strip_suffix("ms") {
        (stripped, "millis")
    } else if let Some(stripped) = threshold_str
        .strip_suffix("us")
        .or_else(|| threshold_str.strip_suffix("µs"))
    {
        (stripped, "micros")
    } else if let Some(stripped) = threshold_str.strip_suffix("ns") {
        (stripped, "nanos")
    } else if let Some(stripped) = threshold_str.strip_suffix('s') {
        (stripped, "secs")
    } else {
        return Err(syn::Error::new(
            proc_macro2::Span::call_site(),
            "Threshold must end with 'ms', 'us', 'ns', or 's'",
        ));
    };

    let value: u64 = value_str.parse().map_err(|_| {
        syn::Error::new(
            proc_macro2::Span::call_site(),
            format!("Invalid threshold value: {}", value_str),
        )
    })?;

    let duration = match unit {
        "secs" => quote! { std::time::Duration::from_secs(#value) },
        "millis" => quote! { std::time::Duration::from_millis(#value) },
        "micros" => quote! { std::time::Duration::from_micros(#value) },
        "nanos" => quote! { std::time::Duration::from_nanos(#value) },
        _ => unreachable!(),
    };

    Ok(duration)
}

/// Attribute macro for measuring function execution time
///
/// # Arguments
///
/// * `name` - Custom name for the function in logs (default: function name)
/// * `level` - Log level: "debug" (uses eprintln!) or "info" (uses println!) (default: "info")
/// * `threshold` - Minimum duration to log (e.g., "10ms", "1s") (default: always log)
///
/// # Examples
///
/// ```rust
/// use hemera::hemera;
///
/// #[hemera]
/// fn example() {
///     // Function body
/// }
///
/// #[hemera(name = "MyFunction", level = "debug", threshold = "10ms")]
/// fn example_with_options() {
///     // Function body
/// }
///
/// #[hemera]
/// async fn async_example() {
///     // Async function body
/// }
/// ```
#[proc_macro_attribute]
pub fn hemera(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);

    let config = if attr.is_empty() {
        HemeraConfig {
            name: None,
            level: None,
            threshold: None,
        }
    } else {
        match syn::parse::<HemeraConfig>(attr) {
            Ok(config) => config,
            Err(e) => return e.to_compile_error().into(),
        }
    };

    let ItemFn {
        attrs,
        vis,
        sig,
        block,
    } = input;

    let fn_name = &sig.ident;
    let display_name = config.name.unwrap_or_else(|| fn_name.to_string());

    let use_debug = matches!(config.level.as_deref(), Some("debug"));

    let threshold_check = if let Some(threshold_str) = config.threshold {
        match parse_threshold(&threshold_str) {
            Ok(duration) => quote! {
                if __hemera_elapsed >= #duration
            },
            Err(e) => return e.to_compile_error().into(),
        }
    } else {
        quote! { if true }
    };

    let is_async = sig.asyncness.is_some();

    let print_stmt = if use_debug {
        quote! {
            eprintln!("⏱ Function `{}` executed in {:.3?}", #display_name, __hemera_elapsed);
        }
    } else {
        quote! {
            println!("⏱ Function `{}` executed in {:.3?}", #display_name, __hemera_elapsed);
        }
    };

    let timing_code = if is_async {
        quote! {
            let __hemera_start = std::time::Instant::now();
            let __hemera_result = async move { #block }.await;
            let __hemera_elapsed = __hemera_start.elapsed();
            #threshold_check {
                #print_stmt
            }
            __hemera_result
        }
    } else {
        quote! {
            let __hemera_start = std::time::Instant::now();
            let __hemera_result = (|| #block)();
            let __hemera_elapsed = __hemera_start.elapsed();
            #threshold_check {
                #print_stmt
            }
            __hemera_result
        }
    };

    let tracing_wrapper = if cfg!(feature = "tracing") {
        quote! {
            let __hemera_span = tracing::info_span!("hemera", function = #display_name);
            let __hemera_enter = __hemera_span.enter();
        }
    } else {
        quote! {}
    };

    let new_block = quote! {
        {
            #tracing_wrapper
            #timing_code
        }
    };

    let output = quote! {
        #(#attrs)*
        #vis #sig {
            #new_block
        }
    };

    output.into()
}
