# RFC 100 — GUI Module Boundaries

**Status.** Proposed — **revised 2026-08-10** after design review
`.git-exclude/reviewed/068-rfc100-gui-module-boundaries-design-review-2026-08-10.md`
returned Needs changes / No-Go. Three blocking findings corrected: the update
loop's split mechanism (§3.1), stale figures throughout, and the review method
(§6a). **Awaiting re-review before implementation.**

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

`crates/aaai-gui/src/app.rs` is **2,757 lines / 2,168 ELOC** — more than four
times the project's own "split strongly recommended" threshold — and carries an
inline `#[cfg(test)] mod tests`, which the project's rules mark ❌ Bad and
DEC-003 forbids. `views/mod.rs` likewise contradicts DEC-003's no-`mod.rs`
rule.

> **Figures regenerated 2026-08-10** against `b981f19`, after RFC 107's
> repository-wide rustfmt adoption. Every number in the original draft predated
> it and was low by roughly 10% — `app.rs` alone grew 233 lines. The §3 table is
> the implementation map, so stale line numbers there would have sent an
> implementer cutting in the wrong places.

This RFC corrects both. It is behaviour-neutral: no test may change name, body,
or count.

Its purpose is capacity. RFC 101's guided flow adds a screen family, its
messages, and its state — all of which land in `app.rs`. Adding them to a
2,168-ELOC file with inline tests would make that work unreviewable.

## 2. Problem

| Item | Observed | Rule |
|---|---|---|
| `app.rs` size | 2,757 lines / **2,168 ELOC** | `project-instructions-rust-gui.md`: consider splitting > 300 ELOC; **strongly recommended > 500** |
| `app.rs` tests | inline `#[cfg(test)] mod tests` at `:2615` | ❌ Bad per the same rules; DEC-003 requires `foo/tests.rs` |
| `views/mod.rs` | 9 lines of `pub mod` declarations | DEC-003: Rust 2024 module system, **no `mod.rs`** |
| Sibling convention | `ignore.rs` + `ignore/`, `progress.rs` + `progress/` | `app.rs` and `views/` are the module's only outliers |

`ROADMAP.md` "Milestone review rules" item 6 requires a boundary assessment for
any workstream touching a file above 500 ELOC and forbids increasing
concentration without explicit rationale. RFC 101 cannot satisfy that while
`app.rs` stands as it is.

## 3. Observed structure and the split it implies

`app.rs` already has clean internal seams. Measured on `b981f19`:

| Lines | Content | Destination |
|---|---|---|
| 38–201 | `PaneKind`, `DiffViewMode`, `FocusTarget`, `OpeningValidation`, `Screen`, `FilterMode`, `BatchApproveState`, `FieldError`, `InspectorValidation`, `InspectorState` | `app/state.rs` |
| 202–399 | `struct App` and `impl Default for App` | stays in `app.rs` |
| 400–597 | `enum Message` — **103 variants** | `app/message.rs` |
| 598–2601 | `impl App` — the update loop | §3.1 |
| 2602–2614 | `dnd_sub()` | `app/subscription.rs` |
| 2615–end | inline `mod tests` | `app/tests.rs` |

Everything except the update loop is a file move.

### 3.1 The update loop needs two phases, not one

**`update()` is a single `match msg` with 107 inline arm bodies.** A `match`
expression cannot be split across files, so no amount of "distributing `impl
App` blocks" reaches it. That distribution works on **methods**; the arms are
not methods yet.

> **Corrected 2026-08-10.** This section previously said the loop splits "with
> `impl App` blocks distributed across those files … so no type moves and no
> visibility widens", describing a file move. The work is a restructure. An
> implementer would have discovered that only after starting.

| Phase | Work | Reviewable because |
|---|---|---|
| **A** | Extract each of the 107 arm bodies into a named `impl App` method. `update()` becomes a thin dispatcher. **No file is created or moved.** | The diff stays in one file, so each extraction can be checked against the arm it replaced |
| **B** | Move those methods into `app/update/*.rs` grouped by family. **No body changes.** | Pure relocation, and provably so once A is verified |

**They must not be combined.** In a single diff a reviewer cannot distinguish a
moved body from a modified one, which is precisely the failure §7's first risk
row names.

### 3.2 The family seam already exists in the source

`update()` carries **24 section comments** marking message families — `Opening`,
`File tree`, `Inspector`, `Approve`, `Batch`, `Re-run / save / report`,
`navigation`, `profiles`, `Locale`, `navigation guard`, `keyboard help overlay`,
`Settings dialog`, `Overlays`, `Toasts`, and others.

Phase B groups these into roughly six files. **The seam is not a judgement the
implementer has to invent** — it is already recorded in the code by whoever
wrote it, which is what makes phase B mechanical rather than architectural.

### 3.3 Four recursive `self.update()` calls

`app.rs:862–863` (`ApproveEntry` then `SaveDefinition`) and `:1347` / `:1376`
(`SelectEntry`). After phase B these become **cross-family** calls.

Not an obstacle — a dispatcher handles them — but §7's mitigation *"a family
stays whole"* addresses fragmentation **within** a family and says nothing about
recursion **between** families. Named here so they are not discovered mid-move.

## 4. Goals and non-goals

### 4.1 Goals

- **No file outside `views/` in `crates/aaai-gui/src/` exceeds 500 ELOC.**
  `views/*.rs` is deferred to RFC 101 and exempt here — see below.
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

> **The views exemption, stated honestly (added 2026-08-10).** Three
> `views/*.rs` files exceed the 500-ELOC bar on `b981f19`:
> `main_view.rs` **831**, `inspector.rs` **757**, `diff_view.rs` **650**
> (`opening.rs` is 491, just under).
>
> The goal originally read *"or exceeds it with a recorded rationale"*, which
> together with this non-goal would have let **V2 pass on three exemptions** —
> the gate satisfied by paperwork rather than by fix. §4.1 now excludes `views/`
> explicitly instead, so the gate says what it means.
>
> Deferring is the right call: **RFC 101 reworks these files anyway** for the
> guided flow, and RFC 099 §7a already records that the three-pane layout needs
> rethinking at 800 × 550. Splitting them here would be work RFC 101 undoes.
> But it means **V2 does not deliver "every GUI file under the limit"** — only
> every non-`views` file — and the ROADMAP's V2 wording should not be read as
> promising more than that.

## 5. Selected design

One commit per step, in this order.

1. `views/mod.rs` → `views.rs`, content unchanged, matching `ignore.rs` and
   `progress.rs`.
2. Extract `app.rs`'s inline tests to `app/tests.rs`, opening with
   `use super::*;` exactly as `ignore/tests.rs` does.
3. Extract state types, `Message`, and `dnd_sub` per §3.
4. **Phase A** — extract the 107 `update()` arm bodies into named `impl App`
   methods, leaving `update()` a dispatcher. **Still one file.**
5. **Phase B** — move those methods into `app/update/*.rs` by family, using the
   §3.2 section comments as the grouping. **No body changes.**
6. `app.rs` retains the `App` struct, its `Default`, and the module
   declarations.

Each step is independently compilable and independently revertible. **Steps 4
and 5 must not be combined** — §3.1.

## 6. Acceptance contract — gate V2

1. No `mod.rs` under `crates/aaai-gui/src/`.
2. No `#[cfg(test)]` module inline in any `crates/aaai-gui/src/**/*.rs`
   alongside implementation.
3. Every file under `crates/aaai-gui/src/` **except `views/*.rs`** is
   ≤ 500 ELOC. The `views/` exemption is §4.2's, not a per-file rationale.
4. `cargo +1.91 test --workspace --locked` — **every count unchanged**:
   aaai **146**, CLI unit **13**, CLI integration **97**, GUI **27**,
   doctests **3**. *(Refreshed 2026-08-10; the previous 144 / 8 / 91 predated
   RFC 103. Re-measure against current `main` before relying on these.)*
5. **Phase A bodies are byte-identical.** Each extracted method's body must
   match the arm body it replaced, modulo indentation, demonstrated by a
   scripted diff in the evidence package. See §6a.
6. `cargo +1.91 clippy -p aaai-gui --all-targets -- -D warnings` passes.
7. No diff outside `crates/aaai-gui/src/`.
8. No `Message` variant added, removed, or renamed.

## 6a. Why acceptance item 5 changed

The contract originally rested on **test names and counts being unchanged**, and
§11's third review question asked whether that was sufficient. It is not, and
the measurement is worse than it appears.

`app.rs` contains **9 tests**:

| Tests | What they cover |
|---:|---|
| 5 | `rfc064_suggest_patterns_*` — a pure helper function |
| 2 | `FieldError` struct shape |
| **2** | **`update()` itself** — `DiffFailed`, `RerunDiffReady` |

Restructuring 107 arm bodies would be guarded by **two tests touching the code
being restructured**. Test-name invariance proves nobody renamed a test. It says
nothing about the 105 arms nobody tests.

**The replacement borrows RFC 107's method.** That RFC faced the same problem —
a large mechanical diff nobody can read — and solved it by requiring the diff be
*regenerable*: a reviewer reproduces it rather than reads it. The equivalent
here is that phase A's extracted bodies must be **byte-identical to the arms
they replaced**, modulo indentation, shown by a scripted diff.

That is checkable, mechanical, and far stronger than a count. It also makes
phase B nearly free to accept: if bodies provably did not change in A, and B
moves them without editing, the whole restructure is verified.

**If that check cannot be produced, stop and escalate.** The alternative is
accepting a 107-arm restructure on the strength of two tests, which is not a
trade this RFC should make silently.

## 7. Risks and mitigations

| Risk | Mitigation |
|---|---|
| A behavioural change slips into a "mechanical" move | §6 item 5's byte-identical-body check, not test counts — §6a explains why counts are insufficient here |
| Phases A and B combined into one diff | §3.1 forbids it; a reviewer cannot distinguish a moved body from a modified one |
| Cross-family recursion discovered mid-move | §3.3 names all four sites up front |
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

## 11. Review questions — answered

Design review `.git-exclude/reviewed/068-rfc100-gui-module-boundaries-design-review-2026-08-10.md`
returned **Needs changes / No-Go** and answered all four. Resolutions are folded
into the sections above; recorded here so the questions are not reopened.

| Question | Answer |
|---|---|
| 1. Is the message-family split the right seam? | **Yes, but not as a first move.** The seam is right and already marked in the source (§3.2); the sequencing was missing. Split into phase A and phase B — §3.1 |
| 2. Is ≤ 500 ELOC the right bar, given the rules say *consider* at 300? | **Yes.** 300 is "consider", 500 is "strongly recommended"; adopting the stronger figure as a gate is correct. The defect was §4.2's exemption, not the number |
| 3. Is the test-name diff sufficient proof of behaviour neutrality? | **No.** Two of nine tests touch `update()`. Replaced by the byte-identical-body check — §6a |
| 4. Should `views/*.rs` above 500 ELOC be split here? | **No.** RFC 101 reworks them; splitting twice wastes the work. But §4.1 now excludes them explicitly rather than letting the gate pass on three rationales |

## 12. Sources

- `.git-exclude/reviewed/037-gui-uiux-gap-analysis-2026-07-28.md` §6
- `.git-exclude/reviewed/038-gui-remediation-roadmap-and-milestones-2026-07-28.md`
- `.git-exclude/rules/project-instructions-rust-gui.md` — ELOC and test-layout rules
- DEC-003 — Rust 2024 modules, no `mod.rs`, tests in `foo/tests.rs`
- `ROADMAP.md` — "Milestone review rules" item 6
