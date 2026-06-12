# RFC 033 — pick_list display/value separation

**Status.** Implemented (v0.22.0 — Phase 14)
**Tracks.** GUI Message protocol design, i18n completeness
**Touches.** `crates/aaai-gui/src/app.rs` (Message variants +
2 handlers), `crates/aaai-gui/src/views/inspector.rs` (2 pick_list
call sites), `crates/aaai-gui/locales/{en,ja}.yaml` (5 new keys ×
2 locales), tests.

## Summary

RFC 032 migrated 20 of the 25 user-facing strings in
`views/*.rs` and **explicitly deferred 5**: the `pick_list`
option strings in `inspector.rs` that double as both display
labels and `Message::*(String)` protocol payloads:

- `"Added"`, `"Removed"` — LineMatch action picker
- `"Added lines"`, `"Removed lines"`, `"All changed lines"` —
  RegexTarget picker

i18n'ing these requires a **Message protocol refactor**:
change the message payloads from `String` to the corresponding
enum (`LineAction`, `RegexTarget`), and update the pick_list
calls to use a small adapter type that pairs localized display
strings with enum values.

This RFC does that refactor, adds the 5 new i18n keys, and
closes the GUI i18n loop completely.

## Why now

The deferral in RFC 032 was principled — mixing a protocol
refactor with a text-migration sweep would conflate two kinds
of risk in one diff. Phase 13 has now demonstrated that
text-only sweeps work cleanly when scoped carefully; the
remaining work is the protocol piece, and it's small enough
to stand on its own.

After this RFC, `aaai-gui`'s entire user-facing string surface
goes through `t!()`. The only remaining English text is what
flows through `aaai-core` (`AuditStatus::Display`,
`is_approvable()` errors) — and that's documented v1→v2
territory, not Phase 13 scope.

## What the pick_list pattern looks like today

In `inspector.rs` line 294-298:

```rust
let action_pick = pick_list(
    &["Added", "Removed"][..],
    Some(if rule.action == LineAction::Added { "Added" } else { "Removed" }),
    move |s: &str| Message::LineRuleActionChanged(i, s.to_string()),
).padding(4);
```

The `&str` flows in three layers:
1. Option list (display)
2. Currently-selected value (display, matched against options)
3. Selection callback payload (protocol value, parsed back into `LineAction`)

When the message handler receives `Message::LineRuleActionChanged(i, s)`,
it does string matching to decode `s` back into `LineAction`:

```rust
let action = match s.as_str() {
    "Added"   => LineAction::Added,
    "Removed" => LineAction::Removed,
    _ => return Task::none(),  // silently drop unknown
};
```

This works as long as the display strings match the matching
constants. Localising the display would silently break the
match — Japanese users picking `"追加"` would have their action
silently dropped.

## External design

### New adapter type

A small wrapper that pairs an enum value with its localized
display string. Implements `Display` (used by `pick_list`) and
`PartialEq` (so the picker can identify the currently-selected
option).

```rust
/// Pairs a Rust enum variant with its localized display label
/// for use as a pick_list option. The variant is the canonical
/// value; the label is the human-readable form for the current
/// locale.
#[derive(Debug, Clone)]
pub struct LocalizedOption<T: Clone + PartialEq> {
    pub value: T,
    pub label: String,
}

impl<T: Clone + PartialEq> std::fmt::Display for LocalizedOption<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.label)
    }
}

impl<T: Clone + PartialEq> PartialEq for LocalizedOption<T> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl<T: Clone + PartialEq> Eq for LocalizedOption<T> {}
```

The `PartialEq` implementation compares by `value` only, not
by `label` — so changing the locale doesn't break "this option
is selected" identity. That's the key trick that makes pick_list
work with localized labels.

### Updated Message variants

```rust
// Before:
LineRuleActionChanged(usize, String),
RegexTargetChanged(String),

// After:
LineRuleActionChanged(usize, LineAction),
RegexTargetChanged(RegexTarget),
```

The handlers drop the string-matching step entirely — they
receive the enum directly.

### Updated pick_list call sites

```rust
// LineMatch action picker (was &["Added", "Removed"]):
let action_options: Vec<LocalizedOption<LineAction>> = vec![
    LocalizedOption {
        value: LineAction::Added,
        label: t!("inspector.action_added").to_string(),
    },
    LocalizedOption {
        value: LineAction::Removed,
        label: t!("inspector.action_removed").to_string(),
    },
];
let action_selected = action_options.iter()
    .find(|o| o.value == rule.action)
    .cloned();
let action_pick = pick_list(
    action_options,
    action_selected,
    move |o: LocalizedOption<LineAction>| Message::LineRuleActionChanged(i, o.value),
).padding(4);
```

The pick_list now sees the localized `label` (for display) and
the enum `value` (for identity). The callback receives the full
`LocalizedOption` and extracts the enum.

### Same shape for RegexTarget

```rust
let target_options: Vec<LocalizedOption<RegexTarget>> = vec![
    LocalizedOption { value: RegexTarget::AddedLines,      label: t!("inspector.target_added_lines").to_string() },
    LocalizedOption { value: RegexTarget::RemovedLines,    label: t!("inspector.target_removed_lines").to_string() },
    LocalizedOption { value: RegexTarget::AllChangedLines, label: t!("inspector.target_all_changed_lines").to_string() },
];
let target_selected = target_options.iter().find(|o| &o.value == target).cloned();
let target_pick = pick_list(
    target_options,
    target_selected,
    |o: LocalizedOption<RegexTarget>| Message::RegexTargetChanged(o.value),
).padding(4);
```

### Display-block label (line 329, 335)

The two non-picker occurrences of `"Added"`/`"Removed"` in
inspector.rs (lines 329, 335) are inside a YAML-style preview
block:

```rust
text(format!("- action: {action_label}")).size(11)
```

This is a **preview of the YAML that will be saved to disk**.
The saved YAML uses the literal enum names "Added"/"Removed"
because that's the serde representation. Localising the preview
would mislead the user about what's actually saved.

**Out of scope** (already documented in RFC 032 §1.3). The
display strings on lines 329, 335 stay English and continue to
use `&str` directly.

The 2 `t!()` keys added by this RFC are for the **picker
options only**, not the preview.

## Internal design

### 5 new i18n keys (×2 locales = 10 entries)

```yaml
# en.yaml
inspector:
  action_added:             "Added"
  action_removed:           "Removed"
  target_added_lines:       "Added lines"
  target_removed_lines:     "Removed lines"
  target_all_changed_lines: "All changed lines"

# ja.yaml
inspector:
  action_added:             "追加"
  action_removed:           "削除"
  target_added_lines:       "追加された行"
  target_removed_lines:     "削除された行"
  target_all_changed_lines: "変更されたすべての行"
```

The Japanese translations follow conventional UI vocabulary —
short noun forms for action labels (追加/削除), descriptive
phrases for target labels (○○された行 / 変更された…).

### Adapter type location

`crates/aaai-gui/src/util.rs` already houses small utility types
(humanize_since etc.). `LocalizedOption<T>` fits there as a
generic helper. If future RFCs add more picker surfaces, they
can reuse the same type.

### Message handler simplification

```rust
// Before:
Message::LineRuleActionChanged(i, action_str) => {
    let action = match action_str.as_str() {
        "Added"   => LineAction::Added,
        "Removed" => LineAction::Removed,
        _ => return Task::none(),
    };
    // ... rest uses `action`
}

// After:
Message::LineRuleActionChanged(i, action) => {
    // ... rest uses `action` directly, no parsing
}
```

The decode step disappears. Less code, no unreachable arms, no
silent-drop on unknown strings.

### Locale counts

| Pre-RFC | Post-RFC |
|---|---|
| 157 / 157 / 157 | **162 / 162 / 162** (+5 keys × 2 locales) |

## Testing

Two new unit tests on `LocalizedOption` covering its identity
semantics:

```rust
#[test]
fn localized_option_equality_ignores_label() {
    let a = LocalizedOption { value: LineAction::Added, label: "Added".into() };
    let b = LocalizedOption { value: LineAction::Added, label: "追加".into() };
    assert_eq!(a, b);  // same value, different label
}

#[test]
fn localized_option_inequality_by_value() {
    let a = LocalizedOption { value: LineAction::Added,   label: "X".into() };
    let b = LocalizedOption { value: LineAction::Removed, label: "X".into() };
    assert_ne!(a, b);  // different value, same label
}
```

aaai-gui test count: 11 → 13.

`scripts/check-i18n-keys.py --quiet` must return 0/0/0 with
162/162/162. mdbook smoke test continues to pass.

## Acceptance criteria

- [ ] `LocalizedOption<T>` adapter type added to `util.rs`
- [ ] `Message::LineRuleActionChanged` payload changed from `String`
      to `LineAction`
- [ ] `Message::RegexTargetChanged` payload changed from `String`
      to `RegexTarget`
- [ ] Both message handlers simplified (string-matching arms removed)
- [ ] Both pick_list call sites updated to use `LocalizedOption`
- [ ] 5 new i18n keys defined in en.yaml + ja.yaml
- [ ] 2 new unit tests on `LocalizedOption`
- [ ] `scripts/check-i18n-keys.py --quiet` returns 0/0/0 (162/162/162)
- [ ] `cargo check --workspace --all-targets` warning-free
- [ ] All existing tests pass (97 / 70 / 13 after additions)
- [ ] CHANGELOG entry under `[Unreleased]`
- [ ] **Lines 329/335 preview labels NOT touched** (RFC 032 out-of-scope per the YAML-preview rule)

## Open questions

None at acceptance. Future work this RFC enables:

- **Other pick_list surfaces.** As `LocalizedOption<T>` is
  generic, future picker UIs (strategy selector, filter mode,
  etc.) can use the same pattern. The strategy picker in
  `batch.rs` (`STRATEGIES: &[&str] = &["None", "Checksum", …]`)
  is a possible follow-up — it has the same display-vs-value
  conflation, but the values are strategy struct shapes rather
  than simple enums, requiring a thin discriminator enum first.
- **GUI i18n verification end-to-end.** Once this RFC lands,
  every user-facing string in `aaai-gui` flows through `t!()`
  (except the YAML preview's enum names, by design). An operator
  walkthrough in Japanese locale becomes the last gate for
  "GUI is fully bilingual."
