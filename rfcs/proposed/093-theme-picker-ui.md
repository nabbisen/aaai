# RFC 093 — Theme Picker UI

**Status:** Proposed
**Target release:** pre-v1.0.0 (after RFC 092)
**Related area:** GUI settings, theming, preferences persistence
**Depends on:** RFC 092 (design system adoption — provides the token pipeline)
**Related RFCs:** RFC 094 (high-contrast themes — adds options to this picker)
**Authors:** nabbisen / project maintainers

---

## 1. Summary

The `SetTheme` message exists in aaai but has **no sender**: there is no UI
control anywhere that lets a user change the theme. The theme is read from
`prefs.yaml` on startup and is otherwise immutable through the GUI. This RFC
adds a theme picker to the Settings dialog so the theme becomes user-selectable
and the `SetTheme` message becomes live.

This RFC is small and self-contained. It exists separately from RFC 092 because
the token pipeline (RFC 092) is useful on its own, and the picker UI is useful
only once that pipeline exists. Splitting them keeps each reviewable and
revertible.

---

## 2. Motivation

### 2.1 The dead SetTheme message

A code audit found that `Message::SetTheme(AppTheme)` is constructed nowhere.
Its handler runs (it persists the theme and, after RFC 092, recomputes
`design_tokens`), but nothing ever emits it. This is latent dead code that has
existed since the `AppTheme` enum was introduced.

Two ways to resolve dead code: delete it, or wire it. Deleting it would mean
aaai ships v1.0 with a `Dark` theme variant that users can only reach by
hand-editing `~/.aaai/prefs.yaml` — a poor experience that also makes the
forthcoming high-contrast themes (RFC 094) inaccessible. Wiring it is the right
resolution: the enum, the persistence, and the handler are all already correct;
only the UI control is missing.

### 2.2 Accessibility dependency

RFC 094 introduces high-contrast themes specifically for users who need them.
A high-contrast theme that cannot be selected through the UI is useless to its
target audience (who are the least likely to hand-edit a YAML file). The theme
picker is therefore a hard prerequisite for RFC 094 delivering any value.

---

## 3. Detailed design

### 3.1 Location

The picker goes in the existing Settings dialog (`settings_dialog.rs`), which
today contains a language picker and an ignored-directories editor. A theme
picker is the natural third setting and requires no new dialog or navigation.

### 3.2 Control type

Use an `iced::widget::pick_list`, mirroring the existing language picker in the
same dialog for visual and interaction consistency. A pick_list (dropdown) is
preferred over a radio group because:

- It matches the adjacent language control (consistency).
- It scales cleanly when RFC 094 adds two more options (5 total) without
  consuming vertical dialog space.
- It is keyboard-navigable (arrow keys + Enter), satisfying ABDD keyboard
  completeness.

### 3.3 Options and labels

Under this RFC alone (before RFC 094), the picker offers three options:

| Value | Display label (en) | Display label (ja) |
|---|---|---|
| `Theme::Light` | Light | ライト |
| `Theme::Dark` | Dark | ダーク |
| `Theme::System` | Match system | システムに合わせる |

After RFC 094, two more rows appear (High Contrast Light / Dark). This RFC must
build the options list by iterating a `Theme::all()` slice (added here) rather
than hardcoding three entries, so RFC 094 only has to extend the enum and the
slice — not touch the picker code.

```rust
impl Theme {
    /// All user-selectable themes, in display order.
    /// RFC 094 appends the two high-contrast variants here.
    pub fn all() -> &'static [Theme] {
        &[Theme::Light, Theme::Dark, Theme::System]
    }
}
```

### 3.4 i18n keys

New keys under the `settings.*` namespace (both en and ja):

```
settings.theme              "Theme" / "テーマ"
settings.theme_light        "Light" / "ライト"
settings.theme_dark         "Dark"  / "ダーク"
settings.theme_system       "Match system" / "システムに合わせる"
```

The label-resolution function maps a `Theme` value to its i18n key, so RFC 094
adds only two more keys and two more match arms.

### 3.5 Draft vs commit semantics

The Settings dialog already uses a draft/commit pattern (`settings_draft` holds
pending changes; Save commits, Cancel discards). The theme picker follows the
same pattern:

- Selecting a theme in the picker updates `settings_draft.theme` only.
- **Live preview (recommended):** on selection, also emit `SetTheme(draft)` so
  the user sees the theme applied immediately behind the dialog. On Cancel,
  emit `SetTheme(original)` to revert.
- **Alternative (simpler):** apply only on Save. No live preview.

**Recommendation:** live preview. Theme choice is inherently visual; a user
picking "Dark" wants to see dark immediately, and reverting on Cancel is one
extra message. The slight added complexity is justified by the UX gain. The
implementer should confirm the revert-on-Cancel path with a manual test.

### 3.6 Message flow

```
User opens Settings
  └─ pick_list shows current theme (from settings_draft.theme)

User selects "Dark"
  ├─ Message::SettingsThemeChanged(Theme::Dark)
  │     └─ settings_draft.theme = Dark
  │     └─ (live preview) → also dispatch SetTheme(Dark)
  └─ dialog re-renders with Dark selected

User clicks Save
  ├─ Message::SettingsSave
  │     └─ commit draft → prefs.theme = Dark, prefs.save()
  │     └─ SetTheme(Dark) already applied (live preview)

  OR User clicks Cancel
  ├─ Message::SettingsCancel
  │     └─ discard draft
  │     └─ (live preview) → SetTheme(original_theme) to revert
```

A new message `SettingsThemeChanged(Theme)` is added (parallel to the existing
`SettingsLanguageChanged(String)`). The existing `SetTheme(Theme)` is the
apply/commit message and finally gets its sender.

### 3.7 Persistence

No schema change. `prefs.theme` already exists and already serializes (it is a
`Theme` enum with serde). The `Dark`/`System` values already round-trip; this
RFC just makes them reachable. RFC 094's two new variants will need the serde
`rename` attributes that RFC 094 specifies.

---

## 4. Testing plan

1. Unit test: `Theme::all()` returns the three (later five) variants in display
   order.
2. Unit test: the theme→i18n-key resolver covers every `Theme::all()` entry
   (guards against a variant being added to the enum but not the picker).
3. Manual: open Settings, change theme, observe live preview, Save, reopen app,
   confirm persistence.
4. Manual: open Settings, change theme, observe live preview, Cancel, confirm
   revert to the original theme.
5. Manual: keyboard-only operation of the picker (Tab to it, arrow keys, Enter).

---

## 5. Open questions

### 5.1 System theme detection

`Theme::System` currently falls back to Light (iced 0.14 exposes no reliable
OS-dark-mode query). This RFC ships `System` as an option that behaves like
Light until detection is available. **Question:** should `System` be hidden
from the picker until it genuinely detects the OS theme, to avoid a "Match
system" option that silently does nothing on a dark-mode OS?

**Recommendation:** hide `System` from `Theme::all()` until detection works,
but keep the enum variant (it deserializes from existing prefs files). This
avoids shipping a visibly broken option. The implementer should make
`Theme::all()` exclude `System` and add a `// TODO(RFC future): include System
once OS detection lands` comment. This is a reversal of §3.3's table — resolve
it during implementation review; the conservative choice is to hide it.

### 5.2 Live preview scope

Live preview restyles the whole window behind a modal dialog. Confirm under
Xvfb that the dialog backdrop and the restyled window compose correctly and
there is no flash or z-order glitch when the theme changes with the dialog open.

---

## 6. Alternatives considered

- **Radio group instead of pick_list.** Rejected: inconsistent with the
  adjacent language picker; consumes vertical space that grows with RFC 094.
- **Theme toggle in the toolbar (not Settings).** Rejected: theme is a
  persistent preference, not a frequent action; it belongs with other
  preferences, not in the primary action bar.
- **Delete the Dark variant and ship light-only for v1.0.** Rejected: throws
  away working, persisted, tested theme infrastructure and forecloses RFC 094.
