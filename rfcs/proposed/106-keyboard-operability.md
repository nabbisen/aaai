# RFC 106 — Keyboard Operability

**Status.** Proposed

**Tracks.** NF-4 / ABDD keyboard completeness. No existing milestone covers
this — see §8.

**Depends.** RFC 100 (GUI module boundaries), which restructures the file this
RFC changes. Scheduled after it, alongside RFC 104.

**Design owner.** high-capability model

**Decision owner.** nabbisen, project owner

**Proposed implementer.** mid-capability model, after design review and owner
approval

**Evidence location.** `.git-exclude/evidence/106-keyboard-operability/`

**Touches.** `crates/aaai-gui/src/app.rs`'s keyboard subscription and focus
handling, or its post-RFC-100 successor, plus a test sibling, plus
`iced_test = "0.14"` as a **dev-dependency** of `crates/aaai-gui`. No engine,
CLI, persisted-format, or i18n change; no runtime dependency added.

**Handoff.** Required, after acceptance.

---

## 1. Summary

**The Opening screen cannot be operated by keyboard at all.** `Tab` reaches no
control and `Enter` activates none, so a keyboard-only user cannot start an
audit. This is the application's entry point and its first-run surface.

Established in
`.git-exclude/reviewed/060-nf4-keyboard-inoperability-finding-2026-08-10.md`,
with the channel proven live first — an earlier finding of the same shape was
withdrawn because the input channel had never been shown to work, and the
distinction is the reason this one is trustworthy.

## 2. The defect

### 2.1 Observed

Fresh instance, Opening screen, window focused at both compositor and X11
level, keystroke delivery proven by typing into a text field:

- `Tab` × 3 → **three byte-identical screenshots**. No focus ring anywhere.
- `Return` → no dialog, no focus ring, window count unchanged.
- `Tab` on the `Main` screen → also no visible change.

### 2.2 Cause

`app.rs` intercepts both keys globally through `iced::keyboard::listen()`:

```rust
(Key::Named(Named::Tab),   _) => Message::FocusNext,
(Key::Named(Named::Enter), _) => Message::FocusInspectorReason,
```

and `FocusNext` does only this:

```rust
self.focus_target = match self.focus_target {
    FocusTarget::FileTree  => FocusTarget::Inspector,
    FocusTarget::Inspector => FocusTarget::FileTree,
    FocusTarget::Search    => FocusTarget::FileTree,
};
```

It cycles an enum naming three panes of the **`Main`** screen. **It never calls
iced's `focus_next()` operation**, so no widget focus moves on any screen.
`Enter` → `FocusInspectorReason` is likewise `Main`-only.

On the Opening screen, which has neither a file tree nor an inspector, `Tab`
mutates a field nothing renders and `Enter` targets a widget that does not
exist.

### 2.3 Why it went unnoticed

RFC 005 ("Keyboard Navigation & Focus", `done/`, v0.12.0) introduced
`FocusNext`. Pane cycling was a reasonable design **for the workspace**, which
was the screen that existed then. RFC 015 redesigned the Opening screen at
v0.18.0 and the keyboard model was never revisited.

It is not a regression, and nothing in flight caused it.

## 3. Goals and non-goals

### 3.1 Goals

1. Every interactive control on the Opening screen is reachable by `Tab` and
   activatable by `Enter` or `Space`.
2. The same holds on `Main`, without losing RFC 005's pane cycling if that
   remains wanted — see §5.
3. A test proves traversal, not merely that a message fires.

### 3.2 Non-goals

- New shortcuts, or changes to existing ones (`Ctrl+S`, `Ctrl+Z`, `/`, `?`,
  `Ctrl+Enter`, `Escape`, arrow-key selection).
- Screen-reader support. A separate and much larger question; this RFC does not
  claim it and must not be read as delivering it.
- Focus-ring visual design. Whatever iced draws by default is accepted here;
  styling it is RFC 101's business if it matters.

## 4. Selected design

**Stop intercepting `Tab`, and let the toolkit traverse widgets.**

| Option | Assessment |
|---|---|
| Teach `FocusNext` about the Opening screen | **Rejected.** Grows a bespoke focus system that already fails to move real widget focus. Each new screen would need another arm |
| **Delegate `Tab` to iced's `focus_next()` / `focus_previous()` operations** | **Selected.** Uses the toolkit's own mechanism, fixes every screen at once including screens not yet written, and deletes code rather than adding it |
| Leave `Tab` intercepted, bind a different key for widget traversal | Rejected. `Tab` is the convention; a second traversal key is a workaround users must be taught |

`iced::widget::operation::{focus_next, focus_previous}` already exist and the
codebase already uses `operation::scroll_to` in the RFC 069 scroll-sync
handlers, so this introduces no new mechanism.

## 5. The open question this RFC must settle

**What happens to RFC 005's pane cycling?**

`focus_target` drives which of the three `Main` panes is considered active.
Delegating `Tab` to widget traversal removes its only trigger. Three ways
forward, and the review should pick one rather than leaving it to
implementation:

| Option | Consequence |
|---|---|
| Drop pane cycling entirely | Simplest. RFC 005's stated behaviour is lost, and `focus_target` and its handlers can be deleted — but RFC 005 is `done/` and its behaviour was deliberate |
| **Move pane cycling to `F6` / `Shift+F6`** | Keeps both. Costs one shortcut and a help-overlay entry, so it touches i18n, which §3.2 otherwise avoids |
| Keep `Tab` cycling panes **only on `Main`**, delegate elsewhere | Preserves RFC 005 exactly where it applies. But `Main` is where the most controls are, so the screen with the greatest need keeps the broken behaviour |

**Recommendation: the second, bound to `F6` / `Shift+F6`.** It is the only
option that leaves both behaviours correct, and one i18n key against a whole
screen being keyboard-inoperable is a good trade.

> **Key corrected 2026-08-19.** This recommended `Ctrl+Tab`, chosen without
> knowing a convention existed. **snora independently reached this RFC's whole
> finding** and recommends `F6` / `Shift+F6`:
>
> > snora does not take Tab or Shift+Tab. Tab means "next control" to iced and
> > to your users; a framework claiming it for region cycling would break
> > in-pane navigation for every application with a form or a text input.
>
> That is §2.2's defect, described by a different party from first principles,
> before seeing our code. `F6` is the established desktop idiom for pane
> cycling and costs the same single key. RFC 108 §6.2 records the exchange.
>
> Their `snora_core::focus::next_zone` helper is **not** adopted: its model is
> `AppLayout`'s four zones (Header / SideBar / Body / Footer) and ours is three
> panes. Take the binding and the principle, not the enum.

## 6. Acceptance contract

1. On the Opening screen, `Tab` reaches every interactive control in visual
   order: both `Pick folder` buttons, `Optional settings`, `Check changes`, and
   each recent-project `Open` and delete control.
2. `Enter` and `Space` activate the focused control.
3. The same holds on `Main` for the toolbar, filter bar, file tree, inspector
   fields, and bottom bar.
4. Whichever §5 option is chosen is implemented and its behaviour tested.
5. **A headless `iced_test` test asserts traversal behaviourally** — see §6a.
   Not that `Message::FocusNext` fires, and not an internal focus flag: that
   pressing `Tab` the expected number of times and then `Enter` produces the
   activation message of the expected control. **Written before the fix, it must
   fail.**
6. No new shortcut beyond §5's choice; existing bindings unchanged.
7. RFC 099's V1 greps still pass; GUI test count grows only by the named tests.

## 6a. How traversal is asserted (added 2026-08-19)

Investigation: `.git-exclude/reviewed/069-iced-test-simulator-investigation-2026-08-19.md`.

`iced_test` **0.14.0** matches our iced 0.14.0 exactly and runs **headless — no
compositor, no display, no GPU dependency of ours**. Add it as a dev-dependency
of `crates/aaai-gui` and report the build-time delta.

Two calls carry the test:

- **`tap_key(key) -> event::Status`** — `Captured` or `Ignored`, so "did anything
  handle this key?" is directly answerable;
- **`into_messages()`** — the messages an input produced, so "what did it do?"
  is directly answerable.

**What is not available:** querying which widget holds focus. There is no
`find_focused` equivalent without iced's `advanced` feature, which we do not
enable. So do not try to assert a focus position.

**Assert the effect instead**, which is what a user experiences anyway:

```
tap_key(Tab); tap_key(Tab); tap_key(Enter)
  → assert the activation message of the control expected at that position
```

**This fails today**, which is the point: `Tab` produces `Message::FocusNext`,
which cycles a pane enum and moves no widget focus, so the `Enter` that follows
activates nothing. The guard is red before the fix and green after — the
standard this project has arrived at repeatedly.

`snapshot` / `matches_image` / `matches_hash` also exist and are **declined** —
golden values that invalidate whenever snora ships an appearance change, three
times in eleven releases. RFC 108 §6.3 records the reasoning.

## 7. Verification note

**Keyboard input is testable on this setup with no new tooling.** `xdotool`
XTEST reaches an XWayland-hosted client; the procedure and its traps are in
`.git-exclude/rules/gui-automation-niri-xdotool.md` §5.

Two rules from that document are not optional here, because both produced wrong
conclusions on exactly this question:

- **Prove key delivery with `xdotool type` into a text field before recording
  any negative result.** A binding that does nothing looks identical to a dead
  channel.
- **Check that the binding under test has a visible effect on the screen under
  test.** `?` renders an overlay only on `Main`; pressing it on Opening proves
  nothing either way.

## 8. Milestone placement

No existing milestone covers keyboard operability. MG1 (V1) is visual, MG2 (V2)
is module boundaries, and M5G (U1) is the guided flow — U1 does mention
"keyboard acceptance", but M5G is the largest remaining milestone and gating a
basic-operability fix behind it would leave the entry screen inoperable until
very late.

**Proposal: attach this RFC to MG2's release unit as a sibling of RFC 104**,
both landing after RFC 100. That gives the GUI sequence
**100 → 104 → 106 → 101**, and M5G/U1 then inherits a working keyboard rather
than being the first place anyone checks.

Owner decision, since it adds an item to a unit.

## 9. Risks

| Risk | Mitigation |
|---|---|
| Delegating `Tab` changes behaviour RFC 005 specified | §5 makes it an explicit decision rather than a side effect; whichever option is chosen is recorded against RFC 005 as a superseding note |
| iced's default traversal order does not match visual order | Acceptance item 1 tests order, not just reachability. If iced cannot be made to match, stop and escalate — reordering the view tree to fix focus order is a layout change and out of scope |
| Focus is not assertable in a unit test | Acceptance item 5 covers this explicitly rather than allowing a silent downgrade to "we clicked around and it seemed fine" |
| Scope creep into screen-reader support | §3.2 excludes it |

## 10. Sources

- `.git-exclude/reviewed/060-nf4-keyboard-inoperability-finding-2026-08-10.md`
- `crates/aaai-gui/src/app.rs` — keyboard subscription, `FocusNext` handler
- RFC 005 (`done/`), RFC 015 (`done/`)
- `.git-exclude/rules/gui-automation-niri-xdotool.md` §5
