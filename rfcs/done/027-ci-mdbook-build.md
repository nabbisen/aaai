# RFC 027 — CI mdbook build job

**Status.** Implemented (v0.21.0 — Phase 13)
**Tracks.** Continuous Integration, documentation hygiene
**Touches.** `.github/workflows/ci.yaml` (new job), CHANGELOG.

## Summary

Add a CI job that runs `mdbook build` against both `docs/` (English)
and `docs/ja/` (Japanese). The job fails the workflow if either
book has unresolved cross-references, missing SUMMARY entries,
malformed markdown, or encoding issues.

During Phase 12 the docs surface was reshaped substantially — six
chapters were rewritten from Japanese-on-English-path to proper
English, two new chapters were added (`compatibility.md`,
`abdd-audit.md`), and many cross-references were threaded between
them. The `mdbook build` smoke test caught a real orphan
(`abdd-audit.md` was missing from both `SUMMARY.md` files). Without
CI enforcement, future drift will silently re-introduce the same
class of problem.

## Why this matters

The current `ci.yaml` has jobs for:
- Workspace `cargo check` (linux / macos / windows)
- `cargo test`
- `cargo fmt --check`
- MSRV (Rust 1.91)
- i18n key audit
- Visual-verification status (informational)

There is **no automated check** that the documentation builds.
A contributor who renames a chapter or adds a broken cross-link
will only discover the breakage when someone manually runs
`mdbook build` locally — or when the rendered site goes up and
404s start appearing.

Catching this in CI is cheap (mdbook installs in seconds and a
build completes in well under a minute) and prevents a small
class of footguns from reaching `main`.

## External design

A new top-level job, named to fit the existing pattern (`check-msrv`,
`i18n-key-audit`, `visual-verification-status`):

```yaml
docs-build:
  name: mdbook build (en + ja)
  runs-on: ubuntu-latest
  steps:
    - uses: actions/checkout@v6
    - name: Install mdbook
      run: cargo install mdbook --version "^0.4" --no-default-features
    - name: Build English book
      run: mdbook build docs/
    - name: Build Japanese book
      run: mdbook build docs/ja/
```

The job is **blocking** (fails the workflow on error) because the
checks it performs are non-trivial to recover from after merge:
broken cross-references decay readability of the rendered site,
and silent SUMMARY drift is exactly the kind of latent bug
Phase 12 surfaced.

### Triggering conditions

The job runs on the same triggers as the other CI jobs (push to
`main`, pull requests targeting `main`). There's no path filter
because:

1. The docs and the SUMMARY are intimately tied to the
   surrounding `docs/src/`, `docs/ja/src/` trees — a code change
   that adds a new public API and the corresponding doc update
   should be checked together.
2. The build is fast (a few seconds after the mdbook install).
3. Path filters add their own complexity (the "everyone forgets
   to update the filter when adding a docs directory" failure
   mode).

If the cost becomes a problem, we revisit with a path filter as
a follow-up.

## Internal design

### mdbook version

Pinned to `^0.4` (currently 0.4.52). The pin avoids surprise
breakage from a hypothetical 0.5 that could change rendering or
CLI surface. Bumping the pin is a separate decision callable out
in CHANGELOG.

### `--no-default-features`

mdbook's default features include `search` (which pulls in a
search-index generator and inflates the install time
significantly). The book authors don't currently rely on search
in CI — operators inspect the rendered HTML manually — so
disabling it is the right trade.

If we later want to ship a fully-featured rendered site (e.g.
on GitHub Pages with working search), that's a separate concern
that lands in the release/publish workflow, not the CI gate.

### Caching

The mdbook binary is small (~9 MB) and `cargo install` is fast.
A cache layer (e.g. `Swatinem/rust-cache`) would shave a few
seconds, but the complexity-to-savings ratio doesn't justify it
for v1. The job runs in under a minute end-to-end without
caching.

## Testing

The job itself is the test. The build either succeeds (good) or
fails (bad). There's no auxiliary test surface to add.

Locally, the same check is `~/.cargo/bin/mdbook build docs/` and
`mdbook build docs/ja/` — both should complete cleanly.

## Acceptance criteria

- [ ] `.github/workflows/ci.yaml` has a new `docs-build` job
- [ ] The job installs mdbook with `--no-default-features` and
      version pin `^0.4`
- [ ] The job runs `mdbook build docs/` and `mdbook build docs/ja/`
- [ ] The job is part of the default workflow (no `if:` guard)
- [ ] Both `docs/release-prep-v0.20.0.md` Step 0 and the new CI
      job perform the same smoke test
- [ ] CHANGELOG entry under `[Unreleased]` records the addition

## Open questions

None at acceptance. Possible refinements (rendered-site publish
to GitHub Pages, link-check integration via lychee, etc.) belong
to separate follow-up RFCs.
