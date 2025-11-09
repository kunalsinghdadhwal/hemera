# Hemera Quick Reference

## Installation
```toml
[dependencies]
hemera = "0.1"

# With tracing support
hemera = { version = "0.1", features = ["tracing"] }
```

## Basic Usage

### Simple Timing
```rust
use hemera::hemera;

#[hemera]
fn my_function() {
    // Your code
}
// Output: ⏱ Function `my_function` executed in 123.456µs
```

### Async Functions
```rust
#[hemera]
async fn fetch_data() -> String {
    // Async code
}
```

## Attributes

| Attribute | Values | Default | Example |
|-----------|--------|---------|---------|
| `name` | String | Function name | `name = "CustomLabel"` |
| `level` | `"info"` \| `"debug"` | `"info"` | `level = "debug"` |
| `threshold` | Duration string | None (always log) | `threshold = "10ms"` |

### Duration Units
- `s` - seconds (e.g., `"1s"`)
- `ms` - milliseconds (e.g., `"100ms"`)
- `us` or `µs` - microseconds (e.g., `"500us"`)
- `ns` - nanoseconds (e.g., `"1000ns"`)

## Common Patterns

### Debug Logging
```rust
#[hemera(level = "debug")]
fn debug_operation() {
    // Uses eprintln! instead of println!
}
```

### Threshold-based Logging
```rust
#[hemera(threshold = "100ms")]
fn maybe_slow() {
    // Only logs if execution > 100ms
}
```

### Custom Name
```rust
#[hemera(name = "DatabaseQuery")]
fn query_users() {
    // Shows as "DatabaseQuery" in logs
}
```

### All Options Combined
```rust
#[hemera(
    name = "CriticalOperation",
    level = "debug",
    threshold = "50ms"
)]
async fn critical_task() -> Result<(), Error> {
    // Custom name, debug level, 50ms threshold
}
```

## Examples

### Sync Function
```rust
#[hemera(threshold = "1ms")]
fn process_data(data: &[u8]) -> Vec<u8> {
    data.iter().map(|&x| x * 2).collect()
}
```

### Async Function
```rust
#[hemera(name = "API_Call")]
async fn call_api() -> Result<Response, Error> {
    reqwest::get("https://api.example.com").await
}
```

### Generic Function
```rust
#[hemera]
fn transform<T: Clone>(value: T) -> T {
    value.clone()
}
```

## Output Format
```
⏱ Function `function_name` executed in 123.456ms
```

Where:
- Function name can be overridden with `name` attribute
- Duration is formatted with 3 decimal places
- Unit is automatically chosen (ns, µs, ms, s)

## Performance
- **Overhead**: ~36 nanoseconds per instrumented call
- **Zero cost** when threshold prevents logging

## Commands
```bash
# Run examples
cargo run --example basic
cargo run --example async_example

# Run benchmarks (generates HTML reports)
cargo bench

# View benchmark results
open target/criterion/report/index.html

# Run tests
cargo test --all-features
```

## Tips
1. Use `level = "debug"` for development, remove for production
2. Set thresholds to reduce noise in high-frequency functions
3. Combine with `tracing` feature for structured logging
4. Use custom names for better log readability
5. Test with examples before deploying

## Error Handling
- Macro preserves all function signatures
- Return values are properly forwarded
- Panics are handled gracefully
- Works with `Result`, `Option`, and all return types

## Feature Flags
- `tracing` - Enables tracing spans integration

---
For more details, see the [full README](README.md)
