# RFC 099 — GUI Visual Foundation

**Status.** Proposed

**Tracks.** `ROADMAP.md` MG1 / WS-14 / gate V1

**Depends.** RFC 092 (design system adoption), RFC 094 (high-contrast themes),
RFC 095 / D0 §7.1 (approved GUI baseline)

**Design owner.** requirements architect

**Decision owner.** nabbisen, project owner

**Proposed implementer.** GUI developer (mid-capability model), after design
review by the high-capability model and explicit owner approval

**Verification operator.** A GUI/UX operator with a real display, for the V1
visual evidence. This is a capability requirement, not an independence
requirement — the owner may fill it (see
`.git-exclude/reviewed/041-rfc099-t6-t7-verification-tooling-decision-2026-07-28.md`
§Q5). Independence applies to the review of that evidence, not its capture.

**Environment boundary.** Local development plus the hosted B0 matrix for test
counts. Visual acceptance requires a real display — Xvfb renders iced/wgpu
panes black and is not acceptable evidence.

**Evidence location.** `.git-exclude/evidence/099-gui-visual-foundation/`

**Touches.** `crates/aaai-gui/src/` only — `design_tokens.rs`, `style.rs`,
`views/*.rs`, the single hardcoded colour and `.size()` call in `app.rs`'s
footer view (`view_footer`; structure otherwise untouched — RFC 100 owns
`app.rs` splitting), `main.rs` (the one `mod contrast_check;` declaration the
new test requires), and the new `contrast_check.rs` / `contrast_check/tests.rs`
module. No engine, CLI, persisted format, public API, dependency, manifest,
workflow, or release change.

**Handoff.** Required:
[`rfcs/handoffs/099-gui-visual-foundation/README.md`](../handoffs/099-gui-visual-foundation/README.md)

## 1. Summary

The GUI hardcodes every text size, every spacing value, and 128 colours, while
the design system it already depends on ships a typography scale, a spacing
scale, line heights, and contrast helpers that the GUI references **zero
times**.

This RFC adopts what is already there. It adds no dependency and defines no new
scale. It is net code removal.

It also closes a falsified requirement: NF-4 claims "≥ 4.5:1 contrast", and four
measured text pairs fail that in light mode, where they were designed.

## 2. Problem and observed baseline

Measured at `4ed8acfb292ed249078fbc03bc9d1555de3d1260`; full analysis in
`.git-exclude/reviewed/037-gui-uiux-gap-analysis-2026-07-28.md`.

### 2.1 Typography has no scale

`.size(N)` is called **173 times** with **12 distinct hardcoded values** — 9,
10, 11, 12, 13, 14, 15, 16, 17, 22, 32, 48. **About 90 % of all text is 13 px
or smaller**; twelve instances are 10 px or below, four at 9 px. iced's default
is 16 px. No size is token-derived.

Nine adjacent values is not a scale. Irregular type is what makes an interface
read as unfinished, independently of any single choice.

### 2.2 Spacing has no rhythm

`.spacing()` / `.padding()` use **12 distinct values** (0–16), dominated by
`spacing(4)` 26×, `spacing(0)` 21×, `spacing(2)` 8×, `spacing(1)` 5×. Gaps of
1–3 px are indistinguishable from zero at typical DPI, so element grouping is
effectively arbitrary.

### 2.3 Line height is never set

The GUI references `line_height` **zero times**. Every text block renders at
the toolkit default rather than a designed ratio. This is a substantial part of
"difficult to read" that is independent of font size.

### 2.4 Colour bypasses the token system, and NF-4 is not met

`design_tokens.rs` is 23 lines and resolves only a palette, while the views
contain **128 hardcoded `Color::from_rgb` calls** across 11 files — fixed
light-mode values that do not respond to the theme. `style.rs:32` hardcodes the
panel background, so the toolbar, filter bar, and bottom action bar stay
near-white in **both dark themes**.

Computed by WCAG relative luminance, in light mode:

| Usage | Foreground | Background | Ratio | AA 4.5:1 |
|---|---|---|---:|:--:|
| Dashboard muted label | `0.55,0.55,0.60` | `0.98,0.98,0.99` | 3.17:1 | ❌ |
| Muted text on panel | `0.5,0.5,0.5` | `0.96,0.97,0.98` | 3.71:1 | ❌ |
| Muted text on card | `0.5,0.5,0.5` | `0.98,0.98,0.99` | 3.81:1 | ❌ |
| Diff secondary text | `0.45,0.47,0.52` | `0.96,0.97,0.98` | 4.13:1 | ❌ |

All are used at 11–13 px, so the 3.0:1 large-text allowance does not apply.
This survived because the manual ABDD pass was never performed (RISK-004, open
since before the current program).

### 2.5 The approved scope does not cover any of this

RFC 095 §7.1's eight requirements are entirely about information architecture.
§11 explicitly excludes "layout, pane structure, exact placement, spacing,
colours" from the approved decision. WS-11 inherits that silence. Implemented
exactly to specification, the guided flow would be better organised and just as
hard to read.

## 3. What `snora 0.25.1` already provides

Already a dependency; already resolved into `App::design_tokens`.

`Typography::default_roles()`:

| Role | Size | Line height | Purpose |
|---|---:|---:|---|
| `body` | 16.0 | 1.4 | ordinary explanatory text |
| `body_small` | 14.0 | 1.35 | secondary metadata, compact help |
| `label` | 14.0 | 1.2 | button, field, chip labels |
| `title` | 18.0 | 1.3 | card, dialog, notice title |
| `heading` | 24.0 | 1.25 | page or section heading |
| `display` | 32.0 | 1.2 | rare major page title |

`Spacing::comfortable()`: `xs` 4, `sm` 8, `md` 12, `lg` 16, `xl` 24, `xxl` 32.

Also exported and unused: `Radius`, `Density`, `Emphasis`, `Tone`,
`FocusTokens`, and `contrast::{contrast_ratio, relative_luminance,
composite_over}`.

## 4. Goals and non-goals

### 4.1 Goals

- Every text size resolves from `tokens.typography`, chosen by **role**.
- Every text block carries its role's `line_height`.
- Every spacing and padding value resolves from `tokens.spacing`.
- Every colour resolves from the token palette; zero `Color::from_rgb` outside
  `design_tokens.rs`.
- NF-4 contrast verified by an **automated test** over all four presets, using
  `snora::design::contrast`.
- Behaviour, i18n keys, message protocol, and test counts unchanged.

### 4.2 Non-goals

- Adding any dependency, or defining any new scale.
- The guided flow — MG3 owns RFC 095 §7.1.
- Splitting `app.rs` or removing `views/mod.rs` — MG2.
- Changing status-colour semantics; DEC-005 and DEC-011 are settled.
- New screens, features, or UI expansion (RFC 095 §11).
- Selecting a font family; snora leaves that application-owned and this RFC
  does not change it.

## 5. Selected design

### 5.1 Map by role, never by nearest number

A size is replaced by the role matching the element's **purpose**, not by the
closest number. `body_small` (14) is the correct replacement for an 11 px muted
label even though 14 ≠ 11, and that increase is the point.

| Element purpose | Role |
|---|---|
| Primary explanatory text, reason field, diff body | `body` |
| Secondary metadata, hints, counts, SHA labels, help | `body_small` |
| Button, field, chip, filter, tab labels | `label` |
| Card, dialog, notice, pane titles | `title` |
| Screen and section headings | `heading` |
| Opening hero, empty-state hero | `display` |

Ambiguous sites are resolved in the handoff's per-file table, not by
implementer judgement.

### 5.2 Spacing

Map to the nearest **larger** step, never smaller: 0 stays 0 where deliberate
(adjacent elements sharing a border); 1–4 → `xs`; 5–8 → `sm`; 9–12 → `md`;
13–16 → `lg`; 17–24 → `xl`; 25 and above → `xxl`.

### 5.3 Colour

Replace all 128 `Color::from_rgb` sites with palette lookups. `style.rs:32`'s
hardcoded panel background is the highest-priority single fix — it breaks both
dark themes.

### 5.4 Contrast as a test, not an inspection

Add a GUI unit test that, for each of the four presets, resolves every
text-on-surface pair the views use and asserts `contrast_ratio` ≥ 4.5 for
normal text and ≥ 7.0 for the two high-contrast presets, using
`snora::design::contrast`. This converts NF-4 from an outstanding manual pass
into a gate that cannot silently regress.

## 6. Acceptance contract — gate V1

1. `grep -rE "\.size\([0-9]" crates/aaai-gui/src/` returns nothing.
2. `grep -rn "Color::from_rgb" crates/aaai-gui/src/` returns nothing outside
   `design_tokens.rs`.
3. Every text role application passes its token `line_height`.
4. The §5.4 contrast test passes for all four presets.
5. `cargo +1.91 test --workspace --locked` — GUI tests **26 + the new contrast
   test**; all other counts unchanged.
6. `python3 scripts/check-i18n-keys.py` clean; no i18n key added or removed.
7. Real-display screenshots for all four themes at **800 × 550** and at a
   typical working size, showing no clipped, overlapped, or scroll-trapped
   content.
8. No diff outside `crates/aaai-gui/src/`.

## 7. Risks and mitigations

| Risk | Mitigation |
|---|---|
| **Layout breaks at 800 × 550.** Body text moves 11–12 px → 16 px and spacing 1–4 px → 4–12 px. RFC 095 §7.1 item 7 requires usability at that size. This is the principal risk. | Acceptance item 7 makes it a blocking check. Remedies, in order: reduce nesting depth; move secondary content behind progressive disclosure; allow region scrolling. **Not** permitted: reintroducing ad-hoc small sizes. |
| **No compact density is available.** `Density::Compact` exists in snora but is documented "reserved; not resolved in v0.20"; only `Spacing::comfortable()` is provided, and every preset uses `Comfortable`. | If 800 × 550 cannot be met at comfortable spacing, stop and amend this RFC. The sanctioned routes are an upstream snora request for a resolved compact scale, or an owner-approved documented local override — not per-site shrinking. |
| Role mapping is judgement-heavy and could drift per file | The handoff carries a per-file mapping table; ambiguity escalates rather than being resolved locally. |
| A later change reintroduces literals | Acceptance items 1–2 become a repository scan the guided-flow work must also pass; recorded in the U1 gate. |
| Visual regression in the expert workspace | Screenshots for all four themes before and after; the three-pane surface must remain usable, per RFC 095 §7.1. |

## 8. Implementation sequence

1. Owner confirms implementer capacity and a GUI/UX verification operator with
   a real display.
2. Independent architecture review accepts this RFC and its handoff.
3. Owner explicitly approves implementation.
4. Developer executes the handoff, one concern per commit: colour, then
   typography and line height, then spacing.
5. Developer captures local evidence and the contrast test result.
6. Verification operator captures real-display screenshots for four themes at
   both sizes.
7. Independent implementation review.
8. Owner integrates; hosted B0 confirms counts on all three platforms.

If 800 × 550 cannot be satisfied, stop at step 4 and amend before continuing.

## 9. Compatibility

No engine, CLI, persisted-format, public-API, or i18n-key change. `prefs.yaml`
theme selection is unaffected. The visible result changes — text is larger and
more widely spaced — which is the intent, and is why real-display evidence is
required rather than optional.

## 10. Alternatives considered

| Option | Decision |
|---|---|
| Adopt the existing `snora` scales | **Selected.** Zero new dependency; net code removal; the values were designed for these themes. |
| Define a project-local type and spacing scale | Rejected: duplicates a dependency already paid for, and would drift from the palette it must pair with. |
| Adopt `libcosmic`'s design system | Rejected: `libcosmic` is built on a *fork* of iced and is not adoptable on stock iced 0.14. |
| Raise sizes ad hoc without a scale | Rejected: leaves the irregularity that reads as unsophisticated, and fixes nothing structurally. |
| Defer to MG3 and do it inside the guided flow | Rejected: new screens built on an ad-hoc visual layer add more sites to convert, and the expert workspace would stay unreadable. |
| Fix only the four failing contrast pairs | Rejected: leaves 124 hardcoded colours that still break both dark themes. |

## 11. Review questions

1. Is role-based mapping (§5.1) specified precisely enough to implement without
   further design judgement?
2. Is the 800 × 550 risk adequately mitigated, given no compact density exists?
3. Should the contrast test cover every text-on-surface pair, or a reviewed
   representative set — and who approves that set?
4. Is one-concern-per-commit the right granularity for review, given the diff
   spans roughly 300 call sites?
5. Does anything here encroach on RFC 095 §7.1 or MG3's guided-flow ownership?
6. Is the NF-4 test sufficient to close RISK-004's contrast portion, or does
   the manual ABDD sheet remain separately required?

## 12. Sources

- `.git-exclude/reviewed/037-gui-uiux-gap-analysis-2026-07-28.md`
- `.git-exclude/reviewed/038-gui-remediation-roadmap-and-milestones-2026-07-28.md`
- RFC 092, RFC 094, RFC 095 §7.1 and §11
- `snora 0.25.1` — `design::{Typography, Spacing, TextRole, Density, contrast}`
- `ROADMAP.md` — MG1/V1, "Milestone review rules" item 6
- `.git-exclude/rules/project-instructions-rust-gui.md` — UI/UX principles
