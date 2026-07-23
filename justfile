set shell := ["sh", "-cu"]

# Run the simulator and launch the rerun viewer.
run:
    cargo run -p sim

# Run all workspace tests.
test:
    cargo test --workspace --all-targets

# Run all project validation checks.
check:
    cargo fmt --all --check
    cargo check --workspace --all-targets
    cargo clippy --workspace --all-targets -- -D clippy::pedantic -D clippy::nursery
