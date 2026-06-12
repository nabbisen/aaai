# RFC 037 — Async rerun with audit-dirty indicator

**Status.** Implemented (v0.23.0 — Phase 15)
**Tracks.** GUI responsiveness, deferred RFC 021 §3.2 banner
**Touches.** `crates/aaai-gui/src/app.rs` (new message + helper,
4 call-site changes, handler), `crates/aaai-gui/src/views/main_view.rs`
(toolbar stale/rerunning indicator), `crates/aaai-gui/locales/{en,ja}.yaml`
(3 new keys × 2 locales).

## Summary

`rerun_audit()` currently runs `DiffEngine::compare_with_ignore` **on the UI
thread**, blocking the window for potentially seconds on large folders. The
`audit_dirty` flag introduced in RFC 021 §3.2 was intended to drive a "stale
results" indicator in the toolbar, but the comment at line 760 documents why
it never fires: the flag is set and immediately cleared in the same synchronous
call, so the view never renders the intermediate state.

This RFC:

1. **Converts all rerun paths to async** using the same `Task::perform +
   spawn_blocking` pattern already used by the initial `StartAudit` handler.
2. **Makes `audit_dirty` actually visible** — while the async diff is running,
   the toolbar shows "↻ Re-running…" instead of the previous stale result.
3. **Fixes the one remaining hardcoded toast body** in the rerun path
   ("Audit re-evaluated.") by migrating it to `t!()`.

## What the code looks like today

Three call sites all use the same sync helper:

```rust
// site 1 — Message::RerunAudit (toolbar ▶ / Ctrl+R)
self.rerun_audit();
self.push_toast(..., "Audit re-evaluated.");

// site 2 — Message::ApproveEntry (inside approve path)
self.audit_dirty = true;
self.rerun_audit();
self.push_toast(toast.approved, &path);

// site 3 — Message::UndoApproval
self.audit_dirty = true;
self.rerun_audit();
self.push_toast(toast.undo, ...);

// The blocking helper:
pub fn rerun_audit(&mut self) {
    if let Some(def) = &self.definition {
        let fresh_diffs = DiffEngine::compare_with_ignore(&before, &after, &self.active_ignore);
        self.diffs = fresh_diffs;
        self.audit_result = Some(AuditEngine::evaluate(&self.diffs, def));
        self.audit_dirty = false;
    }
}
```

## What changes

### New message

```rust
/// Carries the result of the background diff triggered by `start_async_rerun()`.
RerunDiffReady(Result<Vec<DiffEntry>, String>),
```

The `String` arm carries a human-readable error (not a structured type) — same
pattern as other error surfaces in the GUI.

### New `App::start_async_rerun()` helper

Replaces the sync `rerun_audit()` at all three call sites:

```rust
fn start_async_rerun(&mut self) -> Task<Message> {
    let before  = PathBuf::from(&self.before_path);
    let after   = PathBuf::from(&self.after_path);
    let ignore  = self.active_ignore.clone();

    if !before.is_dir() || !after.is_dir() {
        return Task::none();
    }

    self.is_loading = true;
    self.load_progress = Some(t!("progress.rerunning").to_string());
    // audit_dirty stays true until RerunDiffReady fires —
    // this is what makes the toolbar indicator visible.

    Task::perform(
        async move {
            tokio::task::spawn_blocking(move || {
                DiffEngine::compare_with_ignore(&before, &after, &ignore)
                    .map_err(|e| e.to_string())
            })
            .await
            .map_err(|e| e.to_string())
            .and_then(|r| r)
        },
        Message::RerunDiffReady,
    )
}
```

### Updated call sites

**Site 1 — `Message::RerunAudit`:**

```rust
// Before:
self.rerun_audit();
self.push_toast(..., "Audit re-evaluated.");

// After:
return self.start_async_rerun();
// Toast now fires from RerunDiffReady
```

**Sites 2 & 3 — approve / undo:**

```rust
// Before:
self.audit_dirty = true;
self.rerun_audit();
self.push_toast(approve/undo toast);

// After:
self.audit_dirty = true;
self.push_toast(approve/undo toast);    // push BEFORE returning
return self.start_async_rerun();
```

The approve/undo toasts fire immediately; the rerun completion
toast ("Re-evaluated.") fires when `RerunDiffReady` arrives.

### New `RerunDiffReady` handler

```rust
Message::RerunDiffReady(result) => {
    self.is_loading = false;
    self.load_progress = None;
    match result {
        Ok(fresh_diffs) => {
            if let Some(def) = &self.definition {
                self.diffs = fresh_diffs;
                let def = def.clone();
                self.audit_result = Some(AuditEngine::evaluate(&self.diffs, &def));
            }
            self.audit_dirty = false;
            self.push_toast(
                ToastIntent::Info,
                t!("toast.rerun").as_ref(),
                t!("toast.rerun_complete").as_ref(),
            );
        }
        Err(e) => {
            self.push_toast(
                ToastIntent::Error,
                t!("toast.rerun").as_ref(),
                &e,
            );
        }
    }
}
```

### Toolbar indicator

In `build_toolbar()` the status label gains a new leading case:

```rust
let status_label: Element<'_, Message> = if app.audit_dirty && app.is_loading {
    // Async rerun in progress — show "Re-running…" instead of stale result
    text(t!("toolbar.rerunning").to_string())
        .size(11)
        .color(theme::PENDING_COLOR)
        .into()
} else if let Some(result) = &app.audit_result {
    // ... existing logic unchanged ...
} else {
    // ... existing empty state unchanged ...
};
```

`PENDING_COLOR` gives the warm amber already used for Pending status,
visually consistent and distinct from OK (green) / FAILED (red).

### `rerun_audit()` helper removal

The sync helper at the bottom of `impl App` is deleted. Callers have
been converted to `start_async_rerun()`.

### i18n (3 new keys × 2 locales = 6 entries)

```yaml
# en.yaml
progress:
  rerunning: "Re-running audit…"
toolbar:
  rerunning: "Re-running…"
toast:
  rerun_complete: "Audit re-evaluated."

# ja.yaml
progress:
  rerunning: "監査を再実行中…"
toolbar:
  rerunning: "再実行中…"
toast:
  rerun_complete: "再評価完了。"
```

The hardcoded `"Audit re-evaluated."` at the old `Message::RerunAudit`
handler is removed; the i18n'd version fires from `RerunDiffReady`.

### Locale counts

| Pre-RFC | Post-RFC |
|---|---|
| 189 / 189 / 189 | **192 / 192 / 192** |

## Acceptance criteria

- [ ] `Message::RerunDiffReady(Result<Vec<DiffEntry>, String>)` added
- [ ] `fn start_async_rerun(&mut self) -> Task<Message>` implemented
- [ ] All 4 `rerun_audit()` call sites replaced
- [ ] `pub fn rerun_audit` helper removed
- [ ] `RerunDiffReady` handler implemented
- [ ] Toolbar shows `t!("toolbar.rerunning")` when `audit_dirty && is_loading`
- [ ] 3 new i18n keys in en.yaml + ja.yaml
- [ ] Hardcoded `"Audit re-evaluated."` removed from app.rs
- [ ] `check-i18n-keys.py --quiet` → 0/0/0 (192/192/192)
- [ ] `cargo check --workspace --all-targets` warning-free
- [ ] All existing tests pass (99 / 70 / 15)

## Open questions

None at acceptance.

### What this RFC does NOT do

- **Auto-rerun on save** — Saving the definition persists it but doesn't
  change the in-memory `AuditResult`. If desired ("save should trigger a
  fresh audit"), that's a separate RFC. For now, explicit ▶ / Ctrl+R or
  approve/undo trigger reruns as before.
- **"⚠ Stale" indicator when not loading** — With this RFC, every mutation
  that sets `audit_dirty = true` immediately starts `start_async_rerun()`,
  so `audit_dirty && !is_loading` is never visible in practice. A future
  RFC could add a deliberate "pause before rerun" mode for large projects.
- **Cancellation** — The background diff cannot currently be cancelled.
  A cancel button could be added later using `Task::abort`.
