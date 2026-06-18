# Releasing aaai

This document describes how to publish a new release of aaai to crates.io.

## Prerequisites

- You are a crates.io owner of `aaai-core`, `aaai-cli`, and `aaai-gui`.
- Your local working tree is clean and all tests pass (`cargo test --workspace`).
- The version number in `Cargo.toml` (workspace root) matches the release you intend to publish.

## Publish order

The three crates must be published **in order**, because `aaai-cli` and `aaai-gui`
both depend on `aaai-core`. Publishing out of order will fail because crates.io
resolves version dependencies against its own index.

```
aaai-core → aaai-cli → aaai-gui
```

## Steps

### 1. Verify packaging

```sh
# Dry-run for each crate — inspect the file list and any warnings
cargo package -p aaai-core
cargo package -p aaai-cli  --no-verify
cargo package -p aaai-gui  --no-verify
```

`--no-verify` is required for `aaai-cli` and `aaai-gui` when running locally,
because their `aaai-core` path dependency does not yet exist on crates.io.
Once `aaai-core` has been published and indexed (see step 3), the flag is no
longer needed for subsequent re-runs, but it does no harm.

### 2. Publish aaai-core

```sh
cargo publish -p aaai-core
```

### 3. Wait for crates.io indexing

crates.io typically indexes a new crate within 1–3 minutes. You can confirm
indexing by checking `https://crates.io/crates/aaai-core` or running:

```sh
cargo search aaai-core
```

Do not proceed to step 4 until the new version appears in the index.

### 4. Publish aaai-cli

```sh
cargo publish -p aaai-cli
```

### 5. Publish aaai-gui

```sh
cargo publish -p aaai-gui
```

### 6. Tag the release

```sh
git tag v$(grep '^version' Cargo.toml | head -1 | grep -oP '"\K[^"]+')
git push origin --tags
```

This triggers the GitHub Actions release workflow, which builds binaries for
Linux, macOS, and Windows and attaches them to a GitHub Release.

## Version bumping

Use `scripts/bump-version.sh` to update the version across all workspace
`Cargo.toml` files atomically. Direct edits to `Cargo.toml` risk truncation on
some platforms — the script uses `sed -i` to avoid this.

```sh
./scripts/bump-version.sh 0.32.0
```

## Notes

- Never publish or tag `v1.0.0` without explicit confirmation from the project maintainer.
- The `aaai-gui` crate is published for documentation completeness but is not
  intended to be used as a library dependency.
