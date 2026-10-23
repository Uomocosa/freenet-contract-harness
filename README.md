# freenet-contract-harness

Test harness that exercises the universal Freenet contract suite (CRDT laws,
four-function wiring, rejection safety, Broken-flag liveness) against a
contract's state/merge.

## Dependencies and releases

This crate depends on no sibling repository; it is consumed by
`freenet-libp2p-bevy-plugin`'s contract test harness at a release tag.

A git content filter (`lele-pins`) rewrites those pins to sibling `path =`
dependencies in your working tree, so an edit in a dependency is picked up by the
next `cargo build` here with no release and no command. Install it once per
machine:

    cd ../lele-release && just setup

Never hand-edit a pin. From the fleet root, `just release <patch|minor|major>`
bumps every changed repo, re-pins its dependents, tags `vX.Y.Z` and pushes; see
`lele-release/README.md` for `status`, `--only`/`--exclude` and `--dry-run`. A
plain clone without the filter resolves the committed tags, which is exactly what
CI sees.

## Environment

```sh
rustup toolchain install nightly    # rust-toolchain.toml pins channel + components
cargo install cargo-nextest
cargo install just
cargo install --git https://github.com/Uomocosa/lele-rust-linter lele-lint
cargo binstall prek                 # hook runner (`prek install` via `just setup`)
```

Pure Rust — no system libraries.

## Tasks

```sh
just build | just clippy | just fmt | just test | just lint
just setup    # one-time per clone: install the prek git hooks
just check    # run the full gate (same as CI)
just ci
```
