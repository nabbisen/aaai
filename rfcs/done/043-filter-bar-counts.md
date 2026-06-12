# RFC 043 — Status counts in filter bar + bottom-bar count i18n

**Status.** Implemented (v0.23.0 — Phase 15)
**Tracks.** Main screen discoverability, i18n completeness
**Touches.** `crates/aaai-gui/src/views/main_view.rs` (filter bar + count label),
`crates/aaai-gui/locales/{en,ja}.yaml` (1 new key × 2).

## Summary

Two items:

**Item A — Hardcoded Japanese in the bottom-bar count label.**
`main_view.rs` line 488 uses `format!("{}件の差分中 {}件が未解決", ...)` — a
Japanese-only format string that shows the wrong language for English users.
Migrated to `t!("filter.count_summary", total = ..., unresolved = ...)`.

**Item B — Live counts on filter bar buttons.**
The filter bar shows `All | Changed only | Pending | Failed & Error` as static
labels. Adding live counts (`All (14) | Changed only | Pending (2) | Failed & Error (1)`)
gives at-a-glance status orientation without switching tabs or inspecting the toolbar.

Rules:
- Counts only appear when `audit_result` is `Some` (no count during loading or before audit)
- "Changed only" has no simple count (it's not purely status-based), so it shows no count
- "All" → `summary.total`
- "Pending" → `summary.pending`
- "Failed & Error" → `summary.failed + summary.error`

The count is appended inline: `"Pending (2)"`.

## i18n

```yaml
# en.yaml
filter:
  count_summary: "%{unresolved} of %{total} unresolved"

# ja.yaml
filter:
  count_summary: "%{total}件中 %{unresolved}件が未解決"
```

## Acceptance criteria

- [ ] `format!("{}件の差分中 {}件が未解決", ...)` replaced with `t!()` call
- [ ] `filter.count_summary` key in en.yaml + ja.yaml
- [ ] Filter bar buttons show `(count)` when `audit_result` is `Some`
- [ ] "Changed only" button shows no count (non-status filter)
- [ ] `check-i18n-keys.py --quiet` → 0/0/0 (216/216/216)
- [ ] `cargo check --workspace --all-targets` warning-free
- [ ] All tests pass (99 / 70 / 15)
