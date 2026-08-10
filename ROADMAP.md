# aaai ROADMAP

## Current program — v1 trustworthiness remediation

**Planning baseline:** 2026-07-17 architecture preparation review at `ec561c5`;
revised in response to the roadmap architecture review at
`.git-exclude/reviewed/002-v1-remediation-roadmap-architecture-review-2026-07-17.md`

**Planning status:** Roadmap accepted with non-blocking notes. M0/D0 and M1/S1
completed on 2026-07-17. RFC 095 establishes the owner-approved v1 authority,
and RFC 096 establishes isolated test/user-state boundaries. RFC 097 implements
the reviewed hosted Linux/macOS/Windows B0 workflow and serialized-integration
governance. M1C/B0 and WS-03 are complete when the exact `main` SHA containing
this lifecycle transition passes B0; until then its completion claims are not
operative and unrelated continuation remains blocked. Store/MSIX remains
deferred from v1. No release activity is authorized.

**Pacing model:** milestone **order and dependencies** are authoritative;
**calendar windows are not.** Dates were removed on 2026-07-29 after two
milestones completed weeks before their scheduled windows opened — M1C on
2026-07-21 against an Aug 17–28 window, and M2's symlink contract on 2026-07-29
against Aug 31 – Sep 11. The original schedule was expressed in person-weeks,
which does not describe this project's actual constraints.

**Release posture:** feature freeze; no v1.0 readiness, Store-submission, or
release-candidate claim until Milestone 7 passes.

**The freeze does not suspend v0 development releases.** It prohibits v1.0
readiness claims, Store submission, and release-candidate claims — not cutting
`0.4x` releases. `.git-exclude/rules/project-instructions-rust-gui.md` requires
regular releases at logical breaking points, delivered as tarballs, and that
requirement remains in force throughout this programme. Between 2026-06-24 and
2026-07-29 it was not met: four RFCs resolved and 25 commits landed with no
release, because the freeze was misread as a release freeze. Recorded so the
two documents are not read as conflicting again. See
`.git-exclude/reviewed/050-release-cycle-audit-and-proposal-2026-07-29.md`.

This program is the authoritative schedule for current work. The phase history
below remains the record of shipped pre-v1 development, but its older v1 dates,
scope statements, and readiness claims do not control this program.

### Release units

Releases are ordered and thematic. **Unit order is authoritative; version
numbers are assigned when a unit is cut**, for the same reason milestone order
is authoritative and dates are not — scope may merge or split, and a number
promised in advance becomes a commitment the evidence has not earned.

A unit is cut when its milestones reach their exit gates and `main` is green.
Consecutive small units may batch; a unit is never split across releases.

**Shipped:**

| Release | Unit | Date | SHA |
|---|---|---|---|
| [`v0.41.0`](https://github.com/nabbisen/aaai/releases/tag/v0.41.0) | 1 — Safety foundations | 2026-08-10 | `354fb2a2ef64a2f6c54dae4957cf03f039c98165` |

> **A unit ships whatever `main` carries, not only its own themes.** RFC 099
> T1–T5 belonged to unit 2 but was already merged when unit 1 was cut, so it
> shipped in `v0.41.0` — and its T8 defect became a release blocker for a unit
> that did not nominally contain it. Plan the next cut on what is *in `main`*,
> not on the unit's own contents.

| # | Unit theme | Contains | Entry gate |
|---:|---|---|---|
| 1 | **Safety foundations** — **SHIPPED as `v0.41.0`, 2026-08-10** | RFC 095 (v1 authority), 096 (test-state isolation), 097 (hosted CI), 098 (selected-folder and symlink policy), 102 (B0 trigger scope), 103 (safe output surfaces), **plus RFC 099 T1–T8** — `main` already carried the visual work, so it shipped with the unit rather than waiting for unit 2 | Gate met 2026-07-31 (S1, B0, S2's output half). Held by owner decision for RFC 099 T8, which fixed a diff control unreachable at 800 × 550 (RFC 099 §6a) rather than shipping it as a known limitation. Cut at `354fb2a2ef64a2f6c54dae4957cf03f039c98165` |
| 2 | **Readable GUI** | RFC 100 (module boundaries). *RFC 099 was this unit's other half and shipped early in unit 1 — see the note above.* | V2 |
| 2a | **Close the output boundary** | RFC 104 (GUI report export masking) — the last surface where NF-14 does not hold, and the last thing S2 waits on; **RFC 106** (keyboard operability) rides with it, both landing after RFC 100 | S2 |
| 3 | **Durable saves** | M3 — atomic definition persistence | P1 |
| 4 | **Supply chain and code quality** | M4A (advisory disposition), M4B (format, lint, maintainability) | C0, C2 |
| 5 | **Scale** | M5 — staged and bounded diff engine; M4C release automation folds in here | E1, C1 |
| 6 | **Guided experience** | RFC 101 — guided review flow | U1 |
| 7 | **Documentation truth** | M6 — product and documentation convergence | D1 |
| — | *Release-candidate decision* | M7 assembles prior evidence; the owner alone decides whether and when v1.0 follows | R1 |

Unit 1 is unusually large because it accumulated during the missed-cadence
window; everything in it is already merged, so it cannot be retroactively
split. Units 2 onward are sized to one nameable outcome each.

**Not in any unit, deliberately:** RFC 105 (visual verification scope) is a
process RFC — it changes how evidence is produced and where it lives, ships no
user-visible behaviour, and rides whichever release it happens to land in.
RFC 098 shipped in `v0.41.0` with unit 1 but is still in `rfcs/proposed/`
pending a disposition audit of its §9.1 adversarial matrix.

**Per-release requirements** (org §6.6): objective; included and excluded
scope; deliverables; quality criteria; major risks; completion criteria. Plus
the tarball `project-instructions-rust-gui.md` requires — files at archive
root, version-suffixed name, per DEC-008.

**Entry criteria.** `main` green on hosted B0; the `CHANGELOG.md` section
written; no open High security finding without recorded owner acceptance.

**Exit criteria.** Tag pushed **as `RELEASING.md` specifies** — `v`-prefixed —
and `release.yaml` observed to actually run; tarball produced and its layout
verified by hand; `[Unreleased]` cleared. Every release states explicitly that
v1 readiness remains gated on M7, so cadence never implies readiness.

**Unit 1 additionally exercises the release pipeline for the first time.**
`release.yaml` has never run — 53 historical tags omit the `v` prefix its
trigger and `RELEASING.md:69` both require — so its build matrix, archive
production, and upload steps are unverified. Treat that release as their first
test, not as a routine cut.

### Scheduling assumptions

- Work is serial: one milestone at a time, in the dependency order below.
  Design, review, implementation, and evidence remain distinct steps.
- **Developer capacity is not the limiting resource.** Elapsed time is governed
  by owner decisions, owner-embodied work that no other role can perform
  (real-display verification), hosted CI round-trips serialized under RFC 097,
  and the number of correction cycles a milestone needs. Estimating in
  person-days misdescribes all four.
- Parallel work may be introduced only after independent owners are named for
  the overlapping milestones, review capacity is confirmed, and changed-file
  ownership does not conflict.
- All updates to `main` are serialized through one integration authority. A
  failed or missing B0 result on the latest pushed `main` SHA stops the next
  unrelated integration and downstream continuation. Required-check
  enforcement must be activated before another person, bot, merge queue, or
  mechanism gains independent update authority.
- Each RFC is approved before its implementation begins. A milestone's
  position in the order is not authorization to implement an unapproved RFC.
- Security, data-safety, and cross-platform correctness take priority over UI
  simplification and new features.
- External dependency remediation and hosted CI availability may move dates.
  A missed gate moves dependent milestones; it does not reduce acceptance
  criteria.
- v1.0.0 tagging, publishing, Store submission, signing, commits, and pushes
  continue to require explicit maintainer authorization.

### Program outcomes

The program is complete only when all of the following are true:

1. One owner-approved v1 scope and GUI baseline governs requirements, docs,
   compatibility promises, and testing.
2. Tests cannot read, print, prune, or otherwise mutate an operator's real
   aaai configuration or history.
3. Selected-folder containment, report encoding, spreadsheet safety, and
   secret masking are specified and enforced for untrusted inputs.
4. Definition saving and locking are safe under Windows overwrite,
   concurrent-writer, collision, and recovery cases.
5. CI, formatting, lint, dependency audit, release automation, and release
   documentation can produce a valid green signal for the current crate names.
6. The large-tree requirement is backed by a staged implementation and measured
   acceptance budgets, or is explicitly narrowed before v1.
7. Public documentation matches executable behavior and actual distribution
   availability.
8. Cross-platform, performance, GUI, accessibility, visual, and approved direct
   packaging/distribution evidence is complete enough for a separate release
   review.

### Serial milestone schedule

> **On the "Implementation entry dependency" column, reviewed 2026-08-10.**
> This table was written for one implementer working strictly in order, so
> "X comes after Y" and "X depends on Y" were recorded identically. They are
> not the same thing, and once work moves out of order the difference matters:
> a sequencing note read as a dependency produces a false constraint.
>
> Three cells were corrected on owner approval:
>
> - **M3** — dropped its dependency on RFC 104's GUI export masking, which has
>   no bearing on atomic file replacement (F1);
> - **MG1** — dropped the dependency-audit gate, which typography never needed,
>   and which had in fact been unmet all the way through release (F3);
> - **M4B** — dropped the same dependency-audit gate; a formatting policy and a
>   `cargo audit` result are unrelated in both directions (F10).
>
> **M5**'s "P1 and C1 complete" is flagged but unchanged, pending decision. See
> `.git-exclude/reviewed/064-roadmap-consistency-audit-2026-08-10.md`.
>
> The remaining entries were reviewed and are substantive. When adding a
> milestone, state which kind each dependency is: *needs* or *follows*.

**Gate codes in one line each.** The entry-dependency column below refers to
gates by code; full evidence contracts are in *Gate evidence contracts* further
down.

| Code | Means |
|---|---|
| **D0** | v1 scope and authority decided |
| **S1** | tests never touch the operator's real config |
| **B0** | the three-OS hosted matrix is green |
| **S2** | untrusted input and output are safe — symlinks, encoding, masking |
| **P1** | saving files is crash- and race-safe |
| **C0** | dependency security audit is clean, or exceptions are formally accepted |
| **C2** | formatting and lint policy is settled and passing |
| **C1** | the whole CI, security, package, and release workflow is green |
| **V1** | GUI draws from design tokens; contrast verified |
| **V2** | GUI files split below the size limit, behaviour unchanged |
| **E1** | the diff engine meets its performance budget |
| **U1** | the guided GUI workflow is accepted |
| **D1** | documentation matches actual behaviour |
| **R1** | independent review finds nothing blocking |

| Milestone | Paced by | Outcome | Implementation entry dependency | Exit gate |
|---|---|---|---|---|
| **M0 — Program charter and v1 authority** | Complete | Owner-approved source precedence, v1 scope, feature and distribution disposition, and guided-vs-3-pane GUI decision | Architecture review accepted as planning input | **D0 passed:** RFC 095 establishes the normative baseline, feature scope, guided/expert GUI, compatibility boundary, direct distribution set, and Store/MSIX deferral |
| **M1 — Safe tests and local-state boundary** | Complete | Every CLI subprocess test uses isolated home/config directories with deny/canary regression protection | D0 complete; approved RFC 096 | **S1 passed:** isolated local full-suite pass plus synthetic no-resolution/non-disclosure/non-mutation evidence |
| **M1C — Safe hosted-CI bootstrap** | Complete | Correct crate names, isolated tests, MSRV build, and usable Linux/macOS/Windows test matrix, without waiting for later format/release cleanup | S1 complete; implemented RFC 097 | **B0 passed when the containing lifecycle SHA is green:** safe hosted platform matrix blocks serialized continuation or independently authorized integration under the applicable mode |
| **M2 — Untrusted input/output boundary** | **Partially complete** — WS-04 done; WS-05 paced by dev cycles + CI, entered on owner decision | Symlink classification and containment followed by contextual encoding, spreadsheet neutralization, and consistent masking across approved formats/surfaces | D0, S1, and B0 complete; output contract depends on symlink reporting contract | **S2:** threat model and adversarial platform-matrix tests pass |
| **M3 — Cross-platform persistence** | Dev cycles + CI; cross-platform persistence needs the three-OS matrix per attempt | Windows-safe atomic replacement, atomic lock acquisition, recovery, collision, and concurrency behavior | B0 complete; **S2's symlink and engine/CLI output halves** complete; approved persistence RFC — *narrowed 2026-08-10, owner-approved (audit F1): the former "S2 complete" would park persistence behind RFC 104's GUI export masking, which has no bearing on atomic file replacement. WS-06's own row records this dependency as "for serial sequencing"* | **P1:** Linux/macOS/Windows persistence matrix passes |
| **M4A — Supply-chain disposition** | Owner decision per advisory disposition; dev cycles for upgrades | Vulnerabilities and warnings updated, mitigated, or accepted through an owner-approved exception with expiry | B0 complete; dependency feasibility checked | **C0:** dependency audit has no unapproved finding |
| **MG1 — GUI visual foundation** | **Owner display** — T1–T5 complete; T6/T7 are the only remaining work and no other role can perform them | All GUI text, spacing, and colour derive from the existing `snora` tokens; line heights applied; NF-4 contrast verified by automated test | Approved RFC 099 — *corrected 2026-08-10, owner-approved (audit F3): this read "C0 complete; approved RFC 099". C0 is supply-chain disposition and has no bearing on typography or colour tokens. MG1 was designed, implemented, reviewed eight times, and released in `v0.41.0` with C0 unmet, and no review caught it — the dependency was wrong, not skipped* | **V1:** token adoption complete and the contrast gate is green on all four themes |
| **MG2 — GUI module boundaries** | Dev cycles + CI; mechanical, behaviour-neutral | `app.rs` split below the ELOC threshold; `views/mod.rs` removed per DEC-003; behaviour-neutral | V1 complete; approved RFC 100 | **V2:** boundaries corrected with every test count unchanged |
| **M4B — Format, lint, and maintainability policy** | Owner decision on formatting policy, then dev cycles + CI | One formatting policy, clean Clippy, module/file-boundary rules for touched oversized files | Owner decision on the formatting policy — *narrowed 2026-08-10, owner-approved (audit F10): this read "C0 complete". A formatting and lint policy does not depend on the dependency security audit, in either direction. **M4B may begin now**, and doing so would clear the repository-wide `cargo fmt --check` failure that has been noise in every implementation report* | **C2:** format and lint gates pass under the approved policy |
| **M4C — Release automation and operations** | Owner — release credentials, package/publish dry runs, distribution decisions | Version tooling, crate identity, package/publish workflow, release documentation, and approved direct-distribution automation repaired | C0 and C2 complete | **C1:** complete hosted CI, security, package, and release workflow is green |
| **M5 — Scalable diff architecture** | **Measurement-bound** — fixtures and benchmark hardware; does not compress with agent throughput | Staged bounded scan and lazy retention, or an owner-approved narrowed normative scale requirement with replacement budgets | D0, S2, P1, and C1 complete | **E1:** approved full-scale or replacement-scale budgets pass |
| **M5G — Guided GUI convergence** | Dev cycles + **owner display** for visual, keyboard, and guided/expert acceptance; the largest remaining milestone | Implement and test the simplified beginner path on the MG1 visual foundation while preserving the approved three-pane expert surface; the guided path must not reintroduce hardcoded sizes, spacing, or colours | V2, WS-05, and WS-10 complete; approved RFC 101 | **U1:** approved GUI, keyboard, and visual acceptance passes |
| **M6 — Product and documentation convergence** | Dev cycles + owner decision on the supported docs toolchain range | Test-backed compatibility acceptance plus README, mdBook, RFC lifecycle, roadmap history, and public distribution claims reconciled to D0 and E1 | M1–M5G complete | **D1:** supported docs toolchain builds and docs match executable behavior and normative scope |
| **M7 — Release-candidate evidence** | Owner + verification operator; assembly of prior evidence, not new development | Assemble—not redefine—cross-platform, package/publish, direct distribution, performance, real-display GUI/ABDD, and visual evidence | M1–M6 complete | **R1:** independent review has zero blocking findings |

No release is scheduled by this table. Passing M7 creates a review point at
which the maintainer may authorize a release candidate; it does not authorize a
tag, publish, Store submission, or v1.0.0 release by itself.

### Dependency and critical path

```text
Architecture review
  --> M0/D0
  --> M1/S1
  --> M1C/B0
  --> M2/S2 ....................... (output half done; GUI half waits on MG2 — see below)
  --> M4A/C0
  --> MG1/V1 ...................... shipped v0.41.0
  --> MG2/V2
  --> RFC 104 .................... closes S2's remaining GUI half
  --> M3/P1
  --> M4B/C2
  --> M4C/C1
  --> M5/E1
  --> M5G/U1
  --> M6/D1
  --> M7/R1
```

> **Corrected 2026-08-10.** MG1 and MG2 were absent from this graph even though
> MG2's gate V2 is an entry dependency of M5G. They are now shown.
>
> The order also changed, and not by choice. **RFC 104 tracks M2 / WS-05 / S2
> but is scheduled after RFC 100 (V2)** — its acceptance test must drive the GUI
> export path and read the written file, and the GUI's tests are non-hermetic
> until MG2 fixes them. So S2's remaining half now sits *after* MG2, and
> everything whose entry names S2 moved with it.
>
> **M3 is the case that matters.** Its entry reads "B0 and S2 complete", which
> would park atomic file persistence behind GUI work it has no bearing on.
> Whether to narrow that entry is an owner decision, recorded as F1 in
> `.git-exclude/reviewed/064-roadmap-consistency-audit-2026-08-10.md`. **Until
> it is decided, the order above is what the stated gates actually imply** —
> shown honestly rather than left as the older, no-longer-true sequence.

This is the one-primary-implementer critical path and contains no
implementation overlap. D0 selected the guided redesign, so M5G is part of the
required path. The path defines **order, not duration** — see the pacing model
above.

### Parallel-work policy

No parallel work is currently approved. Milestones may overlap only when the
roadmap records named independent implementers, maintainer review capacity,
security/QA ownership, and non-conflicting changed-file boundaries for every
overlap. Until then, the serial order above is the only order used for
commitments or risk reporting.

Note that parallelism is unlikely to be the useful lever here. Where a
milestone is paced by owner decisions, owner-embodied verification, or
measurement, adding implementer capacity does not shorten it. The levers that
do work are batching owner decisions, reducing correction cycles, and removing
avoidable hosted round-trips.

### B0 serialized-integration policy

B0 may operate in **serialized-integration continuity mode** while all updates
to `main` pass through one designated integration authority and no person, bot,
merge queue, or other mechanism is authorized to update `main` independently.
**Independent update authority** means project authorization to update `main`
without routing that update through the designated integration authority and
its current-SHA B0 wait. Technical GitHub capability alone is not represented
as independent project authority. Developer and branch count are not safety
properties: multiple developers, branches, and pull requests are permitted
when their updates are integrated one at a time by the designated authority.
Every updated `main` SHA triggers B0 and is observed before another unrelated
integration or downstream continuation.

In this mode, a latest `main` SHA without a successful `B0 / gate` is a
stop-work condition. Only diagnosis, correction, and evidence work may
continue. No unrelated product or RFC implementation may be pushed, no
downstream gate may consume that SHA, and no release, tag, or publish operation
may proceed. A deterministic failure is corrected at a new SHA; the failed run
is retained rather than laundered by rerunning unchanged code. This is
post-push detection, so `main` can temporarily be red; the owner accepts that
risk only under the conditions above.

Before another person, bot, merge queue, or mechanism gains independent
authority to update `main`, work stops for enforcement activation. The owner
must select the actual merge-control path, amend and re-review RFC 097 for
`merge_group` if a merge queue is selected, configure the exact `B0 / gate`
required check, record the resulting rule state, and verify it on that path
through independent review. Independently authorized integration resumes only
after that evidence is accepted.

At entry to each later workstream, the integration-control record confirms that
updates to `main` remain serialized and no independent updater exists. If not,
enforcement activation blocks entry.
This policy does not waive any B0 platform cell or failure, and does not alter
the later C0, C2, or C1 contracts.

The durable record is
`.git-exclude/evidence/097-safe-hosted-ci-bootstrap/continuity-mode.md`. It
contains a closure entry and a new entry at every later workstream boundary.
Each entry identifies the boundary and timestamp, the exact current `main` SHA
and B0 run, the designated integration authority, the current integration-
control and observed repository-rule state, and either continued serialized
eligibility or blocked entry pending enforcement. It does not record or
require developer count, branch count, or a sole-developer disclaimer.

### Gate evidence contracts

| Gate | Minimum auditable evidence |
|---|---|
| **D0 — passed 2026-07-17** | RFC 095 records owner-approved normative-source precedence, supported feature inventory and release disposition, guided/expert GUI baseline, compatibility ownership, direct v1 distribution set, Store/MSIX deferral, and feature freeze |
| **S1 — passed 2026-07-17 (local Linux)** | RFC 096 centralizes state resolution; all subprocess helpers inject isolated platform home/config variables; deny/canary roots cover fallback lookups; destructive history fixtures exceed the prune threshold; captured streams and exact synthetic snapshots prove no canary disclosure or mutation. Hosted platform and declared-MSRV evidence remains B0. |
| **B0 — conditional pass; operative only when the lifecycle-transition SHA is green** | Hosted Linux/macOS/Windows jobs use current crate names, the isolated test harness, and the declared MSRV. In serialized-integration continuity mode, the exact latest pushed `main` SHA is green, failure or missing evidence blocks the next unrelated integration and downstream continuation, and `continuity-mode.md` records the designated integration authority plus integration-control and observed enforcement state. Before any independent updater is authorized, the exact `B0 / gate` context is configured and verified as required on the actual merge-control path. The passing result for the exact lifecycle-transition SHA makes this gate completion operative. |
| **S2 — output half discharged 2026-07-31; symlink half open** | Approved threat model; symlink/path/error reporting contract; adversarial encoding, formula, masking, cross-root-link, and platform tests run through B0; and the behavioural Windows non-name-surrogate reparse fixture required by RFC 098 §9.1 — **discharged 2026-07-29**, run `30456899630`, `windows-2025-vs2026`. **Encoding, formula-neutralisation, and masking are discharged by RFC 103**, run `30551474446` — the adversarial field × surface guard executes on all three hosted platforms. **Still open:** the approved threat model (RFC 098 §14's ten confirmations, never recorded as made); the **cross-root-link cases, which have no owner** — RFC 098 §9.1 tests links to a third directory, not to the other compared root, and `.git-exclude/reviewed/045-...md` §4 reassigned this case class to WS-05, where RFC 103 did not pick it up; and GUI report-export masking, now owned by **RFC 104** |
| **P1** | Existing-target overwrite, atomic lock acquisition, concurrent writers, stale lock, temp collision, interrupted save, and recovery tests pass on Linux/macOS/Windows |
| **V1** | No `.size(N)` literal and no `Color::from_rgb` outside `design_tokens.rs`; every text role carries its token line height; an automated test proves >= 4.5:1 for normal text and >= 7:1 for both high-contrast presets across all four themes; GUI counts unchanged plus the contrast test; real-display screenshots at 800x550 and a working size |
| **V2** | No `mod.rs` and no inline test module under `crates/aaai-gui/src/`; every file <= 500 ELOC or listed with a rationale; every test name and count unchanged, proven by a `--list` diff |
| **C0** | `cargo audit` has no unapproved vulnerability/warning; every temporary exception records reachability, mitigation, owner, expiry, and removal condition |
| **C2** | Approved formatting policy is documented; format and Clippy commands pass; touched oversized files do not gain further concentration and stable boundaries are split where practical |
| **C1** | Complete hosted build/test/docs/security workflow, version-bump self-test, package/publish dry runs, and D0-approved distribution automation pass |
| **E1** | Either the approved 10k/large-file budgets pass, or an owner-approved normative scope amendment defines replacement fixtures and budgets that pass; D1 and R1 consume the chosen branch |
| **U1** | RFC-approved beginner/expert workflow, keyboard path, i18n, visual, and accessibility acceptance evidence passes on the required display/platform set; and the MG1 token foundation is intact — the guided path reintroduces no `.size(N)` literal or `Color::from_rgb` |
| **D1** | Supported mdBook/toolchain range is declared and green; public docs, executable help, GUI, RFC index, compatibility text, E1 scale branch, and D0 distribution claims agree |
| **R1** | Independent architecture/release review reports zero blocking findings; `Accept with notes` is permitted only when every note is explicitly non-blocking for the later release decision |

D0 excludes Store/MSIX from the v1 distribution set. Store artwork, MSIX
installation, terminal-alias validation, signing, certification, and submission
evidence are therefore deferred and public documentation must not claim
availability. Adding that channel requires an owner-approved RFC 095/D0
amendment and new ownership/evidence assignments.

### Planned workstream and RFC sequence

Workstream IDs are stable planning labels. RFC numbers are assigned
sequentially only when proposal files are created, so optional or later-revised
work does not reserve or leave an unexplained RFC-number gap. Detailed design
and approval begin only after this roadmap revision is accepted.

| Order | Workstream | Design responsibility | Contract dependency | Handoff requirement |
|---:|---|---|---|---|
| 1 | **WS-01 — v1 authority, scope, GUI, and distribution baseline** | **Completed by RFC 095/D0.** Normative source precedence; supported feature inventory and evidence ownership; guided/expert GUI; compatibility and direct distribution boundary | None | No developer handoff; owner decision is the implemented artifact |
| 2 | **WS-02 — test environment and user-state isolation** | **Completed by RFC 096/S1.** Per-test state sandbox; subprocess-helper migration; canary capture/snapshot contract; destructive regression evidence | WS-01 gate only for program sequencing, not technical design | No separate handoff; RFC 096 contains the complete migration table and checklist |
| 3 | **WS-03 — safe hosted-CI bootstrap** | **Completed by RFC 097/B0, conditional on the exact lifecycle-transition SHA passing B0.** Current crate names; isolated tests; MSRV; Linux/macOS/Windows bootstrap matrix that blocks serialized continuation or independently authorized integration under the applicable mode | WS-02 | Completed CI developer/operations handoff under `rfcs/handoffs/097-safe-hosted-ci-bootstrap/` |
| 4 | **WS-04 — selected-folder and symlink policy** | Link classification, containment, platform behavior, and shared path/error/reporting semantics | WS-01, WS-03 | **Required** platform developer/QA handoff |
| 5 | **WS-05 — safe reports, exports, and masking** | Contextual encoding, spreadsheet neutralization, and masking across D0-approved Markdown/HTML/JSON/SARIF/CSV/TSV and GUI/CLI surfaces | WS-04 reporting contract, WS-03 test matrix | **Required** security developer/adversarial-QA handoff |
| 6 | **WS-06 — atomic definition persistence** | Atomic replace, lock ownership, stale recovery, temp naming, crash and concurrent-writer behavior | WS-03, WS-05 gate for serial sequencing | **Required** cross-platform developer/QA handoff |
| 7 | **WS-07 — dependency and supply-chain disposition** | Advisory upgrade, reachability, mitigation, exception, expiry, and removal policy | WS-03 | **Required** security/release handoff |
| 8 | **WS-08 — formatting, lint, and touched-file maintainability** | Formatting decision; Clippy cleanup; module/file splitting criteria where remediation touches oversized files | WS-07 for serial sequencing | RFC-contained migration and verification checklist |
| 9 | **WS-09 — release automation and operational repair** | Crate identity; version self-test; hosted workflow completion; package/publish dry runs; D0-approved direct-distribution automation and release docs | WS-07, WS-08 | **Required** CI/release developer/operator handoff |
| 10 | **WS-10 — staged and bounded diff engine** | Metadata/content phases, lazy data, memory bounds, cancellation, progress, fixtures, benchmark budgets or normative narrowing | WS-01, WS-04, WS-09 | **Required** multi-step implementation/performance handoff |
| 11 | **WS-11 — guided v1 GUI convergence** | Implement the D0-selected guided path while preserving the approved three-pane expert surface | WS-05 and WS-10 | **Required** UI/UX developer and visual-QA handoff |
| 12 | **WS-12 — product and documentation convergence** | Produce and pass the RFC 095 compatibility acceptance matrix; reconcile public truth, mdBook range, RFC lifecycle/index, E1 branch, and D0 distribution claims | WS-01–WS-11 | Documentation and compatibility acceptance matrix may live in the RFC |
| 13 | **WS-13 — release-candidate evidence plan** | Assemble prior gates; define operators, environments, evidence locations, rerun/rollback commands, and independent review input without redefining acceptance | WS-01–WS-12 | **Required** release/QA operator handoff |

WS-01 is complete as RFC 095/D0, WS-02 is complete as RFC 096/S1, and WS-03 is
complete as RFC 097/B0 once the exact lifecycle-transition SHA is green. Later
numbers follow actual creation order.

WS-04 defines the shared external path/error/reporting contract before WS-05
is designed. Implementation may remain independently reviewable, but the two
workstreams cannot invent incompatible symlink outcomes.

### Evidence ownership and capacity gates

Named people and runner/environment access are not yet assumed. Before a
workstream enters its design/review window, its review request must record the
required names, capacity, environments, fixtures, and evidence location below.
An unassigned required role or unavailable environment blocks entry and moves
dependent dates.

| Evidence area | Accountable role required | Assignment point |
|---|---|---|
| Primary implementation and maintainer review capacity | Implementer and maintainer reviewer | Every workstream review request |
| Linux/macOS/Windows runners and MSRV | CI owner | WS-03 review |
| Threat model, symlink abuse cases, output injection, masking, advisory exceptions | Security reviewer and adversarial QA owner | WS-04, WS-05, and WS-07 reviews |
| Windows overwrite, locking, collision, crash, and recovery | Cross-platform persistence QA owner | WS-06 review |
| 10k/large-file fixtures, benchmark hardware, variance and replacement-budget policy | Performance owner and QA reviewer | WS-10 review |
| Supported mdBook/toolchain range and public-doc reconciliation | Documentation owner | WS-12 review |
| Real-display keyboard, ABDD, visual, and guided/expert-flow evidence | GUI/UX verification operator | WS-11 review |
| Package/publish dry runs and D0-approved direct distribution artifacts | Release operator | WS-09 and WS-13 reviews |

Required handoffs must be approved before implementation or operator evidence
begins. Each handoff contains the platform/fixture matrix, abuse or failure
cases, commands, expected artifacts, evidence paths, rollback steps, and
escalation conditions appropriate to its workstream.

### Milestone review rules

At each milestone boundary:

1. Implementation stops at a reviewable state.
2. The review request cites the approved RFC, changed surfaces, threat-model or
   compatibility impact, and observed commands.
3. Blocking findings stay in the same milestone until resolved or explicitly
   re-planned by the maintainer.
4. Approval of the review request creates a commit point; only then is a
   suggested commit message provided.
5. Handoffs are stored under `rfcs/handoffs/NNN-slug/` and inherit lifecycle
   state from their RFC. They never use a separate Draft/Proposed/Done status.
6. Any workstream touching a Rust file above 500 ELOC includes a boundary
   assessment. It must not increase concentration without explicit rationale,
   and splits stable logical/test-helper boundaries when that is safe within
   scope.

### Deferred until after the v1 release decision

- New end-user features unrelated to remediation
- Additional languages beyond English and Japanese
- Automated Store submission or signing infrastructure
- Screen-reader support that requires unavailable iced platform hooks
- UI expansion beyond the owner-approved WS-01 baseline
- Performance enhancements beyond the budgets approved for WS-10

---

## Historical pre-v1 phase record

The following sections are retained as development history. Completion marks
mean the listed work landed in its pre-v1 release; they are not current
release-readiness evidence.

## Phase 1 — Core ✅ (v0.1.0)
- Folder diff detection (Added / Removed / Modified / Unchanged / TypeChanged / Unreadable / Incomparable)
- Audit definition YAML (version 1)
- Content audit strategies: None, Checksum, LineMatch, Regex, Exact
- CLI: `aaai audit`, `aaai snap`, `aaai report`
- GUI: Opening screen, 3-pane main screen (file tree / diff viewer / inspector)
- Approval flow with mandatory reason
- Markdown and JSON report output

## Phase 2 — Quality & Completeness ✅ (v0.2.0)
- tests.rs — unit/integration tests separated per module (別紙 requirement)
- i18n — GUI multilingual: Japanese (primary) + English (別紙 requirement)
- Toast subscription — properly wired TTL management in GUI
- File tree filter — filter by status and diff type
- Batch approval — select multiple Pending entries and approve with shared reason
- Path pattern matching — glob-based audit rules (logs/*.log, build/**)
- CLI: --verbose / --quiet / --json-output

## Phase 3 — Integrations ✅ (v0.3.0)
- **Approver tracking** — approved_by / approved_at stamped on each AuditEntry at approval time
- **Expiry dates** — expires_at field; expired entries shown as warnings in CLI and GUI
- **Ticket linkage** — ticket field on AuditEntry (JIRA-123, INF-42, etc.) shown in reports
- **Ignore patterns** — .aaaiignore file (gitignore-style) to exclude paths from diff
- **Audit history** — ~/.aaai/history.jsonl records every audit run with summary counts
- **Rule templates** — built-in named templates for common audit patterns (version bump, port change…)
- **Audit profiles** — named before/after/definition combos saved to ~/.aaai/profiles.yaml
- **CLI: granular exit codes** — 0=PASSED, 1=FAILED, 2=PENDING, 3=ERROR, 4=CONFIG_ERROR
- **CLI: aaai check** — validate a definition file without running a diff
- **CLI: aaai history** — show recent audit runs from the history file
- **CLI: aaai snap --template** — apply a rule template when generating snapshots
- **GUI: inspector Phase 3 fields** — ticket, approved_by, expires_at display/edit
- **GUI: expiry warning badges** — visual indicator on expired / expiring-soon entries
- **GUI: rule template picker** — "Apply template" dropdown in inspector
- **GUI: profile manager** — Opening screen shows saved profiles for quick reload

## Phase 4 — Advanced ✅ (v0.4.0)
- **Binary diff summary** — is_binary flag + file size tracking; binary-specific diff viewer panel
- **Secret masking** — regex-based masking applied to CLI output, report generator, and diff viewer
- **Project-level config** — .aaai.yaml auto-discovery: default_definition, default_ignore, approver_name, mask_patterns
- **Parallel diff engine** — rayon-based parallel file reading for large folder performance
- **Diff statistics** — lines_added / lines_removed / lines_changed per DiffEntry, shown in CLI and GUI
- **Content size warnings** — warn when Exact / LineMatch strategies are applied to large files (>1 MB)
- **CLI: aaai config** — init / show .aaai.yaml project config
- **CLI: --mask-secrets** flag on audit and report commands
- **GUI: binary file info panel** — show type, size, before/after hash for binary diffs
- **GUI: diff statistics bar** — lines changed count shown in diff viewer header

## Phase 5 — Polish ✅ (v0.5.0)
- **Shell completion** — `aaai completions <shell>` for bash/zsh/fish/powershell via clap_complete
- **Watch mode** — `aaai watch` re-runs audit when before/after/definition files change (notify crate)
- **Progress tracking** — channel-based DiffProgress events; CLI progress bar via indicatif
- **HTML report** — third output format alongside Markdown and JSON
- **`aaai snap --dry-run`** — preview what would be generated without writing
- **`aaai dashboard`** — CLI summary dashboard with colour-coded stat cards
- **GUI: dashboard view** — summary cards as the default landing pane before selecting a file
- **GUI: file tree search** — incremental path filter input above the file tree
- **CLI integration tests** — end-to-end tests driving the real binary
- **Image diff (basic)** — detect image type for binary panel (PNG/JPEG/GIF/WebP) and show metadata

## Phase 6 — Production Readiness ✅ (v0.6.0)
- **Entry versioning** — created_at / updated_at auto-stamped on AuditEntry at approval time
- **`aaai diff`** — raw folder diff without an audit definition (plain change listing)
- **`aaai merge`** — merge two audit definition files with conflict detection
- **SARIF output** — SARIF v2.1.0 report format for GitHub/GitLab CI annotations
- **Report --include-diff** — optionally embed actual diff text in Markdown / HTML reports
- **GitHub Actions CI/CD** — .github/workflows/ci.yaml and release.yaml
- **GUI keyboard shortcuts** — Ctrl+S (save), Ctrl+R (re-run), Escape (deselect), / (search)
- **Undo last approval** — revert the most recently upserted entry
- **Complete documentation** — all docs/src/ chapters populated (strategies, CI integration, FAQ)
- **`aaai completions --install`** — auto-install completion script to shell config

## Phase 7 — v1.0 Quality ✅ (v0.7.0)
- **Non-blocking async diff** — tokio::spawn-based diff engine; GUI remains interactive during large scans
- **GUI theme support** — light / dark / system-follow theme selection stored in ~/.aaai/prefs.yaml
- **`aaai init` wizard** — interactive project setup: define before/after paths, create .aaai.yaml, generate starter definition
- **History analytics** — `aaai history --stats` showing trend graphs (pass rate, pending counts over time)
- **Large file warnings** — emit AuditWarning when Exact or LineMatch strategy applied to files >1 MB
- **`aaai export`** — export audit entries to CSV / TSV for external review in spreadsheets
- **Audit write locking** — `.aaai.lock` prevents concurrent definition file writes
- **Zero-warning build** — all cargo fix suggestions applied; Clippy clean at -D warnings level
- **`aaai snap --approver`** — pre-fill approver_name from project config or flag when generating entries
- **GUI: resizable panes** — drag handles between file tree / diff view / inspector

## Phase 8 — v1.0 comes closer ✅ (v0.8.0)
- **`aaai snap --approver`** — pre-fill approved_by from project config or --approver flag
- **Async GUI diff** — tokio-based non-blocking diff in aaai-gui; spinner during scan
- **`aaai version`** — detailed version info: crate version, build profile, Rust toolchain
- **`aaai lint`** — best-practice linter for definition files; emits categorized warnings
- **Property-based tests** — proptest fuzzing on masking engine, ignore rules, glob matching
- **Benchmarks** — criterion benchmarks for diff engine and masking engine
- **`aaai snap --suggest-glob`** — detect common path prefixes and suggest glob rules
- **README badges** — test count, version, license, CI status shields
- **v1.0.0 release prep** — full Apache-2.0 LICENSE text, AUTHORS, Cargo.lock finalised

## Phase 9 — Documentation & Test Completeness ✅ (v0.9.0)
- **Complete docs** — gui.md (full 3-pane walkthrough), cli.md (all 15 commands with examples), getting-started.md updated with aaai init
- **CLI test coverage** — integration tests for: completions, config --init, dashboard, init --non-interactive, lint --json-output, version --json-output
- **AuditWarning suppression** — suppress_warnings: list in .aaai.yaml; CLI --suppress-warnings flag on audit
- **History rotation** — --max-entries on history command; history store prune() method
- **report --no-history** — --no-history flag for report command (already on audit)
- **aaai audit --warn-only** — exit 0 even with warnings (do not fail on warning-only conditions)

## Phase 10 — GUI Polish ✅ (v0.10.0)
- **Resizable panes** — replace fixed widths with iced PaneGrid; user can drag dividers between file tree / diff view / inspector
- **Dark / Light theme** — toggle via footer picker; persisted to ~/.aaai/prefs.yaml
- **Directory collapse** — click directory entries in file tree to fold/unfold children
- **Unchanged files toggle** — show/hide Unchanged entries separately from the filter bar
- **Inspector: auto-focus reason field** — focus the reason text input when an entry is selected

## Phase 11 — GUI UI/UX Production Ready ✅ (v0.11.0 – v0.12.0)

### RFC 001 — CLI Output UX Consistency (v0.11.0)
- Result-first 4-zone layout; symbol-based status `✓ ⚠ ✗ ! —`; next-action hint

### RFC 004 — Opening Screen Input Validation (v0.11.0)
- `OpeningValidation`: inline real-time feedback; Start button disabled when paths invalid

### RFC 002 — Inspector Validation & Primary Action (v0.12.0)
- `InspectorValidation`: per-field Checksum/LineMatch/Regex/Exact checks; `ApproveAndSave` primary action

### RFC 003 — ABDD Status Display (v0.12.0)
- `status_badge()`: symbol + text per row; neutral diff badge; ABDD compliance

### RFC 005 — Keyboard Navigation & Focus (v0.12.0)
- `FocusTarget`; Tab/`/`/Enter/Ctrl+E/Escape shortcuts; complete keyboard flow


## Phase 12 — v0.19.0 の答え合わせ：視覚検証 / i18n / docs 刷新 + Phase 13〜16 統合 ✅ (v0.20.0)

**目的**: v0.15.0 〜 v0.19.0 で実装した UI/UX が「設計書通りに動くこと」を視覚的に保証する。

**実績**: 当初 Phase 12 は RFC 017〜019 のみを対象としていたが、開発の流れで Phase 13〜16 のスコープ（RFC 020〜025）も v0.20.0 で先行実装した。結果、Phase 12 は当初計画の 3 倍超のスコープで完了。

### RFC 017 — Visual Verification Harness & Protocol ✅
- 設計書 `aaai_uiux_design.pdf` の各画面要素に対してスクリーンショット + チェック項目のエビデンスを `verification/` に保存する手順を確立
- `scripts/list-unverified-rfcs.sh` で未検証 RFC を一覧化
- CI に「未検証 RFC 数」のステップを追加
- `docs/templates/visual-verification-operators-guide.md` に RFC 020〜023 まで含む包括チェックリストを整備

### RFC 018 — i18n Locale Fallback Strategies ✅ (partial — §3.4 only)
- §3.4 i18n キー監査スクリプト `scripts/check-i18n-keys.py` を新規実装
- 動的 `t!()` 呼び出しサイト（`make_btn` パターン）への false-positive 対策
- B/C 対策は RFC 016 視覚検証で問題が残った場合に限り着手（条件付き）

### RFC 019 — Documentation Refresh for v0.15–v0.19 Realities ✅
- `docs/src/gui.md` / `docs/ja/src/gui.md` を v0.20.0 の実装に合わせて全面刷新
- 加えて `cli-auditing.md` / `cli-reporting.md` / `cli-setup.md` / `cli-workflow.md` / `getting-started.md` が「英語パスに日本語が放置」状態だったため、6 ファイルを真の英語化
- 結果として `docs/src/*.md` 全 15 章が真の英語、`docs/ja/src/*.md` 全 15 章が真の日本語に到達

### RFC 020 — ABDD Audit & Action-oriented Errors ✅ (originally Phase 13)
- `docs/src/abdd-audit.md` / `docs/ja/src/abdd-audit.md` 新規（合格 / 例外 (記録) 付きのチェックシート）
- `UserError { message, hint }` 構造体導入、`error.<context>.<short_id>.{message,hint}` 形式の新 i18n キー
- Opening 画面のエラーバナー（赤いメッセージ + グレーのヒントの 2 行構成）
- Regex 入力時の regex101.com 誘導

### RFC 021 — Screen Navigation Continuity ✅ partial (originally Phase 14)
- Save / Report の完了マーク `✓ Saved Nm ago` + 30 秒 tick refresh
- `last_saved_at`, `last_reported_at`, `audit_dirty` を `App` に追加
- 新 i18n キー `banner.*`, `relative.*`
- **deferred**: `audit_dirty` バナー — sync rerun アーキ下では発火しないため、アーキ decoupling 後に再起 RFC で対応

### RFC 022 — Empty States & First-run Guidance ✅ (originally Phase 14)
- Opening プロファイル空のときの ① ② ③ オンボーディングパネル
- `audit_result` が None のときの file_tree / diff_panel / inspector それぞれの空状態
- 新しい i18n キー `empty_state.*`

### RFC 023 — Opening Drag-and-Drop & Recent Polish ✅ (originally Phase 15)
- iced 0.14 の `Event::Window::FileDropped` を用いた DnD 受け入れ
- `AuditProfile.last_used_at: Option<DateTime<Utc>>` (`#[serde(default)]`) で前方互換性確保
- `sort_by_recent` と `humanize_since` 相対時刻フォーマッタ（7 ユニットテスト付き）
- 新しい i18n キー `relative.*`, `error.opening.drop_invalid_kind.*`, `opening.drop_here`

### RFC 024 — CLI Dashboard & Help Discoverability ✅ (originally Phase 15)
- 全 16 サブコマンドの clap `after_help` に「Next steps」ブロックを追加
- `next_action_hint()` ヘルパを `audit.rs` と `dashboard.rs` で共有
- 新規 `aaai exit-codes` サブコマンド（5 値の終了コード表を印刷、v1.x 安定）
- aaai-cli tests 54 → 70 (+16)

### RFC 025 — v1.0.0 Release Preparation ✅ partial (originally Phase 16)
- `docs/src/compatibility.md` / `docs/ja/src/compatibility.md` 新規（v1.x 互換性契約）
- CLI 16 コマンド名 / 5 終了コード / 7 キーボードショートカット / 設定ファイル `#[serde(default)]` ポリシーを SemVer 解釈で明示
- 破壊的変更パイプライン（opt-in → ≥1 minor deprecation → 次 major で削除）
- **deferred**: 実際の v1.0.0 リリース判定ゲート通過とリリースタールボール cut-over は Phase 16 で

### 19 件の pre-existing バグも併せて修正
Phase 12 実装中の体系的な spot-check で v0.19.0 までに紛れ込んでいた 19 件の既存バグが判明し、すべて修正:
- GUI: `open_error` silent failure、regex error の hint 欠落、toolbar.* リテラル表示、`abdd-audit.md` mdbook ナビ孤立、23 件の dead i18n キー、`make_btn` false-positive
- ドキュメント: gui.md が日本語のまま英語パスに置かれていた、5 つの追加 CLI ガイドも同様、`aaai exit-codes` 未文書化、`compatibility.md` の crates.io 公開状態誤記、`overview.md` のコマンド数 15、`faq.md` の書き込み対象不足、`ci-integration.md` の終了コード安定性宣言欠落
- Toolchain: `Cargo.toml` に rust-version 不在、CI MSRV check が edition 2024 と非互換な 1.81 を指定（プロジェクト指示書通り 1.91 に統一）

## Phases 13〜16 — Phase 12 に統合済み ✅

当初計画では Phase 13〜16 で個別にリリース予定だった内容は、すべて Phase 12 / v0.20.0 で先行実装した:

| 当初予定 | 当初 RFC | 実際の着地 |
|---|---|---|
| ~~Phase 13 (v0.21.0)~~ | RFC 020 | v0.20.0 |
| ~~Phase 14 (v0.22.0)~~ | RFC 021, 022 | v0.20.0（RFC 021 は partial） |
| ~~Phase 15 (v0.23.0)~~ | RFC 023, 024 | v0.20.0 |
| ~~Phase 16 (v1.0.0)~~ | RFC 025 | v0.20.0（docs groundwork のみ。実リリースゲート通過は別途） |

---

## Phase 13 — Post-v0.20.0 followup ✅ (v0.21.0)

v0.20.0 では 9 件の RFC が同時並行で実装され、いくつかの deferred 項目と新たに発覚した課題が残った。Phase 13 ではエラー UX パターンの統一化と GUI i18n の完全化を中心に **7 件の RFC を順次実装** し、v0.21.0 としてリリース:

### RFC 026 — Toast Error Hints & i18n Key Re-introduction ✅
- RFC 020 の `message + hint` パターンを toast 側にも拡張
- `App::push_toast_with_hint`, `App::push_user_error_toast`, `UserError::from_i18n` の 3 helper 追加
- Phase 12 dead-key sweep で削除されていた 4 件の i18n キーを復活 (`error.{inspector.invalid_regex,save.failed}.{message,hint}`)
- 3 call-sites (save_failed × 2, inspector regex × 1) を refactor
- `scripts/check-i18n-keys.py` に `UserError::from_i18n("prefix")` パターン認識を追加
- pre-existing bug 修正: `util::tests::within_a_minute_is_just_now` の `-1s` を `+1s` に

### RFC 027 — CI mdbook build job ✅
- `.github/workflows/ci.yaml` に新規 `docs-build` ジョブ（mdbook `^0.4` `--no-default-features` pin）
- 英語 + 日本語両ロケールの `mdbook build` を blocking として実行
- Phase 12 で発覚した orphan chapter 問題などを merge 前にハンマー的に防ぐ
- CI ジョブ計 5 → 6

### RFC 028 — FieldError native `hint` field ✅
- RFC 026 で workaround として使った `💡` インライン合成を構造分離
- `FieldError { field, message, hint: Option<String> }` に拡張
- Inspector render path を muted-style 第 2 行表示に対応
- 6 construction sites を更新 (1 with `Some`, 5 with `None`)
- 2 new unit tests, aaai-gui tests 9 → 11

### RFC 029 — FieldError i18n migration ✅
- Inspector validation の 4 件の英語ハードコード文字列を `t!()` 経由で i18n 化
- `error.inspector.{invalid_sha256,empty_rules,empty_rule_line,empty_expected}.message` 新規キー (8 entries)
- Phase 12 の i18n 完備の方針を Inspector まで拡張

### RFC 030 — FieldError hint authoring (selective) ✅
- RFC 028/029 の延長として、メッセージから action が自明でない 2 サイトに hint 追加
  - `invalid_sha256`: SHA-256 生成方法 + 過去監査結果からのコピー
  - `empty_rules`: `+ Add rule` / `+ ルール追加` ボタンへの誘導
- 残り 2 サイト (`empty_rule_line`, `empty_expected`) は **意図的に hint 未追加** (メッセージが action を示すため noise になる)
- 「hint を埋めるのは魅力的だが選択的に」という RFC 028 §3 の判断基準を実践

### RFC 031 — User-facing string i18n migration sweep in `app.rs` ✅
- `app.rs` の grep で **8 件**の英語ハードコード文字列を発見・全て i18n 化
  - Progress (`Comparing folders…`)
  - Batch validation (`Reason must not be empty.`)
  - Inspector validation (`Reason is required…`, `Use YYYY-MM-DD format.`)
  - Opening inline validation (`Before/After folder is required.`, `Folder not found.`, `Path is not a directory.`)
- `progress.*` namespace 新設
- `app.rs` 内の user-facing ハードコード文字列を **0 件**に
- aaai-core の `is_approvable()` 由来エラー文言は v1→v2 major bump 必要として deferred を明示

### RFC 032 — `views/*.rs` user-facing string i18n migration ✅
- 6 view file の grep で **20 件**の英語ハードコード文字列を発見・全て i18n 化
  - `batch.rs` (1), `dashboard.rs` (3), `diff_view.rs` (9), `inspector.rs` (4), `main_view.rs` (2), `opening.rs` (1)
- `rust-i18n` の `%{name}` placeholder substitution を初めて導入 (opening.rs format string)
- pick_list の display ↔ Message protocol 兼用文字列 5 件 (`Added`/`Removed`/`Added lines` 系) は明示的に out-of-scope (RFC 033 で別途扱う)
- views/*.rs の in-scope ハードコード文字列を **0 件**に
- main.* namespace 新設

### Phase 13 metrics

| Metric | v0.20.0 end | v0.21.0 end |
|---|---|---|
| RFCs in done/ | 26 | **33** (+7) |
| i18n keys (code/en/ja) | 119 | **157/157/157** (+38) |
| aaai-core tests | 97 | 97 |
| aaai-cli tests | 70 | 70 |
| aaai-gui tests | 8+1f | **11** (+3, +1 bugfix) |
| CI jobs | 5 | **6** (+1 docs-build) |
| GUI hardcoded user-facing strings | 多数 | **0** (app.rs + views in-scope) |

### Process learning recorded

RFC 031 の draft で「これが最後の hardcoded string」を 3 回間違えた経験を RFC 自体に記録。後続の RFC 032 ではこの教訓を活かし、**draft 前に網羅 grep を流して in-scope / out-of-scope の境界を確定** してから実装に着手。結果、RFC 032 は scope creep ゼロで 1 回の implementation pass で完了。

---

## Phase 14 — GUI i18n endgame ✅ (v0.22.0)

Phase 13 で deferred になった項目および v0.21.0 リリース時点で残る課題を整理:

### deferred from Phase 13

- **RFC 033 — pick_list display/value separation**: `views/inspector.rs` の `Added`/`Removed`/`Added lines` 系 5 件の i18n 化。Message protocol を `LineAction`/`RegexTarget` enum 直接渡しに変更し、display label と internal value を分離する adapter 型を導入する必要あり (RFC 032 で out-of-scope として明示)
- **`expires_at` chrono error i18n**: 現在は GUI 側で `.is_err()` boolean として扱い独自メッセージで上書きしている。chrono の `ParseErrorKind` variant 別の翻訳 table を持つかどうか設計検討が必要
- **aaai-core `Result<(), String>` → structured enum 化**: `AuditEntry::is_approvable()` および `AuditStrategy::validate()` のエラー文言を i18n 化するには aaai-core public API を `Result<(), ApprovalError>` 形式の structured enum に変える必要あり。これは v1 → v2 の **major version bump** (compatibility.md 上の契約)。他の aaai-core API breaking changes (もしあれば) と bundle して扱うのが望ましい
- **`AuditStatus::Display` impl の i18n**: dashboard.rs L70 で `r.status.to_string()` を直接表示しているが、これも aaai-core API 変更の同範囲

### v0.20.0 視覚検証の結果次第

- **RFC 020 ABDD 監査シート埋め込み作業**: operator の検証結果を `docs/src/abdd-audit.md` に転記
- **RFC 022/023 視覚検証で判明した不整合の修正**: 個別 fix RFC として切る

### アーキテクチャ系

- **RFC 034 (想定) — audit-dirty rerun decoupling**: RFC 021 で deferred になったバナー機能を有意義に動かすため、definition mutation と synchronous rerun を decouple する設計。`Task::perform` ベースの async rerun に切り替え、その間 `audit_dirty=true` が visible になる仕組み

### CI / 開発体験

- **`mdbook test` で code sample をテスト**: 現状 0 件だが、CLI 使用例の動作検証など用に追加する価値ありか検討

---

## Phase 15 — Polish & Correctness ✅ (v0.23.0)

9 RFCs shipped in v0.23.0, focused on UX completeness, data correctness, and docs:

- **RFC 036** — App Settings dialog (language switcher + global ignored directories)
- **RFC 037** — Async rerun with audit-dirty toolbar indicator
- **RFC 038** — Keyboard shortcuts `?` help overlay
- **RFC 039** — Revert-to-Pending in Inspector + Opening screen profile delete
- **RFC 040** — Report export with native save-file picker (MD + JSON from GUI)
- **RFC 041** — Unsaved-changes navigation guard dialog
- **RFC 042** — Dynamic window title + auto-profile on audit run
- **RFC 043** — Status counts in filter bar + bottom-bar count i18n fix
- **RFC 044** — `expires_at` enforcement in audit engine (bug fix; 2 new tests)

### Phase 15 metrics

| Metric | v0.22.0 end | v0.23.0 end |
|---|---|---|
| RFCs done | 36 | 45 |
| aaai-core tests | 99 | 101 |
| aaai-cli tests | 70 | 70 |
| aaai-gui tests | 15 | 15 |
| Total tests | 184 | 186 |
| i18n keys (EN/JA) | 192/192 | 216/216 |
| Warnings | 0 | 0 |


## Phase 16 — v1.0.0 リリース判定 (改めて)

v0.20.0 で RFC 025 docs groundwork は landed したが、**実リリースゲート通過 → tag/publish → GitHub Release** は別個に行う。

### v1.0.0 リリース判定ゲート

| グループ | ゲート | 関連 RFC |
|---|---|---|
| 機能 | G1 視覚検証 / G2 i18n / G3 ABDD / G4 エラー文 / G5 画面リレーション / G6 空状態 / G7 Opening / G8 CLI | 017〜024 |
| 品質 | Q1 cargo test / Q2 clippy / Q3 fmt / Q4 mdbook / Q5 release ビルド / Q6 packaging | — |
| ドキュメント | D1 README / D2 ROADMAP / D3 CHANGELOG / D4 compatibility.md | 019 |

すべての G ゲート は v0.20.0 のコード/ドキュメントレベルでは満たされている。**残るは operator の視覚検証エビデンス**。

### v1.0.0 リリース手順

`docs/release-prep-v0.20.0.md` の 11 ステップを v0.20.0 ベースとして利用し、`v1.0.0` への version bump と CHANGELOG プロモートを行うだけで足りる見込み。

### v1.0.0 以降の互換性宣言 (`docs/src/compatibility.md`)

v0.20.0 で landed 済み。詳細は当該ドキュメント参照。

## v1.1.0 以降（暫定）

v1.0.0 ソーク中に別 RFC で議論する候補:
- スクリーンリーダー対応（iced 側の a11y サポート待ち）
- 追加言語（zh / ko）
- `aaai-core` の独立 crate 化

## Phase 16 — Workflow & UX Completeness ✅ (v0.24.0)

9 RFCs shipped in v0.24.0, focused on making the approval workflow frictionless:

- **RFC 045** — Opening screen Optional settings cleanup (remove `.aaaiignore`, rename "Audit definition" → "Approvals file", remove hint)
- **RFC 046** — Save-as dialog for new approvals files (fixes new-user dead-end on first `Ctrl+S`)
- **RFC 047** — Profile approvals visibility (auto-expand Optional settings when definition is loaded)
- **RFC 048** — Inspector progressive disclosure + profile row simplification ("Less is more")
- **RFC 049** — Inspector validation visibility + Approvals file placeholder
- **RFC 050** — Auto-advance to next Pending entry after approval
- **RFC 051** — `Ctrl+Enter` keyboard shortcut for approval
- **RFC 052** — Auto-select first Pending entry on audit start
- **RFC 053** — Dashboard all-clear CTA buttons (Export Report / New Audit)

### Phase 16 metrics

| Metric | v0.23.0 end | v0.24.0 end |
|---|---|---|
| RFCs done | 45 | 54 |
| Total tests | 186 | 186 |
| i18n keys (EN/JA) | 216/216 | 219/219 |
| Warnings | 0 | 0 |

## Phase 17 — Glob Rules & Power Workflow ✅ (v0.25.0)

Theme: Make aaai practical for large repositories and team workflows.
aaai-core already supports glob-pattern entries; this phase exposes
them through the GUI, completes two nearly-finished CLI commands, and
adds targeted power-user features.

### GUI: glob pattern entries (RFC 054–055)

**RFC 054 — Glob pattern entries in Inspector**
When looking at a specific diff (e.g. `node_modules/lodash/README.md`),
the user can choose to approve as a glob (`node_modules/**`) that covers
many matching files at once. The Inspector gains a "Use pattern ▸"
toggle that expands a pattern text input pre-filled with the current
path. On approval, the glob entry is saved; auto-advance (RFC 050)
skips all other diffs already matched by the new pattern.

**RFC 055 — Auto-suggest glob patterns**
When the "Use pattern" toggle opens, suggest two or three candidate
patterns derived from the current path:
- Parent-directory wildcard (`parent/**`)
- Extension wildcard (`**/*.ext`)
- Direct parent + filename (`parent/*.ext`)
Clicking a chip fills the pattern field. User can also type freely.

### CLI: complete near-done stubs (RFC 056–057)

**RFC 056 — `aaai watch` completion**
Complete the `watch.rs` stub: debounced file-system watcher using
`notify`; re-runs audit on any change to Before, After, or the
definition file; prints a compact diff-summary on each run.

**RFC 057 — `aaai export` completion**
Complete the `export.rs` stub: write `path, diff_type, status, reason,
strategy, ticket, approved_by, approved_at, expires_at, note` as
CSV or TSV to stdout or a file.

### Polish (RFC 058)

**RFC 058 — Pending count in window title**
Window title shows `aaai — audit.yaml ● (12 pending)` so the user
can see audit progress from the OS taskbar without switching focus.

### Phase 17 metrics

| Metric | v0.24.0 | v0.25.0 |
|---|---|---|
| RFCs done | 54 | 59 |
| aaai-cli tests | 70 | 74 |
| Total tests | 186 | 190 |
| i18n keys | 219/219 | 225/225 |
| GUI glob support | ✗ | ✓ |
| `aaai watch` complete | stub | ✓ |
| `aaai export` complete | stub | ✓ |

| Metric | v0.24.0 | v0.25.0 target |
|---|---|---|
| RFCs done | 54 | 59 |
| i18n keys | 219 | ~230 |
| GUI glob support | ✗ | ✓ |
| `aaai watch` complete | stub | ✓ |
| `aaai export` complete | stub | ✓ |

## Phase 18 — CLI Completeness II ✅ (v0.26.0)

Theme: Activate the final batch of pre-written CLI stubs with tests.
Every command below was already feature-complete as a stub;
Phase 18 confirms each compiles, adds integration tests,
and wires them into the release.

| RFC | Command | Highlights |
|---|---|---|
| 059 | `aaai lint` | Best-practice linter: duplicate paths, short reasons, expired entries, empty LineMatch rules, disabled entries; `--json-output`; exits 1 on errors |
| 060 | `aaai merge` | Merge two definition files; overlay wins on conflict; `--detect-conflicts`, `--dry-run`; atomic write |
| 061 | `aaai check` | Validate a definition file; expired/expiring-soon detection; `--all` to list every entry |
| 062 | `aaai history` | Show recent audit runs from `~/.aaai/history.jsonl`; `--stats` trend analysis; `--prune N` rotation |
| 063 | `aaai dashboard` | CLI dashboard with stat cards, attention list, next-action hint |
| 064 | GUI: `suggest_patterns` tests | Unit tests for glob suggestion algorithm; validates all 3 chip types and edge cases |

### Phase 18 metrics

| Metric | v0.25.0 | v0.26.0 |
|---|---|---|
| RFCs done | 59 | 65 |
| aaai-cli tests | 74 | 86 |
| aaai-gui tests | 15 | 20 |
| Total tests | 190 | 207 |
| Remaining unactivated stubs | 5 | 0 |

## Phase 19 — v1.0.0 Readiness ✅ (v0.27.0)

Theme: Close the last test coverage gaps before v1.0.0.
Every feature ships with direct unit tests; all CLI stubs activated.

| RFC | Description |
|---|---|
| 065 | `aaai init` activation — last unactivated stub; `--non-interactive` testable |
| 066 | aaai-core `AuditDefinition` direct unit tests — `find_entry` exact + glob, `is_glob`, `glob_matches`, `expired_entries`, `expiring_soon`, `is_approvable` |
| 067 | README fix + minor accuracy pass |

### Phase 19 target metrics

| Metric | v0.26.0 | v0.27.0 target |
|---|---|---|
| RFCs done | 65 | 68 |
| aaai-cli tests | 86 | ~90 |
| aaai-core tests | 101 | ~115 |
| aaai-core tests | 101 | 111 |
| aaai-cli tests | 86 | 89 |
| Total tests | 207 | 220 |
| Unactivated stubs | 1 (`init`) | 0 |

## Phase 20 — GUI & UI/UX Quality ✅ (v0.29.0)

Theme: Address layout, visual stability, and interaction quality issues
that have accumulated across feature phases. No new functionality —
purely improving the user experience of what already exists.

| RFC | Issue | File |
|---|---|---|
| 069 | Diff pane scroll sync — left/right panels now scroll together | `diff_view.rs`, `app.rs` |
| 070 | Toolbar layout stability + Undo relocation | `main_view.rs` |
| 071 | Search bar moved inside file tree pane | `main_view.rs` |
| 072 | Status badge compact pill + cleaner icon glyphs | `main_view.rs` |
| 073 | Bottom bar hidden when no file is selected | `main_view.rs` |

## Phase 21 — Explainable to Newcomers ✅ (v0.30.0)

Theme: Close the knowledge gaps a first-time user faces, with
just-in-time micro-guidance rather than tutorials or tooltip clutter.
Follows "less is more" — one sentence or example at the exact moment
the question arises, ignorable by experts.

| RFC | Novice gap | Approach |
|---|---|---|
| 074 | "What do I write in the reason box?" | Wire a placeholder into the reason field + show a diff-type-aware example line ("e.g. Port changed 80→8080 per ticket INF-42") |
| 075 | "Which strategy, and why?" | Pre-select the recommended strategy per diff type; mark it "(recommended)"; rewrite descriptions in plain language |
| 076 | "What does this status mean?" | A `?` by the status filters opens a 4-line plain-language legend |
| 077 | First-audit orientation | One dismissible coach line above the file tree after the first audit |

### Phase 21 target metrics

| Metric | v0.29.0 | v0.30.0 target |
|---|---|---|
| RFCs done | 74 | 78 |
| i18n keys | 222 | ~240 |

## Phase 22 — Newcomer UX Continuation ✅ (v0.31.0)

Continuing the Phase 21 theme with three targeted fixes uncovered by
the post-21 audit.

| RFC | Gap | Fix |
|---|---|---|
| 078 | `□ Open` stale icon in diff empty-state strings | Update to `← Open` |
| 079 | Opening onboarding explains HOW but not WHY | Add a one-line context sentence before the numbered steps |
| 080 | Checksum strategy: no hint on how to get the hash | Add a "how to get this" micro-hint below the SHA-256 field |

## Phase 23 — Pre-1.0 Housekeeping ✅ (v0.31.1)

Patch release addressing stale documentation and publish-chain readiness.

| RFC | Work |
|---|---|
| 081 | docs/src/gui.md + docs/ja/src/gui.md — update for Phase 20–22 changes |
| 082 | Fix aaai-core README path warning + add RELEASING.md |

## Phase 24 — Plain-Language GUI ✅ (v0.32.0)

Theme: Adopt the UI/UX architect review's plain-language recommendations.
The GUI should read like a calm review assistant, not an audit console.
Internal vocabulary (the `AuditStatus` enum, CLI output, reports, docs)
stays precise; only GUI display strings become friendlier. This keeps
CLI and GUI judgment logic identical (design doc p.9) while making the
GUI approachable to non-technical reviewers.

| RFC | Scope |
|---|---|
| 083 | Plain-language action labels — toolbar, opening screen, bottom bar |
| 084 | Plain-language status labels + hints — file tree, legend, filter bar, toolbar badge |
| 085 | Plain-language strategy labels — picker options + section header + inspector title |
| 086 | Navigation guard — hide "Discard and Leave" behind a secondary step |

## Phase 25 — Guided Interactions ✅ (v0.33.0)

Theme: Close the final gaps identified in the architect review's acceptance
criteria. Every disabled primary action now explains what is missing.

| RFC | Gap | Fix |
|---|---|---|
| 087 | "Save and continue" silently disabled | Tooltip: "Write why this change is OK first." / "Choose a file first." |
| 088 | "Check changes" silently disabled | Inline hint below button: what is still needed |
| 089 | Help overlay uses old pre-Phase-24 labels | Update to plain-language action names |
| 090 | Bottom bar count says "N of M unresolved" | Reword to "N of M need review" / "All N reviewed" |

## RFC 091 — Windows Store Packaging Model

One Store product, two executables, one shared core.

| Task | Status |
|---|---|
| CI: add aaai-gui to Build + MSRV + windows-gui-build job | ✅ Done |
| Release: build both binaries; CLI/GUI/full Windows archives | ✅ Done |
| packaging/windows/ — manifest template, scripts, placeholder assets | ✅ Done |
| .github/workflows/windows-msix.yaml — MSIX candidate workflow | ✅ Done |
| docs/src/windows-store.md | ✅ Done |
| Store submission (manual, post-v1.0.0) | ⏳ Pending |
| Replace placeholder assets with real artwork | ⏳ Pending |
| Local install validation (Task E) | ⏳ Pending |
