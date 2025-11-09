# Contributing to Hemera

Thank you for your interest in contributing to Hemera! We welcome contributions from everyone.

## Getting Started

1. Fork the repository
2. Clone your fork: `git clone https://github.com/kunalsinghdadhwal/hemera.git`
3. Create a new branch: `git checkout -b feature/my-feature`
4. Make your changes
5. Run tests: `cargo test --all-features`
6. Run formatting: `cargo fmt`
7. Run clippy: `cargo clippy --all-features`
8. Commit your changes: `git commit -m 'Add some feature'`
9. Push to the branch: `git push origin feature/my-feature`
10. Open a Pull Request

## Development Setup

```bash
# Clone the repository
git clone https://github.com/kunalsinghdadhwal/hemera.git
cd hemera

# Build the project
cargo build

# Run tests
cargo test --all-features

# Run examples
cd examples
cargo run --example basic
cargo run --example async_example
cd ..

# Run benchmarks
cargo bench
```

## Code Style

- Follow the [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Use `cargo fmt` for formatting
- Use `cargo clippy` and fix all warnings
- Write documentation for public APIs
- Add tests for new features

## Testing

- Add unit tests for new functionality
- Ensure all tests pass: `cargo test --all-features`
- Test both with and without features: `cargo test --no-default-features`

## Code of Conduct

Please be respectful and constructive in all interactions.

## Questions?

Feel free to open an issue for questions or discussions!
