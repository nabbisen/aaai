# RFC 112 — Rename and layout conformance: implementer handoff

**RFC.** [`rfcs/done/112-rename-and-layout-conformance.md`](../../done/112-rename-and-layout-conformance.md)

**Status.** **Accepted by the owner 2026-10-18.** Ready to implement.

**Baseline.** `main` at `838a07b`. **Re-measure before relying on any line
number here** — see §6.

**Evidence location.** `.git-exclude/evidence/112-rename-and-layout-conformance/`

| Role | Who | Scope |
|---|---|---|
| Design owner | high-capability model | The RFC and this handoff |
| Implementer | GUI/CLI developer | T1–T3 |
| Reviewer | high-capability model | On request |
| Integrator | nabbisen | Commit, push, observe CI |

---

## 1. What this is

**Five mechanical edits across four files.** No behaviour changes, no public
API, no persisted format, no dependency. The whole change is under an hour.

Two decisions were made and never finished: **DEC-002** renamed the engine
crate in v0.39.0, and **DEC-003** mandates the Rust 2024 module layout. This
closes the places each one missed.

**This is the easiest RFC on the board.** The risk is not difficulty — it is
sweeping too widely. §2 is the part to read carefully.

## 2. Read this first: two `aaai-core` mentions must survive

A blind find-and-replace across the repository is the one way to get this
wrong.

| File | Mention | Action |
|---|---|---|
| `RELEASING.md:91` | documents `aaai-core` as an **orphaned crates.io name** nobody should publish to | **Leave.** Removing it loses the warning |
| `CHANGELOG.md` | records the v0.39.0 rename | **Leave.** It is history |
| `.github/workflows/ci.yaml:6` | a comment narrating why the workflow is parked, mentioning the stale name as one reason | **Leave.** It describes the past; rewriting it would make the header describe a state that never existed |

Everything else named in §3 is in scope. The acceptance greps in §5 are scoped
to `crates/` and `.github/` deliberately, so they do not trip on the first two.

## 3. Tasks

### T1 — Finish the rename (commit 1)

| File | Change |
|---|---|
| `crates/aaai/src/lib.rs:1` | `//! # aaai-core` → `//! # aaai` |
| `crates/aaai/src/lib.rs:12` | the module-map header `//! aaai-core` → `//! aaai` |
| `crates/aaai-gui/src/util.rs:94` | `` `aaai-core` continues to expose `` → `` `aaai` continues to expose `` |
| `.github/workflows/ci.yaml:47` | `-p aaai-core` → `-p aaai` |
| `.github/workflows/ci.yaml:50` | `-p aaai-core` → `-p aaai` |
| `.github/workflows/ci.yaml:84` | `-p aaai-core` → `-p aaai` |

`crates/aaai/src/lib.rs:1` is the **docs.rs landing page** for the published
`aaai` crate. That is why a cosmetic doc-comment edit is worth an RFC.

### T2 — `cmd/mod.rs` → `cmd.rs` (commit 1 or 2)

```sh
git mv crates/aaai-cli/src/cmd/mod.rs crates/aaai-cli/src/cmd.rs
```

Seventeen lines of `pub mod` declarations. **Nothing else changes** — the
module path `crate::cmd::*` resolves identically either way, and the compiler
is the proof. If anything fails to build, stop and report; it would mean
something depends on the layout in a way this handoff did not anticipate.

### T3 — Extract the inline test module (commit 1 or 2)

`crates/aaai-cli/src/cmd/next_hint.rs:65` onward is `#[cfg(test)] mod tests {`
with a body, in a 132-line file. Extract to
`crates/aaai-cli/src/cmd/next_hint/tests.rs`, leaving
`#[cfg(test)]\nmod tests;` behind.

**Same operation as RFC 111's T8**, which did exactly this three times. Same
discipline: the extracted body must be **dedent-identical** to the module it
replaced, verified by a dedent-and-compare rather than by eye.

**This is the last one in the workspace.** After it,
`find crates -name mod.rs` and `grep -rn "^mod tests {" crates/*/src/` both
return nothing.

## 4. What this does not do

**It does not unpark `ci.yaml`.** RFC 102 §5.4 parked it for three reasons;
two are already resolved — rustfmt by RFC 107, advisories on 2026-09-17 — and
this clears the third. A fourth, never listed in the header, remains:
`clippy --all-targets -- -D warnings` fails on findings in both crates. That
is C2 lint debt and is **out of scope here**.

So expect `ci.yaml` to remain parked after this lands. Unparking is the
owner's decision, and the `audit-security` job is independently restartable
without waiting for the lint work.

**Also out of scope:** any other dead-code or lint sweep; anything on
crates.io; restructuring `aaai-cli` further.

## 5. Acceptance

RFC §5 is the contract. Four notes:

**Items 1–2** — `grep -rn "aaai-core" crates/` returns nothing;
`grep -rn "aaai-core" .github/` returns **only** `ci.yaml:6`. Do not widen
these greps past `crates/` and `.github/` — §2 says why.

**Item 5** — the dedent-identity demonstration for T3. Show it, do not assert
it.

**Item 6** — counts must stay **146 / 13 / 97 / 27 / 3**. Re-measure on
current `main` first. If any count moves, **stop and report**: a moved count
after a pure file move means something was not a pure file move.

**Item 8** — `clippy -p aaai-cli --all-targets --no-deps`, **not**
`-- -D warnings`. The crate carries pre-existing findings and that flag fails
on all of them. The check is that no *new* finding appears.

## 6. A standing check this project has learned four times

**Any line number, ELOC figure, or test count in this document is stale until
you re-measure it on current `main`.**

RFC 100 shipped with three acceptance items written wider than its own scope;
RFC 111's removal table needed regenerating after RFC 100 moved everything;
RFC 104's §2 quoted a block from a file that no longer held it; RFC 109's
tables predicted a migration that had been overtaken by the time it ran.
Re-measure first; if a figure is wrong, report it rather than working around
it.

## 7. When you are done

Package a review request as usual, entry point stated in chat. Then Integrator
pushes and B0 runs.

This RFC is independent of the GUI sequence — **104 → 106 → 110 → 101** — and
can land before, after, or between any of them.
