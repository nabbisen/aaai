# RFC 044 — expires_at enforcement in the audit engine

**Status.** Implemented (v0.23.0 — Phase 15)
**Tracks.** Audit correctness, aaai-core
**Touches.** `crates/aaai-core/src/audit/engine.rs` (one early-return),
`crates/aaai-core/src/audit/tests.rs` (2 new tests). No GUI, no i18n changes.

## Problem

`AuditEngine::evaluate_one()` never called `entry.is_expired()`.
An entry with `expires_at` in the past returned `AuditStatus::Ok` as if it were
a valid approval.  The Inspector showed the red "EXPIRED" badge correctly (the
GUI layer already called `is_expired()`), but the file tree showed a green
checkmark — inconsistent and incorrect.

## Fix

A single early-return before the strategy check. The expired entry data is
preserved (`entry: Some(...)`) so the Inspector can display the old approval
alongside the badge.

```
Engine evaluation order:
  1. Unchanged diff → Ok (unchanged = auto pass)
  2. No matching entry → Pending
  3. Entry disabled → Ignored
  4. RFC 044: Entry expired → Pending  ← new
  5. Empty reason → Pending
  6. Diff-type mismatch → Failed
  7. Content strategy evaluation → Ok / Failed
```

## Tests added (aaai-core: 99 → 101)

- `expired_entry_returns_pending` — entry with `expires_at = 2000-01-01` → `Pending`
- `not_yet_expired_entry_is_ok` — entry with `expires_at = tomorrow` → `Ok`
