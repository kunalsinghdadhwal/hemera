# Hemera

[![Crates.io](https://img.shields.io/crates/v/hemera.svg)](https://crates.io/crates/hemera)
[![Documentation](https://docs.rs/hemera/badge.svg)](https://docs.rs/hemera)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://github.com/kunalsinghdadhwal/hemera/blob/main/LICENSE)
[![CI](https://github.com/kunalsinghdadhwal/hemera/workflows/CI/badge.svg)](https://github.com/kunalsinghdadhwal/hemera/actions)

**Inevitable timing for Rust functions measure execution with divine precision.**

Hemera is a lightweight, zero-overhead procedural macro for measuring function execution time in Rust. Named after the Greek primordial goddess of the day, Hemera brings clarity and insight to your code's performance characteristics.

## Features

- **Zero-cost abstraction**: Minimal runtime overhead (~36 nanoseconds)
- **Sync & Async support**: Works seamlessly with both synchronous and asynchronous functions
- **Flexible logging**: Choose between `println!` and `eprintln!`
- **Conditional logging**: Set thresholds to only log slow executions
- **Custom labels**: Override function names in logs
- **Tracing integration**: Optional support for the `tracing` ecosystem
- **Generics support**: Works with generic functions and lifetimes
- **Easy to use**: Just add `#[measure_time]` to any function

## Installation

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

## Usage

### Basic Example

```rust
use hemera::measure_time;

#[measure_time]
fn calculate_fibonacci(n: u32) -> u32 {
    if n <= 1 {
        n
    } else {
        calculate_fibonacci(n - 1) + calculate_fibonacci(n - 2)
    }
}

fn main() {
    let result = calculate_fibonacci(10);
    // Output: [TIMING] Function 'calculate_fibonacci' executed in 23.456µs
}
```

### Async Functions

```rust
use hemera::measure_time;

#[measure_time]
async fn fetch_data() -> Result<String, Box<dyn std::error::Error>> {
    // Your async code here
    Ok("data".to_string())
}

#[tokio::main]
async fn main() {
    let data = fetch_data().await.unwrap();
    // Output: [TIMING] Function 'fetch_data' executed in 1.234ms
}
```

### Custom Name

```rust
#[measure_time(name = "DatabaseQuery")]
fn query_users() -> Vec<User> {
    // Your code here
}
// Output: [TIMING] Function 'DatabaseQuery' executed in 45.678ms
```

### Debug Level Logging

Use `eprintln!` instead of `println!`:

```rust
#[measure_time(level = "debug")]
fn debug_operation() {
    // Your code here
}
```

### Threshold-based Logging

Only log if execution time exceeds a threshold:

```rust
#[measure_time(threshold = "10ms")]
fn maybe_slow_operation() {
    // Only logs if this takes more than 10ms
}
```

Supported units: `s` (seconds), `ms` (milliseconds), `us` (microseconds), `ns` (nanoseconds)

### Combine All Options

```rust
#[measure_time(name = "CriticalOperation", level = "debug", threshold = "100ms")]
async fn critical_operation() -> Result<(), Error> {
    // Your code here
}
```

### Generics Support

Hemera works seamlessly with generic functions:

```rust
#[measure_time]
fn process<T: Clone>(value: T) -> T {
    value.clone()
}

#[measure_time(name = "GenericAsync")]
async fn async_process<T: Send>(value: T) -> T {
    value
}
```

## Configuration Options

| Attribute | Type | Default | Description |
|-----------|------|---------|-------------|
| `name` | `String` | Function name | Custom label for the function in logs |
| `level` | `"info"` \| `"debug"` | `"info"` | Log level (`info` uses `println!`, `debug` uses `eprintln!`) |
| `threshold` | `String` | None | Minimum duration to log (e.g., `"10ms"`, `"1s"`) |

## Feature Flags

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

## Roadmap

| Feature | Status |
|---------|--------|
| Sync functions | Complete |
| Async functions | Complete |
| Threshold filtering | Complete |
| Custom naming | Complete |
| Debug/Info levels | Complete |
| Tracing integration | Complete |
| Block-level measurement | Planned |
| Statistical aggregation | Planned |
| Custom output formatters | Planned |

## Examples

Check out the [examples](examples/) directory for more usage examples:

- [`basic.rs`](examples/basic.rs) - Synchronous function examples
- [`async_example.rs`](examples/async_example.rs) - Async function examples

Run examples with:

```bash
cargo run --example basic
cargo run --example async_example
```

## Benchmarks

Run benchmarks to measure the macro's overhead:

```bash
cargo bench
```

Benchmark results are saved as HTML reports in `target/criterion/`. Open `target/criterion/report/index.html` in your browser to view detailed performance analysis with charts and statistics.

## Testing

Run the test suite:

```bash
# Run all tests
cargo test --all-features

# Run tests without features
cargo test --no-default-features

# Run with tracing feature
cargo test --features tracing
```

---

**Author:** [Kunal Singh Dadhwal](https://github.com/kunalsinghdadhwal)
