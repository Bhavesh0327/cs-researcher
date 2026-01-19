# Contributing to CS Researcher

We welcome contributions! Whether it's fixing bugs, adding new discovery layers, or improving documentation.

## How to Contribute

1. **Fork the Repository**: Create your own copy of the project.
2. **Create a Branch**: `git checkout -b feature/your-feature-name`
3. **Make Changes**: Implement your changes and ensure the code compiles.
4. **Test Your Changes**: Run `cargo run` and verify the output.
5. **Submit a Pull Request**: Describe your changes and why they are useful.

## Development Guidelines

- **Style**: Follow standard Rust idioms. Use `cargo fmt` before committing.
- **Layers**: If adding a new API, implement it in `src/layers/discovery.rs` and extend the `PaperMetadata` struct if necessary.
- **Fuzzy Matching**: Any new matching logic should be integrated into the `resolution` layer.

## Reporting Issues
Please use the GitHub Issue tracker to report bugs or suggest features.
