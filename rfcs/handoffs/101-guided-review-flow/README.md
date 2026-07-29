# RFC 101 — Guided review flow: developer handoff

Companion to [`RFC 101`](../../proposed/101-guided-review-flow.md). The RFC
records what was decided and why; this records how to implement and verify it
safely. It must not override the RFC.

## 1. Authority and entry conditions

Begin only after **all** of:

- RFC 099 / **V1** and RFC 100 / **V2** have passed and are integrated;
- WS-05's report contract and WS-10's progress API are available — the Checking
  and Save-report screens consume them;
- the high-capability model's design review accepts RFC 101 and this handoff;
- the owner explicitly approves implementation;
- a GUI/UX verification operator with a **real display** is available;
- `main` is green on hosted B0 and the working tree is clean.

If WS-05 or WS-10 has not landed, the Review, Start, and Done screens may
proceed; **Checking and Save report must not** (§8).

## 2. Role split

| Role | Party | Scope |
|---|---|---|
| Implementer | GUI developer | T1–T8 |
| Verification operator | GUI/UX operator, real display | T9 visual and keyboard evidence |
| Architect | RFC 101 author | Consulted on §7.1 interpretation; does not implement |
| Integrator | nabbisen | Reviews, commits, pushes, observes B0 |

## 3. The rule that prevents the main failure mode

**One state, one `Message` set.** A guided control and its expert-workspace
equivalent must dispatch the *identical* `Message` variant. If a guided screen
needs a new message, ask whether the expert surface should use it too — the
answer is almost always yes.

Never add a parallel state field, a parallel save path, or a second audit call.
DEC-001 forbids audit logic in a front-end, and two divergent front-end paths
is the same defect one level down.

## 4. Build order — Review card first

Deliberately **not** user-journey order. The Review card is the highest-value
screen and the only core screen with no external dependency.

| Step | Screen | Depends on | Commit |
|---|---|---|---|
| T1 | Guided shell + surface switch (`For technical users`, and back) | — | 1 |
| T2 | **Review one change** — decision card, reason field dominant, details collapsed | — | 2 |
| T3 | Start — folder pickers, plain language, no `audit.yaml` | — | 3 |
| T4 | Done | — | 4 |
| T5 | Save report | **WS-05** | 5 |
| T6 | Checking — progress only | **WS-10** | 6 |
| T7 | Problem state | — | 7 |
| T8 | i18n sweep — en + ja | all | 8 |

## 5. Per-step requirements

**T1 — shell and switch.** Persist the surface choice in `prefs.yaml` as an
additive field with `#[serde(default)]`. An existing `prefs.yaml` without it
must load unchanged — test this explicitly. Round-tripping guided → expert →
guided must preserve unsaved reason text and current selection; reuse the
existing nav-guard (RFC 041) rather than writing a new one.

**T2 — Review card.** The reason field is the most prominent input on the
screen. `Save this answer` is the single dominant action. Side-by-side text is
available but **not** the first focus. Strategy names, raw YAML terms, the full
status taxonomy, and technical errors sit behind `Details`.

**T3 — Start.** `Original folder` and `Changed folder`, explained in plain
language. `audit.yaml` must not appear anywhere on this screen — the wireframe
checklist tests exactly this.

**T5 — Save report.** Default to the easy-to-read format; other formats behind
details. Follow WS-05's contract; do not anticipate it.

**T6 — Checking.** Progress only, no actions. Consume WS-10's progress API as
delivered; if it is provisional, stop (§8).

**T7 — Problem state.** Recovery steps visible, technical detail behind
details. Primary action `Show me what to do`.

**T8 — i18n.** Every user-facing string via `rust-i18n`, in **both** `en` and
`ja`. No hardcoded English. After editing locale YAML, `cargo clean -p aaai-gui`
may be needed — `rust-i18n` is a compile-time macro.

## 6. Vocabulary — normative

| Internal | Shown |
|---|---|
| before directory | Original folder |
| after directory | Changed folder |
| `audit.yaml` | Review record |
| strategy | How aaai checks it next time |
| pending | Needs your answer |
| failed | Changed again |
| error | Couldn't check |
| ignored | Skipped |
| approve | Save this answer |

## 7. Verification

```sh
cargo +1.91 fmt --check -p aaai-gui
cargo +1.91 clippy -p aaai-gui --all-targets -- -D warnings
cargo +1.91 test --workspace --locked
python3 scripts/check-i18n-keys.py
git diff --check

# RFC 099 foundation must remain intact
grep -rE "\.size\([0-9]" crates/aaai-gui/src/            # must return nothing
grep -rn "Color::from_rgb" crates/aaai-gui/src/          # nothing outside design_tokens.rs

# RFC 100 boundaries must remain intact
find crates/aaai-gui/src -name mod.rs                    # must return nothing
for f in $(find crates/aaai-gui/src -name '*.rs'); do
  echo "$(grep -vE '^\s*(//|$)' "$f" | wc -l) $f"; done | sort -rn | head
```

Regressing V1 or V2 fails U1. Those two greps are the cheapest way to catch it,
and they must be run on every commit, not only at the end.

**T9 — real-display evidence** (verification operator): all five screens, all
four themes, at 800 × 550 and a typical working size; plus a full keyboard pass
Start → Checking → Review → Done → Save report. Xvfb is not acceptable.

## 8. Stop and escalation conditions

Stop and request an RFC amendment when:

- 800 × 550 cannot be met — **do not** shrink type or bypass RFC 099's tokens;
- WS-10's progress API or WS-05's report contract is provisional or absent when
  T5/T6 is reached;
- a guided screen appears to need its own state field, save path, or audit
  call;
- a `Message` variant would diverge between guided and expert surfaces;
- new audit capability seems necessary — RFC 095 §11 defers UI expansion;
- an engine, CLI, persisted-format, or public-API change appears necessary;
- the guided → expert round trip cannot preserve unsaved work.

## 9. Evidence package

Create `.git-exclude/evidence/101-guided-review-flow/`:

```
environment.md          toolchain, OS, display
screens/                T9 screenshots, <theme>-<screen>-<size>.png
keyboard-path.md        full keyboard pass, per step
wireframe-checklist.md  the twelve wireframe acceptance items, each with evidence
roundtrip.md            guided → expert → guided preserving unsaved work
prefs-compat.md         an old prefs.yaml loading unchanged
foundation-scans.log    the V1 and V2 regression greps
local-results.md        fmt, clippy, test, i18n, diff --check
scope.diffstat          final boundary
hosted-runs.md          the B0 run for the integrated SHA
```

## 10. Must not

- Add a second audit implementation, state model, or save path.
- Introduce any `.size(N)` literal or `Color::from_rgb`.
- Add, split, or reintroduce a `mod.rs`, or an inline `#[cfg(test)]` module.
- Change engine, CLI, persisted formats beyond the one additive `prefs.yaml`
  field, or the public API.
- Remove or degrade the expert three-pane workspace.
- Add a feature not present in the current surface.

## 11. Rollback

Before integration: discard the working tree. After integration: revert the
offending commit through the normal reviewed path. The eight steps are
independently revertible; T1's surface switch can be disabled to fall back to
the expert workspace as the default without reverting the screens themselves.
