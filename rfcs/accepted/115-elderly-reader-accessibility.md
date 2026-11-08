# RFC 115 — Elderly-Reader Accessibility: Text Size, Target Size, Reachability

**Status.** **Accepted** — the decisions this RFC records were authorized by the
owner on 2026-11-04 from
`.git-exclude/review-requests/086-elderly-accessibility-assessment-review-request.md`
§6. Lives in `rfcs/accepted/`; the folder is the source of truth for this state
(RFC 000).

**Tracks.** ABDD accessibility, extended to the elderly-reader case that WCAG
does not cover. Successor in spirit to RFC 094 (high-contrast themes), whose
accommodation this RFC makes reachable.

**Depends.** Nothing. **This RFC's unit 1 now goes first** — see §9.

It was originally scheduled behind RFC 106, because §S1 and RFC 106 both edit
the footer. RFC 106 was **held on 2026-11-04** by its own §6b (iced 0.14 cannot
focus a button), so waiting behind it would stall unit 1 behind a decision that
has nothing to do with it. The two keep **separate handoffs** either way.

**Design owner.** high-capability model

**Decision owner.** nabbisen, project owner

**Proposed implementer.** mid-capability model, per slice

**Evidence location.** `.git-exclude/evidence/115-elderly-reader-accessibility/`

**Touches.** `crates/aaai-gui/src/main.rs` (window settings), `app/view.rs`
(footer), a new `crates/aaai-gui/src/target_size_check/` test module,
`crates/aaai/src/profile/prefs.rs` (one new preference),
`crates/aaai-gui/src/views/settings_dialog.rs`, and the `body_small` call sites
across `crates/aaai-gui/src/views/`. **Persisted-format change in §S3** —
`UserPrefs` gains a field; must default cleanly on an existing `prefs.yaml`.

**Handoff.** Required. `rfcs/handoffs/115-elderly-reader-accessibility/` —
written for §S0–§S2 (unit 1); §S3–§S4 get a second handoff when that unit is
scheduled.

---

## 1. Summary

aaai's accessibility work is sound and largely unreachable by the people it was
built for.

Three measured facts, from review-request 086:

1. **85 % of the GUI's text renders at 14 px.** `typography.body_small` is
   applied 142 times and `typography.label` 138, against **18** uses of the
   16 px `body` role that snora designates for ordinary explanatory text.
2. **There is no text-size control.** `UserPrefs` holds theme, language and
   ignored directories. Nothing for scale.
3. **The theme picker — including RFC 094's two high-contrast presets — sits
   behind a 24.8 px borderless glyph in the footer whose only label is a hover
   tooltip.**

Fact 3 is the structural one. Our answer to "can the colour be kinder to
elderly users" is *choose a high-contrast theme*, and that answer is gated
behind the least accessible control in the application. RFC 094's programme is
tested, correct, and inert for its intended audience.

**This RFC is deliberately sliced**, smallest and most valuable first, because
§S1 alone recovers most of RFC 094's lost value at the cost of one view
function.

## 2. Why our gates did not catch any of this

Worth stating before the design, because the gap is systemic rather than
accidental.

**WCAG sets no minimum font size.** `contrast_check` proves our text is
high-contrast; nothing proves it is large enough to read. A 14 px label at
7:1 passes every assertion we own. Gate V1 ("GUI tokens + contrast") is
satisfied by construction and says nothing about legibility.

**Our 44 px rule has never been measured against our own UI.**
`docs/src/abdd-audit.md` §6 carries a measurement table whose every Observed
and Verdict cell is blank — summary `0 / 0 / 0` — except the toast `×`, which
came from snora's letter rather than from us. We have been enforcing the rule
on a dependency's control while leaving our own unmeasured.

**The cause is that §6 is a manual table.** A hand-filled table of pixel
measurements rots the moment a padding token changes, and nobody is reminded.
That is the same failure RFC 105 §5.3 names — *assert what can be asserted* —
and target size is assertable: it is arithmetic over tokens, not a judgement
call. §S2 fixes the mechanism, not just the numbers.

## 3. Goals and non-goals

### 3.1 Goals

- Make the existing accommodations **findable** by a first-time elderly user.
- Give the user **control over text size**, rather than guessing one default.
- Make target size an **assertion**, so §6 cannot silently rot again.
- Keep the default appearance recognisable to current users.

### 3.2 Non-goals

- **Not changing the default theme.** Light stays. Imposing a high-contrast
  look on everyone to fix a discoverability problem is the wrong trade; §S1
  fixes the discoverability directly (086 §6 decision 6).
- **Not a palette change.** The colour work is done — every foreground role
  including `accent` is asserted at AA in Light/Dark and AAA in both
  high-contrast presets. The problem is size and reachability, not hue.
- **Not screen-reader support.** iced 0.14 exposes no accessibility tree; this
  is `docs/src/abdd-audit.md` §7's declared v1.0 limitation and is upstream.
- **Not a general theming or font-family system.** One scale multiplier, not a
  typography editor.

## 4. The distinction this RFC rests on

**"Less is more" is right for this audience. Progressive disclosure is not what
it means.**

Hiding a control trades visual clutter for recall and discovery — and recall
and discovery are precisely what decline with age. For an elderly reader "less
is more" means **fewer controls, each larger and permanently visible**, not the
same number of controls hidden behind icons.

aaai currently gets this right in one place and wrong in another:

- **Right:** the opening screen collapses optional settings
  (`app.rs:168`, `optional_settings_expanded: false`) with the two required
  folder pickers prominent. The collapsed content is genuinely optional, and
  `app/update/save.rs:92,271` auto-expands it when something inside needs
  attention. **Keep this unchanged.**
- **Wrong:** Help and Settings, the only application-wide controls, are
  footer glyphs with no label, no border and no fill.

The rule this RFC adopts: **a control that is the only route to an
accessibility accommodation is never icon-only, never tooltip-only, and never
below 44 px.**

## 5. Slices

Ordered by value per unit of cost. Each is independently shippable and each
leaves the application in a releasable state.

### S0 — Window default 1280 × 720

`crates/aaai-gui/src/main.rs` passes no `window::Settings`, so we inherit
iced's default **1024 × 768** (`iced_core-0.14.0/src/window/settings.rs:106`).

Add:

```rust
.window(iced::window::Settings {
    size: iced::Size::new(1280.0, 720.0),
    ..Default::default()
})
```

**Two reasons, and the second is the load-bearing one.** It makes GUI evidence
runs reproducible without a nested compositor (review-request 085 §8 item 3).
More importantly it is a **precondition for §S3**: enlarging text at
1024 × 768 crowds the layout immediately, and discovering that during §S3
would turn a preference into a redesign.

**User-visible**, so it carries a changelog entry. Authorized 2026-11-04.

> **Corrected 2026-11-09 — the first reason is false on a tiling compositor.**
> Implementation (review-request 087 §1.1) showed the app *requests*
> 1280 × 720 correctly (`WM_NORMAL_HINTS … user specified size: 1280 by 720`)
> and **niri ignores the request for a tiled window**, as tiling window
> managers generally do. So S0 does **not** make evidence reproducible without
> a nested compositor; I asserted that without checking it. S0 stands on the
> second reason, which is unaffected, and on what users of floating window
> managers — the Windows, macOS and mainstream Linux desktops — will see.
> Acceptance condition 1 is judged on what the application controls: the
> request.
>
> Reproducible baselines therefore need a nested compositor on this machine.
> None is installed; that is an owner decision, needed before §S3–§S4's
> single baseline regeneration and not before.

### S1 — Label the Help and Settings controls, and give them 44 px targets

The defect, at `crates/aaai-gui/src/app/view.rs:130-169`:

| Property | Current | Source |
|---|---|---|
| Content | `?` and `⚙`, no text | `view.rs:132,153` |
| Size | `typography.label` — 14 px | `view.rs:133,154` |
| Padding | `[spacing.xs, spacing.sm]` = `[4, 8]` | `view.rs:137,158` |
| Style | `ghost` — `background: TRANSPARENT`, `Border::default()` (width **0**) | `snora-style-0.38.0/src/button.rs:138-155` |
| Explanation | hover tooltip only | `view.rs:147,167` |

Height is deterministic: `14.0 × 1.2 + 4 + 4 = 24.8 px`. Width is the glyph
advance plus 16. So each control is about **25 × 25** — clearing WCAG 2.5.8
(AA, 24 × 24) by 0.8 px on height, and failing the **44 px bar
`docs/src/abdd-audit.md` §6 sets as our own ABDD common rule.**

At rest there is no box, no outline and no fill. **The only cue that either is
a button is that the glyph is `palette.accent`-coloured** — a colour-only
affordance, which is the spirit of SC 1.4.1 even where the letter does not
apply.

**Required changes.**

1. **A visible text label** beside each glyph.
   **Settings needs no new string:** `settings.button_tooltip` is already
   `Settings` / `設定` in both locales (`locales/en.yaml:260`,
   `locales/ja.yaml:249`) and reads correctly as a label.
   **Help does.** `help.title` is `Keyboard Shortcuts` /
   `キーボードショートカット` (`en.yaml:275`, `ja.yaml:264`) — that is the
   overlay's own title and is too long for a footer control. Add one short key
   in both locales (`Help` / `ヘルプ` is the obvious pair) rather than
   stretching the footer to fit the overlay's title. `i18n-key-audit` enforces
   the both-locales half, so the only real requirement is that the key is added
   to **both** files in the same change.
2. **A resting border or filled surface**, so the control reads as a control
   before hover. `btn_secondary`'s resting treatment rather than `ghost` is the
   obvious candidate; the implementer picks from `style.rs`'s existing helpers
   rather than writing a new one.
3. **≥ 44 × 44 px**, achieved through padding, not by enlarging the glyph —
   enlarging the glyph changes the footer's visual weight, padding does not.
4. **Keep the tooltip.** It becomes redundant for sighted mouse users and stays
   useful for the truncated case.

**Do not change `typography.label`'s 14 px here.** Those 138 sites are control
labels across the whole application; moving them belongs with §S3's multiplier,
and doing it twice wastes a baseline regeneration.

### S2 — Assert target size instead of measuring it by hand

Add `crates/aaai-gui/src/target_size_check/`, in the exact shape of
`crates/aaai-gui/src/contrast_check/`: a documented module, explicit
exemptions, traceability to the requirement.

**What it asserts.** For every interactive control the views build, that its
computed height and width from tokens — `typography.<role>.size ×
line_height + padding` — reach **44 px on both axes**, with each exemption
named and justified in the module rather than in prose elsewhere.

**Why this is assertable and the table was not.** Target size is arithmetic
over `typography` and `spacing` tokens. It needs no screenshot, no compositor
and no human. It also cannot rot: a padding change that drops a control below
44 px fails the build, where the manual table would simply have gone stale.
This is RFC 105 §5.3's rule applied to the one section of the ABDD audit that
was pure manual measurement.

**Known exemption to record on arrival.** The snora toast `×` is 24 × 24,
supplied by the dependency and not changeable by us
(`docs/src/abdd-audit.md` §6). It is an **exemption with a reason**, not a
failure — and recording it in the module is what closes 086 §6 decision 2 and
the owner decision §6 has been carrying since 2026-09-26.

**Then rewrite `docs/src/abdd-audit.md` §6** to state the rule, point at the
assertion, and list the exemptions — deleting the empty table. The audit
document stops being a place where measurements are promised and becomes a
place where they are cited. Both language copies.

> **Measured 2026-11-09 — 50 of 53 controls are under 44 px.** The scan
> (review-request 087 §4) found heights from 22.3 to 42.8 px; only the two
> footer controls §S1 rebuilt pass, plus the reason editor and one block.
> The test cannot land red: a red `main` is a stop-work condition. **It lands
> as a ratchet** — the 50 are listed in the module as *known debt* with their
> measured heights; any control not on the list must reach 44 px, a listed one
> may not get worse, and a listed one that reaches 44 px must be removed. The
> rule is enforced for every new control from the day it lands, and the list
> can only shrink. `#[ignore]` was rejected because it would stop enforcing the
> rule for new controls too, just as RFC 106 and RFC 116 are about to touch
> every button. Review: `.git-exclude/reviewed/087-rfc115-unit1-review-2026-11-09.md` §3.
>
> **The toast `×` exemption is retired by RFC 116**, which sets
> `Tokens::targets.min = 44.0` — snora 0.57 sizes the toast close mark from
> that token. The exemption recorded here is correct for 0.38 and temporary.

### S2b — Burn down the known-debt list (authorized 2026-11-09)

**Authorized by the owner on 2026-11-09**, with the rule as written below:
44 × 44 for every interactive control, no tiers. Its handoff is written after
RFC 116 and the rescoped RFC 106 land, because those two decide what is left on
the list.

Empty the ratchet list. **The rule is 44 × 44 for every interactive control**,
with no tier of "secondary" targets: a tier boundary is a judgement call no
test can enforce, and it would be the first thing to drift. A text input at
36.8 px and a button at 24.8 px fail the same rule for the same user.

**Sequenced after RFC 116 and the rescoped RFC 106**, not before. Most of the
50 are buttons; RFC 106 option (d) rebuilds them as snora controls, and RFC 116
sets the token that makes every snora control at least 44 px. Hand-padding
those buttons first would be work thrown away a unit later. What remains after
both — text inputs, pick lists, any button snora cannot express — is this
slice's actual scope, and the ratchet list will say exactly what it is.

Folded in, from review-request 087 §7:

- **A.** The recent-project **Open** button collapses to a ~12 px sliver beside
  a long path — content-dependent, invisible to the static scan, and the
  Opening screen's only route into a saved review. Fix it, and add a case the
  scan *can* see (a minimum width on the control) so it cannot recur.
- **B.** Two pick lists bypass the typography tokens with literals:
  `.text_size(11)` at `crates/aaai-gui/src/views/inspector.rs:324` and
  `.text_size(13)` at `crates/aaai-gui/src/views/settings_dialog.rs:65`.
  11 px is the smallest text in the application. Move both to a role, and
  extend RFC 099's V1 grep to `.text_size(` so the next one fails.
- **C.** `bottombar.approve_and_save` carries a `// ABDD ≥44px` comment and
  measures 40.8 px. The comment goes when the control passes, not before.

Done when the ratchet list is empty and `every_control_reaches_44px` asserts
the plain rule with no list at all.

### S3 — A text-size preference

**The highest-impact item in this RFC, and the largest.**

`UserPrefs` (`crates/aaai/src/profile/prefs.rs:70-90`) gains one field: a text
scale, as a small enum rather than a free float, so the persisted value cannot
express something unrenderable.

```
Normal (1.00) · Large (1.25) · Larger (1.50)
```

**Applied as a multiplier over the whole role scale**, not as per-role
overrides, so snora's proportions survive and `title` stays larger than `body`
at every setting. The resolver belongs next to `design_tokens::tokens_for()`,
which is already the single place tokens are produced.

**Requirements.**

- **Default `Normal`**, so no existing user's appearance changes on upgrade.
- **`#[serde(default)]`**, so an existing `prefs.yaml` without the field loads
  unchanged — this is a persisted-format change and DEC-004's care applies.
- **The control goes in the settings dialog beside the theme picker**, because
  after §S1 that dialog is reachable. A user who needs large text must not need
  to read small text to find the setting: **render the control's own label at
  the scale it selects**, so the choice previews itself.
- **No window resize on change.** The setting re-renders; it does not relaunch.
- **Scrolling must survive `Larger`.** Every view that assumes its content fits
  needs checking at 1.50 — this is the real cost of the slice, and §S0 is what
  keeps it from being a redesign.

### S4 — Reassign the `body_small` sites

Inside §S3's unit, not before it.

Audit the **142** `typography.body_small` applications and move every one that
is not genuinely secondary metadata to `typography.body`. snora's own guidance
is the test (`snora-design-0.38.0/src/typography.rs:26-30`): `body_small` is
*"secondary metadata / compact help"*, and *"longer text surfaces (notices,
empty states, guidance) should use roles with readable line-height."*

**Mechanically a token swap; not free.** 14 → 16 px reflows every view it
touches, so each needs re-verification and the baselines regenerate once.
Running it inside §S3 means one regeneration instead of two.

**Expected outcome**, to be reported rather than assumed: most of the 142 are
genuinely body text and move; a minority — timestamps, byte counts, path
metadata — stay. The implementer reports the final split, and the number of
sites that stayed is the interesting figure.

## 6. Acceptance contract

| # | Slice | Condition |
|---|---|---|
| 1 | S0 | Window opens at 1280 × 720; changelog entry present |
| 2 | S1 | Both controls carry a visible text label from the existing i18n keys, have a resting border or fill, and measure ≥ 44 × 44 |
| 3 | S1 | At most one new i18n key (the Help label), present in **both** locales; `i18n-key-audit` green |
| 4 | S2 | `target_size_check` asserts every interactive control at 44 px, with every exemption named and justified in the module |
| 5 | S2 | `docs/src/abdd-audit.md` §6's empty table is gone, replaced by the rule, a pointer to the assertion, and the exemption list — both languages |
| 6 | S3 | An existing `prefs.yaml` with no scale field loads and renders identically to before |
| 7 | S3 | At `Larger`, every view remains fully reachable — no clipped or unscrollable content |
| 8 | S3 | The scale control renders its own label at the scale it selects |
| 9 | S4 | The `body_small` → `body` split is reported with the count that stayed and why |
| 10 | all | Gate V1 re-run; baselines regenerated **once, after S4** — never per unit. Earlier units capture only to answer named questions (corrected 2026-11-09: the unit 1 handoff said otherwise) |

## 7. What this RFC does not fix, and why

**AA versus AAA on the default theme.** Contrast sensitivity declines with age;
AA's 4.5:1 is calibrated to roughly 20/40 acuity and AAA's 7:1 to roughly
20/80, so for a 70+ audience AAA is the realistic target — and our default
theme is AA. This RFC **does not** change that, because enlarged text at AA is
more readable than small text at AAA, and §S1 plus §S3 deliver the enlargement
and the choice. Revisit only if evidence after §S3 says otherwise.

**Non-text contrast (SC 1.4.11), and a finding that raises this RFC's
priority.** `contrast_check` covers text-on-surface only, by its own header, so
the focused appearance is unasserted. Investigating that for 086 §6 decision 5
turned up something larger, now recorded as **RFC 106 §6b**: in iced 0.14 only
`text_input` and `text_editor` can hold focus — **`button` cannot be focused at
all.**

The assertion itself belongs to RFC 106 and is being added there, scoped to the
widgets that can hold focus (RFC 106 §6b.6). But the finding matters here too:
**if buttons cannot be reached by keyboard, the mouse path is the only path for
most of this application.** That makes this RFC's target-size work (§S1, §S2)
and text-size work (§S3) load-bearing rather than merely desirable — they are
not a second accessibility route alongside the keyboard, they are the primary
one for every button in the GUI.

**Screen readers.** Upstream — `docs/src/abdd-audit.md` §7.

## 8. Risks

| Risk | Mitigation |
|---|---|
| §S3's reflow turns into a redesign | §S0 first; §S3's acceptance condition 7 is explicit about `Larger` |
| §S4 enlarges text that should have stayed small, making views feel cramped | The implementer reports the split rather than applying it silently; snora's own role guidance is the test |
| A persisted-format change breaks an existing profile | Acceptance condition 6 tests exactly that case |
| §S2's assertion fails on controls we did not know about | That is the point. A failure list is the measurement §6 never produced; triage it, do not weaken the assertion |
| §S1's resting border changes the footer's look enough to bother current users | It is two controls in a footer, and the alternative is leaving RFC 094 unreachable |

## 9. Milestone placement

**Unit 1 (§S0 + §S1 + §S2) is the next thing the dev team does.** Small, high
value, no persisted-format change, and no dependency on anything held.

Then **§S3 + §S4 as their own unit**, with a single baseline regeneration.

The two units are deliberately unequal. Unit 1 is a few hours and recovers most
of RFC 094's lost value; unit 2 is the larger piece of design work in this RFC
and should not be allowed to delay unit 1.

**Why this moved ahead of RFC 106.** RFC 106 is held on its §6b decision. Two
consequences point the same way. First, unit 1 is unblocked and RFC 106 is not,
so ordering them the other way would stall it for no reason. Second, §6b's
finding makes unit 1 *more* important: if buttons cannot be keyboard-focused,
the pointer is the only route to every button in this GUI, and target size
stops being a secondary accommodation. **§S1 and RFC 106 both edit the footer;
unit 1 landing first means RFC 106 adapts to it, which is the cheaper
direction** — unit 1 is three contained changes, RFC 106 is a traversal
rework.

> **Sequence as of 2026-11-09.** (1) Unit 1 lands — S0 + S1 now, S2 after its
> corrections. (2) The advisory lockfile update. (3) RFC 116, the snora 0.57
> migration. (4) RFC 106, rescoped. (5) §S2b, if authorized. (6) §S3 + §S4,
> with the one baseline regeneration — needing a nested compositor first.
> RFC 116 and RFC 106 moved ahead of §S2b because they fix most of its list
> for free (§S2b's own sequencing note).

## 10. Sources

- `.git-exclude/review-requests/086-elderly-accessibility-assessment-review-request.md`
  — the measurements this RFC is built on
- `snora-design-0.38.0/src/typography.rs:26-30,52-78` — the role scale and its
  stated purposes
- `snora-style-0.38.0/src/button.rs:138-155` — `ghost`'s transparent background
  and zero-width border
- `iced_core-0.14.0/src/window/settings.rs:106` — the 1024 × 768 default
- `crates/aaai-gui/src/contrast_check/tests.rs` — the shape §S2 follows, and
  the header that scopes it to text-on-surface
- `docs/src/abdd-audit.md` §6, §7 — the empty table and the screen-reader
  limitation
- RFC 094 (high-contrast themes), RFC 105 §5.3 (assert what can be asserted),
  RFC 106 (keyboard operability)
