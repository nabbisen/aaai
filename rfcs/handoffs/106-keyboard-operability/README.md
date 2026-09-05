# RFC 106 — Keyboard operability: implementer handoff

**RFC.** [`rfcs/proposed/106-keyboard-operability.md`](../../proposed/106-keyboard-operability.md)

**Status.** §5's open question is **settled** — `F6` / `Shift+F6`, owner-approved
2026-09-05. **The RFC itself is not yet accepted for implementation.** Do not
start until it is.

**Baseline.** `main` at `8dcfafd`. **Re-measure before relying on any line
number here** — see §7.

**Evidence location.** `.git-exclude/evidence/106-keyboard-operability/`

| Role | Who | Scope |
|---|---|---|
| Design owner | high-capability model | The RFC and this handoff |
| Implementer | GUI developer | T1–T5 |
| Reviewer | high-capability model | On request |
| Integrator | nabbisen | Commit, push, observe CI |

---

## 1. What this is

**The Opening screen cannot be operated by keyboard at all.** `Tab` reaches no
control and `Enter` activates none, so a keyboard-only user cannot start an
audit. This is the application's entry point.

The cause is not a missing feature. `app.rs` intercepts `Tab` globally and maps
it to `Message::FocusNext`, which cycles an enum naming three panes of the
**`Main`** screen and **never calls iced's `focus_next()`**. No widget focus
moves on any screen.

**The fix deletes code rather than adding it:** stop intercepting `Tab`, let the
toolkit traverse.

## 2. The decision that was open, and is now closed

RFC 005 gave `Tab` the job of cycling the three `Main` panes. Delegating `Tab`
to widget traversal removes its only trigger.

**Owner decision, 2026-09-05: pane cycling moves to `F6` / `Shift+F6`.** RFC
005's behaviour survives; `Tab` and `Shift+Tab` become ordinary traversal.

**Use `snora::keyboard::cycle_zones(key, modifiers)`** rather than hand-writing
the decode. It is a pure key decoder — `F6` → `Cycle::Forward`, `Shift+F6` →
`Cycle::Backward`, `None` otherwise (`snora/src/keyboard.rs:122-131`). It never
mentions `FocusZone` or `AppLayout`.

**Do not adopt `snora_core::focus::next_zone`.** Its model is `AppLayout`'s four
zones (Header / SideBar / Body / Footer); ours is three panes. Take the binding
and the decoder, not the enum. This was decided in RFC §5 and is unchanged.

## 3. The snora bump this needs

`Cycle` is only nameable from the `snora` facade at 0.39+, and our pin is 0.38.
Per RFC §5a, the manifest line becomes:

```toml
snora = { version = "0.44", default-features = false, features = ["design"] }
```

**Two changes in one line, one B0 run.**

- **`"0.44"`** — 0.39 for `snora::focus`, 0.41 for overlay pointer containment,
  0.42 for toast `Warning`/`Info` contrast repairs on the default path. 0.43 and
  0.44 add nothing we compile against but `^0.42` would not admit them.
- **`default-features = false`** — snora's default enables `widgets`, and we
  import nothing from it. **Do not justify this on binary size:** another
  consumer measured that delta at exactly zero, byte-identical either way. The
  reasons are compile time and a manifest that states what we actually depend
  on.

**If anything fails to compile with `default-features = false`, report it rather
than restoring the default.** That would mean we depend on `widgets` somewhere
the inventory missed, which is itself the finding.

Expect the toast appearance to change — `Warning` text white → black, `Info`
fill and text. Both are contrast repairs. We hold no visual baselines, so
nothing should break; if something does, that is a finding.

## 4. Tasks

### T1 — The failing test, first (commit 1)

Per RFC §6a. Add `iced_test = "0.14"` as a **dev-dependency** of
`crates/aaai-gui` and report the build-time delta.

Assert **traversal by effect**, not focus position:

```
tap_key(Tab); tap_key(Tab); tap_key(Enter)
  → assert the activation message of the control expected at that position
```

**There is no `find_focused` without iced's `advanced` feature, which we do not
enable. Do not try to assert a focus position.** `tap_key` returns
`event::Status`, and `into_messages()` gives what an input produced; those two
calls carry the test.

**This must fail before T2.** `Tab` produces `FocusNext`, which moves no widget
focus, so the `Enter` that follows activates nothing. Red before, green after.

`snapshot` / `matches_image` / `matches_hash` exist and are **declined** —
golden values invalidate whenever snora ships an appearance change, which has
now happened four times in the span we track.

### T2 — Delegate `Tab` (commit 2)

Replace the `Tab` / `Shift+Tab` interception with
`iced::widget::operation::{focus_next, focus_previous}`. The codebase already
uses `operation::scroll_to` in the RFC 069 scroll-sync handlers, so this
introduces no new mechanism.

`Enter` currently maps to `Message::FocusInspectorReason`, which is `Main`-only.
It must stop swallowing `Enter` on every screen; activation of the focused
control is the toolkit's job.

### T3 — Pane cycling on `F6` (commit 2 or 3)

Wire `cycle_zones`. Keep `FocusTarget` and its three-pane model; only the
trigger changes.

**One i18n key** for the help overlay's entry, both locales. That is the single
key RFC §3.2's "no i18n change" is relaxed for, by the same owner decision that
chose `F6`.

### T4 — Focus-trapping gap, recorded not solved (same commit)

**§4 makes an existing latent defect observable:** once `Tab` moves real focus, a
keyboard user can Tab out of an open dialog. snora has trapping *staged, not
shipped* — it needs iced's `advanced` feature, which is their open decision.

**Do not build a trap.** Record it: the keyboard help overlay must not claim
modal focus containment. See RFC §5b.

### T5 — Evidence

RFC §7's two rules are not optional, because both produced wrong conclusions on
exactly this question before:

- **Prove key delivery with `xdotool type` into a text field before recording
  any negative result.** A binding that does nothing looks identical to a dead
  channel.
- **Check the binding under test has a visible effect on the screen under
  test.** `?` renders an overlay only on `Main`; pressing it on Opening proves
  nothing either way.

Procedure and traps: `.git-exclude/rules/gui-automation-niri-xdotool.md` §5.

## 5. Acceptance

RFC §6 is the contract. Four notes:

**Item 1** — every interactive control on Opening, in **visual order**: both
`Pick folder` buttons, `Optional settings`, `Check changes`, and each
recent-project `Open` and delete control. Order, not just reachability. If iced
cannot be made to match visual order, **stop and escalate** — reordering the view
tree is a layout change and out of scope.

**Item 5** — the test must have failed before T2. Say so in the evidence and
show the failure.

**Item 7** — RFC 099's V1 greps still pass. Note the count is expected to grow
by the named tests only; report the measured GUI figure and why, not a
prediction.

**Clippy** — `--no-deps`, not `-- -D warnings`: the crate carries 13 pre-existing
findings and that flag fails on all of them. The check is that no new finding
appears.

## 6. Out of scope

- **New shortcuts** beyond `F6` / `Shift+F6`. Existing bindings unchanged:
  `Ctrl+S`, `Ctrl+Z`, `/`, `?`, `Ctrl+Enter`, `Escape`, arrow-key selection.
- **Screen-reader support.** A separate and much larger question. This RFC does
  not deliver it and must not be described as doing so.
- **Focus-ring visual design.** Whatever iced draws by default is accepted;
  styling it is RFC 101's business. `tokens.focus.{ring_color, ring_width,
  ring_offset}` is already reachable if RFC 101 wants it.
- **The three modal overlays.** RFC 110 owns them, and it follows this RFC.
- **The S1 test-hermeticity gap** (RFC 104 §7a) and **the C2 lint debt**.

## 7. A standing check this project has learned three times

**Any line number, ELOC figure, or test count in this document is stale until
you re-measure it on current `main`.**

RFC 100 shipped with three acceptance items written wider than its own scope;
RFC 111's removal table had to be regenerated after RFC 100 moved everything;
RFC 104's §2 quoted a code block from a file that no longer held it. Re-measure
first; if a figure is wrong, report it rather than working around it.

## 8. When you are done

Package a review request as usual, entry point stated in chat. Then Integrator
pushes and B0 runs.

Sequence after this: **110 → 101**.
