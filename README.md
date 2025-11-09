# ⏱ Hemera

[![Crates.io](https://img.shields.io/crates/v/hemera.svg)](https://crates.io/crates/hemera)
[![Documentation](https://docs.rs/hemera/badge.svg)](https://docs.rs/hemera)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![CI](https://github.com/kunalsinghdadhwal/hemera/workflows/CI/badge.svg)](https://github.com/kunalsinghdadhwal/hemera/actions)

**Inevitable timing for Rust functions—measure execution with divine precision.** ✨

Hemera is a lightweight, zero-cost procedural macro for measuring function execution time in Rust. Named after the Greek primordial goddess of the day, Hemera brings illumination to your code's performance characteristics.

## 🚀 Features

- **Zero-cost abstraction**: Minimal runtime overhead
- **Sync & Async support**: Works seamlessly with both synchronous and asynchronous functions
- **Flexible logging**: Choose between `println!` and `eprintln!`
- **Conditional logging**: Set thresholds to only log slow executions
- **Custom labels**: Override function names in logs
- **Tracing integration**: Optional support for the `tracing` ecosystem
- **Generics support**: Works with generic functions and lifetimes
- **Easy to use**: Just add `#[hemera]` to any function

## 📦 Installation

Add Hemera to your `Cargo.toml`:

```bash
cargo add hemera
```

Or manually:

```toml
[dependencies]
hemera = "0.1"
```

For tracing integration:

```toml
[dependencies]
hemera = { version = "0.1", features = ["tracing"] }
```

## 🎯 Usage

### Basic Example

```rust
use hemera::hemera;

#[hemera]
fn calculate_fibonacci(n: u32) -> u32 {
    if n <= 1 {
        n
    } else {
        calculate_fibonacci(n - 1) + calculate_fibonacci(n - 2)
    }
}

fn main() {
    let result = calculate_fibonacci(10);
    // Output: ⏱ Function `calculate_fibonacci` executed in 23.456µs
}
```

### Async Functions

```rust
use hemera::hemera;

#[hemera]
async fn fetch_data() -> Result<String, Box<dyn std::error::Error>> {
    // Your async code here
    Ok("data".to_string())
}

#[tokio::main]
async fn main() {
    let data = fetch_data().await.unwrap();
    // Output: ⏱ Function `fetch_data` executed in 1.234ms
}
```

### Custom Name

```rust
#[hemera(name = "DatabaseQuery")]
fn query_users() -> Vec<User> {
    // Your code here
}
// Output: ⏱ Function `DatabaseQuery` executed in 45.678ms
```

### Debug Level Logging

Use `eprintln!` instead of `println!`:

```rust
#[hemera(level = "debug")]
fn debug_operation() {
    // Your code here
}
```

### Threshold-based Logging

Only log if execution time exceeds a threshold:

```rust
#[hemera(threshold = "10ms")]
fn maybe_slow_operation() {
    // Only logs if this takes more than 10ms
}
```

Supported units: `s` (seconds), `ms` (milliseconds), `us` (microseconds), `ns` (nanoseconds)

### Combine All Options

```rust
#[hemera(name = "CriticalOperation", level = "debug", threshold = "100ms")]
async fn critical_operation() -> Result<(), Error> {
    // Your code here
}
```

### Generics Support

Hemera works seamlessly with generic functions:

```rust
#[hemera]
fn process<T: Clone>(value: T) -> T {
    value.clone()
}

#[hemera(name = "GenericAsync")]
async fn async_process<T: Send>(value: T) -> T {
    value
}
```

## 🔧 Configuration Options

| Attribute | Type | Default | Description |
|-----------|------|---------|-------------|
| `name` | `String` | Function name | Custom label for the function in logs |
| `level` | `"info"` \| `"debug"` | `"info"` | Log level (`info` uses `println!`, `debug` uses `eprintln!`) |
| `threshold` | `String` | None | Minimum duration to log (e.g., `"10ms"`, `"1s"`) |

## 🌟 Feature Flags

| Feature | Description |
|---------|-------------|
| `tracing` | Enable integration with the `tracing` crate |

### Using with Tracing

```toml
[dependencies]
hemera = { version = "0.1", features = ["tracing"] }
tracing = "0.1"
```

When the `tracing` feature is enabled, Hemera automatically creates tracing spans around your functions.

## 📊 Roadmap

| Feature | Status |
|---------|--------|
| ✅ Sync functions | Complete |
| ✅ Async functions | Complete |
| ✅ Threshold filtering | Complete |
| ✅ Custom naming | Complete |
| ✅ Debug/Info levels | Complete |
| ✅ Tracing integration | Complete |
| ⏳ Block-level measurement | Planned |
| ⏳ Statistical aggregation | Planned |
| ⏳ Custom output formatters | Planned |

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## 📝 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- Named after Hemera, the Greek primordial goddess of the day
- Inspired by the need for simple, efficient timing in Rust
- Built with love using `syn`, `quote`, and `proc-macro2`

## 📚 Examples

Check out the [examples](examples/) directory for more usage examples:

- [`basic.rs`](examples/basic.rs) - Synchronous function examples
- [`async_example.rs`](examples/async_example.rs) - Async function examples

Run examples with:

```bash
cargo run --example basic
cargo run --example async_example
```

## 🏃 Benchmarks

Run benchmarks to measure the macro's overhead:

```bash
cargo bench
```

Benchmark results are saved as HTML reports in `target/criterion/`. Open `target/criterion/report/index.html` in your browser to view detailed performance analysis with charts and statistics.

---

Made with ⏱ by [Kunal Singh Dadhwal](https://github.com/kunalsinghdadhwal)
