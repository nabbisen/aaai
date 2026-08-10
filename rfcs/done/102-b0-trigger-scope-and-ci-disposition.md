# RFC 102 — B0 Trigger Scope and Legacy CI Disposition

**Status.** Implemented — **released in `v0.41.0`** (2026-08-10, release
unit 1). Landed 2026-07-31 in
`3c514715e0d37be43d1ff769b1189df861f07791`; verified by V1
(`30510808044`, docs-only) and V2 (`30551474446`, code). Acceptance items 4, 5,
and 6 are discharged by construction rather than by hosted execution — see §11.
Ships in release unit 1. Two follow-ups are logged in §11.4 and belong to C2,
not to this RFC.

**Tracks.** Operating cost of the B0 gate; interim disposition of `ci.yaml` pending C2

**Depends.** RFC 097 (B0 authority and serialized-integration policy), which this
RFC narrows without weakening

**Design owner.** high-capability model

**Decision owner.** nabbisen, project owner

**Proposed implementer.** mid-capability model, after design review and owner approval

**Evidence location.** `.git-exclude/evidence/102-b0-trigger-scope/`

**Touches.** `.github/workflows/b0.yaml` and `.github/workflows/ci.yaml` triggers
only. No product code, no test, no persisted format, no dependency, no new action
vendor.

**Handoff.** Required:
[`rfcs/handoffs/102-b0-trigger-scope/README.md`](../handoffs/102-b0-trigger-scope/README.md)

## 1. Summary

B0 runs its full three-OS matrix on **every** push to `main`, including pushes
that cannot change what it tests. `ci.yaml` also runs on every push and has
failed on every push, by design, because its repair is deferred to C0/C2/C1.

This RFC scopes B0 to the changes that can affect its result, and parks the
legacy workflow until C2 repairs it — **without weakening RFC 097's invariant
that every `main` SHA carries a successful `B0 / gate` conclusion.**

## 2. Observed cost

Measured from run pairs on `0df132366667cfe5b5fa891b8abdd952c09d5422`, a
**documentation-only** commit, using GitHub's runner multipliers (Linux 1×,
Windows 2×, macOS 10×):

| Workflow | Jobs | Billable-equivalent | Conclusion |
|---|---:|---:|---|
| B0 Hosted Bootstrap (`30460833149`) | 4 | 16 min | success |
| CI (`30460838652`) | 9 | 29 min | **failure** |

**45 billable-equivalent minutes to verify a Markdown change.**

> **Corrected 2026-07-31.** This table originally read ~15 min (B0) + ~12 min
> (CI) ≈ 27. The B0 figure was close; the CI figure was understated by more
> than a factor of two, consistent with a 13-second macOS job having been
> counted near its wall-clock value rather than its billed value — GitHub bills
> each job rounded **up** to the next whole minute *before* applying the runner
> multiplier, so that job alone bills 1 min × 10 = 10. Re-measured per job in
> `.git-exclude/evidence/102-b0-trigger-scope/cost-comparison.md`. The error was
> in this RFC's favour: the real saving is larger than the RFC claimed, so the
> decision was correct for a better reason than it stated.

`ci.yaml` has failed on five consecutive pushes — `9fdd1bc2`, `4ed8acfb`,
`ee6af7cd`, `aa0e3aa5`, `0df13236`. Each failure is already known and
documented: `cargo fmt --check` is red repository-wide under DEC-006, the MSRV
job names the nonexistent `aaai-core`, and the security job reports the open
advisories. No run has produced new information.

Two of the last four pushes to `main` changed only Markdown.

## 3. Problem

RFC 097 chose `on: push: branches: [main]` deliberately: under
serialized-integration continuity there is no branch protection, so detection is
post-push, and the invariant "the latest pushed `main` SHA is green" requires a
run per SHA.

That reasoning is sound but over-broad. **The invariant only needs to hold for
SHAs that can change the answer.** When a commit touches no input to
`cargo test --workspace --locked`, the previous green result still describes the
current code. Scoping to that set is not a weakening; it is the same guarantee,
correctly bounded.

A naive `paths-ignore` would break the invariant a different way: a skipped
workflow emits **no** `B0 / gate` status, and RFC 097 treats a latest `main` SHA
without a successful gate as a stop-work condition. Confirmed in the current
gate logic, which runs `test "${MATRIX_RESULT}" = "success"` and would see
`skipped`.

## 4. Goals and non-goals

### 4.1 Goals

- Skip the platform matrix when a push cannot affect its result.
- **Preserve literally** the rule that every `main` SHA carries a successful
  `B0 / gate` conclusion.
- Keep the skip decision fail-safe: an unrecognised path must cause B0 to run.
- Address hosted-image drift, the one legitimate reason to re-test unchanged
  code, at a cost proportional to the risk.
- Stop paying for a workflow that cannot pass until its repair is scheduled.

### 4.2 Non-goals

- Removing any platform cell, or reducing the matrix. Three-OS coverage is the
  whole point of B0.
- Repairing `ci.yaml`. That is C0/C2/C1 work and this RFC does not pre-empt it.
- Changing the B0 command, toolchain, or acceptance contract.
- Adding an action vendor. RFC 097 §5.2 defers pin policy to WS-07/C0, so the
  change detection is implemented with `git`, not a marketplace action.

## 5. Selected design

### 5.1 Denylist, not allowlist — the direction matters

Change detection uses an **ignore list**, not a match list.

With an allowlist (`paths: ['crates/**', 'Cargo.*']`), a newly introduced build
input at an unlisted location — a root `build.rs`, a new crate directory, a
generated fixture — would silently **skip** B0. That fails open.

With a denylist, anything not explicitly recognised as inert causes B0 to
**run**. That fails safe, which is the correct bias for a gate whose absence is
a stop-work condition.

**Inert paths** (skip permitted when a push touches *only* these):

```
**/*.md
rfcs/**
docs/**
ROADMAP.md  README.md  CHANGELOG.md  LICENSE  NOTICE  TERMS_OF_USE.md
packaging/**
.github/workflows/ci.yaml
.github/workflows/release.yaml
.github/workflows/windows-msix.yaml
```

Everything else — notably `crates/**`, `Cargo.toml`, `Cargo.lock`, and
`.github/workflows/b0.yaml` itself — forces a run.
`crates/aaai-gui/locales/*.yaml` feed a compile-time macro and are build inputs
despite being data.

**`crates/**` is checked first and is categorically non-ignorable, ahead of the
ignore list.** An earlier draft of this section stated that requirement in prose
but left it unenforced against the `**/*.md` term, which would have classified
`crates/aaai/README.md` as inert. That is harmless while no markdown feeds the
build — verified: no `include_str!` or `include_bytes!` exists in `crates/`, and
`readme =` keys are packaging metadata only — but
`#![doc = include_str!("../README.md")]` is a routine idiom, and adding it would
have made a README edit change the doctest surface while B0 silently skipped.

A path-classification rule of this kind must be enforced by a precedence check,
not by prose. See
`.git-exclude/reviewed/048-rfc102-b0-trigger-scope-implementation-review-2026-07-29.md`
§3.

### 5.2 Keep the gate always reporting

The platform matrix becomes conditional. The gate does not.

A `changes` job (Linux, seconds) computes whether the push touched anything
outside the inert set and exposes it as an output. `b0-platform` runs only when
it did. `b0-gate` keeps `if: always()` and succeeds when **either**:

- the platform matrix result is `success`; or
- **all three** of: `needs.changes.result == 'success'`, the changes output is
  `false`, and the platform matrix result is `skipped`.

The first condition of that triple is load-bearing and was missing from an
earlier draft of this section. If the `changes` job itself **fails**, its output
is empty — which is not `'true'`, so `b0-platform` would skip, and a gate that
tested only "output is not true" would then report **success on an untested
SHA**. Requiring `changes` to have succeeded closes that path.

The gate must continue to fail for `failure`, `cancelled`, and for `skipped`
arising from any other cause. A matrix skipped because a dependency failed must
never be read as a pass.

Every `main` SHA therefore still carries a `B0 / gate` conclusion, and RFC 097's
invariant holds without amendment.

### 5.3 Scheduled run for image drift

RFC 097 §11 records that hosted `-latest` images change and that B0 is evidence
for the run, not a permanent claim about the image. That is the one real
argument for re-testing unchanged code.

It is an argument about **elapsed time, not commits**: a green run five minutes
ago says nothing about the image next week, and running on every docs commit
does not address it either. Add a weekly `schedule:` trigger, which targets the
risk directly at roughly one run per week instead of one per push.

### 5.4 Park `ci.yaml`

Reduce `ci.yaml` to `workflow_dispatch` only, with a header comment recording
why and what restores it.

This is not repair and not deletion. Every job remains intact and runnable on
demand; C2 restores the automatic triggers as part of its blocking-quality
policy. RFC 097 §5.4 instructed that `ci.yaml` not be edited *during the B0
implementation*, to avoid conflating B0 with legacy repair. That constraint has
served its purpose; changing its trigger now is a separate, deliberate decision
with its own review.

## 6. Acceptance contract

1. A push touching only inert paths produces: `changes` success,
   `b0-platform` skipped, **`B0 / gate` success**.
2. A push touching `crates/**` produces a full three-OS matrix and a gate result
   reflecting it.
3. A push touching both runs the full matrix.
4. A genuine platform failure still fails `B0 / gate`.
5. A matrix skipped for any reason other than "no relevant change" fails the
   gate.
6. `workflow_dispatch` always runs the full matrix.
7. `ci.yaml` no longer triggers on push or pull request and remains dispatchable.
8. No change to the B0 command, matrix, toolchain, permissions, or timeouts.

## 7. Risks and mitigations

| Risk | Mitigation |
|---|---|
| The ignore list wrongly classifies a build input as inert | Denylist fails safe; `crates/**` and `Cargo.*` are never ignorable; the list is short and reviewed |
| `github.event.before` is unusable — branch creation, force-push, or not an ancestor | Treat any unresolvable diff as "changed" and run the full matrix |
| A skipped matrix is misread as a pass | The gate distinguishes skip-with-no-changes from every other skip; acceptance item 5 tests it |
| Hosted image drifts under an inherited green result | Weekly scheduled run (§5.3) |
| Parking `ci.yaml` hides regressions it would have caught | It has caught nothing — it fails at the first step on every run. Its jobs remain dispatchable, and C2 owns restoration |
| Cost saving is assumed rather than measured | Evidence records billable-equivalent minutes before and after on a docs-only push |

## 8. Alternatives considered

| Option | Decision |
|---|---|
| Conditional matrix + always-reporting gate | **Selected.** Preserves RFC 097's invariant literally rather than amending it |
| `paths-ignore` on the workflow | Rejected: emits no `B0 / gate` status, so a docs SHA reads as stop-work |
| `paths` allowlist | Rejected: fails open on any unlisted new build input |
| Amend RFC 097 to let docs SHAs inherit the prior result | Rejected: weakens a safety invariant to save cost, when §5.2 achieves the saving without touching it |
| Drop macOS from the matrix (10× multiplier) | Rejected: the largest single saving and the largest loss of coverage. B0 exists for three-OS evidence |
| Delete `ci.yaml` | Rejected: C2 needs its jobs; parking is reversible, deletion is not |
| Batch pushes and change nothing | Insufficient alone, but adopt it as well — it needs no code and is available immediately |

## 9. Review questions

1. Is the §5.1 inert-path list correct and complete for this repository?
2. Is the gate's skip-discrimination in §5.2 tight enough that no failure mode
   reads as a pass?
3. Is weekly the right cadence for image-drift detection?
4. Should `ci.yaml` be parked, or repaired now rather than at C2?
5. Should the `pull_request` trigger be retained, given that
   serialized-integration means no PRs occur in practice?

## 10. Sources

- `.github/workflows/b0.yaml`, `.github/workflows/ci.yaml`
- RFC 097 §5.1, §5.4, §11, and the serialized-integration policy in `ROADMAP.md`
- Runs `30460833149` and `30460838652`, and the five consecutive CI failures
  listed in §2

## 11. Implementation record

Added at the 2026-07-31 disposition checkpoint
(`.git-exclude/reviewed/055-rfc102-rfc103-disposition-checkpoint-2026-07-31.md`).

Implementation shipped in `3c514715e0d37be43d1ff769b1189df861f07791`, reviewed
in `.git-exclude/reviewed/048-...md` and re-reviewed after the `crates/` guard
correction in `.git-exclude/reviewed/049-...md`.

### 11.1 Acceptance contract disposition

| § 6 item | State | Evidence |
|---|---|---|
| 1. Inert-only push → skip, gate success | **Verified hosted** | V1, run `30510808044` |
| 2. `crates/**` push → full matrix, gate reflects it | **Verified hosted** | V2, run `30551474446` |
| 3. Mixed push runs the full matrix | **Verified hosted** | V2's push range `a48d4d39…4d4996a7` contained `.gitignore`, `rfcs/**`, and `crates/**` |
| 4. A genuine platform failure still fails the gate | **By construction** | §11.2 |
| 5. A matrix skipped for any other reason fails the gate | **By construction** | §11.2 |
| 6. `workflow_dispatch` always runs the full matrix | **By construction** | §11.2 |
| 7. `ci.yaml` no longer auto-triggers, remains dispatchable | **Verified hosted (trigger half)** | Three pushes since `c3ca2723…` produced no CI run; `on: workflow_dispatch:` is the workflow's only trigger |
| 8. No change to B0 command, matrix, toolchain, permissions, timeouts | **Verified** | `workflow-diff.md` |

### 11.2 What "by construction" means here, and its limit

Items 4, 5, and 6 were not exercised on hosted runners. Each would require
deliberately pushing a broken or unusual state to `main`, which
serialized-integration forbids and which would cost more than the property is
worth re-proving.

They rest instead on reading `.github/workflows/b0.yaml`:

- **Item 4.** A platform failure yields `MATRIX_RESULT=failure`. The first gate
  term requires `success`; the second requires `skipped`. Neither matches, so
  control reaches `exit 1`.
- **Item 5.** A matrix skipped for any reason other than no-relevant-change
  implies either `CHANGES_RESULT != success` or `CHANGES_CODE != false`. The
  second gate term is a three-way conjunction over exactly those, so it fails
  and control reaches `exit 1`. This is the D1 correction from
  `.git-exclude/reviewed/047-...md`; without its third term the gate would have
  reported success on an untested SHA.
- **Item 6.** The `detect` step's first branch is
  `if [ "$EVENT_NAME" != "push" ]; then fail_safe`, so every non-push event
  emits `code=true` before any diff is attempted.

**State the limit plainly:** this is a reading of a shell script, not an
observation of the system. The gate's failing paths have never executed on a
hosted runner. That is an accepted residual, not a discharged obligation. The
weekly scheduled run (§5.3) exercises item 6's path incidentally every Monday,
so item 6 will convert to hosted-verified without further action — items 4 and
5 will not.

### 11.3 Cost outcome

Measured in `.git-exclude/evidence/102-b0-trigger-scope/cost-comparison.md`:
45 → **2** billable-equivalent minutes on a documentation-only push, and
45 → **27** on a code push. See the §2 correction for why the "before" figure
differs from this RFC's original text.

### 11.4 Follow-ups, owned by C2 and not by this RFC

1. **`ci.yaml` remains parked.** Its automatic triggers are removed, not
   repaired. The three documented causes — repository-wide `cargo fmt --check`
   debt under DEC-006, the MSRV job naming the nonexistent `aaai-core`, and open
   advisories in the security job — are all still present. C2 owns restoration.
2. **macOS is 20 of the 27 minutes** on a code push, the single dominant cost.
   §8 rejected dropping it, correctly, since B0 exists for three-OS evidence.
   Recorded so that any future cost discussion starts from the right lever
   rather than re-deriving it.
