# RFC 115 handoff — unit 1: §S0, §S1, §S2

**RFC.** [`rfcs/accepted/115-elderly-reader-accessibility.md`](../../accepted/115-elderly-reader-accessibility.md)
**Status.** Accepted; decisions authorized by the owner 2026-11-04.
**Scope of this handoff.** §S0, §S1, §S2 only. **§S3 and §S4 are not in this
unit** and get their own handoff — do not start them.
**Start now.** This was originally scheduled behind RFC 106; RFC 106 is held
on its §6b decision (iced 0.14 cannot focus a button), so this unit goes first.
§S1 edits the footer RFC 106 will also touch — that is expected, and RFC 106
adapts to this unit rather than the reverse.
**Evidence location.** `.git-exclude/evidence/115-elderly-reader-accessibility/`

Read the RFC's §4 before §S1. It is one page and it is the reason §S1 is not
just "make the buttons bigger".

---

## 1. Why this unit exists, in one paragraph

Our two high-contrast themes (RFC 094) are tested, correct, and effectively
undiscoverable: the only route to them is a 24.8 px borderless glyph in the
footer whose sole label is a hover tooltip. This unit makes that route visible
and makes target size an assertion so it cannot rot again. It is a few hours of
work that recovers most of RFC 094's value.

---

## 2. §S0 — window default 1280 × 720

`crates/aaai-gui/src/main.rs` passes no `window::Settings`, so we currently
inherit iced's default **1024 × 768**
(`iced_core-0.14.0/src/window/settings.rs:106`).

Add to the `iced::application(...)` builder chain:

```rust
.window(iced::window::Settings {
    size: iced::Size::new(1280.0, 720.0),
    ..Default::default()
})
```

**This is user-visible** — add a `CHANGELOG.md` entry under the next version.

Do **not** set `min_size`, `position`, or anything else in this unit. One
change, one reason.

---

## 3. §S1 — label the Help and Settings controls

### What is there now

`crates/aaai-gui/src/app/view.rs:130-169`, in `view_footer`:

| Property | Current |
|---|---|
| Content | `?` and `⚙`, no text label |
| Size | `typography.label` — 14 px |
| Padding | `[spacing.xs, spacing.sm]` = `[4, 8]` |
| Style | `btn_ghost` → `snora_style::button::ghost` |
| Explanation | hover tooltip only |

`ghost` is `background: Color::TRANSPARENT` with `Border::default()` — **width
zero** (`snora-style-0.38.0/src/button.rs:138-155`). At rest there is no box,
no outline and no fill, so the only cue that either glyph is a button is that
it is `palette.accent`-coloured.

Height is deterministic and you can check my arithmetic:
`14.0 × 1.2 + 4 + 4 = 24.8 px`.

### What to change

1. **Add a visible text label** beside each glyph.
   - **Settings:** reuse `settings.button_tooltip` — already `Settings` /
     `設定` (`crates/aaai-gui/locales/en.yaml:260`,
     `crates/aaai-gui/locales/ja.yaml:249`). No new key.
   - **Help:** `help.title` is `Keyboard Shortcuts` /
     `キーボードショートカット` (`en.yaml:275`, `ja.yaml:264`) — that is the
     overlay's title and is too long for a footer control. **Add one new key**
     (`Help` / `ヘルプ` is the obvious pair) to **both** locale files in the
     same change. `i18n-key-audit` is blocking and will catch a one-sided add.
2. **Give both a resting border or fill**, so they read as controls before
   hover. Pick from the helpers already in `crates/aaai-gui/src/style.rs` —
   `btn_secondary`'s resting treatment is the obvious candidate. **Do not write
   a new style function**; if none of the existing helpers fits, stop and
   report rather than inventing one.
3. **Reach ≥ 44 × 44 px**, through **padding**, not by enlarging the glyph.
   Enlarging the glyph changes the footer's visual weight; padding does not.
4. **Keep the tooltips.** They become redundant for sighted mouse users and
   stay useful otherwise.

### What not to change

- **Do not touch `typography.label`'s 14 px.** Those are 138 control labels
  across the whole application. They move in §S3 with the scale multiplier;
  moving them now means regenerating baselines twice.
- **Do not move the Settings control out of the footer in this unit.** That is
  a layout decision the RFC leaves open (086 §3f item 2) and it is not
  authorized here.

---

## 4. §S2 — assert target size instead of measuring it by hand

### The problem you are fixing

`docs/src/abdd-audit.md` §6 is a table of pixel measurements whose every
Observed and Verdict cell is blank — summary `0 / 0 / 0` — except the snora
toast `×` row, which came from a snora letter rather than from us. **Our own
44 px rule has never been measured against our own UI.** A hand-filled table
rots the moment a padding token changes and nobody is reminded.

### What to build

`crates/aaai-gui/src/target_size_check/`, in the **exact shape** of
`crates/aaai-gui/src/contrast_check/` — read that module first and copy its
structure: a documented module header explaining what is asserted and why,
explicit named exemptions, traceability to the requirement.

**Assert:** for every interactive control the views build, that its computed
height and width reach **44 px on both axes**, derived from tokens —
`typography.<role>.size × line_height + padding` — not from a screenshot.

This needs **no compositor, no display and no capture**. It is arithmetic over
`typography` and `spacing`. That is the whole point: it runs in CI and it fails
the build when a padding change drops a control below the bar.

### The exemption you must record

The snora toast `×` measures **24 × 24**, is supplied by the dependency, and
cannot be changed by us. Record it in the module as an **exemption with its
reason**, not as a failure. Doing so closes an owner decision
`docs/src/abdd-audit.md` §6 has been carrying since 2026-09-26.

### Expect failures, and do not weaken the assertion

I measured two controls by hand (§S1's `?` and `⚙`, both ≈ 25 × 25). **There
are almost certainly more** — §6's blank rows name toolbar buttons, folder
pickers, `Save and continue`, diff-tab buttons and file-tree rows, none of
which anyone has measured.

**A failure list is the measurement §6 never produced.** Report it and stop.
Do **not** lower the threshold, and do **not** add blanket exemptions to make
the suite green — each exemption needs its own stated reason, and "it currently
fails" is not one. If the list is long, that is a scheduling question for me,
not a reason to soften the test.

### Then rewrite the audit section

`docs/src/abdd-audit.md` §6 — **and `docs/ja/src/threat-model.md`'s sibling
audit file if one exists; check, and if the Japanese copy of the audit does not
exist, say so rather than creating one.** Replace the empty table with: the
rule, a pointer to `target_size_check`, and the exemption list. The audit
document should cite measurements, not promise them.

---

## 5. Acceptance — what I will check

From RFC §6, conditions 1–5 and 10:

| # | Condition |
|---|---|
| 1 | Window opens at 1280 × 720; `CHANGELOG.md` entry present |
| 2 | Both footer controls carry a visible text label, have a resting border or fill, and measure ≥ 44 × 44 |
| 3 | At most one new i18n key (the Help label), in **both** locales; `i18n-key-audit` green |
| 4 | `target_size_check` asserts every interactive control at 44 px, every exemption named and justified **in the module** |
| 5 | `docs/src/abdd-audit.md` §6's empty table is gone — rule, pointer, exemptions |
| 10 | Gate V1 re-run. ~~Baselines: regenerate for this unit~~ — **corrected 2026-11-09: this contradicted RFC §6, which governs.** Baselines regenerate once, after §S4. This unit captures only to answer the S1 question |

Also, unconditionally:

- `cargo fmt --check` and `cargo clippy` clean (RFC 107 / gate C2).
- No `unsafe` (DEC-012).
- The full workflow green (gate C1).

---

## 6. Report back

Three things I specifically want in your report, because they are the parts I
could not determine from here:

1. **The §S2 failure list** — every control that came in under 44 px, with its
   computed size. This is the measurement `docs/src/abdd-audit.md` §6 has owed
   since it was written.
2. **Which `style.rs` helper you used for the resting state**, and whether it
   fit without modification.
3. **The final footer control size**, both axes, so condition 2 is checked
   against a number rather than an assertion that it was done.

If anything in §S2 turns out not to be computable from tokens — a control whose
size depends on runtime content width, for example — **report it as an
exemption candidate with your reasoning** rather than guessing a number or
dropping the control from the suite.
