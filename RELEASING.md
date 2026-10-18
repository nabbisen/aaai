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
git tag $(grep '^version' Cargo.toml | head -1 | grep -oP '"\K[^"]+')
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
- **The tag carries no `v` prefix.** Tag `0.43.0` for workspace version
  `0.43.0`, matching the crates.io version exactly and
  `project-instructions-rust.md`'s "Release tag format". `release.yaml` and
  `windows-msix.yaml` both trigger on `[0-9]+.[0-9]+.[0-9]+`, so a `v`-prefixed
  tag now publishes to crates.io but produces **no GitHub Release and no
  binaries**.

  **Changed 2026-10-18.** `v0.41.0` and `v0.42.0` are the only two prefixed
  tags; every tag before them and every tag after them is unprefixed. They are
  published releases and stay as they are — retagging would break their
  download URLs. Two consequences of the change:
  - The **GitHub Release title is now just `0.43.0`**, not `aaai v0.43.0`.
  - **Asset filenames lose the `v`** — `aaai-cli-0.43.0-<target>.tar.gz` rather
    than `aaai-cli-v0.42.0-<target>.tar.gz`. They are built from the tag name.
    This is consistent with DEC-008, which requires a version suffix and does
    not mandate a prefix.
- **Step 6 requires a `## [<version>]` section in `CHANGELOG.md`.**
  `release.yaml` extracts the release notes from it and now **fails the release**
  if that section is missing or empty, rather than publishing a blank one.
  Write the changelog entry before tagging.
