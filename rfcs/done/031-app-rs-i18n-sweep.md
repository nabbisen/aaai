# RFC 031 — User-facing string i18n migration sweep in `app.rs`

**Status.** Implemented (v0.21.0 — Phase 13)
**Tracks.** GUI i18n parity, scope discipline
**Touches.** `crates/aaai-gui/src/app.rs` (4 sites),
`crates/aaai-gui/locales/{en,ja}.yaml` (4 new keys × 2 locales).
**Defers:** A larger aaai-core API change that would i18n the
`is_approvable()` / `strategy.validate()` error path. See §5.

## Summary

A sweep of `app.rs` for `"<English text>".into()` patterns
turned up **eight** hardcoded user-facing strings that needed
i18n migration:

| Site | Line | String | Context |
|---|---|---|---|
| Progress | 532 | `"Comparing folders…"` | Loading indicator during diff |
| Batch validation | 759 | `"Reason must not be empty."` | Batch approve validation |
| Inspector validation | 1440 | `"Reason is required before approval."` | Single-entry approve validation |
| Inspector validation | 1446 | `"Use YYYY-MM-DD format."` | Date format validation |
| Opening inline | 1535 | `"Before folder is required."` | Inline next-to-field, "before" empty |
| Opening inline | 1539, 1550 | `"Folder not found."` (×2) | Path doesn't exist (before AND after) |
| Opening inline | 1541, 1552 | `"Path is not a directory."` (×2) | Path is a file (before AND after) |
| Opening inline | 1546 | `"After folder is required."` | Inline next-to-field, "after" empty |

The first sweep found the `expires_at_format` site only. A
second sweep with a broader grep caught three more. A third
sweep — through the `validate_opening()` function specifically —
caught the four Opening inline validation strings, which are
distinct from the RFC 020 banner path that uses the existing
`error.opening.{before,after}_not_found.*` keys with path
interpolation.

This is documented openly because the iterative scope discovery
itself is a lesson: blanket sweeps via grep miss things, and
the RFC's "this is the last" claim was repeatedly wrong until
the actual count was checked. Future sweeps should run grep
**before** the RFC scope is fixed, not after.

The RFC also **scopes out** a related but architecturally
heavier piece of work: i18n'ing the errors that flow through
`AuditEntry::is_approvable()` from `aaai-core`. The reasoning
is in §5.

Additionally **out of scope**: `strategy_label: "None"` on line 174.
This is an internal discriminator matched against
`AuditStrategy::label()` output (which lives in aaai-core), not
a direct user-facing string. Changing it would break the
dropdown picker's label-matching logic. i18n'ing strategy labels
requires the aaai-core API surface to expose a separate
display-label vs internal-label distinction — covered in the
same deferred work as `is_approvable()` migration.

## What this RFC does

### 8 new i18n keys (×2 locales = 16 entries)

The progress message gets a new top-level `progress.*` namespace
(no `progress.*` keys exist yet; this RFC seeds it). The validation
errors fit under the existing `error.<surface>.*` namespace
following RFC 020 / 026 / 028 / 029 convention.

The Opening inline validation keys are **distinct** from the
existing `error.opening.{before,after}_not_found.*` (which carry
path interpolation for the banner path). Inline messages are
terse because they appear next to the input field; banner
messages are sentence-length because they replace the whole
opening screen layout.

```yaml
# en.yaml
progress:
  comparing_folders: "Comparing folders…"
error:
  batch:
    reason_required:
      message: "Reason must not be empty."
  inspector:
    reason_required:
      message: "Reason is required before approval."
    expires_at_format:
      message: "Use YYYY-MM-DD format."
  opening:
    before_required:
      message: "Before folder is required."
    after_required:
      message: "After folder is required."
    folder_missing:        # used 2x (before+after inline)
      message: "Folder not found."
    not_a_directory:       # used 2x (before+after inline)
      message: "Path is not a directory."

# ja.yaml
progress:
  comparing_folders: "フォルダを比較中…"
error:
  batch:
    reason_required:
      message: "理由を入力してください。"
  inspector:
    reason_required:
      message: "承認前に理由を入力してください。"
    expires_at_format:
      message: "YYYY-MM-DD 形式で入力してください。"
  opening:
    before_required:
      message: "Before フォルダを指定してください。"
    after_required:
      message: "After フォルダを指定してください。"
    folder_missing:
      message: "フォルダが見つかりません。"
    not_a_directory:
      message: "指定されたパスはフォルダではありません。"
```

The Japanese `before_required` / `after_required` keep the
English "Before" / "After" labels because that's how they appear
in the UI (the input field labels are also "Before" / "After"
to match the established product vocabulary). Only the
verb-phrase part is translated.

### 8 call-site updates

All eight use direct `t!()` (not `from_i18n`) — none have hints.
The reason and folder validation messages already imply the
action ("type a reason" / "pick a real folder"); the progress
message and date format message are descriptive only.

### Locale counts

| Pre-RFC | Post-RFC |
|---|---|
| 129 / 129 / 129 | **137 / 137 / 137** (+8 keys × 2 locales) |

## What this RFC does NOT do

### Deeper aaai-core error i18n (deferred)

The `expires_at`-related error path that runs through
`AuditEntry::is_approvable()` from `aaai-core` is **not** touched:

```rust
match entry.is_approvable() {
    Err(e) => {
        // e is "Path must not be empty." / "Reason must not be
        // empty." / strategy-specific validation strings,
        // all from aaai-core
        self.inspector.validation.strategy_errors.push(FieldError {
            field: "expires_at".into(),
            message: e,
            hint: None,
        });
    }
    Ok(()) => { ... },
}
```

`aaai-core::AuditEntry::is_approvable()` returns `Result<(), String>`.
The strings come from:

- `is_approvable()` itself: 2 hardcoded messages ("Path must not
  be empty.", "Reason must not be empty.")
- `AuditStrategy::validate()`: ~6 hardcoded messages across all
  strategies (Checksum format / length / hex; LineMatch empty;
  Regex parse; Exact empty; etc.)

i18n'ing these requires either:

**A. Change aaai-core's API** to return a structured error enum
(e.g. `enum ApprovalError { PathEmpty, ReasonEmpty, Strategy(StrategyError) }`)
and have aaai-gui translate variants to i18n keys. This is the
architecturally clean fix.

**B. String-match aaai-core's English output** in aaai-gui and
re-map known patterns to keys. Brittle — any wording change in
aaai-core silently breaks the i18n.

Per `docs/src/compatibility.md`, `aaai-core` is published on
crates.io with best-effort-stable API in v1.x. Changing
`is_approvable()`'s return type from `Result<(), String>` to
`Result<(), ApprovalError>` is **breaking** — it requires a
major version bump for `aaai-core` and coordinated bumps for
the v1.x → v2.x consumer surface.

Phase 13's character is small, mechanical, narrow-scope work.
A major-version-bumping API redesign doesn't fit. The deeper
i18n migration belongs to its own RFC under Phase 14 or later,
ideally bundled with other aaai-core API breaking changes
(e.g. when error types get a holistic redesign for v2.0).

This RFC therefore closes the **app.rs-side** i18n loop only,
and explicitly leaves the aaai-core path as residue. The
CHANGELOG calls this out so future maintainers know it's a
known gap, not an oversight.

### Locale counts

| Pre-RFC | Post-RFC |
|---|---|
| 129 / 129 / 129 | **133 / 133 / 133** (+4 keys × 2 locales) |

## Acceptance criteria

- [ ] 4 hardcoded English strings in app.rs replaced with `t!()` calls
- [ ] 4 new keys defined in en.yaml (1 under `progress.*`, 3 under
      `error.<surface>.*`)
- [ ] 4 parallel keys defined in ja.yaml with structurally-equivalent
      Japanese translations
- [ ] `scripts/check-i18n-keys.py --quiet` returns 0/0/0 (133/133/133)
- [ ] `cargo check --workspace --all-targets` warning-free
- [ ] All existing tests pass (97 / 70 / 11)
- [ ] CHANGELOG entry under `[Unreleased]` records both the
      migration sweep and the explicit deferral of the aaai-core path
- [ ] No changes to aaai-core

## Open questions

None at acceptance. Future work explicitly deferred:

- **`is_approvable()` / `strategy.validate()` i18n.** Major-version
  API redesign for aaai-core. A future RFC could bundle this with
  any other aaai-core API breakages planned for v2.0.0 — minimising
  the number of major bumps consumers absorb.
- **Reason-required wording consolidation.** The two `reason_required.message`
  keys (batch vs inspector) have meaningfully different wording today.
  A future UX review could decide whether to unify them.
- **Hint authoring for `reason_required`.** Could explain how to
  add a reason ("Click in the Reason field and describe why this
  change is OK"). Not done here per RFC 028 §3 / RFC 030 rule
  of thumb — the messages already point at the action.
- **CLI / core user-facing strings.** Out of scope. CLI tools
  often stay English-only since they target CI/CD pipelines.

