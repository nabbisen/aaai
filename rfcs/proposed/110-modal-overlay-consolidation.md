# RFC 110 — Modal Overlay Consolidation

**Status.** Proposed — **accepted for implementation by the owner,
2026-08-20.** Under the 4-folder lifecycle an accepted RFC stays in
`proposed/` until it ships; RFC 109 moves it to `accepted/`.

**Tracks.** V1 (GUI tokens + contrast). Extends the gate RFC 099 established
and RFC 108 repaired — see §8.

**Depends.** RFC 100 (GUI module boundaries), which restructures the file this
RFC changes. RFC 108 (snora 0.38.0), which supplies the mechanism §4 selects.
Scheduled after both, alongside RFC 104 and RFC 106.

**Design owner.** high-capability model

**Decision owner.** nabbisen, project owner

**Proposed implementer.** mid-capability model, after design review and owner
approval

**Evidence location.** `.git-exclude/evidence/110-modal-overlay-consolidation/`

**Touches.** `crates/aaai-gui/src/app.rs`'s `view()` overlay composition and
its `EscapeKey` / `CloseModals` handlers, or their post-RFC-100 successors;
`crates/aaai-gui/src/contrast_check/` for the new assertion; RFC 099's V1 grep
script. No engine, CLI, persisted-format, or i18n change; no dependency added
or version changed.

**Handoff.** Required, after acceptance.

---

## 1. Summary

**Three modal overlays paint a hard-coded black backdrop outside the design
system.** They are composited after snora has finished rendering, so they
inherit no token, respond to no preset, and are invisible to the V1 contrast
gate. On the high-contrast presets — the presets that exist precisely for users
who need them — the backdrop is the one surface in the application that ignores
the preset entirely.

Mapping the three overlays for this RFC also surfaced **two behavioural
divergences in the `Escape` path** (§2.3). One of them defeats RFC 086's
deliberate two-step reveal of a data-losing action. Both are pre-existing, both
are fixed by construction by §4's design, and neither was found by any gate.

## 2. The defect

### 2.1 Three literals, outside the system

`view()` builds an `AppLayout`, hands it to snora's `render()`, and then
composites overlays on top of the returned `Element`:

```rust
let base: Element<'_, Message> = render(layout);   // snora ends here

if self.settings_open {
    let backdrop = mouse_area(container(space()…).style(|_| container::Style {
        background: Some(iced::Background::Color(Color {
            r: 0.0, g: 0.0, b: 0.0, a: 0.35,
        })),
        ..Default::default()
    })).on_press(Message::CloseSettings);
    let dialog = iced::widget::center(crate::views::settings_dialog::view(…));
    stack![base, backdrop, dialog].into()
} else if …
```

| Overlay | RFC | `app.rs` | Alpha | Outside click |
|---|---|---:|---:|---|
| Settings dialog | 036 | `:2094` | 0.35 | `Message::CloseSettings` |
| Keyboard help | 038 | `:2127` | 0.35 | `Message::CloseHelp` |
| Navigation guard | 041 | `:2157` | 0.50 | `Message::NavGuardCancel` |

All three are `r: 0.0, g: 0.0, b: 0.0` — pure black, fixed, in every preset.

The **batch approve sheet is not affected**: it goes through
`layout.sheet(Sheet::new(…))` at `:2067` and therefore gets snora's dim
already. It is the counter-example that shows the correct shape is already in
this file.

> **It is also unreachable (2026-08-20).** Nothing anywhere sends
> `Message::OpenBatchSheet`; RFC 007 removed the toolbar button and RFC 008
> made the bottom bar the sole approval mechanism. So `:2067` is a valid
> reference for **how `layout.sheet(...)` is called** and nothing more —
> **do not try to compare against it visually, there is nothing to look at.**
> The whole batch cluster is proposed for deletion ahead of RFC 100
> (`.git-exclude/reviewed/072-snora-0-39-1-review-2026-08-20.md` §4), so this
> referent may be gone by the time this RFC is implemented; the `Sheet` call
> shape is documented upstream regardless.

### 2.2 Why "black at 35%" is wrong, not merely unprincipled

A backdrop is not decoration. It is the boundary that tells a user which
surface is live, and SC 1.4.11 puts a 3:1 floor on the contrast of a non-text
boundary. snora measured exactly this and repaired it in 0.38.0
(`src/design/render.rs:55-70`):

> `snora_design::surfaces::DIM_ALPHA` is `0.44`, repaired because the `light`
> preset's dialog card was measured at 2.85:1 against its own dimmed backdrop,
> below SC 1.4.11's 3:1 floor, by either signal (border or fill).

> **Read this clause correctly (corrected 2026-08-21).** An earlier revision of
> this RFC claimed the trailing clause was withdrawn upstream. **It is not.**
> snora checked and it holds: "below the floor **by either signal**" attaches to
> *below the floor*, not to *2.85*. Pre-repair at `DIM_ALPHA` 0.40 the worst
> `fill ǀ dim` was **2.85** and the worst `border ǀ dim` was **1.00** — under
> 3:1 whichever signal you pick, which is what the sentence says.
>
> What the 0.38→0.39 guide withdrew is a **different** claim: that the border is
> what separates the card from the dim *in normal operation*. It is not, at
> ≈1.00:1 against the dim. `render.rs` never said that.
>
> The lesson survives even though my reading of this sentence did not: it is
> still not a source to derive an assertion from, because it describes a
> **pre-repair failure**, not a pair to test. §6b.1 is what happened when this
> RFC's first draft treated it as one.

Two things follow. First, **0.35 is below the alpha that a peer team measured
as insufficient at 0.40** — our backdrop is weaker than the one that failed.
Second, and more important, snora's fix is not "use a bigger number": the
derivation is `snora_design::surfaces::modal_dim`, which is preset-aware —

> the derivation never tries to move a color away from its own tone; it only
> ever chooses between two fixed, maximally-distinct poles.

On a dark preset, dimming *toward black* reduces separation rather than
increasing it. A fixed black literal cannot express that, at any alpha.

### 2.3 Two divergences found while mapping the overlays

Each overlay has a close handler that does real work, and `Message::EscapeKey`
at `app.rs:1791` re-implements all three by flipping a bool:

```rust
Message::EscapeKey => {
    if self.help_open            { self.help_open = false; }
    else if self.nav_guard_open  { self.nav_guard_open = false; }
    else if self.settings_open   { self.settings_open = false; }
    else                         { self.selected_index = None; }
}
```

**D1 — Escape leaves the nav guard's discard button revealed.** RFC 086 hides
the data-losing "discard" action behind a deliberate second step
(`nav_guard_show_discard`). `NavGuardCancel` (`:1724`) clears **both** flags.
`EscapeKey` clears only `nav_guard_open`. So a guard dismissed with `Escape`
and later reopened presents the discard action already revealed — the two-step
protection is spent, silently, and only for the keyboard path.

**D2 — Escape keeps a theme the user cancelled.** RFC 093 gives the settings
dialog a live theme preview; `CloseSettings` (`:1897`) reverts `self.theme` and
`self.design_tokens` on cancel. `EscapeKey` does not. So `Escape` abandons the
draft while *keeping* the previewed theme applied — the opposite of what
"cancel" means, and inconsistent with both the Cancel button and the backdrop
click.

**A third, latent one:** the two chains disagree on precedence. `view()` tests
settings → help → nav guard; `EscapeKey` tests help → nav guard → settings.
The flags are mutually exclusive today, so nothing misbehaves, but the
disagreement is only invisible because of an invariant neither chain states.

Also noted, not claimed: `EscapeKey` does not touch `batch_sheet_open`, and we
never call snora's `keyboard::dismiss_on_escape`, so the batch sheet appears
not to be `Escape`-dismissible. **Verify before acting** — it is adjacent to
this RFC, not established by it.

### 2.4 Why it went unnoticed

RFC 099 established V1's gate against **token bypass**. It is
`grep -rn "Color::from_rgb" crates/aaai-gui/src/` (RFC 099 §6 item 2). These
three overlays write `Color { r, g, b, a }` — a **struct literal**, which that
pattern does not match. `contrast_check` is equally blind by a different route:
it enumerates token roles, and a backdrop that is not a token has no role to
enumerate.

**And the gate is not a script.** RFC 099 §6 items 1 and 2 are two `grep`
commands written into a prose acceptance contract, re-run by hand whenever
someone remembers. `scripts/` holds `bump-version.sh`, `check-i18n-keys.py`,
and `list-unverified-rfcs.sh` — nothing that runs these. So V1's token gate
survives only as long as each implementer re-reads RFC 099 and each reviewer
re-runs it, and it is checked on no other RFC's diff.

This is the third finding in this area that a gate did not catch, after RFC
108's `text_muted` exemption and RFC 106's keyboard interception. The pattern
is consistent: **our gates check the things that are inside the system, and
each defect was something standing outside it.**

## 3. Goals and non-goals

### 3.1 Goals

1. All three overlays derive their backdrop from the design system, so the
   backdrop responds to preset changes including the high-contrast presets.
2. The V1 grep cannot be bypassed by a `Color { … }` struct literal.
3. D1 and D2 are fixed, and `Escape` and outside-click become the same code
   path rather than two implementations of one intent.

### 3.2 Non-goals

- Changing which controls each dialog offers, or its content, layout, or copy.
- Animation or transition on modal open/close.
- The batch sheet's behaviour, which is already correct (§2.1).
- Adding `Escape` dismissal to the batch sheet. Named in §2.3 so it is not
  lost; it needs its own verification and does not belong in a consolidation.

## 4. Selected design

**Route all three overlays through `AppLayout::dialog`, as the batch sheet
already routes through `AppLayout::sheet`.**

| Option | Assessment |
|---|---|
| Raise the alphas to 0.44 | **Rejected.** Copies snora's number without its reasoning. Still pure black, still preset-blind, still invisible to the gate — and §2.2's point is that a fixed color is wrong on a dark preset at *any* alpha |
| Add a `backdrop` token to our own `Tokens` and style locally | Rejected. Duplicates a derivation snora has already measured and tested, and we would own keeping it correct across every future preset snora ships |
| Reach `snora_design::surfaces::{DIM_ALPHA, modal_dim}` directly | **Rejected as unavailable.** `snora::design` enumerates its re-exports (`src/design.rs:2-5`) and `surfaces` is not among them. Using it means a direct `snora-design` dependency or an upstream ask — both unnecessary given the option below |
| **Pass each overlay to `layout.dialog(Dialog::new(content))` before `render()`** | **Selected.** snora paints the dim, applies `modal_dim`'s preset-aware derivation, styles the dialog card, and installs the outside-click capture. No new dependency, no upstream ask, and it deletes the `stack!` composition rather than adding to it. **At snora ≥ 0.41 it also inherits pointer containment** — see below |

The API is already available to us and needs nothing new:

```rust
// snora::{AppLayout, Dialog} — re-exported at snora/src/lib.rs:84-88,
// engine modules, present regardless of feature selection.
if let Some(content) = self.modal_content() {
    layout = layout.dialog(Dialog::new(content));
}
let base = render(layout);   // snora now owns the backdrop
```

`Dialog` is `Dialog::new(content)` and nothing else
(`snora-core/src/overlay.rs:44-52`); `AppLayout::dialog` takes one
(`snora-core/src/layout.rs:255`). The layout holds **one** dialog slot, which
is sufficient: the three overlays are already an `if / else if / else if`
chain, so at most one is ever composited.

`on_close_modals(Message::CloseModals)` is **already wired** at `:2061` for the
batch sheet, so outside-click arrives with no new plumbing.

### 4.0a A second reason, from snora 0.41.0 (added 2026-09-02)

snora 0.41.0 fixed a real input bug: **overlays did not contain pointer
events.** A click or scroll on a dialog, toast, or modal dim could reach
whatever was rendered beneath it — including, for a dialog, dismissing itself
when its own content was clicked. Four surfaces should have contained pointer
input; only the sheet did.

**Our three overlays are hand-rolled, so that fix reaches them at no version.**
`stack![base, backdrop, dialog]` composes them outside snora entirely. Routing
through `AppLayout::dialog` is what collects the fix, and it applies to the
navigation guard — a confirm-before-discard flow — where snora's own framing is
that *"a dialog could be bypassed by clicking through it to the control it was
meant to be guarding."*

This needs **snora ≥ 0.41**; RFC 106 §5a.2 carries the manifest bump to
`"0.42"`. If RFC 106 lands first, as the sequence has it, this RFC inherits the
version and needs no manifest change of its own.

Whether the defect is observable in our overlays today is **untested** — §6
item 3a requires checking before and after rather than assuming either way.

### 4.1 What this forces, and why that is the point

One dialog slot and one close message mean the three overlays must agree on a
single "which modal is open" question and a single "close it" answer. Concretely:

- a `modal_content()` accessor becomes the **one** place that decides
  precedence, replacing `view()`'s chain and resolving §2.3's disagreement by
  removing the second chain;
- `Message::CloseModals` (`:1947`, today one line closing the batch sheet)
  becomes the **one** dismissal path, delegating to the existing
  `CloseSettings` / `CloseHelp` / `NavGuardCancel` handlers so each keeps its
  real work — the theme revert, the discard-flag reset;
- `Message::EscapeKey`'s three inline arms are **deleted** in favour of
  emitting `CloseModals`, which is what fixes D1 and D2. They are not fixed by
  patching the missing lines into `EscapeKey`; they are fixed by there no
  longer being a second implementation to drift.

That is the substance of this RFC. The backdrop color is the finding; the
duplicated dismissal logic is the cause.

### 4.2 Banked, not adopted: `snora::keyboard::dismiss_on_escape`

snora ships a helper with the shape
`dismiss_on_escape(has_modal, has_menu, on_close_modals, on_close_menus, key)
-> Option<Message>` (`snora/src/keyboard.rs:61-67`), which decides
**modal-beats-menu** precedence in one place.

**Not adopted here.** It resolves modal-vs-menu; this RFC's problem is the
three-way settings / help / nav-guard question, which the helper knows nothing
about. Our `context_menu` and `header_menu` are out of scope.

Recorded so that whoever scopes menu unification later finds it instead of
re-deriving it — the precedence property §4.1 wants already exists upstream for
the adjacent question.

## 5. The open question this RFC must settle

**The navigation guard's 0.50 backdrop becomes 0.44 with every other modal.**

The guard is the only overlay that dims harder, and it guards the only
data-losing action. Two readings, and the review should pick one:

| Reading | Consequence |
|---|---|
| **Deliberate — the heavier dim signals a heavier decision** | Then it is a real design intent that §4 discards, and it needs somewhere to live. snora offers no per-dialog alpha, so it would have to be expressed another way — card emphasis or an intent on the guard's own content, not the backdrop |
| **Incidental — 0.35 and 0.50 were both picked by eye** | Then unifying at a measured 0.44 is a straight improvement and there is nothing to preserve |

> **The pre-check this recommendation was accepted with is not available
> (2026-08-20).** I told the owner the outstanding S4 judgement would test 0.44
> on the batch sheet before implementation. It cannot: that surface is
> unreachable (§2.1). This recommendation now rests on snora's own measurements
> alone — the dim against the card fill at worst **3.16:1** (`dark`), the dim
> against the page background at worst **3.2424:1**, both against a 3.0 floor —
> with visual confirmation arriving at §6 item 3 **after** implementation rather
> than before. §6b no longer helps here: it asserts that snora painted the
> backdrop, not what the backdrop measures (§6b.2). Weaker than described at
> acceptance, and recorded here rather than left implicit.

**Recommendation: treat it as incidental, and unify.** RFC 041 never mentions
an alpha. It says the guard "uses the existing `stack! + backdrop + center()`
modal" pattern (line 48) and specifies only that "backdrop click →
`Message::NavGuardCancel` (same as explicit Cancel)" (line 160). So the guard
was written by copying RFC 036/038's overlay and changing the number without
recording why — which is the definition of incidental. Note too that line 160
specifies exactly the equivalence §4.1 restores. If the owner reads it as
deliberate, the honest response is a separate change that expresses "heavier
decision" through something snora *does* model, not by keeping a literal.

## 6. Acceptance contract

1. No `Color {` struct literal remains in `crates/aaai-gui/src/`, except where
   a recorded rationale accompanies it.
2. All three overlays render through `AppLayout::dialog`; the `stack![base,
   backdrop, dialog]` composition is gone from `view()`.
3. Opening each of the three overlays under each preset — including both
   high-contrast presets — shows a backdrop that visibly changes with the
   preset. Evidence: one screenshot per overlay per preset, per RFC 105's
   scope rules.
3a. **Pointer events are contained** (added 2026-09-02). With each overlay
   open, a click on the dialog's own padding or plain text does **not** dismiss
   it, and a click or scroll over the backdrop does not reach the content
   beneath. Test the **navigation guard** specifically: it gates RFC 086's
   data-losing discard, and a modal that does not block input can be bypassed by
   clicking through to the control it guards. **Check this before the change as
   well as after** — our overlays are hand-rolled, so if the defect is present
   today that is a finding worth recording, not just a box to tick afterwards.
4. **D1 is fixed and tested:** open the nav guard, reveal discard, dismiss with
   `Escape`, reopen — the discard action is hidden again. **Written before the
   fix, this test must fail.**
5. **D2 is fixed and tested:** open settings, change theme (live preview
   applies), dismiss with `Escape` — `self.theme` equals the pre-open value.
   **Written before the fix, this test must fail.**
6. `Escape` and outside-click produce identical state for all three overlays,
   asserted for each.
7. RFC 099 §6 items 1 and 2 pass, plus §6a's new pattern.
8. Each of the three overlays asserts the presence of snora's
   `snora-modal-dim` identifier once open — see §6b. No new contrast assertion;
   §6b.2 says why.
9. Test counts change only by the named tests, and the delta is reported as a
   measured figure, not a predicted one.

## 6a. The V1 grep extension

RFC 099's V1 greps gain a pattern for the construction that slipped through:

```
Color\s*\{
```

Run against `crates/aaai-gui/src/`, excluding `design_tokens/` where token
definitions legitimately construct colors, and excluding test modules. This is
the hole §2.4 identifies; closing it is the durable half of this RFC, and it is
worth landing **whether or not** §4 is accepted.

Note the limit honestly: this catches one more spelling, not all of them. A
grep enumerates known bypasses and will always trail the ways there are to
write a color. §6b is the check that does not depend on spelling.

## 6b. The assertion that does not depend on spelling

> **Rewritten 2026-08-21.** Earlier revisions asserted contrast pairs. That was
> wrong twice over — first the wrong pairs (§6b.1), then the right pairs
> measured on tokens that are not ours to test (§6b.2).

**Assert that the overlays go through snora, by identifier.**

snora attaches stable, documented identifiers to the surfaces it renders
itself. The relevant one is **`snora-modal-dim`** — "the full-window scrim
shown while a dialog or sheet is open" (`snora-0.38.0/src/identifiers.rs:40-49`).
It exists precisely so consumers can assert against snora's own surfaces.

For each of the three overlays, in `iced_test`:

```
open the overlay  →  assert an element with Id "snora-modal-dim" is present
```

**This is the assertion that matches what this RFC changes.** §4 replaces three
hand-rolled `stack![base, backdrop, dialog]` compositions with
`layout.dialog(...)`. The identifier is present if and only if snora painted
the backdrop — so the test fails today, passes after §4, and fails again the
moment anyone reintroduces a local backdrop. It depends on no colour literal,
no spelling, and no token value.

**If `iced_test` cannot select by `Id`**, report that rather than substituting a
weaker check; RFC 106 §6a's investigation covers what the harness can and
cannot do, and this is a question for the same investigation.

### 6b.2 Why the contrast assertions were dropped

The previous revision asserted `dim over background vs background >= 3.0` and
`card border vs background >= 3.0` for every preset, quoting snora's measured
worst cases.

**Those measure snora's tokens, and we do not modify them.**
`design_tokens::tokens_for()` returns `Tokens::light()`, `Tokens::dark()`,
`Tokens::high_contrast_light()`, `Tokens::high_contrast_dark()` unmodified —
no override anywhere in `crates/`. So the assertion would compute snora's
numbers from snora's constants and compare them against snora's threshold. It
can only fail when snora breaks, and snora's own suite
(`src/design/render/tests.rs`) asserts both pairs already and fails first.

**Contrast that is genuinely ours stays asserted.** `contrast_check` covers
text-role-on-surface pairs because *we* choose which role goes on which
surface — that choice is ours and can be wrong, as RFC 108's `text_muted` work
showed. The dim-over-background composition involves no choice of ours at all.

The distinction worth keeping: **assert your own decisions, not your
dependency's constants.**

### 6b.1 Why the first draft was wrong

It asserted `dim vs dialog_card_fill` and `dim vs dialog_card_border`. Both are
mistakes, and the second is not a near-miss:

- **`dim vs card_border` cannot pass.** In `light`, `background` is pure white,
  the border sits at 3.38:1 against it (relative luminance ≈ 0.261), and the
  dim — black at `DIM_ALPHA` 0.44 over white — lands at ≈ 0.273. snora's own
  measured worst points confirm it: `border ǀ dim` is **1.04** in `light` and
  **1.00** in `dark`. The border is invisible against the dim by construction.
  It does clear the floor in both high-contrast presets (4.58 / 4.45, where the
  two signals cross), so the assertion would have passed on two presets and
  failed on two — the worst kind of wrong, since it looks like a real finding.
- **`dim vs card_fill` is degenerate where it matters.** snora pins
  `surface_raised == background` in both light presets as a deliberate token
  choice (`fill_equals_background_in_light_presets_by_token_design`), so in
  `light` and `high_contrast_light` this pair *is* the dim-vs-background pair,
  and in the dark presets it measures something no accessibility criterion
  names. That token fact is precisely **why snora tests the border against the
  background rather than the fill.**

The lesson is the same one §2.4 draws about our greps: an assertion invented
from a plausible reading of a document is not the same as one derived from the
measurement it claims to reproduce. §2.2's quote was read as licensing a
border-against-dim check; it does not.

## 7. Risks

| Risk | Mitigation |
|---|---|
| snora's dialog card styling changes the three dialogs' appearance, not just their backdrop | Real, and expected — snora styles the card (`render.rs:192`). §6 item 3's screenshots make it visible before acceptance rather than after. If a dialog looks wrong inside snora's card, that is a finding about our content, and it should be reported, not worked around with another local `stack!` |
| One dialog slot proves insufficient if two overlays must ever coexist | They are mutually exclusive today by construction (§4). If a future overlay needs to stack, that is a design question for snora and an upstream conversation, not a reason to keep three literals now |
| Unifying dismissal changes behaviour the three RFCs specified separately | §5 settles the one difference that is visible; §4.1's delegation preserves each handler's actual work. Any behaviour that changes is recorded as a superseding note against RFCs 036, 038, and 041 |
| Collides with RFC 100 | Same file, same `view()`, same `update()`. Sequenced after RFC 100 for exactly this reason — see §8 |
| §6b's threshold is snora's, adopted without our own measurement | It is SC 1.4.11's threshold, not snora's; snora's contribution is having measured against it. Our assertion measures our presets independently |

## 8. Milestone placement

This is V1 work — the same gate RFC 099 opened and RFC 108 repaired. V1 sits in
MG1, which is **already closed**, so this reopens a gate rather than extending
an open one. That is the correct reading: V1 was declared met on evidence that
§2.4 shows was incomplete twice over.

**Proposal: attach to MG2's release unit alongside RFC 104 and RFC 106**, all
three landing after RFC 100. Sequence: **100 → 104 → 106 → 110 → 101**. RFC 110
goes last of the four because it touches both `view()` and `update()`, which
RFC 106 also rewires.

**§6a is separable and should not wait.** It adds no crate change and would
have caught this. Recommend landing it immediately, independently of the rest —
and as a script rather than another line of prose, for the reason §2.4 gives.

Owner decision, since it adds an item to a unit and reopens a closed gate.

## 9. Sources

- `crates/aaai-gui/src/app.rs` — `:1724`, `:1791`, `:1897`, `:1947`,
  `:2055-2172`
- `snora` 0.38.0 — `src/design/render.rs:55-70` (the `DIM_ALPHA` repair),
  `src/design.rs:2-5` and `:24-26` (what the `design` facade exports),
  `src/lib.rs:84-88` (engine re-exports)
- `snora-core` 0.38.0 — `src/overlay.rs:44-52` (`Dialog`),
  `src/layout.rs:255` and `:303-308` (`dialog`, `on_close_modals`)
- `.git-exclude/reviewed/071-rfc108-snora-0-38-migration-review-2026-08-20.md` §6
- RFC 036, RFC 038, RFC 041, RFC 086, RFC 093 (all `done/`)
- RFC 099 §6 items 1–2 (the V1 grep gate, as prose), RFC 105 (screenshot
  evidence scope)
