# RFC 112 — Finish the `aaai-core` rename and the DEC-003 layout

**Status.** **Accepted** — approved for implementation by the owner
2026-10-18. Lives in `rfcs/accepted/`; the folder is the source of truth
for this state (RFC 000).

**Tracks.** Conformance debt from two decisions already made — DEC-002 (the
v0.39.0 crate rename) and DEC-003 (Rust 2024 module layout). No new behaviour,
no milestone of its own. Contributes to **C1** (whole workflow green) by
removing one of the three reasons `ci.yaml` is parked.

**Depends.** Nothing. Can land at any time; §7 argues for soon-but-not-urgent.

**Design owner.** high-capability model

**Decision owner.** nabbisen, project owner

**Proposed implementer.** mid-capability model, after approval

**Evidence location.** `.git-exclude/evidence/112-rename-and-layout-conformance/`

**Touches.** `crates/aaai/src/lib.rs`, `crates/aaai-gui/src/util.rs`,
`crates/aaai-cli/src/cmd/mod.rs` (deleted), `crates/aaai-cli/src/cmd.rs`,
`crates/aaai-cli/src/cmd/next_hint.rs`, a new
`crates/aaai-cli/src/cmd/next_hint/tests.rs`, and
`.github/workflows/ci.yaml`. No engine behaviour, no public API, no
persisted format, no dependency.

**Handoff.** Required, after acceptance.

---

## 1. Summary

Two decisions were made and neither was finished.

**DEC-002** renamed the engine crate `aaai-core` → `aaai` in v0.39.0. Sixteen
months later the crate's own rustdoc front page still says `# aaai-core`, and
CI still builds a crate by that name.

**DEC-003** mandates the Rust 2024 module layout — no `mod.rs`, inline tests
extracted to `foo/tests.rs`. RFC 100 enforced it thoroughly, but its acceptance
items were scoped to `crates/aaai-gui/src/`, so `aaai-cli` was never swept and
still has one of each violation.

Neither is a defect in anything shipped. Both are places where the repository
contradicts its own written decisions, which is the kind of drift that makes
the next reader doubt the rest.

## 2. The findings

Each verified on `2a1019b`.

### 2.1 The engine crate's published documentation is titled with a retired name

`crates/aaai/src/lib.rs:1` and `:12`:

```rust
//! # aaai-core
//!
//! Core engine for **aaai** (audit for asset integrity).
…
//! ## Module map
//!
//! ```text
//! aaai-core
```

**This is the docs.rs landing page for the `aaai` crate.** A reader arriving at
<https://docs.rs/aaai> is told the crate is called something else. `aaai-core`
on crates.io is an orphaned name last published at 0.39.0, which
`RELEASING.md:91` correctly documents and instructs nobody to publish to.

**Severity: low impact, high visibility.** Nothing breaks; it is simply wrong
in the single most-read place the crate has.

### 2.2 `aaai-cli` has a `mod.rs`

`crates/aaai-cli/src/cmd/mod.rs`, 17 lines, entirely `pub mod` declarations.

DEC-003 and `project-instructions-rust.md` both forbid it: *"A `foo.rs` and
`foo/` subdirectory may coexist; `mod.rs` is no longer needed."* The fix is to
rename it to `crates/aaai-cli/src/cmd.rs`. Nothing else changes — the module
path `crate::cmd::*` is identical either way.

### 2.3 `aaai-cli` has an inline test module

`crates/aaai-cli/src/cmd/next_hint.rs:65-132` — `#[cfg(test)] mod tests {` with
a body, in a 132-line file.

`project-instructions-rust.md` marks this shape explicitly bad and names the
remedy: `src/some_mod.rs` plus `src/some_mod/tests.rs`. RFC 100's T2 and T8 did
exactly this seven times in `aaai-gui`; this is the eighth, in the crate those
RFCs did not cover.

**It is the only one left.** A sweep of all three crates finds no other `mod.rs`
and no other inline test module.

### 2.4 CI builds a crate that does not exist

`.github/workflows/ci.yaml:47`, `:50`, `:84`:

```yaml
run: cargo build -p aaai-core -p aaai-cli -p aaai-gui
run: cargo test -p aaai-core --lib
- run: cargo check -p aaai-core -p aaai-cli -p aaai-gui
```

This is **RISK-002**, open since v0.39.0 and named in the v0.40.0 handoff
bundle as a "one-line-per-occurrence fix (3 spots)". It is still three spots.

### 2.5 One stale mention that is correct to leave

`crates/aaai-gui/src/util.rs:94` says *"`aaai-core` continues to expose only
`AuditStrategy`"*. The sentence is about the engine crate and should name
`aaai`, so it is in scope — but note that `RELEASING.md:91` and `CHANGELOG.md`
mention `aaai-core` **deliberately**, documenting the orphaned name and the
rename respectively. **Those two must not be swept.** A blind
find-and-replace across the repository would damage both.

## 3. Goals and non-goals

### 3.1 Goals

1. No file in `crates/` names `aaai-core` except where it deliberately
   documents the retired name.
2. `crates/aaai-cli/src/` conforms to DEC-003: no `mod.rs`, no inline test
   module.
3. `ci.yaml`'s three stale `-p aaai-core` arguments name `aaai`.

### 3.2 Non-goals

- **Unparking `ci.yaml`.** This removes one of its three stated blockers; the
  clippy debt remains and the parking decision is the owner's. See §6.
- Any other dead-code or lint sweep.
- Renaming anything on crates.io, or publishing to `aaai-core`.
- Touching `RELEASING.md:91` or `CHANGELOG.md` — §2.5.
- Restructuring `aaai-cli` beyond the two layout fixes.

## 4. Selected design

**Four mechanical changes, no judgement required at implementation time.**

| # | Change | File |
|---|---|---|
| 1 | `# aaai-core` → `# aaai`; module-map header likewise | `crates/aaai/src/lib.rs:1`, `:12` |
| 2 | `` `aaai-core` `` → `` `aaai` `` in the doc comment | `crates/aaai-gui/src/util.rs:94` |
| 3 | `git mv cmd/mod.rs cmd.rs` | `crates/aaai-cli/src/` |
| 4 | Extract the inline test module to a sibling | `crates/aaai-cli/src/cmd/next_hint.rs` → `next_hint/tests.rs` |
| 5 | `-p aaai-core` → `-p aaai`, three occurrences | `.github/workflows/ci.yaml:47`, `:50`, `:84` |

Change 4 follows RFC 100's T2/T8 pattern exactly: the body moves dedented by
one level, `#[cfg(test)] mod tests;` is left behind, and the extracted content
must be **dedent-identical** to what it replaced.

## 5. Acceptance contract

1. `grep -rn "aaai-core" crates/` returns nothing.
2. `grep -rn "aaai-core" .github/` returns **only** the comment at
   `ci.yaml:6`, which narrates the parking history and is prose about the past,
   not a build argument. Leaving it is correct; rewriting it would make the
   header describe a state that never existed.
3. `find crates -name mod.rs` returns nothing.
4. `grep -rn "^mod tests {" crates/*/src/` returns nothing.
5. The extracted test body is **dedent-identical** to the module it replaced,
   demonstrated the way RFC 111's T8 demonstrated it.
6. `cargo +1.91 test --workspace --locked` — **every count unchanged**:
   **146 / 13 / 97 / 27 / 3**. Re-measure on current `main` first; if any count
   moves, stop and report.
7. `cargo +1.91 fmt --check` clean.
8. `cargo +1.91 clippy -p aaai-cli --all-targets --no-deps` introduces no new
   finding. **Not** `-- -D warnings`: the workspace carries pre-existing
   findings and that flag fails on all of them.
9. No diff outside `crates/` and `.github/workflows/ci.yaml`.

## 6. What this does and does not do for CI

`ci.yaml` was parked under RFC 102 §5.4 for three stated reasons. Their state
on `2a1019b`:

| Reason | Now |
|---|---|
| `cargo fmt --check` debt | **Resolved** by RFC 107 — workspace clean |
| Open advisories in `audit-security` | **Resolved** 2026-09-17 — `cargo audit` exits 0 |
| Jobs naming `aaai-core` | **This RFC** |

A fourth blocker the header never listed: `clippy --all-targets -- -D warnings`
fails on findings across both crates. That is the C2 lint debt and is not
touched here.

**So this RFC leaves exactly one blocker standing.** It does not unpark
anything — that remains the owner's decision, and the `audit-security` job is
independently restartable without waiting for the lint work, as recorded
2026-09-17.

## 7. Timing

**Not urgent, and worth doing before RFC 101.** The owner's assessment was "not
urgent" for the `mod.rs`; §2.1 and §2.4 are equally low-impact. But the whole
change is under an hour, needs no design decisions, and every month it waits is
a month the crate's front page carries the wrong name.

**Suggested placement: alongside the C2 lint RFC**, which is the only other
item standing between `ci.yaml` and a green run. Doing them together means one
review and one matrix run for the whole of C1's remaining CI surface. If that
RFC is far off, this one should not wait for it.

## 8. Risks

| Risk | Mitigation |
|---|---|
| A blind sweep damages the two deliberate `aaai-core` mentions | §2.5 names them; acceptance items 1 and 2 are scoped to `crates/` and `.github/` and deliberately exclude `RELEASING.md` and `CHANGELOG.md` |
| The test extraction changes behaviour | Item 5's dedent-identity check, the same one RFC 111 used |
| A count moves | Item 6 makes that a stop-and-report, not something to fix in passing |
| Renaming `cmd/mod.rs` breaks a path | It cannot — `crate::cmd::*` resolves identically for `cmd.rs` and `cmd/mod.rs`. The compiler is the check |

## 9. Sources

- DEC-002, DEC-003 — `.git-exclude/specs/aaai-handoffs-files-260630/decision-log.md`
- `.git-exclude/rules/project-instructions-rust.md` — module style and test layout
- RISK-002 — `.git-exclude/specs/aaai-handoffs-files-260630/project-summary.md` §5
- RFC 100 §6 items 1–2 (scoped to `aaai-gui`), RFC 111 T8 (the extraction pattern)
- `RELEASING.md:91` — why the orphaned name is documented deliberately
