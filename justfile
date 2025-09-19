# Run all CI checks (equivalent to ubuntu-build job)
ci: info check fmt clippy build-debug test-debug doc build-release test-release
    @echo "✅ All CI checks passed!"

# Run quick checks only (faster feedback loop)
quick: check fmt clippy
    @echo "✅ Quick checks passed!"

# Show system and toolchain info
info:
    @echo "System info:"
    @uname -a
    @echo
    @echo "Rust toolchain versions:"
    @rustup show
    @rustc --version
    @cargo --version
    @rustfmt --version || true
    @cargo clippy --version || true
    @echo

# Cargo check
check:
    cargo check --all --all-targets

# Check formatting (CI mode - fails if not formatted)
fmt:
    cargo fmt --all -- --check

# Fix formatting
fmt-fix:
    cargo fmt --all

# Run clippy
clippy:
    cargo clippy --all-targets --all-features

# Build debug
build-debug:
    cargo build --all --all-targets

# Run tests (debug)
test-debug:
    cargo test --all --all-targets

# Generate documentation
doc:
    cargo doc --no-deps --document-private-items

# Build release
build-release:
    cargo build --release --all-targets

# Run tests (release)
test-release:
    cargo test --release --all --all-targets

# macOS-specific tests (if you're on macOS)
macos-test:
    cargo test --all --all-targets
    cargo build --release --all-targets

# Clean all build artifacts
clean:
    cargo clean

# Watch mode - run quick checks on file changes (requires cargo-watch)
watch:
    cargo watch -x "check --all --all-targets" -x "test --all --all-targets"

# Install development dependencies
setup:
    rustup component add rustfmt clippy
    cargo install cargo-watch