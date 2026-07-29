# RFC 101 — Guided Review Flow

**Status.** Proposed

**Tracks.** `ROADMAP.md` MG3 / WS-11 / gate U1

**Depends.** RFC 095 / D0 §7.1 (approved baseline), RFC 099 / V1 (visual
foundation), RFC 100 / V2 (module boundaries), WS-05 (report contract),
WS-10 (progress API)

**Design owner.** requirements architect

**Decision owner.** nabbisen, project owner

**Proposed implementer.** GUI developer (mid-capability model), after design
review by the high-capability model and explicit owner approval

**Verification operator.** A GUI/UX operator with a real display, for the U1
visual and keyboard evidence. Capability requirement, not an independence
requirement; independence applies to the review of that evidence.

**Environment boundary.** Local development plus hosted B0 for counts. Visual
and keyboard acceptance require a real display.

**Evidence location.** `.git-exclude/evidence/101-guided-review-flow/`

**Touches.** `crates/aaai-gui/src/` and `crates/aaai-gui/locales/{en,ja}.yaml`.
No engine, CLI, persisted-format, or public-API change.

**Handoff.** Required:
[`rfcs/handoffs/101-guided-review-flow/README.md`](../handoffs/101-guided-review-flow/README.md)

## 1. Summary

Implement the guided beginner path that RFC 095 / D0 §7.1 approved, while
retaining the existing three-pane workspace as an explicitly selected expert
surface. Both operate on one product state.

This RFC owns the detailed external design §7.1 deferred to WS-11. It does not
reopen §7.1's decision.

## 2. Approved requirements this RFC implements

RFC 095 §7.1 fixes these; they are inputs, not open questions:

1. one dominant next action per screen;
2. the plain-language vocabulary — Original folder, Changed folder, Review
   record, Why is this change OK?, Save this answer, Check again, Save report;
3. strategy variant names, raw YAML terms, the full status taxonomy, technical
   errors, and report-format choice kept behind details;
4. the reason field as the primary review input;
5. text changes exposed on demand, not forced as first focus;
6. unsaved-work guards and keyboard access to primary actions preserved;
7. usable at **800 × 550**;
8. entry to and return from the expert workspace without losing work.

## 3. Screens

From the normative v0.33.2 wireframe:

```
Start → Checking → Review changes one by one → Done → Save report
                     ├─ optional: show file details
                     ├─ optional: show side-by-side text
                     └─ optional: skip this file
```

| Screen | Primary action | Notes |
|---|---|---|
| Start | **Check these folders** | Original / Changed folder pickers; no `audit.yaml` visible |
| Checking | none — progress only | Consumes WS-10's progress API |
| Review one change | **Save this answer** | One dominant decision card; reason field most prominent |
| Done | **Save report** | States plainly that the review record was saved |
| Save report | **Save report** | Defaults to the easy-to-read format; other formats behind details |
| Problem | **Show me what to do** | Recovery steps visible; technical detail behind details |

Removed from the default path: separate status-legend screen, shortcut overlay,
visible strategy picker, large settings view, always-visible three-pane
inspector.

### 3.1 Plain-language vocabulary

Normative, from the wireframe §1.1. Every string goes through `rust-i18n` in
both `en` and `ja`.

| Internal term | Shown to users |
|---|---|
| before directory | Original folder |
| after directory | Changed folder |
| `audit.yaml` | Review record |
| strategy | How aaai checks it next time |
| pending | Needs your answer |
| failed | Changed again |
| error | Couldn't check |
| ignored | Skipped |
| approve | Save this answer |

## 4. Goals and non-goals

### 4.1 Goals

- The guided path is the **default** surface on first run.
- The expert three-pane workspace is reachable via an explicit
  `For technical users` action and returns without losing work.
- One product state: both surfaces share the same engine result, definition,
  selection, dirty state, and persistence code. **No second audit
  implementation** (DEC-001).
- Full keyboard operation of primary actions on both surfaces.
- en and ja parity; no untranslated string.

### 4.2 Non-goals

- Re-deciding §7.1, or removing the expert workspace.
- Engine, CLI, persisted-format, or public-API change.
- New audit capability — this re-presents existing capability.
- Visual foundation work (RFC 099) or module boundaries (RFC 100); both are
  prerequisites, not scope.
- Screen-reader support requiring unavailable iced hooks — deferred by ROADMAP.

## 5. Design constraints

1. **One state, two views.** The guided screens render from the same `App`
   state as the expert workspace. A guided action and its expert equivalent
   must dispatch the same `Message`.
2. **Surface selection is persisted** in `prefs.yaml` via an additive
   `#[serde(default)]` field, so existing files remain readable.
3. **The visual foundation is not bypassed.** Every new widget uses RFC 099's
   token roles. No `.size(N)` literal or `Color::from_rgb` may be introduced —
   this is checked at U1.
4. **Progressive disclosure is the mechanism** for §7.1 item 3, implemented as
   collapsible details, not as separate screens.
5. **Destructive actions stay behind `More choices`**, per the wireframe.

## 6. Acceptance contract — gate U1

1. §7.1 items 1–8 each demonstrated on a real display.
2. The wireframe's twelve-item acceptance checklist passes, including: a
   non-technical user can start without seeing `audit.yaml`; the review screen
   has one dominant decision card; the reason field is its most prominent
   input; the diff is available but not the first focus; destructive
   leave/discard is not visible until `More choices`.
3. Guided → expert → guided round trip preserves unsaved reason text and
   selection.
4. Full keyboard path through Start → Checking → Review → Done → Save report.
5. `python3 scripts/check-i18n-keys.py` clean; en and ja both complete.
6. RFC 099's foundation intact: `grep -rE "\.size\([0-9]" crates/aaai-gui/src/`
   and `grep -rn "Color::from_rgb" crates/aaai-gui/src/` (outside
   `design_tokens.rs`) both return nothing.
7. RFC 100's boundaries intact: no `mod.rs`, no inline test module, no file
   above 500 ELOC without a rationale.
8. `cargo +1.91 test --workspace --locked` passes; GUI test count grows only by
   the guided-flow tests, each named and listed.
9. Real-display screenshots of all five screens in all four themes at 800 × 550
   and a typical working size.
10. An existing `prefs.yaml` without the new field loads without error.

## 7. Risks and mitigations

| Risk | Mitigation |
|---|---|
| **Two implementations diverge** — the classic failure for guided/expert splits, and a direct DEC-001 violation | One state, one `Message` set; a guided control and its expert equivalent dispatch the identical message. Reviewed explicitly at implementation review. |
| 800 × 550 fails, now with more chrome | Same blocking check as RFC 099; progressive disclosure is the primary lever. If it fails, stop and amend — do not shrink type. |
| Round trip loses unsaved work | Acceptance item 3; the existing nav-guard (RFC 041) is reused, not re-implemented. |
| i18n drift — new strings land in `en` only | The i18n audit is already a blocking CI job; acceptance item 5. |
| WS-10's progress API is not final when Checking is built | Checking is the **last** screen implemented; if WS-10 has not landed, stop rather than build against a provisional API. |
| WS-05's report contract shifts under Save report | Same treatment: Save report follows WS-05, not the reverse. |
| Scope creep into new features | RFC 095 §11 defers UI expansion; any new capability stops the work. |

## 8. Implementation sequence

1. V1 and V2 both passed and integrated.
2. WS-05 report contract and WS-10 progress API available.
3. Independent design review accepts this RFC and its handoff.
4. Owner approves implementation.
5. Build in dependency order: **Review card first** (the core, and the screen
   with no external dependency), then Start, Done, Save report, and
   **Checking last** (WS-10), then the Problem state.
6. Real-display and keyboard evidence.
7. Independent implementation review, then owner integration and hosted B0.

Building the Review card first means the highest-value screen is reviewable
before any dependency risk is taken.

## 9. Compatibility

`prefs.yaml` gains one optional field with `#[serde(default)]`; older files load
unchanged. No engine, CLI, report-format, or public-API change. The expert
workspace's behaviour is preserved; RFC 095's compatibility acceptance matrix
(WS-12) covers the guided path as a new surface over existing capability.

## 10. Alternatives considered

| Option | Decision |
|---|---|
| Guided default plus retained expert workspace | **Selected by RFC 095 §7.1.** Implemented here. |
| Guided-only, remove the expert workspace | Rejected by RFC 095: discards working capability and complicates rollback. |
| Retain three-pane as default | Rejected by RFC 095: conflicts with the wireframe and the project's less-is-more rule. |
| A separate guided binary or crate | Rejected: guarantees the divergence risk in §7 and violates DEC-001. |
| Build Checking first, matching user order | Rejected: it carries the WS-10 dependency; the Review card is higher value and dependency-free. |

## 11. Review questions

1. Is "one state, one `Message` set" specified tightly enough to prevent
   divergence, or does it need a structural guard?
2. Is a persisted surface preference right, or should the app choose by
   heuristic on first run?
3. Is building the Review card first the correct order, given users encounter
   Start first?
4. Should the Problem state be a screen or an inline card within Review?
5. Is the 800 × 550 requirement still right after RFC 099 enlarged the base
   type, or should RFC 095 §7.1 item 7 be revisited?
6. Does the guided path change what the compatibility matrix (WS-12) must
   assert?

## 12. Sources

- RFC 095 / D0 §7.1 and §11 — approved GUI baseline
- `aaai-wireframes-simple-v0.33.2` — normative for the guided path
- RFC 099 (visual foundation), RFC 100 (module boundaries)
- `.git-exclude/reviewed/037-gui-uiux-gap-analysis-2026-07-28.md`
- `.git-exclude/reviewed/038-gui-remediation-roadmap-and-milestones-2026-07-28.md`
- `ROADMAP.md` — MG3 / WS-11 / U1
