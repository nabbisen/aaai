# Releasing aaai

This document describes how to publish a new release of aaai to crates.io.

## Prerequisites

- You are a crates.io owner of `aaai`, `aaai-cli`, and `aaai-gui`.
- Your local working tree is clean and all tests pass (`cargo test --workspace`).
- The version number in `Cargo.toml` (workspace root) matches the release you intend to publish.

## Publish order

The three crates must be published **in order**, because `aaai-cli` and `aaai-gui`
both depend on `aaai`. Publishing out of order will fail because crates.io
resolves version dependencies against its own index.

```
aaai → aaai-cli → aaai-gui
```

## Steps

### 1. Verify packaging

```sh
# Dry-run for each crate — inspect the file list and any warnings
cargo package -p aaai
cargo package -p aaai-cli  --no-verify
cargo package -p aaai-gui  --no-verify
```

`--no-verify` is required for `aaai-cli` and `aaai-gui` when running locally,
because their `aaai` path dependency does not yet exist on crates.io.
Once `aaai` has been published and indexed (see step 3), the flag is no
longer needed for subsequent re-runs, but it does no harm.

### 2. Publish aaai

```sh
cargo publish -p aaai
```

### 3. Wait for crates.io indexing

crates.io typically indexes a new crate within 1–3 minutes. You can confirm
indexing by checking `https://crates.io/crates/aaai` or running:

```sh
cargo search aaai
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
- **`aaai-core` on crates.io is an orphaned name**, last published at 0.39.0
  before the engine crate was renamed to `aaai`. Do not publish to it and do not
  yank it — existing 0.39.0 users resolve against it. This document referred to
  it throughout until 2026-07-31.
- **The tag carries a `v` prefix; the crates.io version does not.** Tag
  `v0.41.0` for workspace version `0.41.0`. Only `v`-prefixed tags match
  `release.yaml`'s trigger, so a tag without it publishes to crates.io but
  produces no GitHub Release and no binaries. Every tag before `v0.41.0` omitted
  the prefix, which is why `release.yaml` had never run.
- **Step 6 requires a `## [<version>]` section in `CHANGELOG.md`.**
  `release.yaml` extracts the release notes from it and now **fails the release**
  if that section is missing or empty, rather than publishing a blank one.
  Write the changelog entry before tagging.
