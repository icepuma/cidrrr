# Default recipe runs check
default: check

# Format all code
format:
    cargo fmt --all

# Verify formatting without changing files
format-check:
    cargo fmt --all --check

# Run clippy on all code
lint:
    cargo clippy --all-targets --all-features -- -D warnings -W clippy::pedantic -W clippy::nursery

# Run all tests
test:
    cargo test --all-features --all-targets

# Run all tests with nextest
test-nextest:
    cargo nextest run --all-features --all-targets

# Run complete check (non-mutating)
check: format-check lint test
