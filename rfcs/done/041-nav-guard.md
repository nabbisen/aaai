# RFC 041 — Unsaved-changes navigation guard dialog

**Status.** Implemented (v0.23.0 — Phase 15)
**Tracks.** Data safety, navigation UX
**Touches.** `crates/aaai-gui/src/app.rs` (state, messages, handlers),
`crates/aaai-gui/src/views/nav_guard.rs` (new), `crates/aaai-gui/src/views/mod.rs`,
`crates/aaai-gui/locales/{en,ja}.yaml` (5 new keys × 2).

## Problem

When the user clicks "Back to Opening" (`BackToOpening`) with unsaved
changes (`self.dirty == true`), the current handler shows a passive
warning toast and **blocks navigation without explaining why or what
to do next**:

```rust
Message::BackToOpening => {
    if self.dirty {
        self.push_toast(ToastIntent::Warning,
            t!("toast.unsaved_warning"), t!("toast.save_before_leaving"));
        // Nothing else — user is stuck with no clear path forward.
    } else {
        self.screen = Screen::Opening;
        ...
    }
}
```

The user sees a toast ("Save before leaving or changes will be lost")
but the "Back" button continues to do nothing visible. There's no UI
affordance for "OK, save first" or "Leave anyway."

## Solution

Replace the passive toast with a **3-choice confirmation dialog**:

```
╔══════════════════════════════════════════╗
║  Unsaved Changes                         ║
║                                          ║
║  You have unsaved changes. Save before   ║
║  leaving, or discard them and continue?  ║
║                                          ║
║  [Cancel]  [Discard and Leave]  [Save and Leave] ║
╚══════════════════════════════════════════╝
```

The dialog uses the existing `stack! + backdrop + center()` modal
pattern (RFC 036 / RFC 038).

## New state and messages

```rust
// App field:
pub nav_guard_open: bool,

// Messages:
NavGuardOpen,            // re-used if BackToOpening fires dirty; or direct
NavGuardSaveAndLeave,    // save definition then navigate
NavGuardDiscardAndLeave, // navigate without saving
NavGuardCancel,          // close dialog, stay
```

## Handlers

```rust
Message::BackToOpening => {
    if self.dirty {
        self.nav_guard_open = true;  // show dialog, don't navigate
    } else {
        self.do_leave_to_opening();  // extracted helper
    }
}

Message::NavGuardSaveAndLeave => {
    // Inline save logic (same as SaveDefinition but navigates on success).
    let path = PathBuf::from(&self.definition_path);
    if path.as_os_str().is_empty() {
        self.push_toast(ToastIntent::Error,
            t!("toast.save_failed").as_ref(),
            t!("toast.no_definition_path").as_ref());
        self.nav_guard_open = false;
        return Task::none();
    }
    if let Some(def) = &self.definition {
        match config_io::save(def, &path, true) {
            Ok(()) => {
                self.dirty = false;
                self.last_saved_at = Some(chrono::Utc::now());
                self.nav_guard_open = false;
                self.do_leave_to_opening();
            }
            Err(e) => {
                let user_err = UserError::from_i18n("error.save.failed");
                // show error toast, stay on screen
                self.push_toast_with_hint(
                    ToastIntent::Error,
                    t!("toast.save_failed").as_ref(),
                    &format!("{}\n({})", user_err.message, e),
                    &user_err.hint,
                );
                // do NOT navigate — user needs to resolve save error first
                self.nav_guard_open = false;
            }
        }
    }
}

Message::NavGuardDiscardAndLeave => {
    self.nav_guard_open = false;
    self.dirty = false;            // discard changes
    self.do_leave_to_opening();
}

Message::NavGuardCancel => {
    self.nav_guard_open = false;
}
```

### `do_leave_to_opening()` helper

Extracts the existing navigation logic to avoid duplication:

```rust
fn do_leave_to_opening(&mut self) {
    self.screen = Screen::Opening;
    self.audit_result = None;
    self.diffs.clear();
    self.definition = None;
    self.selected_index = None;
    self.inspector = InspectorState::default();
    self.audit_dirty = false;
    self.help_open = false;    // close any overlay that may be open
}
```

## View

`views/nav_guard.rs` — pure function, no App state except for the
i18n calls:

```rust
pub fn view<'a>() -> Element<'a, Message>
```

Three buttons on the bottom row:
- `[Cancel]` → secondary style
- `[Discard and Leave]` → destructive style (red-ish)
- `[Save and Leave]` → primary style (right-most)

## Modal overlay placement

In `App::view()`, alongside RFC 036/038 overlays:

```rust
} else if self.nav_guard_open && matches!(self.screen, Screen::Main) {
    stack![base, backdrop, center(nav_guard::view())].into()
```

Backdrop click → `Message::NavGuardCancel` (same as explicit Cancel).

## i18n (5 keys × 2 locales)

```yaml
# en.yaml
nav_guard:
  title:              "Unsaved Changes"
  message:            "You have unsaved changes. Save before leaving, or discard them?"
  save_and_leave:     "Save and Leave"
  discard_and_leave:  "Discard and Leave"
  cancel:             "Cancel"

# ja.yaml
nav_guard:
  title:              "未保存の変更があります"
  message:            "未保存の変更があります。保存してから離れますか？"
  save_and_leave:     "保存して離れる"
  discard_and_leave:  "破棄して離れる"
  cancel:             "キャンセル"
```

### Locale counts

| Pre-RFC | Post-RFC |
|---|---|
| 212 / 212 / 212 | **215 / 215 / 215** (+5 new − 2 deprecated old toast keys) (+5 × 2) |

## Acceptance criteria

- [ ] `nav_guard_open: bool` in `App`
- [ ] `BackToOpening` with `dirty` → opens dialog (no longer shows passive toast)
- [ ] `NavGuardSaveAndLeave` saves + navigates on success; shows error + stays on failure
- [ ] `NavGuardDiscardAndLeave` navigates without saving
- [ ] `NavGuardCancel` closes dialog, stays
- [ ] `do_leave_to_opening()` helper replaces duplicated nav logic
- [ ] `views/nav_guard.rs` built; wired into `App::view()` stack
- [ ] 5 new i18n keys in en.yaml + ja.yaml
- [ ] `check-i18n-keys.py --quiet` → 0/0/0 (217/217/217)
- [ ] `cargo check --workspace --all-targets` warning-free
- [ ] All existing tests pass (99 / 70 / 15)
