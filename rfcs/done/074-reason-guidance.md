# RFC 074 — Reason field guidance for newcomers

**Status.** Implemented (v0.30.0 — Phase 21)
**Tracks.** Phase 21 — Explainable to Newcomers
**Touches.** `crates/aaai-gui/src/views/inspector.rs` (placeholder + example
line), `crates/aaai-gui/locales/{en,ja}.yaml` (placeholder + 4 example keys).

## The gap

The reason field is the heart of aaai — every approval requires one, and the
whole product value is that *"half a year later, the next person understands why
this was allowed"* (design doc p.2). Yet a first-time user faces an **empty
textarea with no guidance.** They don't know:

- That a reason is required (the `*` marker helps, but doesn't explain why)
- What a *good* reason looks like vs. a useless one ("changed", "ok", "fixed")

A placeholder string already exists in the codebase (`batch.reason_placeholder`
= "Why are these changes allowed?") but is wired only into the batch-approve
sheet, never the main inspector field.

## Design

Two pieces of just-in-time guidance, both ignorable by experts:

### 1. Placeholder in the empty field

When the reason field is empty, show placeholder text:

> *Why is this change allowed?*

This disappears the moment the user types — standard placeholder behaviour.
iced 0.14's `text_editor` supports `.placeholder()`.

### 2. Diff-type-aware example line

Below the field, a single greyed line shows a concrete example *matched to the
current diff type*, so the example is always relevant to what the user is
looking at:

| Diff type | Example shown |
|---|---|
| Added | *e.g. "New config file for the staging environment (ticket INF-42)."* |
| Removed | *e.g. "Removed deprecated logging config — replaced by structured logs."* |
| Modified | *e.g. "Port changed 80 → 8080 per infra ticket INF-42."* |
| TypeChanged | *e.g. "Path is now a directory — restructured per the new layout."* |
| other | *e.g. "Expected change — see the linked ticket for details."* |

The example is small (size 10), low-contrast grey, and prefixed with a subtle
hint marker. An expert's eye skips it; a newcomer reads it once and learns the
shape of a good reason.

### Why diff-type-aware?

A generic example ("e.g. some reason") teaches nothing. An example that matches
the actual change in front of the user shows them *the kind of justification this
specific situation calls for* — which is exactly the leap a newcomer needs to
make. The cost is four short strings instead of one.

### Visibility

The example line shows only when the field is empty (same condition as the
placeholder). Once the user has written a reason, both the placeholder and the
example disappear, keeping the approved-state inspector clean.

## i18n (5 new keys × 2 locales)

```yaml
inspector:
  reason_placeholder:       "Why is this change allowed?"
  reason_example_added:     "e.g. \"New config file for the staging environment (ticket INF-42).\""
  reason_example_removed:   "e.g. \"Removed deprecated logging config — replaced by structured logs.\""
  reason_example_modified:  "e.g. \"Port changed 80 → 8080 per infra ticket INF-42.\""
  reason_example_generic:   "e.g. \"Expected change — see the linked ticket for details.\""
```

## Acceptance criteria

- [ ] Reason field shows placeholder when empty
- [ ] Diff-type-aware example line shown below the field when empty
- [ ] Both disappear once a reason is typed
- [ ] Example matches the selected entry's diff type
- [ ] 5 new i18n keys in en.yaml + ja.yaml
- [ ] `check-i18n-keys.py --quiet` → 0/0/0 (227/227/227)
- [ ] `cargo check --workspace --all-targets` warning-free
- [ ] All tests pass (111 / 89 / 20)
