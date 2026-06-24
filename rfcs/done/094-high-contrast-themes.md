# RFC 094 — High-Contrast Themes

**Status:** Implemented (v0.38.0)
**Target release:** pre-v1.0.0 (after RFC 092 and RFC 093)
**Related area:** Accessibility (ABDD), theming, snora-design presets
**Depends on:** RFC 092 (token pipeline), RFC 093 (theme picker to select them)
**Authors:** nabbisen / project maintainers

---

## 1. Summary

Add two high-contrast themes — High Contrast Light and High Contrast Dark —
backed by snora-design's `high_contrast_light()` and `high_contrast_dark()`
presets. These presets reach ≥8:1 contrast on status surfaces (versus the ~5:1
of the standard presets), serving users who need elevated contrast for
legibility.

This is the accessibility payoff of the design-system work. It is the third and
last of the three design-system RFCs because it strictly depends on both:
RFC 092 supplies the token pipeline that makes a preset actually restyle the
UI, and RFC 093 supplies the picker that lets a user select it.

---

## 2. Motivation

### 2.1 ABDD commitment

aaai's Accessible by Default Design policy (design doc p.8) commits v1.0 to
colour-independent status, keyboard completeness, and visible focus. High
contrast is not strictly required by that list, but it is the natural next step
and the snora-design presets make it nearly free to add — the AA-tested
high-contrast colors already exist upstream.

### 2.2 Why the standard presets are not enough

The standard light preset puts status colors at ~5:1 — above the 4.5:1 AA
floor, but not by much. Users with low vision, or those working on
poor-quality displays or in high-glare environments, benefit from the
high-contrast presets' ≥8:1 figures. Providing them is a concrete,
low-cost accessibility win.

Measured (snora-design 0.25.1 high_contrast_light, white-on-color):

| Role | Standard light | High-contrast light |
|---|---|---|
| success | 5.02:1 | 8.38:1 |
| warning | 5.43:1 | 8.42:1 |
| danger | 6.54:1 | 8.85:1 |

---

## 3. Detailed design

### 3.1 Theme enum extension

Add two variants to `aaai_core::profile::prefs::Theme`:

```rust
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Theme {
    #[default]
    Light,
    Dark,
    System,
    /// High-contrast light (snora-design preset, >=8:1 status contrast).
    #[serde(rename = "high_contrast_light")]
    HighContrastLight,
    /// High-contrast dark (snora-design preset, >=8:1 status contrast).
    #[serde(rename = "high_contrast_dark")]
    HighContrastDark,
}
```

The explicit `#[serde(rename = ...)]` on the two new variants is required
because `rename_all = "lowercase"` would otherwise produce
`highcontrastlight`, which is unreadable in a hand-inspectable `prefs.yaml`.
The snake_case form is chosen to match the file's other multi-word values.

**Backward compatibility:** existing `prefs.yaml` files contain only
`light`/`dark`/`system`; those continue to deserialize unchanged. The new
values only ever appear in files written after this RFC ships. A file written
by a post-RFC-094 aaai and opened by a pre-RFC-094 aaai would fail to
deserialize the unknown theme value — but since this is all pre-v1.0, and the
prefs loader falls back to `Theme::default()` (Light) on any parse failure,
the degradation is graceful (the user silently gets Light). The implementer
must verify this fallback path exists and add it if not.

### 3.2 Display impl

```rust
impl std::fmt::Display for Theme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Theme::Light             => write!(f, "Light"),
            Theme::Dark              => write!(f, "Dark"),
            Theme::System            => write!(f, "System"),
            Theme::HighContrastLight => write!(f, "High Contrast Light"),
            Theme::HighContrastDark  => write!(f, "High Contrast Dark"),
        }
    }
}
```

### 3.3 Token resolution (extends RFC 092's design_tokens.rs)

```rust
pub fn tokens_for(theme: &AppTheme) -> Tokens {
    match theme {
        AppTheme::Light             => Tokens::light(),
        AppTheme::Dark              => Tokens::dark(),
        AppTheme::System            => Tokens::light(),
        AppTheme::HighContrastLight => Tokens::high_contrast_light(),
        AppTheme::HighContrastDark  => Tokens::high_contrast_dark(),
    }
}
```

### 3.4 Picker list (extends RFC 093's Theme::all)

```rust
pub fn all() -> &'static [Theme] {
    &[
        Theme::Light,
        Theme::Dark,
        // Theme::System,  // per RFC 093 §5.1, hidden until OS detection works
        Theme::HighContrastLight,
        Theme::HighContrastDark,
    ]
}
```

### 3.5 i18n keys (extends RFC 093's settings.* keys)

```
settings.theme_high_contrast_light  "High Contrast Light" / "ハイコントラスト（ライト）"
settings.theme_high_contrast_dark   "High Contrast Dark"  / "ハイコントラスト（ダーク）"
```

### 3.6 iced base theme mapping (extends main.rs)

The iced built-in `Theme` (which styles iced's own widgets like pick_list and
text_input) must map sensibly for the HC variants:

```rust
.theme(|app| match app.theme {
    AppTheme::Dark              => iced::Theme::Dark,
    AppTheme::Light            => iced::Theme::Light,
    AppTheme::System           => iced::Theme::Light,
    // HC themes use the matching light/dark iced base for built-in widget
    // chrome; aaai's own widgets get the high-contrast snora tokens.
    AppTheme::HighContrastLight => iced::Theme::Light,
    AppTheme::HighContrastDark  => iced::Theme::Dark,
})
```

### 3.7 Status colors under high contrast (the Option B reconsidered from RFC 092)

RFC 092 §4.1 recommended keeping OK/Pending/Failed as fixed constants (Option A)
rather than re-deriving them from the active preset (Option B). RFC 094 is
where Option B earns its keep: under a high-contrast theme, the status badges
*should* use the high-contrast `success`/`warning`/`danger` values, not the
standard-contrast constants.

**This requires a decision.** Two implementable approaches:

**Approach 1 — status colors follow the preset (live token read).** Change the
status-color resolver from constants to `app.design_tokens.palette.success`
etc. for OK/Pending/Failed. Error and Ignored remain hardcoded (no snora role),
but provide a high-contrast variant of each, selected on whether the active
theme is an HC theme:

```rust
fn status_color(status: AuditStatus, tokens: &Tokens, hc: bool) -> Color {
    match status {
        AuditStatus::Ok      => to_iced(tokens.palette.success),
        AuditStatus::Pending => to_iced(tokens.palette.warning),
        AuditStatus::Failed  => to_iced(tokens.palette.danger),
        AuditStatus::Error   => if hc { ERROR_HC } else { ERROR_COLOR },
        AuditStatus::Ignored => if hc { IGNORED_HC } else { IGNORED_COLOR },
    }
}
```

**Approach 2 — keep all status colors fixed regardless of theme.** Simpler;
status colors stay AA (≥5:1) in every theme but do not reach the ≥8:1 the HC
preset offers for the three mapped statuses. The HC theme still improves
container/text/border contrast via the tokens; only the status badges stay at
standard contrast.

**Recommendation:** Approach 1, because the entire purpose of an HC theme is
maximal contrast, and leaving the status badges at 5:1 while everything else is
8:1 would be an inconsistent half-measure. The cost is hand-picking two
high-contrast values (`ERROR_HC`, `IGNORED_HC`) and verifying they clear 7:1.
The implementer must compute and test those two values.

### 3.8 ABDD audit sheet update

The ABDD audit sheet (`docs/src/abdd-audit.md`) gains a new section: "High
contrast themes — verify each status badge and key text element reaches ≥7:1
under both HC presets." This is a manual verification row, consistent with the
sheet's existing format.

---

## 4. Testing plan

1. Unit test: `tokens_for` returns the HC presets for the two HC variants, and
   they differ from the standard presets.
2. Unit test: every `Theme::all()` entry has a theme→i18n-key mapping.
3. Unit test (Approach 1): `ERROR_HC` and `IGNORED_HC` clear 7:1 against white,
   using the same contrast helper as the v0.35.0 status-color test.
4. Manual: select each HC theme via the RFC 093 picker; verify the whole UI
   restyles (containers, text, borders, status badges).
5. Manual: greyscale-display check (per ABDD) confirms status remains
   icon+text distinguishable in HC themes.
6. Manual: persistence round-trip — select HC Light, restart, confirm it loads.
7. Manual: open a pre-RFC-094 `prefs.yaml`, confirm it still loads (Light).

---

## 5. Open questions

### 5.1 Approach 1 vs Approach 2 for status colors

Resolved in §3.7 with a recommendation (Approach 1) but flagged for explicit
sign-off at implementation review, because it is the one decision that changes
runtime behavior of the existing (non-HC) themes if done carelessly. Approach 1
must preserve the exact current colors for Light/Dark — only the HC themes get
the elevated values. The implementer must confirm Light/Dark status badges are
pixel-identical before and after.

### 5.2 Dark theme container styling completeness

RFC 092 migrates containers to tokens, but the standard `Tokens::dark()` preset
has not been visually verified in aaai (the spike only exercised light). Before
HC dark ships, the plain Dark theme's container/panel/empty-state rendering must
be visually confirmed under Xvfb. If Dark reveals layout issues (e.g. a
hardcoded light color that RFC 092 missed), those are fixed here or in a RFC 092
follow-up, not deferred.

### 5.3 Number of themes in the picker

With System hidden (RFC 093 §5.1), the picker shows four: Light, Dark, HC Light,
HC Dark. Confirm this is not too many for the dropdown and that the labels are
unambiguous at a glance. Consider grouping ("Standard" / "High contrast") only
if user testing shows confusion — not pre-emptively.

---

## 6. Alternatives considered

- **A single "high contrast" toggle instead of two themes.** Rejected: a toggle
  cannot express "high-contrast *dark*"; users who need both dark and high
  contrast would be unserved. Two explicit themes cover the matrix.
- **Ship HC themes without the picker (config-file only).** Rejected: the users
  who need HC are the least likely to hand-edit YAML. Without RFC 093 the
  feature delivers no value, which is why RFC 093 is a hard dependency.
- **Derive HC from the standard preset by a contrast-boosting transform.**
  Rejected: snora-design already ships hand-tuned, AA-verified HC presets;
  a runtime transform would be less accurate and unverified.
