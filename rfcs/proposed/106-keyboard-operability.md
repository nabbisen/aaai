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

> **Settled 2026-09-05: `F6` / `Shift+F6` approved by the owner.** §5 is closed.
> RFC 005's pane cycling survives, moved off `Tab`; `Tab` and `Shift+Tab`
> delegate to iced's traversal per §4. One i18n key is added for the help
> overlay's entry, which §3.2 otherwise avoids — that cost is accepted as part
> of this decision.

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

> **snora 0.39 addendum, 2026-08-20.** The rejection of `next_zone` stands
> unchanged — it is a rejection of the zone *model*, not of reachability, and
> 0.39 changes nothing about that model.
>
> But `snora::keyboard::cycle_zones(key, modifiers)` is a separate function and
> is worth taking. It is a pure key decoder — `F6` → `Cycle::Forward`,
> `Shift+F6` → `Cycle::Backward`, `None` for everything else
> (`snora/src/keyboard.rs:122-131`). It never mentions `FocusZone` or
> `AppLayout`, so adopting it is literally "the binding and the principle, not
> the enum", with the convention owned upstream instead of hand-written here.
>
> **This needs snora 0.39.** `Cycle` was reachable in 0.38 only through a direct
> `snora-core` dependency, which we do not have; 0.39 re-exports it as
> `snora::focus::Cycle` (`snora/src/lib.rs:108`). Our manifest pins
> `snora = "0.38"`, and `^0.38` does not admit 0.39, so this costs one manifest
> line — see §5a.

## 5a. The snora bump this implies (added 2026-08-20; retargeted 2026-09-02)

§5's addendum needs `snora = "0.39"` in the workspace manifest. Recorded here
rather than assumed, because a dependency bump costs a B0 matrix run and is the
owner's call.

**The bump is as small as a bump gets.** The entire 0.38.0 → 0.39.1 source delta
is three files — `lib.rs` (the `focus` re-export), `keyboard.rs`
(`cycle_zones`), and `design/render/tests.rs` (test-only). No palette value, no
`DIM_ALPHA`, no `render.rs`, no rendered output changes; snora's own guide
states no public item was renamed, removed, or retyped, and the file-level diff
confirms it.

### 5a.2 Retarget to `"0.44"` (added 2026-09-02; 0.44 on 2026-09-05)

snora shipped 0.40, 0.41, 0.41.1 and 0.42 since §5a was written. **The target
becomes `"0.42"`**, and the reason is no longer only `cycle_zones`:

| Release | What it gives us | Affected? |
|---|---|---|
| **0.41.0** | **Overlays now contain pointer events.** Before it, a click or scroll on a dialog, toast or modal dim could reach what was rendered beneath — including a dialog dismissing itself when its own content was clicked | **Yes** — see §5a.3 |
| **0.42.0** | Toast `Warning` text white → black (3.18:1 → 6.60:1), `Info` 4.43:1 → 5.63:1, dismiss `×` fully opaque at every state | **Yes** — we raise toasts at ten call sites |
| **0.42.0** | Drops transitively-enabled `iced` `canvas`/`svg`; **−1.83 MiB binary, −43 packages** | **Benefit only.** We use neither: `grep` for `widget::canvas`/`Svg` across `crates/` returns nothing, and we already declare `iced`'s `tokio` feature ourselves |
| **0.41.1** | Withdrew a WCAG 1.4.1 conformance claim about prefab toasts | Documentation — see §5a.4 |

**No API break reaches us.** 0.42's break is for consumers relying on `canvas`
or `svg` arriving transitively; 0.41's is a behaviour fix with no flag to
restore the old behaviour, because the old behaviour was the bug.

**0.43.0 and 0.44.0 add nothing we compile against** — snora states no API,
appearance, or feature-resolution change, and the file-level story is tests, CI
and their own readiness register. But `^0.42` does not admit them, so targeting
`"0.44"` costs the same single B0 run and leaves a smaller gap to the next bump.
**0.45.0 will be breaking** — it removes `Emphasis` and `Size`, which §5a.4
confirms we do not use.

### 5a.3 The pointer fix matters most to the navigation guard

Our three modal overlays are **hand-rolled** — `stack![base, backdrop, dialog]`
with a `mouse_area` backdrop — so snora's 0.41.0 containment fix does not reach
them at any version. **RFC 110 routing them through `AppLayout::dialog` is what
collects it**, which is a second reason for that RFC beyond the backdrop colour.

The stakes are highest for the navigation guard, which gates RFC 086's
data-losing discard. snora's own framing: *"a dialog could be bypassed by
clicking through it to the control it was meant to be guarding."*

**Not asserted as a live defect here** — our overlays are our own code and I
have not tested click-through against them. **RFC 110's acceptance should
verify it**, and that is recorded there rather than assumed.

### 5a.4 What the 1.4.1 withdrawal costs us: nothing cited, one gap found

snora withdrew a published claim that their prefab toasts and notices
distinguish intent by more than colour. They do not — same text, no icon, no
prefix; colour is the only channel.

**We cite no snora accessibility documentation anywhere** — `grep` for `1.4.1`
and "use of colour" across `docs/`, `rfcs/` and `crates/` finds only SC 1.4.11
(non-text contrast) references, which are a different criterion.

**But we use their prefab toasts at ten call sites**, and the correction found a
real hole on our side: `docs/src/abdd-audit.md` §2 "Status is distinguishable
without colour" lists four rows — file-tree icons, toolbar badge, diff-view
lines, inspector rule blocks — and **toasts are not one of them**, even though
the same document commits v1.0.0 to "colour-independent status display" and
toasts are how the application reports the outcome of every save, export and
re-run. A row has been added.

Not RFC 106's to fix beyond that row; recorded here because this is the RFC that
carries the snora version decision.

### 5a.1 Two corrections to the same manifest line (added 2026-08-21)

Found while checking whether snora 0.40.0's `advanced`-feature change affects
us. It does not — we call `iced::advanced` nowhere, `lucide-icons` is absent
from `Cargo.lock`, and snora's `lucide-icons` feature is opt-in and we do not
opt in. But looking cost nothing and turned up two things about the same line.

**We build `snora-widgets` and never use it.** Our manifest reads
`snora = { version = "0.38", features = ["design"] }` and never disables
default features. snora's `default = ["widgets"]` and `widgets =
["dep:snora-widgets"]`, so the whole widget subcrate compiles into every
build.

Nothing in `crates/` imports from it. Our entire snora surface is:

| Import | Gate |
|---|---|
| `snora::design::{Tokens, Color, style, contrast}` | `design` |
| `snora::{AppLayout, Sheet, SheetEdge, SheetSize, Toast, ToastIntent, ToastPosition}` | engine, ungated |
| `snora::render`, `snora::toast::{sweep_expired, subscription}` | engine, ungated |

Only `snora::widget`, `snora::style`, and `snora::direction` sit behind
`widgets` (`snora-0.38.0/src/lib.rs:112-126`), and we import none of the three.
Note `snora::design::style` — which we *do* use — is the `design` facade's own
path reaching `snora-style` directly (RFC-055), not the `widgets` re-export.

**So the line should become:**

```toml
snora = { version = "0.44", default-features = false, features = ["design"] }
```

Two changes, one line, one B0 run. The version bump §5a already argued for, and
`default-features = false` dropping a subcrate from the build.

> **Correction, 2026-09-05 — the binary-size half of that claim is wrong.**
> Another snora consumer ran exactly this configuration and measured the
> `widgets` delta at **exactly zero: byte-identical binaries either way.** The
> linker removes code nothing calls. snora has since pinned the distinction in
> their own budget — the published figure is the cost of *using* a feature, not
> of enabling it.
>
> **The change is still worth making**, on two remaining grounds: compile time
> and dependency surface (the subcrate is still built), and that the manifest
> should state what we actually depend on — RFC 108 and RFC 110 both describe
> our dependency as "design only", which is currently untrue. Do not justify it
> on binary size, and do not expect the release artifacts to shrink.

**This also makes the manifest true.** RFC 108 and RFC 110 both describe our
dependency as "the `design` feature only". That has never been accurate, and
both were reasoning about a build shape we did not have. The conclusions in
each survive — they turn on what the `design` facade exports, which is
unaffected — but the premise was wrong and is worth not repeating.

**Verify at build, not on this analysis:** if anything fails to compile with
`default-features = false`, report it rather than restoring the default. That
would mean we depend on `widgets` somewhere this inventory missed, which is
itself the finding.

**Recommendation: bump here, in RFC 106, rather than as its own change.** RFC
106 is the first consumer of anything 0.39 adds. Bumping it earlier buys
nothing and spends a matrix run; bumping it later means RFC 106 either
hand-writes the F6 decode or waits. If the owner prefers the bump to travel
with RFC 110 instead, that works equally well — RFC 110 §6b cites 0.39's
corrected figures — but then RFC 106 must land after it, not alongside.

## 5b. Modal focus trapping — inherited, upstream-owned (added 2026-08-20)

**A keyboard user can Tab out of an open dialog or sheet.** This is latent
today only because §2.2's defect means `Tab` moves no widget focus at all. **§4
makes it observable**: the moment `Tab` delegates to iced's traversal, focus
will escape every modal this application has.

**Do not build a focus trap here.** snora's position as of 0.39.1 is *staged,
not shipped*: moving focus between zones already works without iced's
`advanced` feature, but trapping needs the *query* half — "which widget has
focus" — which does need `advanced`, and whether to enable it is an open
decision on their side. A downstream app has already demonstrated the need, so
the trigger to build it has fired.

A bespoke trap in aaai would duplicate the exact mechanism snora is deciding
whether to ship, and RFC 105's own logic applies: a workaround that gets
superseded is worse than a documented wait.

**Required of this RFC instead:** record the gap where a user meets it. The
keyboard help overlay (RFC 038) should not claim modal focus containment, and
§3.2's screen-reader exclusion is not the same statement. If §4 lands before
snora ships trapping, that is an accepted, recorded limitation — not a silent
one.

Source: `.git-exclude/review-requests/072-snora-0-39-1-review-2026-08-20.md`
§4.3, accepted in `.git-exclude/reviewed/072-snora-0-39-1-review-2026-08-20.md`.

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
