# RFC 109 — Sync the lifecycle policy, adopt the 5-folder variant, bound the decision register

**Status.** Proposed

**Tracks.** Project governance. No milestone or gate — this is process, not
product.

**Depends.** Nothing. Should land **between** implementation waves — see §6.

**Design owner.** high-capability model

**Decision owner.** nabbisen, project owner

**Proposed implementer.** mid-capability model, or the architect — this touches
no `crates/` file.

**Evidence location.** `.git-exclude/evidence/109-lifecycle-sync/`

**Touches.** `rfcs/done/000-rfc-lifecycle-policy.md`, the `rfcs/` folder
structure, `rfcs/README.md`, cross-references in every RFC that moves, and
`.git-exclude/specs/decision-log/decision-log.md`. No code.

**Handoff.** Not required — the work is a file move plus a link sweep, fully
specified here.

---

## 1. Summary

Three related corrections, landing together because the lifecycle policy's own
self-application clause says to:

> combine the policy's introduction with the migration into a single, atomic
> change.

1. **The tracked lifecycle policy is stale.** `rfcs/done/000-rfc-lifecycle-policy.md`
   is 500 lines; `.git-exclude/rules/000-rfc-lifecycle-policy.md` is 617 and is
   the current authority. Sync them.
2. **Adopt the 5-folder variant.** Add `accepted/`. This project's governance
   makes "the owner signed off" a distinct event from "the implementer
   finished", which is exactly the condition the policy names.
3. **Bound the decision register**, so `DEC-`/`RISK-` stop drifting toward being
   a parallel RFC system. §5.

## 2. The tracked policy is stale, and in a way that matters

The two copies differ by **117 lines, entirely additive**. Everything the
tracked copy says is still true; it is missing a whole convention:

| Missing from the tracked copy | Effect |
|---|---|
| `## Companion handoffs` — the whole section | The convention this project uses at `rfcs/handoffs/NNN-slug/` is undocumented in its own policy |
| `handoffs/` in the folder layout | A reader of the tracked policy would not know the directory is legitimate |
| "handoff status is inherited from the RFC" | We already practise this; it is written nowhere tracked |
| Two anti-patterns: **turning handoffs into a second lifecycle**, and **letting handoffs override RFC decisions** | Both directly relevant — see below |

The second anti-pattern is worth quoting, because this project has repeatedly
done the right thing without the rule being written down:

> when a handoff discovers a design problem, patch or supersede the RFC first.
> Then update the handoff so it describes execution of the current RFC, not a
> competing design.

That is precisely what happened with **RFC 107** (writing the handoff exposed
two RFC defects; the RFC was corrected first) and **RFC 100** (design review
corrected the RFC, then the handoff was regenerated). The practice is already
right. The policy should say so.

## 3. The 5-folder variant fits this project's stated condition

The policy's test:

> Use this variant if "the maintainer signed off" is a meaningful event distinct
> from "the implementer finished." Skip it otherwise — `accepted/` will sit
> empty in projects where the two events collapse.

On this project those events are emphatically distinct. Owner approval is a
formal gate that every RFC passes through separately from design review, and the
two-tier role structure separates whoever designs from whoever implements.
`accepted/` will not sit empty.

### 3.1 The transition that fills it

**`proposed/` → `accepted/` on the owner's approval to implement.** Design-review
acceptance alone is not enough: it is necessary, not sufficient. The folder
tracks *authorisation*, which is the owner's act.

That distinction is currently invisible. `proposed/` today holds RFCs that have
never been reviewed alongside one whose handoff is written and whose
implementation is authorised — two states a reader cannot tell apart.

## 4. Where today's RFCs land

| RFC | Folder | Why |
|---|---|---|
| **108** — snora 0.38 migration | **`accepted/`** | Owner approved all four §11 decisions 2026-08-19; handoff written |
| **100** — GUI module boundaries | `proposed/` | Design re-review 070 says Accept, but **owner approval has not been given** — it was asked for and the conversation moved on |
| 101 — guided review flow | `proposed/` | Never design-reviewed |
| 104 — GUI export masking | `proposed/` | Never design-reviewed |
| 105 — visual verification scope | `proposed/` | Never design-reviewed — noted repeatedly and still true |
| 106 — keyboard operability | `proposed/` | Never design-reviewed |

**RFC 100 moves to `accepted/` the moment the owner approves it**, which is a
one-line follow-up rather than part of this RFC.

## 5. `DEC-` and `RISK-` are not RFCs — and I was making them behave like some

**They are not RFCs**, and should not become them. But the concern behind the
question is right, and I caused it.

### 5.1 What went wrong in my draft

The register update I drafted on 2026-08-19 minted **DEC-013** (rustfmt) and
**DEC-014** (`text_muted` asserted). Both **restate decisions that RFCs 107 and
108 already own**. That is a second source of truth for the same decision — the
exact failure the policy's new anti-pattern describes for handoffs, arriving by a
different door.

If DEC-013 and RFC 107 ever disagree, a reader cannot tell which governs. The
register should have **pointed**, not restated.

### 5.2 The boundary

| Kind | Home |
|---|---|
| A decision an RFC makes | **The RFC.** The register may carry a one-line pointer, never a restatement |
| A decision predating or outside the RFC system — DEC-001 workspace shape, DEC-004 reason-mandatory, DEC-007, DEC-008, DEC-012 | **The register.** These have no RFC and inventing one retroactively is make-work |
| A **risk** | **The register.** A risk is not a decision and has no lifecycle state; but where an RFC owns it, name that RFC |

**No new `DEC-` entry may be minted for something an RFC decides.** That rule is
the whole of §5.

### 5.3 Migration, later

Rewriting the register to pointers touches every entry and is not urgent. **Do
it after the current implementation wave** — RFC 108 then RFC 100 — so folder
moves and register rewrites are not in flight together. Recorded as a follow-up,
not scheduled here.

What **is** in scope now: drop DEC-013 and DEC-014 from the draft register
before it is adopted, replacing them with pointers to RFC 107 and RFC 108.

## 6. Timing

**Now, before the dev team starts RFC 108.** Three reasons:

- the migration is a file move plus a link sweep, and it gets more expensive
  with every RFC added;
- **handoffs do not move** — the policy is explicit — so RFC 108's handoff path
  is unaffected and the dev team is not disturbed mid-task;
- nothing is currently mid-implementation, which is the same window RFC 107 used
  and the condition self-application recommends.

## 7. Acceptance contract

1. `rfcs/done/000-rfc-lifecycle-policy.md` is byte-identical to
   `.git-exclude/rules/000-rfc-lifecycle-policy.md`, except for a Status-field
   note recording the sync.
2. `rfcs/accepted/` exists and contains **RFC 108**.
3. Every other `proposed/` RFC is unmoved.
4. `rfcs/README.md` gains an **Accepted** table between Proposed and Done.
5. **No `rfcs/handoffs/` directory moves**, per the policy.
6. Every cross-reference to a moved RFC is updated —
   `grep -rn "proposed/108" --include="*.md"` returns nothing.
7. The draft decision log drops DEC-013 and DEC-014 in favour of pointers.
8. One atomic commit, per self-application.

## 8. Risks

| Risk | Mitigation |
|---|---|
| Link rot from the move | Acceptance item 6 is a grep, not a promise. Only one RFC moves, so the sweep is small |
| `accepted/` sits empty and becomes clutter | It has one occupant on day one and a defined transition. If it is still holding one item in six months, that is evidence the two events do collapse here and the variant should be reverted |
| The register migration is forgotten | §5.3 records it; the boundary rule in §5.2 prevents the problem worsening meanwhile |
| Amending a `done/` RFC | Permitted — RFC 000 forbids deleting and renumbering, not amending. The sync makes the tracked copy *match* its own current authority |

## 9. Sources

- `.git-exclude/rules/000-rfc-lifecycle-policy.md` — the current policy,
  including "Folder layout: 5-folder variant", "Companion handoffs", and
  "Self-application"
- `rfcs/done/000-rfc-lifecycle-policy.md` — the stale tracked copy
- `.git-exclude/specs/decision-log/decision-log.md` — the draft register
- Owner instruction, 2026-08-19
