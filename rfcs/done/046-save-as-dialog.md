# RFC 046 — Save-as dialog for new approvals files

**Status.** Implemented (v0.24.0)
**Tracks.** New-user workflow, data persistence
**Touches.** `crates/aaai-gui/src/app.rs` (SaveDefinition handler,
new message, pending-leave flag, NavGuardSaveAndLeave update),
`crates/aaai-gui/locales/{en,ja}.yaml` (1 new key × 2).

## Problem

When a user starts a fresh audit (no "Approvals file" specified in
Optional settings), approves several entries, and presses `Ctrl+S` or
the Save toolbar button, the current handler does:

```rust
if path.as_os_str().is_empty() {
    self.push_toast(ToastIntent::Error, ..., "No definition file path set.");
    return Task::none();   // ← dead end
}
```

There is no way forward. The user has valid approvals in memory but
cannot persist them. The same dead end is hit from `NavGuardSaveAndLeave`
when the user chooses "Save and Leave" with no path set.

## Fix

Replace the error bailout with a native `rfd::AsyncFileDialog::save_file()`
dialog. The chosen path is applied to `App.definition_path` before the
save proceeds, so all subsequent `Ctrl+S` presses go directly to disk
without another dialog.

## New state

```rust
/// RFC 046 — when the save-as dialog is opened from the nav-guard
/// "Save and Leave" path, we need to navigate after the save completes.
/// This flag tells `DefinitionSavePathPicked` to call `do_leave_to_opening()`.
pub pending_leave_to_opening: bool,
```

## New message

```rust
/// RFC 046 — result of the save-file dialog triggered by SaveDefinition
/// or NavGuardSaveAndLeave when `definition_path` is empty.
DefinitionSavePathPicked(Option<std::path::PathBuf>),
```

## Updated handlers

### `SaveDefinition` (empty-path branch)

```rust
// Before:
if path.is_empty() {
    push_toast(Error, "No definition file path set.");
    return Task::none();
}

// After:
if path.is_empty() {
    let title = t!("dialog.save_approvals_file").to_string();
    return Task::perform(
        async move {
            rfd::AsyncFileDialog::new()
                .set_title(title)
                .set_file_name("audit.yaml")
                .add_filter("YAML", &["yaml", "yml"])
                .save_file()
                .await
                .map(|h| h.path().to_path_buf())
        },
        Message::DefinitionSavePathPicked,
    );
}
```

### `NavGuardSaveAndLeave` (empty-path branch)

Before: showed an error toast and closed the nav guard.

After: sets `self.pending_leave_to_opening = true`, closes the nav guard,
then opens the same save-file dialog.

### `DefinitionSavePathPicked`

```rust
Message::DefinitionSavePathPicked(None) => {
    // User cancelled — clear pending-leave, no toast.
    self.pending_leave_to_opening = false;
}

Message::DefinitionSavePathPicked(Some(path)) => {
    self.definition_path = path.display().to_string();
    // Now proceed with the normal save logic (inline — not via
    // Message::SaveDefinition to avoid a round-trip through update()).
    if let Some(def) = &self.definition {
        match config_io::save(def, &path, true) {
            Ok(()) => {
                self.dirty = false;
                self.last_saved_at = Some(chrono::Utc::now());
                self.push_toast(ToastIntent::Success, ..., path);
                if self.pending_leave_to_opening {
                    self.pending_leave_to_opening = false;
                    self.do_leave_to_opening();
                }
            }
            Err(e) => {
                // error toast — stay, don't navigate
                self.pending_leave_to_opening = false;
            }
        }
    }
}
```

## i18n (1 new key × 2 locales)

```yaml
# en.yaml
dialog:
  save_approvals_file: "Save Approvals File"

# ja.yaml
dialog:
  save_approvals_file: "承認ファイルを保存"
```

## Acceptance criteria

- [ ] `SaveDefinition` with empty `definition_path` opens save-file dialog
- [ ] `DefinitionSavePathPicked(Some(path))` saves, sets path, shows success toast
- [ ] `DefinitionSavePathPicked(None)` is a silent no-op
- [ ] After a successful save-as, subsequent `Ctrl+S` saves to the chosen path directly
- [ ] Window title updates to show the new filename (via existing dynamic title logic)
- [ ] `NavGuardSaveAndLeave` with empty path opens save dialog + navigates on success
- [ ] 1 new i18n key in en.yaml + ja.yaml
- [ ] `check-i18n-keys.py --quiet` → 0/0/0 (214/214/214) — net 0 change: +1 new, −1 removed
- [ ] `cargo check --workspace --all-targets` warning-free
- [ ] All existing tests pass (101 / 70 / 15)
