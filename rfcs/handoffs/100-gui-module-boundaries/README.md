# RFC 100 — GUI module boundaries: developer handoff

Companion to [`RFC 100`](../../proposed/100-gui-module-boundaries.md). The RFC
records what was decided and why; this records how to implement and verify it
safely. It must not override the RFC.

## 1. Authority and entry conditions

Begin only after **all** of:

- RFC 099 / gate **V1** has passed and is integrated;
- the high-capability model's design review accepts RFC 100 and this handoff;
- the owner explicitly approves implementation;
- `main` is green on hosted B0 and the working tree is clean.

No display is needed — this changes no rendering.

## 2. Role split

| Role | Party | Scope |
|---|---|---|
| Implementer | GUI developer | T0–T7 |
| Architect | RFC 100 author | Consulted only if a seam in §4 does not hold |
| Integrator | nabbisen | Reviews, commits, pushes, observes B0 |

## 3. The one rule that governs everything here

**This is a move, not a rewrite.** No test name, no test count, no `Message`
variant, no behaviour may change. If a step requires a logic change to compile,
the seam is wrong — stop and escalate (§8).

## 4. Developer sequence — one commit per step

### T0 — Capture the baseline (blocking, before any edit)

```sh
cargo +1.91 test -p aaai-gui -- --list | sort > /tmp/gui-tests-before.txt
wc -l /tmp/gui-tests-before.txt
```

Keep this file. T7 diffs against it. Without it there is no proof of behaviour
neutrality and the work cannot be accepted.

### T1 — `views/mod.rs` → `views.rs` (commit 1)

Move `crates/aaai-gui/src/views/mod.rs` to `crates/aaai-gui/src/views.rs`,
content unchanged. This matches `ignore.rs` and `progress.rs`. Nothing else
changes.

### T2 — Extract `app.rs` tests (commit 2)

Move the inline `#[cfg(test)] mod tests { … }` block (currently from `app.rs:2395`
to end of file) into `crates/aaai-gui/src/app/tests.rs`, removing one
indentation level. The new file opens with `use super::*;`, exactly as
`crates/aaai/src/diff/ignore/tests.rs:1` does. In `app.rs`, replace the block
with:

```rust
#[cfg(test)]
mod tests;
```

### T3 — Extract state types (commit 3)

Move `PaneKind`, `DiffViewMode`, `FocusTarget`, `OpeningValidation`, `Screen`,
`FilterMode`, `BatchApproveState`, `FieldError`, `InspectorValidation`, and
`InspectorState` — with their `impl` blocks — to `app/state.rs`. Re-export from
`app.rs` so no call site outside the module changes.

### T4 — Extract `Message` and subscription (commit 4)

`enum Message` → `app/message.rs`. `dnd_sub()` → `app/subscription.rs`.
Re-export both.

### T5 — Split the update loop (commit 5)

Distribute `impl App`'s methods across `app/update/` **by message family**, not
by line count: opening and folder selection; audit run and rerun; inspector
editing; save and persistence; navigation and focus; dialogs and toasts.

Multiple `impl App` blocks across files are fine. A family stays whole even if
its file ends up larger than a sibling — coherence beats evenness.

`app.rs` keeps `struct App`, `impl Default for App`, and the module
declarations.

### T6 — Visibility audit

List every item whose visibility you widened to make the split compile. Prefer
`pub(crate)` or `pub(super)` over `pub`. Anything that had to become `pub` goes
in the evidence with a one-line reason.

### T7 — Verification

```sh
cargo +1.91 test -p aaai-gui -- --list | sort > /tmp/gui-tests-after.txt
diff /tmp/gui-tests-before.txt /tmp/gui-tests-after.txt   # MUST be empty
cargo +1.91 fmt --check -p aaai-gui
cargo +1.91 clippy -p aaai-gui --all-targets -- -D warnings
cargo +1.91 test --workspace --locked
python3 scripts/check-i18n-keys.py
git diff --check
git diff --stat
for f in $(find crates/aaai-gui/src -name '*.rs'); do
  echo "$(grep -vE '^\s*(//|$)' "$f" | wc -l) $f"
done | sort -rn | head
```

Expected:

- **the `diff` is empty** — this is the acceptance condition, not a formality;
- counts: aaai 144, CLI unit 8, CLI integration 91, **GUI 27**, doctests 3;
- no file over 500 ELOC, or each exception listed with a rationale;
- `find crates/aaai-gui/src -name mod.rs` returns nothing;
- diff confined to `crates/aaai-gui/src/`.

## 5. Evidence package

Create `.git-exclude/evidence/100-gui-module-boundaries/`:

```
environment.md      toolchain, OS
test-names.diff     T0 vs T7 output — must be empty
eloc-before.txt     per-file ELOC before
eloc-after.txt      per-file ELOC after, with rationale for any file > 500
visibility.md       every widened item and its reason
local-results.md    fmt, clippy, test, i18n, diff --check
scope.diffstat      final boundary
hosted-runs.md      the B0 run for the integrated SHA
```

## 6. Required assertions

1. `test-names.diff` is empty.
2. All test counts unchanged: 144 / 8 / 91 / 27 / 3.
3. No `mod.rs` under `crates/aaai-gui/src/`.
4. No inline `#[cfg(test)]` module beside implementation.
5. Every file ≤ 500 ELOC or listed with a rationale.
6. No `Message` variant added, removed, or renamed.
7. No diff outside `crates/aaai-gui/src/`.

## 7. Must not

- Change any behaviour, test, or `Message` variant.
- Change rendering, layout, sizes, spacing, or colours — RFC 099 owns those,
  and reintroducing a `.size(N)` literal or `Color::from_rgb` regresses V1.
- Add a dependency or edit `Cargo.toml` / `Cargo.lock`.
- Add, remove, or rename an i18n key.
- Begin guided-flow work — RFC 101.
- Split `views/*.rs` bodies; only `views/mod.rs` → `views.rs` is in scope.

## 8. Stop and escalation conditions

Stop and request an RFC amendment when:

- a seam in §4 cannot be moved without a logic change;
- `test-names.diff` is non-empty for any reason;
- a test count moves;
- the split would require widening an item to `pub` across the crate boundary;
- a file remains above 500 ELOC with no defensible rationale;
- Clippy raises a new lint that cannot be fixed without changing behaviour.

## 9. Rollback

Before integration: discard the working tree. After integration: revert the
offending commit through the normal reviewed path. The seven steps are
independently revertible by design — T1 through T5 can each be undone without
the others.
