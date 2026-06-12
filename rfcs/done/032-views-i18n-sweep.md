# RFC 032 — `views/*.rs` user-facing string i18n migration

**Status.** Implemented (v0.21.0 — Phase 13)
**Tracks.** GUI i18n parity, view layer
**Touches.** `crates/aaai-gui/src/views/{batch,dashboard,diff_view,inspector,main_view,opening}.rs`,
`crates/aaai-gui/locales/{en,ja}.yaml` (20 new keys × 2 locales),
no test additions.

## Summary

RFC 031 closed the i18n gap in `app.rs`. A parallel sweep through
`crates/aaai-gui/src/views/*.rs` found user-facing English
strings across all six view files. This RFC migrates the
**in-scope** subset to `t!()` lookups, leaving an
**explicitly-deferred subset** for a follow-up RFC.

### In-scope (20 strings)

Static display-only text — labels, headings, placeholders, and
empty-state hints. These render as visible UI without being
used as protocol values:

| File | Strings | Count |
|---|---|---|
| `batch.rs` | "Content Audit Strategy" | 1 |
| `dashboard.rs` | "Needs attention", "All entries are in order.", "Select a file from the left panel to inspect it." | 3 |
| `diff_view.rs` | "Binary file added/removed/modified/" (4 variants), "Size:", "Before SHA-256:", "After SHA-256:", "✓ Hashes match", "✗ Hashes differ" | 9 |
| `inspector.rs` | "No content inspection.", and 3 text-input placeholders ("line content", "regular expression", "exact file content…") | 4 |
| `main_view.rs` | "Search paths… (/ to focus)" placeholder (used 2x — one key, two call sites), "No entries match the current filter." | 2 |
| `opening.rs` | "  before: {}  →  after: {}" — recent-project preview format string | 1 |

### Out-of-scope — pick_list display-as-protocol values (5 strings)

| File | Strings | Why deferred |
|---|---|---|
| `inspector.rs` L295/329/335 | "Added", "Removed" (LineMatch action) | These strings double as `Message::LineRuleActionChanged(String)` payloads — they're matched via `s.as_str()` to update `LineAction` enum |
| `inspector.rs` L372-376 | "Added lines", "Removed lines", "All changed lines" (RegexTarget) | Same pattern: doubles as `Message::RegexTargetChanged(String)` |

Localising these requires a **Message protocol refactor**: change
`Message::LineRuleActionChanged(String)` to `Message::LineRuleActionChanged(LineAction)`
and have the pick_list use a custom adapter type that pairs the
localized display string with the enum variant. That's a wider
change touching the pick_list value plumbing — it warrants its
own focused RFC, not a tag-along to a text-migration sweep.

The deferral is **architectural, not scope-creep avoidance**:
mixing protocol refactor with text migration would conflate two
distinct kinds of risk in one diff.

### Also out-of-scope

- **`diff_type.rs` and `audit_status.rs`** Display impls in
  `aaai-core`: status labels like "OK"/"Pending"/"Failed"/"Error"
  come from `AuditStatus::to_string()` (line 70 in
  dashboard.rs). i18n'ing these requires structural changes to
  `aaai-core`'s public API — same major-version-bump territory as
  RFC 031's deferred `is_approvable()` work.
- **YAML preview formatters** ("- action: Added", "- action:
  Removed" on inspector.rs line 341): the user-visible text is a
  literal preview of what gets serialised to disk, which is
  English-keyed YAML. Translating the preview would mislead the
  user about what's actually saved.

## External design

### New i18n keys (20 × 2 locales = 40 entries)

Following the established `<surface>.<short_id>` / `error.<surface>.<short_id>`
convention. Where a string is naturally "an empty state hint" it
goes under `empty_state.*` (existing namespace). Where it's a
section heading or label, under the relevant surface namespace.

#### `batch.rs`

```yaml
batch:
  content_audit_strategy: "Content Audit Strategy"  # heading
```

#### `dashboard.rs`

```yaml
dashboard:
  needs_attention: "Needs attention"  # section heading

empty_state:
  dashboard_all_clear: "All entries are in order."
  dashboard_select_file: "Select a file from the left panel to inspect it."
```

#### `diff_view.rs`

The 4 "Binary file added/removed/modified/" share a clear
pattern; one approach is one key per variant. An alternative is
to compose from a base string + a verb, but that gets brittle
across locales. Use one key per variant for transparency:

```yaml
diff:
  binary_file_added:    "Binary file added"
  binary_file_removed:  "Binary file removed"
  binary_file_modified: "Binary file modified"
  binary_file:          "Binary file"  # fallback for non-Added/Removed/Modified
  size_label:           "Size:"
  before_sha256_label:  "Before SHA-256:"
  after_sha256_label:   "After SHA-256:"
  hashes_match:         "✓ Hashes match"
  hashes_differ:        "✗ Hashes differ"
```

The ✓ / ✗ characters stay in the value across locales — they're
universal status icons that don't translate.

#### `inspector.rs`

```yaml
inspector:
  no_content_inspection: "No content inspection."
  linematch_line_placeholder:  "line content"
  regex_pattern_placeholder:   "regular expression"
  exact_content_placeholder:   "exact file content..."
```

#### `main_view.rs`

```yaml
main:
  search_placeholder: "Search paths… (/ to focus)"

empty_state:
  no_entries_match_filter: "No entries match the current filter."
```

#### `opening.rs` — format string

The format string `"  before: {}  →  after: {}"` carries
substitution placeholders. `rust-i18n` supports `%{name}` placeholders
when called as `t!("key", name = value)`:

```yaml
opening:
  recent_project_paths: "  before: %{before}  →  after: %{after}"
```

Call site changes from:

```rust
let detail = text(format!(
    "  before: {}  →  after: {}",
    prof.before,
    prof.after
));
```

to:

```rust
let detail = text(t!(
    "opening.recent_project_paths",
    before = prof.before.clone(),
    after  = prof.after.clone(),
).to_string());
```

The leading whitespace and `→` arrow stay in the value because
they're part of the visual layout, not the user-facing copy.

### Japanese translations

Translations follow the conventions already established in
existing keys: terse for headers, imperative-polite for hints,
parallel structure across en/ja within the same key tree.

```yaml
# Sample — full set is in the implementation
batch:
  content_audit_strategy: "内容監査ストラテジー"
dashboard:
  needs_attention: "要対応"
empty_state:
  dashboard_all_clear: "すべてのエントリが整っています。"
  dashboard_select_file: "左パネルからファイルを選択してください。"
diff:
  binary_file_added:    "バイナリファイルを追加"
  binary_file_removed:  "バイナリファイルを削除"
  binary_file_modified: "バイナリファイルを変更"
  binary_file:          "バイナリファイル"
  size_label:           "サイズ:"
  hashes_match:         "✓ ハッシュ値が一致"
  hashes_differ:        "✗ ハッシュ値が不一致"
inspector:
  no_content_inspection: "内容監査なし。"
  linematch_line_placeholder: "行の内容"
  regex_pattern_placeholder:  "正規表現"
  exact_content_placeholder:  "完全一致する内容…"
main:
  search_placeholder: "パスを検索… (/ でフォーカス)"
empty_state:
  no_entries_match_filter: "現在のフィルタに合致するエントリがありません。"
opening:
  recent_project_paths: "  before: %{before}  →  after: %{after}"
```

The placeholders (`%{before}`, `%{after}`) are unchanged in ja
because they hold path values, not translatable text.

## Internal design

### text() vs text_input(placeholder) handling

`text()` calls and `text_input(placeholder)` arguments both
accept `&str` / `impl Display`. `t!()` returns
`Cow<'_, str>`. The call-site shape:

```rust
text(t!("key").to_string()).size(...)
text_input(&t!("key").to_string(), value).on_input(...)
```

For `text_input`, the placeholder is borrowed for the lifetime
of the builder expression, so `to_string()` then `&` is the
ergonomic pattern. (Same shape as existing call sites.)

### Format string substitution

The single format-string migration (opening.rs `recent_project_paths`)
uses `rust-i18n`'s `%{name}` substitution. The audit script
already handles this — keys with placeholders are first-class
citizens and don't show as "divergent."

### Locale counts

| Pre-RFC | Post-RFC |
|---|---|
| 137 / 137 / 137 | **157 / 157 / 157** (+20 keys × 2 locales) |

## Testing

No new unit tests. The migrated text is rendered by view code;
view code is verified visually by operators (see RFC 017 harness).
The `scripts/check-i18n-keys.py --quiet` audit covers structural
correctness.

Existing tests (97 / 70 / 11) continue to pass — none touch view
rendering directly.

## Acceptance criteria

- [ ] 20 in-scope strings migrated to `t!()` calls (one site each,
      except the 2 main_view placeholders sharing one key)
- [ ] 20 new keys defined in en.yaml under appropriate namespaces
- [ ] 20 parallel keys defined in ja.yaml with structurally-equivalent
      translations
- [ ] `scripts/check-i18n-keys.py --quiet` returns 0/0/0 (157/157/157)
- [ ] `cargo check --workspace --all-targets` warning-free
- [ ] All existing tests pass (97 / 70 / 11)
- [ ] CHANGELOG entry under `[Unreleased]` records both the
      migration and the explicit out-of-scope deferrals
- [ ] No changes to aaai-core
- [ ] **The 5 out-of-scope pick_list strings are NOT touched**
      (a follow-up RFC handles the Message protocol refactor)

## Open questions

None at acceptance. Future work this RFC enables:

- **RFC 033 — pick_list display/value separation.** Localise
  the 5 deferred strings by introducing a wrapper type that
  pairs localized display label with internal enum variant. The
  pick_list takes `&[PickListItem]`, the `Message` carries the
  enum variant directly.
- **aaai-core `AuditStatus::Display` i18n.** Same v1→v2 territory
  as the deferred `is_approvable()` work. The dashboard.rs line
  70 `r.status.to_string()` call would route through that.
- **Full GUI i18n verification.** Once RFCs 032 and 033 land,
  every user-facing string in the GUI flows through `t!()`. A
  future operator-side QA pass (RFC 017 harness) confirms the
  Japanese locale renders cleanly across all screens.
