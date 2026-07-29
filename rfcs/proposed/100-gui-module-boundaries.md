# RFC 100 — GUI Module Boundaries

**Status.** Proposed

**Tracks.** `ROADMAP.md` MG2 / WS-15 / gate V2

**Depends.** RFC 099 / V1 (sequencing only — no technical dependency)

**Design owner.** requirements architect

**Decision owner.** nabbisen, project owner

**Proposed implementer.** GUI developer (mid-capability model), after design
review by the high-capability model and explicit owner approval

**Environment boundary.** Local development plus the hosted B0 matrix for test
counts. No display required — this RFC changes no rendering.

**Evidence location.** `.git-exclude/evidence/100-gui-module-boundaries/`

**Touches.** `crates/aaai-gui/src/` file layout only. No behaviour, engine,
CLI, persisted format, public API, dependency, workflow, or i18n change.

**Handoff.** Required:
[`rfcs/handoffs/100-gui-module-boundaries/README.md`](../handoffs/100-gui-module-boundaries/README.md)

## 1. Summary

`crates/aaai-gui/src/app.rs` is **2,524 lines / 1,935 ELOC** — roughly four
times the project's own "split strongly recommended" threshold — and carries an
inline `#[cfg(test)] mod tests`, which the project's rules mark ❌ Bad and
DEC-003 forbids. `views/mod.rs` likewise contradicts DEC-003's no-`mod.rs`
rule.

This RFC corrects both. It is behaviour-neutral: no test may change name, body,
or count.

Its purpose is capacity. RFC 101's guided flow adds a screen family, its
messages, and its state — all of which land in `app.rs`. Adding them to a
1,935-ELOC file with inline tests would make that work unreviewable.

## 2. Problem

| Item | Observed | Rule |
|---|---|---|
| `app.rs` size | 2,524 lines / **1,935 ELOC** | `project-instructions-rust-gui.md`: consider splitting > 300 ELOC; **strongly recommended > 500** |
| `app.rs` tests | inline `#[cfg(test)] mod tests` at `:2395` | ❌ Bad per the same rules; DEC-003 requires `foo/tests.rs` |
| `views/mod.rs` | 9 lines of `pub mod` declarations | DEC-003: Rust 2024 module system, **no `mod.rs`** |
| Sibling convention | `ignore.rs` + `ignore/`, `progress.rs` + `progress/` | `app.rs` and `views/` are the module's only outliers |

`ROADMAP.md` "Milestone review rules" item 6 requires a boundary assessment for
any workstream touching a file above 500 ELOC and forbids increasing
concentration without explicit rationale. RFC 101 cannot satisfy that while
`app.rs` stands as it is.

## 3. Observed structure and the split it implies

`app.rs` already has clean internal seams:

| Lines | Content | Destination |
|---|---|---|
| 41–193 | `PaneKind`, `DiffViewMode`, `FocusTarget`, `OpeningValidation`, `Screen`, `FilterMode`, `BatchApproveState`, `FieldError`, `InspectorValidation`, `InspectorState` | `app/state.rs` |
| 194–391 | `struct App` and `impl Default for App` | stays in `app.rs` |
| 392–589 | `enum Message` (~200 lines) | `app/message.rs` |
| 590–2381 | `impl App` — the update loop, ~1,790 lines | `app/update/*.rs`, split by message family |
| 2382–2394 | `dnd_sub()` | `app/subscription.rs` |
| 2395–2524 | inline `mod tests` | `app/tests.rs` |

The update loop is the only judgement-bearing part. It splits by message family
— opening/folder selection, audit run and rerun, inspector editing, save and
persistence, navigation and focus, dialogs and toasts — with `impl App` blocks
distributed across those files. Rust permits multiple `impl App` blocks in one
crate, so no type moves and no visibility widens.

## 4. Goals and non-goals

### 4.1 Goals

- No file in `crates/aaai-gui/src/` exceeds 500 ELOC, or exceeds it with a
  recorded rationale.
- No inline `#[cfg(test)]` module remains; tests live in `foo/tests.rs`.
- No `mod.rs` remains anywhere in the crate.
- Behaviour, message protocol, i18n keys, and every test name and count are
  unchanged.

### 4.2 Non-goals

- Any rendering, layout, or visual change — RFC 099 owns that.
- Any guided-flow work — RFC 101.
- Changing `Message` variants, state shape, or the update loop's logic.
- Widening visibility beyond what the split mechanically requires.
- Splitting `views/*.rs` files; only `views/mod.rs` → `views.rs` is in scope.

## 5. Selected design

1. `views/mod.rs` → `views.rs`, content unchanged, matching `ignore.rs` and
   `progress.rs`.
2. Extract `app.rs`'s inline tests to `app/tests.rs`, opening with
   `use super::*;` exactly as `ignore/tests.rs` does.
3. Extract state types, `Message`, and `dnd_sub` per §3.
4. Split `impl App` by message family into `app/update/`.
5. `app.rs` retains the `App` struct, its `Default`, and the module
   declarations.

Each step is independently compilable and independently revertible.

## 6. Acceptance contract — gate V2

1. No `mod.rs` under `crates/aaai-gui/src/`.
2. No `#[cfg(test)]` module inline in any `crates/aaai-gui/src/**/*.rs`
   alongside implementation.
3. Every file under `crates/aaai-gui/src/` is ≤ 500 ELOC, or is listed with a
   rationale in the evidence package.
4. `cargo +1.91 test --workspace --locked` — **every count unchanged**:
   aaai 144, CLI unit 8, CLI integration 91, GUI 27 (26 + RFC 099's contrast
   test), doctests 3.
5. Every test name is unchanged — verified by diffing sorted `--list` output
   before and after.
6. `cargo +1.91 clippy -p aaai-gui --all-targets -- -D warnings` passes.
7. No diff outside `crates/aaai-gui/src/`.
8. No `Message` variant added, removed, or renamed.

## 7. Risks and mitigations

| Risk | Mitigation |
|---|---|
| A behavioural change slips into a "mechanical" move | Test names and counts must be byte-identical; `--list` diff is a required artifact |
| Visibility creep — items made `pub` to satisfy the split | Prefer `pub(crate)` or `pub(super)`; every widening is listed in the evidence with a reason |
| The update-loop split fragments related logic | Split by message family, not by line count; a family stays whole even if its file is larger |
| Merge conflict with RFC 099 | Sequence V1 before V2; RFC 099 touches `views/*.rs` bodies, this touches file layout |
| Reviewer cannot audit a large move | One concern per commit, in the §5 order |

## 8. Implementation sequence

1. Independent design review accepts this RFC and its handoff.
2. Owner approves implementation.
3. Developer executes the handoff in the §5 order, one commit per step.
4. Local verification, including the test-name diff.
5. Independent implementation review.
6. Owner integrates; hosted B0 confirms counts on all three platforms.

## 9. Compatibility

None affected. No public API, persisted format, CLI surface, or i18n key
changes. The crate's external behaviour is identical.

## 10. Alternatives considered

| Option | Decision |
|---|---|
| Split `app.rs` by message family, extract tests, drop `mod.rs` | **Selected.** Follows the crate's own sibling convention and DEC-003. |
| Leave `app.rs` and add guided-flow code to it | Rejected: would push it past ~2,500 ELOC and violate ROADMAP item 6 during RFC 101. |
| Split by line count into arbitrary parts | Rejected: fragments related logic and makes review harder, not easier. |
| Defer until RFC 101 and split as part of it | Rejected: mixes a mechanical move with a design change in one unreviewable diff. |
| Extract tests only, leave the size | Rejected: ~1,825 production ELOC still blocks item 6. |

## 11. Review questions

1. Is the §3 message-family split the right seam, or is another decomposition
   better suited to the guided flow that follows?
2. Is "≤ 500 ELOC or a recorded rationale" the right bar, given the rules say
   *consider* at 300?
3. Is the test-name diff sufficient proof of behaviour neutrality?
4. Should `views/*.rs` files above 500 ELOC — `main_view.rs` at 744 — be split
   here, or left to RFC 101 which will rework them?

## 12. Sources

- `.git-exclude/reviewed/037-gui-uiux-gap-analysis-2026-07-28.md` §6
- `.git-exclude/reviewed/038-gui-remediation-roadmap-and-milestones-2026-07-28.md`
- `.git-exclude/rules/project-instructions-rust-gui.md` — ELOC and test-layout rules
- DEC-003 — Rust 2024 modules, no `mod.rs`, tests in `foo/tests.rs`
- `ROADMAP.md` — "Milestone review rules" item 6
