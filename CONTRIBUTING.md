# Contributing to aaai

This document describes formatting policy for the workspace. See
[`RELEASING.md`](RELEASING.md) for the release process.

## Formatting

The project uses **rustfmt with default settings**. There is deliberately
**no `rustfmt.toml`** — defaults, so there is nothing to maintain or argue
about.

Run before committing:

```sh
cargo +1.91 fmt --all
```

`cargo +1.91 fmt --check --all` is part of every RFC's verification command
list and must exit 0.

**Use `+1.91` for both.** The pin is not decoration: this repository has no
`rust-toolchain.toml`, so a bare `cargo fmt` uses whatever toolchain is
ambient. rustfmt's output changes between releases, and formatting with one
version while the check runs another is how a tree ends up permanently red.
They happen to agree today — 1.9.0-stable and 1.8.0-stable produce identical
output on this codebase — but that is luck, not design.

### The hand-alignment convention is retired

Before RFC 107 (2026-08-10), the project hand-aligned `=` signs and trailing
comments to a column:

```rust
let after  = tempfile::tempdir().unwrap();          // aligned '='
format!("after content {i}\n")   // modified           // aligned comment
```

rustfmt cannot be configured to preserve this — its configuration surface has
no alignment option. Do not reintroduce aligned `=` or aligned trailing
comments; let `cargo fmt --all` format the line normally.
