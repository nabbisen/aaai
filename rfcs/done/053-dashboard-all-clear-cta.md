# RFC 053 — Dashboard all-clear CTA buttons

**Status.** Implemented (v0.24.0)
**Tracks.** Workflow terminal state, RFC 052 follow-up
**Touches.** `crates/aaai-gui/src/views/dashboard.rs` (all-clear branch),
`crates/aaai-gui/locales/{en,ja}.yaml` (1 new key × 2).

## Summary

When all audit items are in order (Pending + Failed + Error = 0), the
dashboard previously showed plain text "All entries are in order." and
a misleading hint "Select a file from the left panel to inspect it."
— which doesn't make sense when there's nothing to action.

RFC 053 replaces the empty attention section with two action buttons:

```
All entries are in order.

  [↑ Export Report]   [□ New Audit]
```

The hint "Select a file…" is suppressed in the all-clear state.
When items still need attention (Pending/Failed/Error > 0), the
existing behaviour is unchanged.

1 new i18n key: `dashboard.new_audit` = "New Audit" / "新しい監査".
Total: **219/219/219**.
