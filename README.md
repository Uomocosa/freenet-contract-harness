# freenet-contract-harness

Test harness that exercises the universal Freenet contract suite (CRDT laws,
four-function wiring, rejection safety, Broken-flag liveness) against a
contract's state/merge.

## Environment

```sh
rustup toolchain install nightly    # rust-toolchain.toml pins channel + components
cargo install cargo-nextest
cargo install just
cargo install --git https://github.com/Uomocosa/lele-rust-linter lele-lint
```

Pure Rust — no system libraries.

## Tasks

```sh
just build | just clippy | just fmt | just test | just lint
just hooks      # point git at .githooks/
just ci
```
