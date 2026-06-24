# v0.20.0 Release-Prep Checklist

> **When to use.** All Phase 12 implementation work has landed; operator
> visual verification is complete (RFC 017 cards filled, ABDD sheet
> filled, RFCs 020/021/022/023 visually confirmed). This document walks
> the remaining mechanical steps to cut v0.20.0 as a release tarball.
>
> **What this is not.** A general release process. v0.20.0 has a
> specific, finite set of mechanical steps because Phase 12 work was
> already committed and verified beforehand. Future releases will
> follow a similar shape but the exact list will differ.
>
> **Estimated time.** 30–60 minutes assuming no surprises.

## Pre-flight checks

Before starting the cut-over, confirm operator visual verification is
genuinely done:

```sh
# Should output: 0 unverified
./scripts/list-unverified-rfcs.sh
```

If that prints anything other than `0 unverified` (or just exits 0 on
the strict path), **stop** — verification work remains.

```sh
# Should also be clean
./scripts/check-i18n-keys.py --quiet
```

Should report `0 missing, 0 divergent, 0 unused`.

```sh
# All three crates green
cargo check --workspace --all-targets
cargo test -p aaai --lib
cargo test -p aaai-cli --bin aaai -- --test-threads=1
```

aaai should report 97/97, aaai-cli 70/70.

```sh
# mdbook smoke test
mdbook build docs/
mdbook build docs/ja/
```

Both should complete with no errors.

If anything above fails, stop and resolve. The mechanical steps below
assume a clean baseline.

---

## Step 1 — Version bump

```sh
# Edit Cargo.toml workspace.package.version
sed -i 's/version = "0\.19\.0"/version = "0.20.0"/' Cargo.toml
```

Verify:

```sh
grep '^version' Cargo.toml
# Expected: version = "0.20.0"
```

The three crates inherit via `version.workspace = true`, so no
per-crate edits are needed.

```sh
cargo check --workspace
# Should still build cleanly.
```

---

## Step 2 — Promote CHANGELOG `[Unreleased]` → `[0.20.0]`

Open `CHANGELOG.md`. Find the line:

```
## [Unreleased]
```

Replace with:

```
## [0.20.0] — YYYY-MM-DD
```

substituting the actual ISO date. Then add a new empty `[Unreleased]`
section ABOVE the promoted one for future work:

```markdown
## [Unreleased]

## [0.20.0] — 2026-05-XX

… (your existing content) …
```

Verify the structure with:

```sh
grep -n '^## \[' CHANGELOG.md | head -5
# Should show: Unreleased first, then 0.20.0, then 0.19.x older
```

---

## Step 3 — Move RFCs from `proposed/` to `done/`

Phase 12 introduced RFCs 017 through 025. After visual verification
they all move to `done/`:

```sh
cd rfcs/
for n in 017 018 019 020 021 022 023 024 025; do
  mv proposed/$n-*.md done/
done
cd -
```

Verify:

```sh
ls rfcs/proposed/
# Expected: empty (or only RFCs that genuinely are still in proposal)
ls rfcs/done/ | grep -E '^(017|018|019|020|021|022|023|024|025)' | wc -l
# Expected: 9
```

---

## Step 4 — Update Status field in each moved RFC

Each RFC's front-matter has a `Status:` field. For RFCs landing in
v0.20.0, change it from `Proposed` (or whatever it currently says) to
`Implemented (v0.20.0)`. For RFCs that landed only partially with
known deferrals (e.g. RFC 021), use a more nuanced phrasing:

| RFC | Suggested Status |
|---|---|
| 017 | `Implemented (v0.20.0)` |
| 018 | `Implemented partial (v0.20.0) — §3.4 only; B/C deferred` |
| 019 | `Implemented (v0.20.0)` |
| 020 | `Implemented (v0.20.0)` |
| 021 | `Implemented partial (v0.20.0) — Save/Report marks; audit-dirty banner deferred pending arch decoupling` |
| 022 | `Implemented (v0.20.0)` |
| 023 | `Implemented (v0.20.0)` |
| 024 | `Implemented (v0.20.0)` |
| 025 | `Implemented partial (v0.20.0) — docs groundwork only; full release prep at v1.0.0` |

A grep+sed pattern can do this in bulk, but reviewing each by hand
is safer because of the partial-implementation cases above.

Verify with the helper:

```sh
./scripts/list-unverified-rfcs.sh
# Should still output 0 unverified — Status updates don't add
# verification cards, but they shouldn't remove them either.
```

---

## Step 5 — Update `rfcs/README.md` index

Move each of the 9 RFCs from the "Proposed" table (or section) to the
"Implemented" table. Sort the "Implemented" table by RFC number.

The relative path in the markdown links changes from
`./proposed/NNN-slug.md` to `./done/NNN-slug.md`.

Verify:

```sh
grep -c '\./proposed/' rfcs/README.md
# Expected: 0 (after the moves)
```

---

## Step 6 — Cargo workspace publish dry-run

```sh
# Dry-run publish each crate in dependency order
cargo publish --dry-run -p aaai
cargo publish --dry-run -p aaai-cli
cargo publish --dry-run -p aaai-gui
```

Each should report `Verifying`, then complete cleanly. Watch for any
file-inclusion warnings (e.g. files in `target/` accidentally
included). The actual publish is **not** part of this checklist —
publishing happens via the release workflow on push of the v0.20.0
git tag, or manually if that workflow doesn't exist yet.

---

## Step 7 — Build the release tarball

Per the project's release plan rules, the tarball is the cargo
workspace structure with the version-suffixed filename:

```sh
# From the workspace root
cd ..
tar --exclude='target' --exclude='.git' \
    -czvf aaai-v0.20.0.tar.gz aaai/
```

Verify:

```sh
tar -tzf aaai-v0.20.0.tar.gz | head -5
# Should list: aaai/, aaai/Cargo.toml, aaai/CHANGELOG.md, …

tar -tzf aaai-v0.20.0.tar.gz | grep -c 'target/'
# Expected: 0

ls -lh aaai-v0.20.0.tar.gz
# Sanity-check size — should be a few MB, not hundreds
```

---

## Step 8 — Smoke-test the tarball

Extract to a fresh directory and verify it builds standalone:

```sh
mkdir -p /tmp/aaai-smoke
cd /tmp/aaai-smoke
tar -xzf /path/to/aaai-v0.20.0.tar.gz
cd aaai

cargo check --workspace --all-targets
cargo test -p aaai --lib
cargo test -p aaai-cli --bin aaai -- --test-threads=1
./scripts/check-i18n-keys.py --quiet
mdbook build docs/
mdbook build docs/ja/
```

All six checks should pass against the tarball contents. If they
don't, something was excluded that shouldn't have been (or included
that shouldn't have been).

---

## Step 9 — Git tag and (optional) GitHub Release

```sh
git tag -a v0.20.0 -m "v0.20.0 — Phase 12: UI/UX refresh + accessibility + docs"
git push origin v0.20.0
```

If a release workflow runs on tag push, this triggers crates.io
publish and GitHub Release creation. If not, manually create the
GitHub Release through the UI, attaching the `aaai-v0.20.0.tar.gz`
asset.

---

## Step 10 — Update ROADMAP.md to mark Phase 12 complete

In `ROADMAP.md`, change Phase 12's status from "in progress" to
"complete" (or whatever convention the file uses), and verify Phase
13 / 14 / 15 / 16 are correctly described as future.

---

## What's next after v0.20.0

The next release cycle is Phase 13 (per `rfcs/PLAN.md`). The Phase 12
deferrals carry forward:

- **RFC 018 main work (B/C)** — only if RFC 016 visual verification
  surfaced residual literal-key issues. If RFC 016 verification
  passed, this work can be archived as not-needed.
- **RFC 021 audit-dirty banner** — picks up once a future RFC
  decouples definition mutation from synchronous re-run, making the
  banner visually meaningful.
- **`FieldError` / toast subtitle refactor** — to carry `hint`
  separately, enabling the `error.inspector.invalid_regex.*` and
  `error.save.failed.*` i18n keys to come back. Removed during
  the Phase 12 dead-key sweep; re-add when the refactor lands.

These are not blockers for v0.20.0; they're follow-up work to be
filed as fresh RFCs in their own phase.
