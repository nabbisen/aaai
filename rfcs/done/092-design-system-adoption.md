# RFC 092 — Design System Adoption (snora-design tokens)

**Status:** Implemented (v0.36.0)
**Target release:** pre-v1.0.0 (after the contrast fix in v0.35.0)
**Related area:** GUI styling, accessibility, theming, snora-design integration
**Depends on:** snora-design 0.25.1 (`design` feature)
**Related RFCs:** RFC 093 (theme picker UI), RFC 094 (high-contrast themes)
**Authors:** nabbisen / project maintainers

---

## 1. Summary

aaai currently styles its GUI with hand-rolled color constants (`theme.rs`)
and container style functions (`style.rs`). This RFC proposes migrating that
styling to the snora-design token system (`snora::design`, the `design`
feature on the snora dependency), so that:

1. Container surfaces (cards, panels) derive their background, border, and
   radius from a single resolved `Tokens` bundle rather than per-call literals.
2. Buttons use the four semantic snora-design style functions (`primary`,
   `secondary`, `ghost`, `danger`) while keeping aaai's ABDD-compliant padding.
3. The styling becomes theme-aware in a structured way, which is the
   precondition for RFC 093 (theme picker) and RFC 094 (high-contrast themes).

This RFC covers **container and button styling only**. It deliberately does
**not** cover the theme-picker UI (RFC 093) or high-contrast presets (RFC 094);
those are separable and are specified in their own RFCs so each can be
reviewed, implemented, and reverted independently.

The v0.34.0 evaluation spike (snora 0.18 → 0.25 bump + a throwaway design-token
migration) proved this migration compiles cleanly and is mechanical. This RFC
is the careful, reviewed version of that spike, with the over-reaching parts
removed.

---

## 2. Motivation

### 2.1 What is wrong with the current approach

The current `theme.rs` and `style.rs` carry three problems:

1. **Literal colors scattered across views.** Several view files contain inline
   `Color::from_rgb(...)` literals for borders and backgrounds (e.g. the
   empty-state panels in `main_view.rs`, the dialog chrome in `nav_guard.rs`
   and `settings_dialog.rs`). There is no single source of truth, so a theme
   change requires editing many files.

2. **No path to alternate themes.** The styling is hardcoded to a single light
   palette. There is no structured way to introduce a dark theme that actually
   restyles containers, or the high-contrast themes RFC 094 wants. (The
   `AppTheme` enum has a `Dark` variant, but it only swaps iced's built-in
   widget palette; aaai's own custom containers stay light.)

3. **Accessibility is verified ad hoc.** The contrast fix that precedes this
   RFC (v0.35.0) hardcoded AA-tested values into `theme.rs`. That is correct
   but manual. The snora-design presets are AA-tested at the source, so
   adopting them makes the guarantee structural rather than a property each
   maintainer must re-verify by hand.

### 2.2 Why snora-design specifically

aaai already depends on snora for `AppLayout`, `Sheet`, and `Toast`. The
`design` feature is an additive, opt-in feature on that same dependency — no
new third-party crate is introduced beyond `snora-design`, which snora pulls in
transitively. The token presets are WCAG-AA tested upstream, and the style
functions are exactly the right granularity (they return `button::Style` /
`container::Style`, leaving widget construction — and therefore padding — to
the caller, which aaai needs for its ≥44 px ABDD tap targets).

---

## 3. Detailed design

### 3.1 Dependency

Change the snora dependency in the workspace `Cargo.toml`:

```toml
# Before (v0.35.0)
snora = { version = "0.25.1" }

# After (this RFC)
snora = { version = "0.25.1", features = ["design"] }
```

This activates `snora::design::{Tokens, Color, Palette, ...}` and the style
bridge `snora::design::style::{button, container}`.

### 3.2 Token resolution module

Add `crates/aaai-gui/src/design_tokens.rs`:

```rust
use aaai_core::profile::prefs::Theme as AppTheme;
use snora::design::Tokens;

/// Resolve the snora-design token bundle for an aaai theme preference.
pub fn tokens_for(theme: &AppTheme) -> Tokens {
    match theme {
        AppTheme::Light  => Tokens::light(),
        AppTheme::Dark   => Tokens::dark(),
        AppTheme::System => Tokens::light(),  // until OS query exists
        // HC variants are introduced by RFC 094, not here.
    }
}
```

Note: under this RFC alone, `AppTheme` retains its current three variants
(`Light`, `Dark`, `System`). RFC 094 adds the two high-contrast variants and
extends this `match`. Keeping the variant set unchanged here means this RFC can
ship without RFC 094.

### 3.3 App state

Add one field to `App`:

```rust
pub struct App {
    // ...
    pub theme: AppTheme,
    /// Resolved design tokens for the active theme. Recomputed on theme change.
    pub design_tokens: snora::design::Tokens,
}
```

Initialized in `App::new` from the loaded preference, and recomputed in the
`SetTheme` message handler:

```rust
Message::SetTheme(t) => {
    self.theme = t;
    self.design_tokens = crate::design_tokens::tokens_for(&t);
    self.prefs.theme = t;
    self.prefs.save();
}
```

**Open issue (see §6.1):** `SetTheme` currently has no sender. Under this RFC
the field is still correct and used by every `view()` for container/button
styling; it simply resolves to the persisted theme until RFC 093 adds a picker
that emits `SetTheme`. This RFC does not depend on RFC 093 — the tokens are
read on every render regardless of whether the user can change them yet.

### 3.4 Container style migration

Rewrite `style.rs` so each style function takes `Tokens` by value and returns a
closure (matching snora's own helper convention; by-value avoids binding a
`&Tokens` lifetime into the caller's `view` signature):

```rust
use snora::design::Tokens;
use snora::design::style::container as snora_container;

/// Cards (Opening folder cards, inspector cards). Delegates to snora-design.
pub fn card_style(tokens: Tokens) -> impl Fn(&iced::Theme) -> container::Style {
    move |_theme| snora_container::card_surface(&tokens)
}

/// Toolbar / filter bar / bottom bar. No direct snora equivalent; hand-rolled
/// with a token-derived border color.
pub fn panel_style(tokens: Tokens) -> impl Fn(&iced::Theme) -> container::Style {
    let border = to_iced(tokens.palette.border);
    move |_theme| container::Style { /* token-derived */ ..Default::default() }
}

/// Empty-state placeholder panels. Token-derived border, transparent fill.
pub fn empty_state_panel_style(tokens: Tokens) -> impl Fn(&iced::Theme) -> container::Style {
    // as above
}
```

Call sites change from `card_style` (a bare fn reference) to
`card_style(app.design_tokens.clone())`. The spike measured this as ~8 call
sites across `opening.rs`, `main_view.rs`, and `app.rs`.

`Tokens` is `Clone` and cheap (a handful of color structs); cloning once per
styled container in a retained-mode `view()` is acceptable, and is the pattern
snora's own helpers use.

### 3.5 Button style migration

For each semantic button, replace the iced built-in style with the snora-design
style function, keeping the existing `.padding(...)` for ABDD compliance:

```rust
// Before
button(text("Save and leave").size(13))
    .on_press(Message::NavGuardSaveAndLeave)
    .padding([6.0, 14.0]);  // iced default primary style

// After
let t = app.design_tokens.clone();
button(text("Save and leave").size(13))
    .on_press(Message::NavGuardSaveAndLeave)
    .padding([6.0, 14.0])  // ABDD padding preserved
    .style(move |_theme, status| snora::design::style::button::primary(&t, status));
```

Thin wrappers live in `style.rs` (`btn_primary`, `btn_secondary`, `btn_ghost`,
`btn_danger`) so view code does not import the snora path directly.

**Button-to-variant mapping (complete inventory):**

| Location | Button | Variant |
|---|---|---|
| `nav_guard.rs` | Save and leave | primary |
| `nav_guard.rs` | Stay here | secondary |
| `nav_guard.rs` | Discard and leave | danger |
| `nav_guard.rs` | More choices | ghost |
| `opening.rs` | Check changes | primary |
| `opening.rs` | Pick folder (×2) | secondary |
| `opening.rs` | Recent project rows | ghost |
| `main_view.rs` | Save and continue | primary |
| `main_view.rs` | toolbar buttons | secondary |
| `inspector.rs` | Add rule / Add chip | secondary |
| `settings_dialog.rs` | Save | primary |
| `settings_dialog.rs` | Cancel | secondary |
| `settings_dialog.rs` | Remove dir / Add dir | ghost |
| `batch.rs` | Approve selected | primary |
| `batch.rs` | Close | secondary |

This inventory is the implementation checklist. It is intentionally exhaustive
so the implementer can verify every button is migrated and none is missed.

### 3.6 What this RFC does NOT touch

- **Status colors** (`theme.rs` `OK_COLOR` … `REMOVED_COLOR`). These stay as
  the AA-tested constants from the v0.35.0 contrast fix. See §4 for why they
  are not migrated to snora roles wholesale.
- **The dialog chrome** in `nav_guard.rs` / `settings_dialog.rs` (white
  background, drop shadow). snora-design has no modal-dialog surface role;
  these stay hand-rolled with token-derived border colors.
- **Theme picker UI** — RFC 093.
- **High-contrast themes** — RFC 094.

---

## 4. Status colors: why they are not fully migrated to snora roles

This is the design decision that the snora team's feedback rejection settled,
and it is recorded here so it is not relitigated.

aaai has five audit statuses; snora-design's palette has four status roles:

| aaai status | meaning | snora role | mapping |
|---|---|---|---|
| OK | passed | `success` | ✓ clean |
| Pending | needs review | `warning` | ✓ clean |
| Failed | rule no longer matches | `danger` | ✓ clean |
| Error | file could not be read | — | ✗ no role |
| Ignored | excluded from review | — | ✗ no role |

snora is a general-purpose framework and (per the maintainers' explicit
decision) will not add aaai-specific status roles. Therefore:

- **OK / Pending / Failed** *could* be re-derived from `tokens.palette.success`
  / `warning` / `danger` at runtime. The v0.35.0 fix instead adopted the same
  values as constants. Whether to switch these three to live token reads is an
  **optional sub-decision** of this RFC (§4.1).
- **Error / Ignored** have no snora role and must remain hand-picked constants
  regardless.

### 4.1 Sub-decision: constants vs live token reads for OK/Pending/Failed

**Option A (recommended): keep them as constants.** The v0.35.0 values are
already the snora light-preset values. Keeping them as constants means the
status colors do not change when the theme changes — which is arguably correct,
because a "Failed" badge should look the same urgent red in light and dark
themes. Status semantics are theme-independent.

**Option B: re-derive OK/Pending/Failed from `tokens.palette` at render.** This
makes them follow the active preset (including the future HC presets from RFC
094, where `danger` is an even higher-contrast red). The cost is that Error and
Ignored would then be the only hardcoded status colors, creating a split.

The recommendation is **Option A** for this RFC, with Option B reconsidered
inside RFC 094 specifically for the high-contrast case, where following the
preset is the entire point.

---

## 5. Migration & testing plan

1. Enable the `design` feature.
2. Add `design_tokens.rs` + the `App.design_tokens` field.
3. Migrate `style.rs` container functions; update the ~8 call sites.
4. Migrate the 15 buttons in §3.5's inventory.
5. Verify: `cargo check` 0 warnings; all 213+ tests pass; i18n unchanged.
6. Visual verification under Xvfb (`LIBGL_ALWAYS_SOFTWARE=1
   GALLIUM_DRIVER=llvmpipe`) — confirm cards, panels, and all four button
   variants render with no regression against the v0.35.0 baseline.
7. Add a unit test asserting `tokens_for` returns distinct palettes for Light
   vs Dark (guards the resolver).

No new i18n keys. No CLI changes. No schema changes.

---

## 6. Open questions

### 6.1 SetTheme has no sender

`SetTheme` is dead today. This RFC makes `design_tokens` a live, used field
even so (every render reads it). RFC 093 supplies the sender. **This RFC must
not be blocked on RFC 093** — it ships a correct token pipeline that happens to
resolve to the persisted theme until the picker exists. The implementer should
confirm `cargo` does not warn about `SetTheme` being unused after this RFC; if
it does, the `#[allow(dead_code)]` should be added with a comment pointing at
RFC 093 rather than removing the variant.

### 6.2 Tokens clone cost

Each styled container/button clones `Tokens` once. For a screen with ~30 styled
elements that is ~30 clones per frame. This is expected to be negligible
(measured as no perceptible difference in the spike), but the implementer
should spot-check frame time under a large file tree (1000+ entries) and note
the result in the implementation PR.

### 6.3 Dialog surface role

snora-design has no modal-dialog surface role, so dialog chrome stays
hand-rolled. If a future snora version adds one, a follow-up can adopt it. Not
in scope here.

---

## 7. Alternatives considered

- **Do nothing (keep hand-rolled styling).** Rejected: leaves the literal-color
  scatter and blocks RFC 093/094.
- **Adopt the full snora-design system including its status palette.** Rejected:
  snora will not carry aaai's 5-status vocabulary; forcing Error/Ignored into
  `info`/`text_muted` would lose the design-doc-p.9 status distinction.
- **Write our own token system.** Rejected: duplicates snora-design's AA-tested
  work for no benefit; aaai already depends on snora.
