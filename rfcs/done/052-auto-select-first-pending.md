# RFC 052 — Auto-select first Pending entry on audit start

**Status.** Implemented (v0.24.0)
**Tracks.** RFC 050–051 workflow completion
**Touches.** `crates/aaai-gui/src/app.rs` (DiffReady handler only).
No i18n changes. No new messages.

## Problem

After RFC 050 (auto-advance) and RFC 051 (Ctrl+Enter), the approval
loop is keyboard-driven once started — but the first entry still
requires a manual mouse click on the file tree. The complete workflow
currently looks like:

```
Run audit  →  [click first file in tree]  →  type reason  →  Ctrl+Enter
          →  [auto-advance]  →  type reason  →  Ctrl+Enter  →  …
```

The bracketed step is unnecessary friction.

## Fix

In `Message::DiffReady`, after the audit result is computed, find the
first entry with `AuditStatus::Pending` and dispatch
`Message::SelectEntry(idx)` as an immediate follow-on task.

```rust
// After: self.audit_result = Some(result); self.screen = Screen::Main;
let first_pending: Option<usize> = self.audit_result.as_ref()
    .and_then(|r| {
        r.results.iter()
            .enumerate()
            .find(|(_, far)| far.status == AuditStatus::Pending)
            .map(|(idx, _)| idx)
    });

if let Some(idx) = first_pending {
    return Task::perform(async move { idx }, Message::SelectEntry);
}
```

`Message::SelectEntry` already handles loading the inspector state
(reason, strategy, etc.) from the entry, so no duplication is needed.

## Behaviour

| Situation | Result |
|---|---|
| Audit has Pending items | First Pending entry auto-selected; inspector loads |
| All items already OK (existing approval file) | `selected_index` stays None; dashboard shows the "Passed" verdict |
| Audit has only Failed/Error items | No Pending → dashboard shows, user sees the attention list |
| Rerun after approval | No change (only on `DiffReady`, not `RerunDiffReady`) |

## Complete keyboard-only workflow after RFC 050–052

```
1. [Pick folders on Opening screen]
2. Enter / click Start Audit
3. [Inspector auto-loads first Pending]   ← RFC 052
4. type reason
5. Ctrl+Enter                              ← RFC 051
6. [auto-advance to next Pending]          ← RFC 050
7. type reason
8. Ctrl+Enter
   … repeat …
9. When no more Pending: dashboard shows "Passed"
```

## Acceptance criteria

- [ ] After `DiffReady`, inspector auto-loads the first Pending entry
- [ ] If no Pending entries, dashboard shows (no change)
- [ ] `RerunDiffReady` does not auto-select (no regression to in-session flow)
- [ ] `cargo check --workspace --all-targets` warning-free
- [ ] All tests pass (101 / 70 / 15)
- [ ] i18n unchanged (218/218/218)
