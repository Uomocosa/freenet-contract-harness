set shell := ["bash", "-eu", "-o", "pipefail", "-c"]

# Toolchain: rust-toolchain.toml. Extra tools: cargo-nextest, just, and
# `lele-lint` (`cargo install --git https://github.com/Uomocosa/lele-rust-linter lele-lint`).

build:
    cargo build --all-targets --all-features

clippy:
    cargo clippy --all-targets --all-features -- -D warnings

fmt:
    cargo fmt -- --check

test:
    cargo nextest run --all-targets --all-features

lint:
    lele-lint .

hooks:
    git config core.hooksPath .githooks
    chmod +x .githooks/*

ci: build fmt clippy test
