# RFC 050 — Auto-advance to next Pending entry after approval

**Status.** Implemented (v0.24.0)
**Tracks.** Core approval workflow
**Touches.** `crates/aaai-gui/src/app.rs` (ApproveEntry handler only).
No i18n changes. No new messages.

## Problem

After clicking **Approve & Save**, `selected_index` stays on the
just-approved file. The user must manually click the next Pending
entry in the file tree. For an audit with many files this means:

```
type reason → Approve & Save → click next file → type reason → Approve & Save → …
```

The manual tree-click adds friction and breaks the flow. A user working
through 20 Pending items has to make 20 extra clicks.

## Fix

After a successful approval, scan `audit_result.results` for the
next entry with `AuditStatus::Pending`, skipping the just-approved
path (whose status is still `Pending` in the current results because
the rerun hasn't finished yet). If found, dispatch
`Message::SelectEntry(next_idx)` alongside the existing rerun task.

```rust
// Schematic (full code in implementation):
let approved_path = far.diff.path.clone();
let next_pending_idx: Option<usize> = self.audit_result.as_ref()
    .and_then(|result| {
        let n = result.results.len();
        let start = (idx + 1) % n;
        (0..n)
            .map(|i| (start + i) % n)
            .find(|&i| {
                let r = &result.results[i];
                r.status == AuditStatus::Pending
                    && r.diff.path != approved_path
            })
    });

let rerun = self.start_async_rerun();

if let Some(next_idx) = next_pending_idx {
    return Task::batch([
        rerun,
        Task::perform(
            async move { next_idx },
            Message::SelectEntry,
        ),
    ]);
}
return rerun;  // no more Pending — selection cleared by deselect
```

## Behaviour

| Situation | Result |
|---|---|
| More Pending items exist | Inspector loads next Pending entry; background rerun continues |
| Just approved the last Pending | `selected_index` stays on the approved entry; background rerun updates it to OK |
| User is on `FilterMode::PendingOnly` | Next item in unfiltered list that is Pending |
| Expired approvals (RFC 044) also Pending | Also eligible for auto-advance |

## Why not deselect after approving the last Pending?

When the last Pending is approved, the user stays on that entry and
sees the inspector. The background rerun will shortly update its
status to OK. Deselecting at this point would show the dashboard —
which is actually a good "all clear" signal — but it might feel
abrupt. Staying on the entry lets the user confirm the approval
before seeing the overall state.

*Future: once the rerun clears and shows All OK, a "Congratulations"
or "All approved" state on the dashboard would be a nice touch —
but that is out of scope for this RFC.*

## Acceptance criteria

- [ ] After approval, inspector auto-loads the next Pending entry
- [ ] Auto-advance wraps around (last entry → first Pending earlier in list)
- [ ] Just-approved path is skipped in the advance scan
- [ ] When no more Pending, selection stays on the approved entry
- [ ] `cargo check --workspace --all-targets` warning-free
- [ ] All existing tests pass (101 / 70 / 15)
- [ ] i18n unchanged (217/217/217)
