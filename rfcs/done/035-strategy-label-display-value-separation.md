# RFC 035 — Strategy label display/value separation

**Status.** Implemented (v0.22.0 — Phase 14)
**Tracks.** GUI i18n endgame, Message protocol design
**Touches.** `crates/aaai-gui/src/util.rs` (new `StrategyKind`
enum), `crates/aaai-gui/src/app.rs` (Message variants, handlers,
`InspectorState`), `crates/aaai-gui/src/views/{batch,inspector}.rs`
(pick_list call sites), `crates/aaai-gui/locales/{en,ja}.yaml`
(5 new keys × 2 locales), tests (1-2 new).

## Summary

RFC 034 closed the last format-string / dialog / toast i18n
gap, but the **strategy picker** (used in both the inspector and
the batch sheet) still flows English labels — "None", "Checksum",
"LineMatch", "Regex", "Exact" — through `Message::StrategySelected(String)`
and `Message::BatchStrategySelected(String)`. These strings
double as display labels and protocol values, in the same
pattern RFC 033 fixed for `LineAction` and `RegexTarget`.

The architecture here is slightly different because `AuditStrategy`
is **struct-shaped** (each variant carries different associated
data) — you can't make `LocalizedOption<AuditStrategy>` work
because you can't construct a "default" `AuditStrategy::Checksum`
without choosing a `expected_sha256` value, and `PartialEq`
across the whole struct would also compare the inner data.

The fix is a **discriminator enum** `StrategyKind` that mirrors
`AuditStrategy`'s variants without the associated data. The
picker uses `LocalizedOption<StrategyKind>`. A `to_default_strategy()`
method on `StrategyKind` constructs the corresponding zero-value
`AuditStrategy` (e.g. `Checksum { expected_sha256: String::new() }`)
when the user picks a new kind.

After this RFC, every user-facing string in `aaai-gui` flows
through `t!()` — no caveats.

## Why the discriminator lives in aaai-gui, not aaai-core

`AuditStrategy` is part of `aaai-core`'s public API. Adding a
`StrategyKind` enum there would surface a parallel type that
consumers don't need — `aaai-core` callers serialise / deserialise
`AuditStrategy` directly, not the kind. Keeping `StrategyKind`
inside `aaai-gui::util` makes it a private display-layer concern.

If a future aaai-core API redesign (v2.0) wants a discriminator
upstream, it can promote `StrategyKind` then. Not now.

## What this RFC does

### New `StrategyKind` enum

```rust
/// RFC 035 — discriminator for `AuditStrategy` variants without
/// their associated data. Used as the value type for the
/// strategy picker in `LocalizedOption<StrategyKind>`.
///
/// This is a GUI-layer concern (display + Message protocol);
/// `aaai-core` continues to expose only `AuditStrategy`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StrategyKind {
    None,
    Checksum,
    LineMatch,
    Regex,
    Exact,
}

impl StrategyKind {
    /// Construct a zero-value `AuditStrategy` for this kind.
    /// Used when the picker selects a new kind — the inspector
    /// state's strategy is replaced with a fresh default of
    /// that variant.
    pub fn to_default_strategy(self) -> AuditStrategy { ... }

    /// Read the kind from an existing strategy.
    pub fn from_strategy(s: &AuditStrategy) -> StrategyKind { ... }

    /// Localised label for the picker. Resolves through
    /// `inspector.strategy_{none,checksum,linematch,regex,exact}`.
    pub fn label(self) -> String { ... }
}
```

### Message protocol changes

```rust
// Before (RFC 034):
StrategySelected(String),
BatchStrategySelected(String),

// After (RFC 035):
StrategySelected(StrategyKind),
BatchStrategySelected(StrategyKind),
```

The handlers drop their `strategy_from_label()` calls entirely:

```rust
// Before:
Message::StrategySelected(label) => {
    self.inspector.strategy_label = label.clone();
    self.inspector.strategy = strategy_from_label(&label);
    self.validate_inspector();
}

// After:
Message::StrategySelected(kind) => {
    self.inspector.strategy_kind = kind;
    self.inspector.strategy = kind.to_default_strategy();
    self.validate_inspector();
}
```

### `InspectorState` shape change

```rust
// Before:
pub strategy_label: String,

// After:
pub strategy_kind: StrategyKind,
```

The field name change reflects what it actually holds — a
discriminator, not a label. Render-time label generation comes
from `kind.label()`.

### `strategy_from_label()` removed

The function disappears. Its 5-arm match converts
`StrategyKind` → `AuditStrategy` instead, living on
`StrategyKind::to_default_strategy()`.

### `STRATEGIES: &[&str]` constants removed

Both `views/batch.rs` and `views/inspector.rs` have:

```rust
const STRATEGIES: &[&str] = &["None", "Checksum", "LineMatch", "Regex", "Exact"];
```

Both get removed. Each pick_list builds its options inline from
`StrategyKind`'s variants:

```rust
let strategy_options: Vec<LocalizedOption<StrategyKind>> = vec![
    LocalizedOption { value: StrategyKind::None,      label: t!("inspector.strategy_none").to_string() },
    LocalizedOption { value: StrategyKind::Checksum,  label: t!("inspector.strategy_checksum").to_string() },
    LocalizedOption { value: StrategyKind::LineMatch, label: t!("inspector.strategy_linematch").to_string() },
    LocalizedOption { value: StrategyKind::Regex,     label: t!("inspector.strategy_regex").to_string() },
    LocalizedOption { value: StrategyKind::Exact,     label: t!("inspector.strategy_exact").to_string() },
];
```

### 5 new i18n keys (×2 locales = 10 entries)

```yaml
# en.yaml
inspector:
  strategy_none:      "None"
  strategy_checksum:  "Checksum"
  strategy_linematch: "LineMatch"
  strategy_regex:     "Regex"
  strategy_exact:     "Exact"

# ja.yaml
inspector:
  strategy_none:      "なし"
  strategy_checksum:  "チェックサム"
  strategy_linematch: "行マッチ"
  strategy_regex:     "正規表現"
  strategy_exact:     "完全一致"
```

The English keys preserve the existing terms (which are
documented in `docs/`). Japanese uses native technical
vocabulary that's consistent with the rest of the inspector
namespace.

## What this RFC does NOT do

### Rename `AuditStrategy` variants

The `aaai-core` enum's variant names ("None"/"Checksum"/etc.)
stay as-is. They're the canonical serde representation in
saved YAML files, exposed via the `#[serde(tag = "type")]`
attribute. Changing them would break every existing
`audit.yaml` file. Out of scope.

### Localise `AuditStrategy::label()`

The `label()` method on `AuditStrategy` itself stays returning
`&'static str`. It's used internally for the `#[allow(dead_code)]`
helper and any external aaai-core consumer that wants the
canonical English label. Localising it would require either
returning a `String` (allocation per call) or threading a locale
through aaai-core's API — neither is justified for what should
be a constant-time enum-to-string helper.

The GUI picker uses `StrategyKind::label()` instead, which
calls `t!()`.

### Strategy `description()` method

`AuditStrategy::description()` (also in aaai-core) returns
English help text. Same v1→v2 territory as `is_approvable()`
errors — out of scope here. The GUI doesn't currently render
this string anywhere visible to users; if it ever does, that
RFC handles the i18n migration.

## Internal design

### `to_default_strategy()` mirrors current `strategy_from_label()`

```rust
impl StrategyKind {
    pub fn to_default_strategy(self) -> AuditStrategy {
        match self {
            StrategyKind::None      => AuditStrategy::None,
            StrategyKind::Checksum  => AuditStrategy::Checksum { expected_sha256: String::new() },
            StrategyKind::LineMatch => AuditStrategy::LineMatch { rules: Vec::new() },
            StrategyKind::Regex     => AuditStrategy::Regex { pattern: String::new(), target: RegexTarget::AddedLines },
            StrategyKind::Exact     => AuditStrategy::Exact { expected_content: String::new() },
        }
    }

    pub fn from_strategy(s: &AuditStrategy) -> StrategyKind {
        match s {
            AuditStrategy::None         => StrategyKind::None,
            AuditStrategy::Checksum {..} => StrategyKind::Checksum,
            AuditStrategy::LineMatch{..} => StrategyKind::LineMatch,
            AuditStrategy::Regex {..}    => StrategyKind::Regex,
            AuditStrategy::Exact {..}    => StrategyKind::Exact,
        }
    }

    pub fn label(self) -> String {
        match self {
            StrategyKind::None      => t!("inspector.strategy_none"),
            StrategyKind::Checksum  => t!("inspector.strategy_checksum"),
            StrategyKind::LineMatch => t!("inspector.strategy_linematch"),
            StrategyKind::Regex     => t!("inspector.strategy_regex"),
            StrategyKind::Exact     => t!("inspector.strategy_exact"),
        }.to_string()
    }
}
```

The same enum has both display and protocol responsibilities,
unified through `StrategyKind`.

### `InspectorState` initialization

The `Default` impl uses `StrategyKind::None`. Loading an entry
from the audit definition derives the kind from the strategy:

```rust
// app.rs L567 — when loading an entry:
strategy_kind: StrategyKind::from_strategy(&entry.strategy),
```

### Locale counts

| Pre-RFC | Post-RFC |
|---|---|
| 175 / 175 / 175 | **180 / 180 / 180** (+5 keys × 2 locales) |

## Testing

Two new unit tests on `StrategyKind`:

```rust
#[test]
fn strategy_kind_roundtrips_through_strategy() {
    for kind in [StrategyKind::None, StrategyKind::Checksum,
                 StrategyKind::LineMatch, StrategyKind::Regex,
                 StrategyKind::Exact] {
        let strategy = kind.to_default_strategy();
        assert_eq!(StrategyKind::from_strategy(&strategy), kind);
    }
}

#[test]
fn strategy_kind_default_is_none() {
    // The Default for AuditStrategy is None; verify our discriminator
    // matches that.
    let default_strategy = AuditStrategy::default();
    assert_eq!(StrategyKind::from_strategy(&default_strategy), StrategyKind::None);
}
```

aaai-gui test count: 13 → 15.

## Acceptance criteria

- [ ] `StrategyKind` enum added to `util.rs` with all 5 variants
- [ ] `to_default_strategy()`, `from_strategy()`, `label()` methods implemented
- [ ] `Message::StrategySelected` and `Message::BatchStrategySelected`
      payloads changed from `String` to `StrategyKind`
- [ ] Both handlers simplified (no more `strategy_from_label`)
- [ ] `InspectorState.strategy_label: String` → `strategy_kind: StrategyKind`
      with all 5 usage sites updated
- [ ] `strategy_from_label` function removed from app.rs
- [ ] `STRATEGIES: &[&str]` constants removed from views/{batch,inspector}.rs
- [ ] Both pick_list call sites use `LocalizedOption<StrategyKind>`
- [ ] 5 new i18n keys defined in en.yaml + ja.yaml
- [ ] 2 new unit tests on `StrategyKind` round-tripping
- [ ] `scripts/check-i18n-keys.py --quiet` returns 0/0/0 (180/180/180)
- [ ] `cargo check --workspace --all-targets` warning-free
- [ ] All existing tests pass (97 / 70 / 15 after additions)
- [ ] CHANGELOG entry under `[Unreleased]`

## Open questions

None at acceptance.

After this RFC lands, the GUI i18n loop is **unconditionally
closed**. The only remaining English strings in `aaai-gui`
flow from `aaai-core`:

- `AuditStatus::Display` (used in dashboard.rs L70)
- `AuditEntry::is_approvable()` errors
- `AuditStrategy::label()` / `description()` (now unused by GUI)
- YAML preview formatters in inspector.rs (by design — preview
  of disk-saved YAML must match the saved English keys)

All of these are either documented v1→v2 territory or
intentionally English-by-design (YAML preview). After RFC 035,
the operator can do a Japanese-locale walkthrough and find
zero accidental English text.
