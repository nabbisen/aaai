# RFC 106 — Keyboard Operability

**Status.** **Accepted, implementation held** — approved by the owner
2026-10-18; §5 settled on `F6` / `Shift+F6` the same day. Lives in
`rfcs/accepted/`; the folder is the source of truth for this state (RFC 000).

> **Held 2026-11-04 — §6b is blocking.** iced 0.14's `button` is not focusable
> and has no focused state, so **§4's selected design cannot satisfy §6
> condition 1**, every control of which is a button. Found before
> implementation started. §6b.4 carries the options; §6b.7 adds option (d)
> from snora 0.57. **The owner chose (d) on 2026-11-09.** The handoff is
> rewritten around snora's focusable controls **after RFC 116 lands**, against
> the post-bump tree rather than a reading of snora's notes; until then it
> stays marked do-not-start. **The snora bump moved to RFC 116** (§5a).

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

> **Moved to RFC 116, 2026-11-09.** The bump is no longer part of this RFC.
> It was held here when §6b held this RFC, while RFC 110 and RFC 115 also need
> it, so it now ships on its own: `rfcs/accepted/116-snora-0-57-migration.md`.
> **Read RFC 116 for the manifest line** — in particular, §5a.1's plan to drop
> snora's `widgets` feature is reversed there, because §6b.7's option (d) uses
> it. What follows is kept as the record of how the target was chosen; it is
> not an instruction.

§5's addendum needs `snora = "0.39"` in the workspace manifest. Recorded here
rather than assumed, because a dependency bump costs a B0 matrix run and is the
owner's call.

**The bump is as small as a bump gets.** The entire 0.38.0 → 0.39.1 source delta
is three files — `lib.rs` (the `focus` re-export), `keyboard.rs`
(`cycle_zones`), and `design/render/tests.rs` (test-only). No palette value, no
`DIM_ALPHA`, no `render.rs`, no rendered output changes; snora's own guide
states no public item was renamed, removed, or retyped, and the file-level diff
confirms it.

### 5a.2 Retarget to `"0.53"` (added 2026-09-02; retimed 09-05, 09-06, 09-26, 10-28)

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

**0.43.0, 0.44.0 and 0.46.0 add nothing we compile against** — no API,
appearance, or feature-resolution change between them. **0.45.0 is breaking**:
it removed `Emphasis` and `Size`, and snora shipped that removal partly on our
confirmation that we use neither (§5a.4). We are provably immune, and taking it
now — while that confirmation is fresh and the removal is the only breaking
change in the span — is better than absorbing it later inside a larger jump.

`^0.44` does not admit 0.45 or 0.46, so this is one more one-line edit and the
same single B0 run.

**0.51.0 adds one reason that touches a surface we use.** From 0.41 to 0.50 the
toast close button measured **25.00 × 23.40**, failing snora's own 24 × 24
minimum on the height axis; 0.51.0 makes it exactly 24 × 24, asserted by a test.
We raise toasts at ten call sites, so that control is in our UI. **It still does
not meet the 44 px bar `docs/src/abdd-audit.md` §6 sets** — see that file's
`Toast dismiss ×` row, which needs an owner decision of its own.

0.51.0's other two findings — the active-tab indicator and its label — **do not
reach us.** We use neither snora's `TabBar` nor its tooltips; `app.rs:14` imports
only `AppLayout`, `Toast`, `ToastIntent`, `ToastPosition` and `render`.

**This target has now moved six times** (0.39 → 0.42 → 0.44 → 0.46 → 0.51 →
0.53) in nine weeks, because snora ships every few days. **The version written here is a
tested floor, not a claim about what is current.** What governs is the standing
instruction: **re-check at implementation time, read the migration guide of
anything newer, and report what you find either way.** I will stop recording
each release against this section individually — snora has stopped sending
per-release letters for the same reason, and a number revised five times informs
less than the instruction to go and look.

> **Retargeted to `"0.53"`, 2026-10-28.** snora resumed sending letters, and two
> arrived (0.52.0, 0.53.0; assessed in
> `.git-exclude/review-requests/084-snora-0-52-0-53-assessment-review-request.md`).
> Neither breaks any API we use, and the sentence above still stands — **what
> governs is re-checking at implementation time**, not the number written here.
>
> Three things in the span are worth carrying into the bump, none of them
> blocking:
>
> - **Long unbroken text now wraps instead of being cut off** (0.52.0). This is
>   a fix we benefit from directly: we put file paths in toast bodies at three
>   sites in `crates/aaai-gui/src/app/update/save.rs`, and they were truncating.
> - **Modal dim rendering changed** (0.53.0) — dark-theme dims are now visible,
>   light-theme dims slightly darker. This changes GUI baselines, not any
>   assertion; see RFC 110 §6b.3.
> - **Breadcrumbs gained an underline** (+1 px row height, 0.53.0). Baselines
>   only; we do not size any layout around that row.
>
> One item to **verify before relying on it**: RFC 110 §6b asserts the presence
> of the `snora-modal-dim` identifier, read from `snora-0.38.0`. Confirm it
> still exists and is still spelled that way in 0.53 at implementation time, and
> report either way.
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
snora = { version = "0.53", default-features = false, features = ["design"] }
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
8. **The focused appearance of `text_input` / `text_editor` is asserted
   against WCAG SC 1.4.11 (non-text contrast, ≥ 3:1)** in all four presets —
   see §6b.6. Buttons are excluded because iced 0.14 cannot focus one; that
   exclusion is §6b's subject and is **not** settled by this condition.

**Conditions 1–3 are suspended pending §6b's decision** — as written they name
controls that iced 0.14 cannot focus.

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

## 6b. **Blocking** — iced 0.14 cannot focus a button, so §4's design cannot meet §6 condition 1 (added 2026-11-04)

Found while investigating the owner's authorization to assert the focus ring
against SC 1.4.11
(`.git-exclude/review-requests/086-elderly-accessibility-assessment-review-request.md`
§6 decision 5). The assertion turned out not to be the finding. **There is
nothing to assert, because on most of our controls there is no focus at all.**

**This is a defect in this RFC, not in the implementation.** It was found before
the dev team started. §4 and §6 condition 1 need an owner decision before T1.

### 6b.1 The measurement

In iced 0.14, only **`text_input`** and **`text_editor`** implement the
`Focusable` operation — a grep for `operation::Focusable` across
`iced_widget-0.14.2/src/` returns those two and `helpers.rs`, and nothing else.

**`button` is not focusable and has no focused state.**
`iced::widget::button::Status` is `Active | Hovered | Pressed | Disabled`
(`iced_widget-0.14.2/src/button.rs:471-480`), and the string `focus` does not
appear in `button.rs` at all. So a button cannot be focused programmatically,
and a style function can never be told that one is focused.

`focus_next()` "focuses the next focusable widget"
(`iced_core-0.14.0/src/widget/operation/focusable.rs:162-165`) — which means
the next **`Focusable`** widget, and no button is one.

### 6b.2 What that does to §6 condition 1

Condition 1 requires that `Tab` reach *"both `Pick folder` buttons,
`Optional settings`, `Check changes`, and each recent-project `Open` and delete
control."* **Every control it names is a `button`.**

The Opening screen holds **6 buttons and 1 `text_input`**
(`crates/aaai-gui/src/views/opening.rs`). Under §4's selected design — delegate
`Tab` to `focus_next()` — pressing `Tab` there would cycle a single widget, the
lone text input, and reach **none of the six buttons the condition names**.

So **§4's design cannot satisfy §6 condition 1 on iced 0.14.** Conditions 2
and 3 inherit the same limit.

### 6b.3 Why our own tests would not have caught it

§6a asserts traversal **by effect** — `tap_key(Tab) … tap_key(Enter)` then
check the activation message. That is the right shape for what it tests, and it
is blind to this: a test that never asks "is anything visible?" passes whether
or not the user can see where they are. This project has made that exact
mistake before, in GUI automation, and the lesson is the same — **an effect the
screen never shows is not an effect the user has.**

### 6b.4 Options

| Option | Assessment |
|---|---|
| **(a) Build app-level focus** — extend `FocusTarget`, render our own ring from snora's unused `FocusTokens`, dispatch `Enter`/`Space` to the logically focused control | **Rejected.** This is precisely the "bespoke focus system" §4 refused, and larger: it means owning focus, traversal order, and activation for every widget type on every screen, forever, and then unwinding it when iced makes buttons focusable |
| **(b) Keep §4, narrow condition 1, record the gap** | **Recommended.** §4 is still right — it uses the toolkit's mechanism and deletes code. Deliver it for the widgets that *can* hold focus, restate condition 1 accordingly, and record the button gap where the user meets it |
| **(c) Raise it upstream** | **Recommended, alongside (b).** `button` focusability is iced's to fix and would benefit every consumer. snora is our existing channel and depends on iced |

### 6b.5 What (b) delivers, and what it does not

**Does deliver**, and it is not negligible: `Tab` starts working for the text
fields, including the **reason field** — the one input DEC-004 makes mandatory
and the most-used control in the application. It also deletes the broken
interception §2.2 documents, which is a correctness fix on its own.

**Does not deliver:** keyboard reach for buttons. Under (b) this RFC no longer
makes the GUI fully keyboard-operable, and **its title overstates what it
ships.** That is the owner's call to make, not mine to quietly absorb.

Record the gap the way §5b already records modal trapping: in the **keyboard
help overlay** (RFC 038), so the limitation is stated where a keyboard user
meets it rather than only in an RFC. `docs/src/abdd-audit.md` §1 ("Tab order
matches reading order") must also be scoped to focusable widgets rather than
left implying full coverage.

### 6b.6 The SC 1.4.11 assertion the owner authorized

It survives, scoped to where focus can exist:

- **`text_input` / `text_editor`:** `Status::Focused { is_hovered }` exists
  (`text_input.rs:1697-1709`), so the focused appearance is real and
  assertable. Assert the focused indication at **≥ 3:1** against the adjacent
  surface, in all four presets, following `contrast_check`'s shape — a
  documented module, named exemptions, traceability.
- **Buttons:** nothing to assert until (a) or (c) changes the situation.

Note that we do **not** currently apply snora's `FocusTokens` anywhere — no
`tokens.focus`, `ring_color`, `ring_width` or `ring_offset` appears in
`crates/aaai-gui/src`. The ring we render today is whatever iced draws for a
focused text input. The handoff's §6 bullet deferring "focus-ring visual
design" to RFC 101 rested on the premise that *"whatever iced draws by default
is accepted"*; that premise is sound for text inputs and vacuous for buttons,
which is the whole of this finding.

### 6b.7 Option (d) — snora 0.57 ships focusable controls (added 2026-11-09)

snora reached §6b's finding independently, in nearly the same words —
*"iced 0.14's `button` handles mouse and touch events only, and registers no
focus target. Only `text_input` and `text_editor` implement `Focusable`"*
(`snora-interaction-0.57.0/src/lib.rs`) — and shipped the fix. In 0.57:

- **snora's controls are focus targets** that Enter and Space press, publishing
  the message a click publishes, with a focus ring drawn from
  `Tokens::focus` (`snora-0.57.0/src/lib.rs`, doc on `pub mod design`).
- **`snora::design::button::{primary, secondary, ghost, danger, link}`** are
  such controls. They are **`widgets`-gated** (`snora-0.57.0/src/design.rs:103-111`),
  which is why RFC 116 keeps that feature.
- **`snora::keyboard::traverse(key, modifiers)`** routes plain Tab / Shift+Tab
  (`snora-0.57.0/src/keyboard.rs:337`) — iced binds no Tab itself.
- Each control's target is at least `Tokens::targets.min`, which RFC 116 sets
  to 44 — so a migrated button fixes RFC 115's target-size shortfall too.

**The limit.** Those five constructors take a **plain string label** and return
a finished `Element` (`secondary(tokens, label, on_press)`,
`snora-widgets-0.57.0/src/design/button.rs:120-126`) — no glyph-and-label row,
no custom content. The general-content `Control` exists in
`snora-interaction`, but snora does not re-export it and states that
applications should not depend on that crate. So **buttons with rich content**
— file-tree rows, suggestion chips, anything that is not one string — are not
reachable through the supported API today.

| Option | Assessment |
|---|---|
| **(d) Adopt snora 0.57's controls — chosen by the owner 2026-11-09** — text-label buttons become `design::button::*`, Tab routes through `keyboard::traverse`, and snora is asked to expose a general-content control for the rest | **Recommended, replacing (b) + (c).** Condition 1 becomes achievable for most controls, the focus ring is snora's and assertable, and the bespoke-focus objection to (a) disappears because snora owns focus, not us |
| (b) + (c), as recommended on 2026-11-04 | Superseded. (c) is answered — upstream shipped it |

**What (d) costs.** It touches every view that builds a button, so it is the
larger of the two. Rescoping this RFC under (d) means: T1–T5 rewritten around
`design::button::*` and `keyboard::traverse`; §6 condition 1 kept, scoped to
controls built from those constructors, with rich-content buttons listed by
name as a recorded gap until snora answers; §6b.6's SC 1.4.11 assertion
extended to snora's ring via `Tokens::focus`, the colour we pass. One question
goes to snora (`.git-exclude/upstream/snora/send/draft/aaai-to-snora-2026-11-09.md`).

**Sequencing.** After RFC 116. The rescoped handoff is mine to write once the
owner chooses.

**Three additions the rescoped handoff must carry** (2026-11-11, from the updated
GUI rules; review-request 089 §2, §5; the third from review 090 §4):

- **T0 — the shared session script.** Promote the dev team's RFC 115
  `launch.sh` to `.git-exclude/tools/gui-session.sh`, mechanics only, with
  aaai's facts in its header (089 §4).
- **The level-2 harness.** `iced_test` arrives here anyway. Build the headless
  matrix — label visibility, overflow, focusable hit area — over the axes that
  exist (language × theme × window size). Once buttons are snora controls they
  are `Focusable`, so their hit area is **measured**, which is what this RFC's
  focus work and RFC 115's target-size work both need. I verify one cell end to
  end before writing the handoff.
- **T0b — an injection point so tests never read the operator's settings.**
  `App::default()` calls `UserPrefs::load()` and `ProfileStore::load()`, which
  read the real `prefs.yaml` and `profiles.yaml`. That is gate S1's recorded GUI
  gap (RFC 104 §7a): read-only, but a test's result can depend on the machine.
  The level-2 matrix would build `App` for every cell, so it must be closed
  **before** the harness exists. RFC 104 §7a already names the shape: build
  `App` from injected prefs and profiles (`UserPrefs::load_from` exists), and
  move every test that uses `App::default()` or `..Default::default()` onto
  it. Today that is `app/tests.rs` — two RFC 098 tests and two RFC 116 tests.

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
