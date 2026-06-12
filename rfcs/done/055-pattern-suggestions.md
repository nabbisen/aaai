# RFC 055 — Auto-suggest glob patterns from current path

**Status.** Implemented (v0.25.0 — Phase 17)
**Tracks.** Phase 17, RFC 054 follow-up
**Touches.** `crates/aaai-gui/src/app.rs` (suggestion helper, ToggleUsePattern
update, new message), `crates/aaai-gui/src/views/inspector.rs` (suggestion
chips), `crates/aaai-gui/locales/{en,ja}.yaml` (1 new key × 2).

## Summary

RFC 054 exposed glob patterns through a text input, but left the user
to type the glob manually. For a file like `node_modules/lodash/README.md`,
users who know glob syntax write `node_modules/**` — users who don't
know glob syntax are still blocked.

This RFC adds up to three clickable suggestion chips that appear when
the "▸ Use pattern" toggle opens. Clicking a chip fills the pattern
field.

## Suggestion algorithm

Given a path with N parts (e.g. `node_modules/lodash/README.md` → 3 parts):

| Condition | Suggestion | Example |
|---|---|---|
| Depth ≥ 2 | `{parts[0]}/**` | `node_modules/**` |
| Depth ≥ 2 and has extension | `{parts[0]}/**/*.{ext}` | `node_modules/**/*.md` |
| Has extension | `**/*.{ext}` | `**/*.md` |

Suggestions are deduplicated and filtered to max 3.
Single-component paths (no `/`) → no suggestions.

```rust
fn suggest_patterns(path: &str) -> Vec<String> {
    let parts: Vec<&str> = path.split('/').collect();
    let mut seen = std::collections::HashSet::new();
    let mut out  = Vec::new();

    let push = |s: String, seen: &mut _, out: &mut Vec<String>| {
        if seen.insert(s.clone()) && out.len() < 3 { out.push(s); }
    };

    let ext = path.rsplit('.').next()
        .filter(|e| !e.contains('/') && e.len() <= 6);

    if parts.len() >= 2 {
        push(format!("{}/**", parts[0]), &mut seen, &mut out);
        if let Some(e) = ext {
            push(format!("{}/**/*.{}", parts[0], e), &mut seen, &mut out);
        }
    }
    if let Some(e) = ext {
        push(format!("**/*.{}", e), &mut seen, &mut out);
    }
    out
}
```

## State + messages

```rust
// InspectorState
pub pattern_suggestions: Vec<String>,

// New message
ApplyPatternSuggestion(String),
```

`pattern_suggestions` is populated when `ToggleUsePattern` fires.
`ApplyPatternSuggestion(s)` sets `pattern_path = s` and re-validates.

## Inspector view: suggestion chips

When `use_pattern` is true and `pattern_suggestions` is non-empty,
show a compact chip row below the pattern input:

```
Pattern  [node_modules/lodash/**____]   ✓
         [node_modules/**]  [node_modules/**/*.md]  [**/*.md]
```

Chips are small `button::text`-style elements. Clicking fills the
input; validation runs immediately.

## i18n (1 new key × 2 locales)

```yaml
# en.yaml
inspector:
  pattern_suggestions: "Suggestions:"

# ja.yaml
inspector:
  pattern_suggestions: "候補:"
```

## Acceptance criteria

- [ ] `suggest_patterns()` returns correct chips for depth ≥ 2 paths
- [ ] `suggest_patterns()` returns correct chip for extension-only
- [ ] `suggest_patterns()` returns `[]` for single-component paths
- [ ] `pattern_suggestions` populated on `ToggleUsePattern`
- [ ] `ApplyPatternSuggestion` fills the input + validates
- [ ] Chips visible in Inspector when suggestions exist
- [ ] 1 new i18n key in en.yaml + ja.yaml
- [ ] `check-i18n-keys.py --quiet` → 0/0/0 (225/225/225)
- [ ] All tests pass (101 / 70 / 15)
