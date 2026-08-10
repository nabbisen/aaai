# RFC 105 — Visual Verification: Scope, Assertions, and Evidence Location

**Status.** Proposed

**Tracks.** Successor to RFC 017 (Visual Verification Harness & Protocol). Settles
where GUI verification evidence lives and which checks are tests rather than
screenshots.

**Depends.** RFC 017 (implemented, superseded in part by this RFC), RFC 099
(whose V1 gate is the forcing case), RFC 000 (lifecycle policy)

**Design owner.** high-capability model

**Decision owner.** nabbisen, project owner

**Proposed implementer.** mid-capability model, after design review and owner
approval

**Evidence location.** `.git-exclude/evidence/105-visual-verification/`

**Touches.** `rfcs/done/017-visual-verification-harness.md` (Status field only),
`scripts/list-unverified-rfcs.sh`, `docs/templates/visual-verification-template.md`,
`docs/src/testing.md` and its Japanese counterpart, `.gitignore`, and a new test
module under `crates/aaai-gui/src/`. No product behaviour change.

**Handoff.** Required, after acceptance.

---

## 1. Summary

RFC 017 built a visual verification harness. Every mechanical completion
condition it set was met, and all of them still hold today. **Its protocol has
2% adoption**: 99 of 101 done RFCs are reported UNVERIFIED, and the only RFC
carrying a verification card is RFC 017 itself.

This RFC does not blame the execution. It argues the protocol asked for
something unachievable, and replaces the unachievable part with two things that
can hold: a **scoped** obligation, and **assertions instead of screenshots**
wherever the property can be asserted.

## 2. What RFC 017 built, and what actually happened

### 2.1 The machinery works

| RFC 017 §5 completion condition | State today |
|---|---|
| `scripts/list-unverified-rfcs.sh` executable, enumerates unverified RFCs | **Holds** |
| `.gitignore` excludes the verification directory | **Holds** |
| `docs/templates/visual-verification-template.md` exists | **Holds** |
| CI logs the unverified count | **Holds** |

Nothing is broken. This is not a repair RFC.

### 2.2 The protocol did not take

```
$ ./scripts/list-unverified-rfcs.sh
…
Visual verification: 99 / 101 RFC(s) unverified (RFC 000 excluded).
```

One RFC has a `## Visual Verification` card, and it is RFC 017. RFC 099 — the
current GUI RFC, the one whose entire purpose is visual — does not reference the
protocol at all.

### 2.3 Root cause: the denominator includes RFCs with no visual surface

The script counts **every** RFC in `done/`. Among the 99 it reports unverified
are RFC 027 (a CI mdbook job), RFC 044 (`expires_at` enforcement in the audit
engine), RFC 057 (`aaai export`), RFC 066 (unit tests), and RFCs 095, 096, 097,
102, and 103 — none of which touch a pixel. RFC 103 is CLI and engine code
exclusively, and it is reported UNVERIFIED.

A metric that can never reach zero stops being read. 99/101 is not a backlog;
it is a permanently red light that everyone learned to walk past. The protocol
made *every* RFC owe a visual verification card, so *no* RFC owed one in
practice.

**This is the defect. Everything else follows from it.**

### 2.4 Second problem: two evidence locations, chosen by nobody

RFC 017 §3.1 specifies `rfcs/verification/<NNN>/` for screenshots, gitignored.
That directory has never existed.

RFC 099's T6 screenshots are in
`.git-exclude/evidence/099-gui-visual-foundation/screenshots/` — four PNGs, all
four themes, Opening screen at 800×550, captured 2026-07-28. RFCs 098, 102, and
103 all use the same `.git-exclude/evidence/<NNN>-<slug>/` convention for their
own evidence.

The newer convention won on merit and without a decision: it holds *all* of an
RFC's evidence — hosted runs, local results, revert demonstrations, diffstats —
not screenshots alone. Splitting screenshots into a separate tree would put two
halves of one RFC's evidence in two places.

### 2.5 What is not the problem

For the record, because I asserted otherwise earlier and it is worth correcting
where a future reader will find it: **RFC 000 does not forbid amending a done
RFC.** It forbids deleting and renumbering, and requires the Status field to
match the folder. RFC 017 §2.1's instruction to append a card to a `done/` file
therefore never conflicted with the lifecycle policy, and that conflict is not
why adoption failed.

## 3. Goals and non-goals

### 3.1 Goals

1. The unverified count reaches zero when the work is actually done, so a
   non-zero count means something.
2. Every visual property that *can* be asserted mechanically is a test, not a
   screenshot.
3. One evidence location for one RFC.
4. RFC 017's Status stops claiming a protocol that is not in force.

### 3.2 Non-goals

- Pixel-perfect screenshot diffing. RFC 017 §4.3 rejected it for good reasons
  — OS and font rendering differ — and this RFC does not revisit that.
- Automated GUI driving as a requirement. §5.4 permits it and deliberately does
  not mandate it.
- Retroactively verifying the 99. See §5.1.
- Changing RFC 099's V1 gate contract. This RFC changes where evidence lives and
  what form it takes, not what V1 demands.

## 4. The central distinction: assertable versus judgemental

RFC 017 treated visual verification as one thing: a human looks and records what
they saw. Two years of that produced one card.

Some visual properties do not need a human at all:

| Property | Assertable? | Why |
|---|---|---|
| Text contrast ≥ 4.5:1 (7:1 high-contrast) | **Yes** | Already asserted — `crates/aaai-gui/src/contrast_check/tests.rs` |
| Content fits at the minimum window size | **Yes** | Overflow is a computed layout fact |
| No hardcoded `.size(N)` or `Color::from_rgb` | **Yes** | Already asserted — RFC 099 V1 greps |
| Keyboard focus order | **Yes** | Application logic, not rendering |
| Is the visual hierarchy readable | **No** | Judgement |
| Does this screen look finished | **No** | Judgement |
| Is the Japanese text awkward at this width | **No** | Judgement |

`contrast_check` is the proof this works. It asserts a property across all four
themes, documents its exemptions (`text_muted`, by token contract), and traces
each assertion back to the measured figures in the RFC 099 gap analysis. It runs
in CI, needs no display, and cannot rot silently.

**The rule this RFC adopts:** if a property can be asserted, it must be a test.
Screenshots are reserved for judgement, and there should be few of them.

Eight PNGs that nobody diffs are theatre. One test that goes red when content
overflows at 800×550 is a gate.

## 5. Selected design

### 5.1 Scope the obligation

A `## Visual Verification` card is required from an RFC that meets **both**
conditions:

1. its Touches line includes `crates/aaai-gui/` — it changes a visual surface;
   **and**
2. its number is **≥ 106** — it was accepted after this policy.

Everything else is **out of scope**, not unverified.

> **Corrected 2026-08-10, before this RFC landed.** §5.1 originally had only
> condition 1, and §6 item 2 required the rescoped count to be "zero or a small
> number the owner recognises" or the rule was wrong. **Measured, it was 63 of
> 101** — down from 99, and nowhere near zero.
>
> The reason is structural: the GUI has been the subject of ~64 RFCs across the
> project's life and almost none carry a card, so scoping *by visual surface*
> does not shrink the problem, because the GUI RFCs **are** the problem. The
> count still could not reach zero, which is the exact defect this RFC exists
> to fix — I would have reproduced RFC 017's failure while claiming to correct
> it.
>
> There was also a contradiction between the rule as written and as
> implementable: the prose said RFCs "that shipped before this RFC" are out of
> scope, but a Touches-line check cannot know when something shipped. Condition
> 2 makes that testable instead of aspirational.
>
> Measured with both conditions: **0 today**, and RFC 106 — whose Touches line
> names `crates/aaai-gui/src/app.rs` — will be caught the moment it moves to
> `done/`. That is the behaviour §6 item 2 asks for.

Retroactive verification of the 63 is **not** required, matching RFC 017 §2.4's
own P3 tier, which also declined it. If a future reader needs a card for a
shipped RFC, they open a verification RFC for it.

### 5.2 One evidence location

`.git-exclude/evidence/<NNN>-<slug>/screenshots/` is the location.
`rfcs/verification/` is retired — it never existed, so nothing moves.

The verification **card** stays in the RFC, where a reader finds it without
access to `.git-exclude/`. The card records what was checked and the verdict;
the screenshots are the backing evidence and remain uncommitted.

This means an in-scope RFC gets its card appended when it moves to `done/`, or
in the amendment that records its verification. That is an ordinary architect
amendment, permitted by RFC 000 (§2.5).

### 5.3 Assertions first

An in-scope RFC's verification obligation is discharged in this order:

1. **Assert what can be asserted**, as tests under `crates/aaai-gui/src/`,
   following the `contrast_check` shape: a documented module, explicit
   exemptions, traceability to the requirement.
2. **Screenshot only what remains**, and say in the card why each one needed a
   human.

A card whose checks are *all* screenshots must justify that, because it usually
means an assertion was available and skipped.

### 5.4 Automated capture is permitted, not required

Where a project member has tooling to drive the GUI, they may use it. Capture
tooling is **not** part of this RFC and must not become a second protocol: it
produces evidence, it does not define what evidence is required.

If capture tooling is later maintained rather than ad-hoc, it needs its own RFC.
A script that encodes a protocol becomes the protocol, because the executable
version always wins over the documented one — which is how RFC 017 lost.

### 5.5 RFC 017's disposition

RFC 017 stays in `done/` — it was implemented, and RFC 000 forbids deletion. Its
Status field is amended to name this RFC:

```markdown
**Status.** Implemented (v0.20.0); scope and evidence location superseded by
RFC 105 (2026-08-03). The harness described here was built and still runs; its
protocol applied to every RFC rather than only visual ones, which is why
adoption stalled at 2%.
```

Per RFC 000's "Status fields that lie" anti-pattern, a Status that claims an
in-force protocol when 2% follow it is exactly the friction that policy warns
about.

## 6. Acceptance contract

1. `scripts/list-unverified-rfcs.sh` reports only in-scope RFCs, and its output
   states the scope rule.
2. Run against today's tree, its count is **zero or a small number the owner
   recognises** — not 99. If it is not, the scope rule in §5.1 is wrong and must
   be corrected before this RFC lands.
   > **Exercised 2026-08-10, before landing.** The first §5.1 rule produced
   > **63**; the item fired, and §5.1 was corrected rather than the threshold
   > relaxed. The corrected rule measures **0**. This item has now done the job
   > it was written for, which is the strongest evidence available that it is
   > load-bearing rather than decorative — the same standard §4 demands of the
   > assertions this RFC introduces.
3. A minimum-window-size overflow assertion exists under
   `crates/aaai-gui/src/`, follows the `contrast_check` shape, and **fails on
   the current Opening screen at 800×550** — see §7.
4. `docs/templates/visual-verification-template.md` reflects §5.3's
   assertions-first ordering.
5. RFC 017's Status field names this RFC.
6. `docs/src/testing.md` and its Japanese counterpart describe the scope rule
   and the single evidence location.
7. `.gitignore`'s `verification/` entry is removed, or a comment records why it
   is kept.
8. No product behaviour change; GUI test count grows only by the named new
   assertion.

## 7. The forcing case: this must fail first

Acceptance item 3 requires the overflow assertion to be **red when written**.

The Opening screen currently overflows at 800×550 — a scrollbar is visible in
`.git-exclude/evidence/099-gui-visual-foundation/screenshots/dark-opening-800x550.png`
and reproduces on a fresh capture. RFC 099's V1 names 800×550 as a required
size, and T6 recorded the scrollbar as unresolved.

So the assertion has a known-failing case available on day one. Writing it green
would prove nothing; writing it red proves it measures the thing V1 cares about.

**Whether to fix the overflow is RFC 099's decision, not this RFC's.** This RFC
only requires that the failure become mechanically visible instead of a note in
an evidence file.

## 8. Risks

| Risk | Mitigation |
|---|---|
| The scope rule excludes an RFC that should have been verified | Touches lines are explicit; an RFC that changes GUI code and omits `crates/aaai-gui/` from Touches has a worse problem than this rule |
| "Assertable" is argued case by case until everything is a screenshot again | §4's table is the default answer; a card claiming a property is unassertable must say why |
| A third evidence location appears later | §5.2 names one; RFC 000's cross-reference-rot anti-pattern applies |
| This RFC becomes the next RFC 017 — a protocol nobody follows | Acceptance item 2 is the guard: if the count is not near zero on day one, the design is wrong and does not land |
| It delays the v0.41.0 release | It does not gate the release. RFC 099 T6/T7 may be captured manually under the old convention first; §5.2 costs nothing to apply afterwards |

## 9. Alternatives considered

| Option | Decision |
|---|---|
| Fix `list-unverified-rfcs.sh` to scope, change nothing else | Rejected — makes the number honest but leaves screenshots as the only verification form, which is what did not scale |
| Retroactively verify all 99 | Rejected — enormous, and most have no visual surface. RFC 017 §2.4 already declined the equivalent |
| Delete RFC 017 | Rejected — RFC 000's first anti-pattern. It records why the harness exists |
| Amend RFC 017 in place instead of a successor | Rejected — the change is a scope reversal, not a correction, and RFC 017's implementation history stays readable if left intact |
| Pixel-diffing screenshots in CI | Rejected — RFC 017 §4.3's reasoning holds; OS font rendering differs and the diffs would be noise |
| Require automated GUI capture | Rejected — §5.4. It would make a tool the protocol |

## 10. Review questions

1. Is `crates/aaai-gui/` in the Touches line the right scope test, or should the
   card be required whenever an RFC changes user-visible output of any kind,
   including CLI formatting?
2. Should the verification card live in the RFC (§5.2) or in the evidence
   directory beside its screenshots?
3. Is acceptance item 2's "zero or a small number the owner recognises" a strong
   enough guard, or should it name an exact expected count?
4. Should the overflow assertion in §7 gate CI immediately, or land as
   `#[ignore]` until RFC 099 decides on the fix?

## 11. Sources

- `rfcs/done/017-visual-verification-harness.md` — §2.1, §3.1, §4.3, §5
- `rfcs/done/000-rfc-lifecycle-policy.md` — anti-patterns "Deleting completed
  RFCs", "Status fields that lie"
- `crates/aaai-gui/src/contrast_check/tests.rs` — the assertion precedent
- `scripts/list-unverified-rfcs.sh` — current output, 99 / 101
- `.git-exclude/evidence/099-gui-visual-foundation/` — the competing location
- RFC 099 §6 (gate V1) and its T6 screenshots
