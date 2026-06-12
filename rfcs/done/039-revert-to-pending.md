# RFC 039 — Revert-to-Pending + Opening profile delete

**Status.** Implemented (v0.23.0 — Phase 15)
**Tracks.** Inspector workflow completeness, Opening screen UX
**Touches.** `crates/aaai-gui/src/app.rs` (1 new message, 1 handler,
1 keyboard shortcut), `crates/aaai-gui/src/views/inspector.rs` (new button),
`crates/aaai-gui/src/views/opening.rs` (delete button per row),
`crates/aaai-gui/locales/{en,ja}.yaml` (3 new keys × 2 locales).

## Summary

Two small UX gaps addressed in one RFC:

**Item A — Inspector "Revert to Pending."** Once an entry is approved
(status `OK`), the only way to unapprove it is via the `Ctrl+Z` undo
stack — which is session-local and can't touch entries loaded from a
saved `audit.yaml`. There is no UI for "remove this approval, make this
entry Pending again." This RFC adds a **Revert to Pending** button in the
Inspector, visible only when the selected entry is OK.

**Item B — Opening screen profile delete.** The recent-projects list has
an `[Open →]` button per row but no delete. `Message::DeleteProfile(idx)`
already exists and is handled (used by a different surface). This RFC
wires a `[×]` button to the end of each profile row.

## Item A — Revert to Pending

### When it appears

The Inspector already loads the existing `AuditEntry` when an OK file is
selected. The new **Revert to Pending** button is shown as a secondary
action alongside the existing "Approve" button — but only when the
currently-selected diff has `AuditStatus::Ok`. For Pending / Failed /
Error / no selection, the button is absent.

### Message

```rust
RevertSelectedEntry,
```

### Handler

```rust
Message::RevertSelectedEntry => {
    if let (Some(idx), Some(def)) = (self.selected_index, &mut self.definition) {
        if let Some(diff) = self.diffs.get(idx) {
            let path = diff.path.clone();
            if let Some(pos) = def.entries.iter().position(|e| e.path == path) {
                def.entries.remove(pos);
                self.dirty = true;
                self.push_toast(
                    ToastIntent::Info,
                    t!("toast.reverted").as_ref(),
                    t!("toast.reverted_path", path = path).as_ref(),
                );
                return self.start_async_rerun();
            }
        }
    }
}
```

### Keyboard shortcut

`Ctrl+Shift+Z` — "undo the undo" mnemonic — on the main screen, visible
in the `?` help overlay (added to the shortcuts table and i18n keys).

### Inspector view

Below the existing action row in the Inspector, when the entry is OK:

```
╔══════════════════════════════╗
║  ...strategy / rules / etc.  ║
║                              ║
║  [Approve]   [Revert to ↻]   ║
╚══════════════════════════════╝
```

The Revert button uses a muted style (not the destructive-red of a delete
action, since it's a non-destructive "unset approval" rather than data
deletion) and sits alongside the existing Approve button.

## Item B — Opening screen profile delete

### `[×]` button

At the right edge of each profile row in `recent_projects_section()`,
a small `[×]` button calls `Message::DeleteProfile(orig_idx)`. The
original usize index is already threaded through (it was already preserved
for `LoadProfile`).

Layout change — before:
```
[Open →]  before/after paths  (time ago)
```

After:
```
[Open →]  before/after paths  (time ago)  [×]
```

No confirmation dialog. The action is fast and recoverable (re-add
profile from the Opening screen inputs).

## i18n keys (3 + 2 shortcut keys = 5 × 2 locales)

```yaml
# en.yaml
toast:
  reverted:       "Reverted to Pending"
  reverted_path:  "Reverted: %{path}"
inspector:
  revert_to_pending: "Revert to Pending"
help:
  revert:         "Revert selected entry to Pending"
  delete_profile: "Delete recent profile"

# ja.yaml
toast:
  reverted:       "Pending に戻しました"
  reverted_path:  "取り消し: %{path}"
inspector:
  revert_to_pending: "Pending に戻す"
help:
  revert:         "選択エントリを Pending に戻す"
  delete_profile: "最近のプロファイルを削除"
```

The shortcut table in `help_overlay.rs` gains two new rows:
- `Ctrl+Shift+Z` → `t!("help.revert")`
- *(no new keyboard shortcut for profile delete — mouse-only action)*

### Locale counts

| Pre-RFC | Post-RFC |
|---|---|
| 206 / 206 / 206 | **210 / 210 / 210** (+4 keys × 2 = 8 entries; `help.delete_profile` removed — mouse-only action) |

## Acceptance criteria

- [ ] `Message::RevertSelectedEntry` defined and handled
- [ ] Revert button appears in Inspector when status is `AuditStatus::Ok`
- [ ] Handler removes entry from definition, sets dirty, triggers async rerun
- [ ] `[×]` button on each Opening screen profile row
- [ ] `Message::DeleteProfile(orig_idx)` sent from the `[×]` button
- [ ] `Ctrl+Shift+Z` mapped to `RevertSelectedEntry` on main screen
- [ ] 5 new i18n keys in en.yaml + ja.yaml
- [ ] Help overlay shortcut table updated (+2 rows: Ctrl+Shift+Z, context note)
- [ ] `check-i18n-keys.py --quiet` → 0/0/0 (211/211/211)
- [ ] `cargo check --workspace --all-targets` warning-free
- [ ] All existing tests pass (99 / 70 / 15)
