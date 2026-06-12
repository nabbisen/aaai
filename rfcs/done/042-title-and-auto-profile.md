# RFC 042 — Dynamic window title + auto-profile on audit run

**Status.** Implemented (v0.23.0 — Phase 15)
**Tracks.** GUI polish, profile UX
**Touches.** `crates/aaai-gui/src/main.rs` (title closure), 
`crates/aaai-gui/src/app.rs` (auto-profile helper, StartAudit hook).
No i18n changes. No test additions.

## Summary

Two independent polish improvements that share a theme —
*the app should track context automatically* rather than requiring
explicit user action.

### Item A — Dynamic window title

The window title is currently a static `t!("app.title")` = "aaai"
regardless of what file is open or whether there are unsaved
changes. Compare with any professional file-based application
(text editor, spreadsheet): the title reflects the current file
and state.

Target formats:

| Situation | Title |
|---|---|
| Opening screen (or no definition) | `aaai` |
| Main screen, definition loaded, clean | `aaai — audit.yaml` |
| Main screen, definition loaded, dirty | `aaai — audit.yaml ●` |
| Main screen, no definition loaded, dirty | `aaai ●` |

The `●` marker (U+25CF BLACK CIRCLE) is the conventional
unsaved-changes indicator used by VS Code, Vim, many IDEs.

**Change:** one line in `main.rs` — the `.title()` closure gains
access to `App` fields.

### Item B — Auto-profile on audit run

Currently the Recent Projects list in the Opening screen only
contains profiles explicitly saved by the user via "Save Profile".
If a user:
1. Opens aaai
2. Picks before/after/definition paths
3. Starts an audit
4. Works through the results
5. Returns to the Opening screen

…the session's paths do **not** appear in Recent Projects. The
user would have to explicitly type a profile name and click
"Save Profile" before starting the audit to get the benefit.

This is friction. Every time the user runs an audit from the
Opening screen, the current paths should automatically appear
(or move to the top of) the Recent Projects list.

**Change:** in the `StartAudit` handler, before launching the
async diff, silently upsert an auto-profile with the current
paths. The profile name is derived from the definition file stem
(`audit.yaml` → `"audit"`) or the before-folder name if no
definition is set.

## Implementation details

### Item A — `main.rs` title closure

```rust
.title(|app: &app::App| {
    let base = t!("app.title").to_string();
    if matches!(app.screen, app::Screen::Main) {
        let fname: Option<&str> = if !app.definition_path.is_empty() {
            std::path::Path::new(&app.definition_path)
                .file_name()
                .and_then(|n| n.to_str())
        } else {
            None
        };
        let dirty = if app.dirty { " ●" } else { "" };
        if let Some(name) = fname {
            return format!("{} — {}{}", base, name, dirty);
        } else if app.dirty {
            return format!("{}{}", base, dirty);
        }
    }
    base
})
```

### Item B — `App::auto_save_profile()` helper

```rust
/// Silently upsert a profile for the current paths.
/// Called at audit start so Recent Projects stays current without
/// requiring the user to explicitly name and save a profile.
///
/// The profile name is derived automatically:
/// - If a definition file is set: use the file stem
///   (`/path/to/audit.yaml` → `"audit"`)
/// - Otherwise: use the before-folder name
///   (`/releases/v1.0.0` → `"v1.0.0"`)
/// - Fallback: `"untitled"`
///
/// I/O errors are swallowed — a failing profile auto-save must
/// never block the audit.
fn auto_save_profile(&mut self) {
    let name = {
        let from_def = (!self.definition_path.is_empty()).then(|| {
            std::path::Path::new(&self.definition_path)
                .file_stem()
                .and_then(|s| s.to_str())
                .map(|s| s.to_string())
        }).flatten();

        let from_before = (!self.before_path.is_empty()).then(|| {
            std::path::Path::new(&self.before_path)
                .file_name()
                .and_then(|s| s.to_str())
                .map(|s| s.to_string())
        }).flatten();

        from_def
            .or(from_before)
            .unwrap_or_else(|| "untitled".to_string())
    };

    let profile = aaai_core::profile::store::AuditProfile {
        name,
        before: self.before_path.clone(),
        after:  self.after_path.clone(),
        definition: if self.definition_path.is_empty() { None }
                    else { Some(self.definition_path.clone()) },
        ignore_file: if self.ignore_path.is_empty() { None }
                     else { Some(self.ignore_path.clone()) },
        last_used_at: Some(chrono::Utc::now()),
    };

    self.profiles.add(profile);
    let _ = self.profiles.save();  // silently swallow I/O errors
}
```

Called at the top of the `StartAudit` handler, before the async
diff is launched. The call is unconditional (even if the audit
fails, the paths were real enough to attempt an audit).

## No i18n changes

Both items are pure code/behaviour changes — no new user-visible
strings are introduced.

## Acceptance criteria

- [ ] `main.rs` title closure shows `aaai — {filename}{dirty?}` on Main screen
- [ ] `app.rs` `auto_save_profile()` helper method
- [ ] `StartAudit` handler calls `self.auto_save_profile()` before the diff task
- [ ] `cargo check --workspace --all-targets` warning-free
- [ ] All existing tests pass (99 / 70 / 15)
- [ ] i18n count unchanged (215/215/215)
