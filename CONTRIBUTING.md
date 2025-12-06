# Contributing to DiskOfflaner

Thank you for your interest in contributing to DiskOfflaner! This document provides guidelines for contributing to the project.

## Code of Conduct

Be respectful, professional, and constructive in all interactions.

## Development Setup

1. **Install Rust**: Get the latest stable Rust toolchain from [rustup.rs](https://rustup.rs/)
2. **Clone the repository**:
   ```bash
   git clone https://github.com/appsjuragan/diskOfflaner-rust.git
   cd diskOfflaner-rust
   ```
3. **Build the project**:
   ```bash
   cargo build
   ```

## Code Quality Standards

### Formatting
All code must be formatted with `rustfmt`:
```bash
cargo fmt --all
```

### Linting
Code must pass `clippy` without warnings:
```bash
cargo clippy --all-targets --all-features -- -D warnings
```

### Testing
Run tests before submitting:
```bash
cargo test --all
```

### Building Release
Verify release builds:
```bash
cargo build --release
```

## Coding Guidelines

1. **Documentation**: Add doc comments for public APIs
2. **Error Handling**: Use `Result` and `anyhow` for error propagation
3. **Safety**: Minimize unsafe code; document when necessary
4. **Cross-platform**: Test on both Windows and Linux when possible
5. **Performance**: Consider binary size and runtime performance
6. **Naming**: Use clear, descriptive names following Rust conventions

## Commit Messages

Use clear, descriptive commit messages:
- Use present tense ("Add feature" not "Added feature")
- First line should be concise (50 chars or less)
- Provide detailed explanation if needed after blank line

Example:
```
Add Linux partition mounting support

- Implement mount/unmount operations for Linux
- Add device path parsing for various Linux disk types
- Update GUI to handle Linux-specific disk identifiers
```

## Pull Request Process

1. Create a feature branch from `main` or `linux-support` (for Linux-specific changes)
2. Make your changes following the guidelines above
3. Ensure all tests pass and code is properly formatted/linted
4. Update CHANGELOG.md with your changes
5. Submit a pull request with a clear description

## Platform-Specific Development

### Windows Development
- Requires Windows 10/11
- Uses WinAPI for disk operations
- Test with both HDD and SSD drives

### Linux Development
- Test on multiple distributions when possible
- Use platform-agnostic paths and conventions
- Verify with different disk naming schemes (sda, nvme0n1, etc.)

## Questions or Issues?

Open an issue on GitHub for:
- Bug reports
- Feature requests
- Documentation improvements
- General questions

## License

By contributing, you agree that your contributions will be licensed under the MIT License.
