# RFC 038 — Keyboard shortcuts help overlay

**Status.** Implemented (v0.23.0 — Phase 15)
**Tracks.** GUI discoverability, polish
**Touches.** `crates/aaai-gui/src/app.rs` (1 string fix, 2 messages,
1 new keyboard handler, view overlay), `crates/aaai-gui/src/views/help_overlay.rs`
(new file), `crates/aaai-gui/src/views/mod.rs`,
`crates/aaai-gui/locales/{en,ja}.yaml` (≈18 keys × 2 locales).

## Summary

There are two small, unrelated items in this RFC:

**Item A — fix the one remaining hardcoded toast body.** The
`CommitBatchApprove` handler formats its toast body as
`&format!("{} entries approved.", count)` — hardcoded English.
This was missed by earlier sweeps because it starts with `{}`
rather than an uppercase letter.

**Item B — `?` keyboard shortcuts help overlay.** The app has at
least 8 keyboard shortcuts (Ctrl+S, Ctrl+R, Ctrl+Z, Ctrl+E,
`↑/↓`, Tab, `/`) that users discover only by reading the source
or the docs. A `?` keypress (or a `?` button in the footer) opens
a modal overlay listing all available shortcuts in a clean two-column
table. The overlay uses the same `stack! + backdrop + center()`
pattern established in RFC 036 (Settings dialog).

## Item A — batch approved count string

```yaml
# en.yaml
toast:
  batch_approved_count: "%{count} entries approved."

# ja.yaml
toast:
  batch_approved_count: "%{count} 件を承認しました。"
```

Call site (app.rs line 849):

```rust
// Before:
&format!("{} entries approved.", count),

// After:
t!("toast.batch_approved_count", count = count.to_string()).as_ref(),
```

## Item B — keyboard help overlay

### Trigger

- Pressing `?` on the main screen opens the help overlay.
- Pressing `?` again, `Escape`, or clicking the backdrop closes it.
- A small `?` button added to the footer (left of ⚙) provides
  a pointer-accessible path to the same overlay.

### Content

A two-column table (`Shortcut | Action`) listing all keyboard
shortcuts currently active on the main screen:

| Shortcut | Action |
|---|---|
| `Ctrl+S` | Save definition |
| `Ctrl+R` | Re-run audit |
| `Ctrl+Z` | Undo last approval |
| `Ctrl+E` | Export report |
| `↑ / ↓` | Navigate file list |
| `Tab / Shift+Tab` | Cycle pane focus |
| `Enter` | Toggle approval |
| `/` | Focus search |
| `?` | Show this help |

### New messages

```rust
ToggleHelp,   // opens or closes the overlay
CloseHelp,    // closes (from backdrop click or Escape)
```

### New state

```rust
pub help_open: bool,
```

Initialized `false`. `ToggleHelp` flips it; `CloseHelp` sets it
false. Only rendered while `screen == Screen::Main` (the shortcuts
are main-screen-specific).

### View integration

Same stack pattern as RFC 036:

```rust
if self.help_open && matches!(self.screen, Screen::Main) {
    stack![base, backdrop, center(help_overlay::view())].into()
} else {
    base
}
```

`help_overlay::view()` renders a static table — no App state
needed, no lifecycle complexity.

### Keyboard handler

In the existing `KeyPressed` dispatch on the main screen:

```rust
(Key::Character("?"), _) => Message::ToggleHelp,
// Escape closes any overlay that is open:
(Key::Named(Named::Escape), _) if self.help_open => Message::CloseHelp,
```

### Footer button

Alongside the existing ⚙ Settings button:

```
[unsaved notice] ─────────  [?]  [⚙]  v0.23.0
```

### i18n keys (~18 × 2 locales)

```yaml
# en.yaml
help:
  title:           "Keyboard Shortcuts"
  shortcut_label:  "Shortcut"
  action_label:    "Action"
  close:           "Close"
  save:            "Save definition"
  rerun:           "Re-run audit"
  undo:            "Undo last approval"
  export:          "Export report"
  navigate:        "Navigate file list"
  cycle_pane:      "Cycle pane focus"
  approve:         "Toggle approval"
  search:          "Focus search"
  show_help:       "Show / hide this help"

# ja.yaml
help:
  title:           "キーボードショートカット"
  shortcut_label:  "ショートカット"
  action_label:    "操作"
  close:           "閉じる"
  save:            "定義ファイルを保存"
  rerun:           "監査を再実行"
  undo:            "最後の承認を取り消し"
  export:          "レポートを出力"
  navigate:        "ファイルリストを移動"
  cycle_pane:      "ペインフォーカスを切り替え"
  approve:         "承認のトグル"
  search:          "検索フォーカス"
  show_help:       "このヘルプを表示 / 非表示"
```

### Locale counts

| Pre-RFC | Post-RFC |
|---|---|
| 192 / 192 / 192 | **206 / 206 / 206** (+1 toast key + 13 help keys = 14 × 2) |

## Acceptance criteria

- [ ] `toast.batch_approved_count` key + `%{count}` substitution at L849
- [ ] `help_open: bool` field in `App`
- [ ] `Message::ToggleHelp` + `Message::CloseHelp` defined and handled
- [ ] `?` key → `ToggleHelp` on main screen
- [ ] `Escape` key → `CloseHelp` when `help_open`
- [ ] `?` button in footer (left of ⚙)
- [ ] `views/help_overlay.rs` — shortcut table, 9 rows
- [ ] Modal overlay via `stack!` (same pattern as RFC 036)
- [ ] 14 new i18n keys in en.yaml + ja.yaml
- [ ] `check-i18n-keys.py --quiet` → 0/0/0 (206/206/206)
- [ ] `cargo check --workspace --all-targets` warning-free
- [ ] All existing tests pass (99 / 70 / 15)
