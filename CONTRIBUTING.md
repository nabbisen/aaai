# Contributing to aaai

This document describes formatting policy for the workspace. See
[`RELEASING.md`](RELEASING.md) for the release process.

## Formatting

The project uses **rustfmt with default settings**. There is deliberately
**no `rustfmt.toml`** — defaults, so there is nothing to maintain or argue
about.

Run before committing:

```sh
cargo fmt --all
```

`cargo +1.91 fmt --check --all` is part of every RFC's verification command
list and must exit 0.

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
