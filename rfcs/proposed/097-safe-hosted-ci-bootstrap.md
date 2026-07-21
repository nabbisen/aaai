# RFC 097 — Safe Hosted-CI Bootstrap

**Status.** Proposed

**Tracks.** `ROADMAP.md` M1C / WS-03 / gate B0

**Depends.** RFC 095 / D0 and RFC 096 / S1

**Design owner.** Codex

**Decision owner.** nabbisen, project owner

**Primary implementer.** Codex; workflow implementation and corrections are
complete and independently accepted

**Required operations owner.** nabbisen, because GitHub Actions availability,
manual dispatch, repository rules, and required-check configuration are
owner-controlled external state

**Independent reviewers.** Architecture/security reviewer for design and
implementation; the owner must separately confirm implementation capacity,
hosted-runner availability, and operations capacity before coding begins

**Environment boundary.** GitHub-hosted `ubuntu-latest`, `macos-latest`, and
`windows-latest` runners with Rust 1.91. Local Linux may be used for preflight
only and cannot satisfy B0.

**Evidence location.** `.git-exclude/evidence/097-safe-hosted-ci-bootstrap/`

**Touches.** A new `.github/workflows/b0.yaml`, the RFC lifecycle index, and
the required handoff under
`rfcs/handoffs/097-safe-hosted-ci-bootstrap/`. The existing
`.github/workflows/ci.yaml` remains unchanged. This RFC does not change product
code, persisted formats, release workflows, packaging, dependencies, public
documentation, or repository rules by itself.

**Handoff.** Required:
[`rfcs/handoffs/097-safe-hosted-ci-bootstrap/README.md`](../handoffs/097-safe-hosted-ci-bootstrap/README.md)

**Observed implementation evidence.** Independent implementation review
accepted the workflow and the hosted B0 run for `main` SHA
`03e9a2d4e00600a8519c4cc704cd2174b840bca7`, run `29798727537`. The two
preceding deterministic Windows failures remain recorded. Repository rules do
not currently require `B0 / gate`.

**Proposed governance amendment.** While the project remains owner-only with
one serial `main` integration line, B0 blocks continuation after a push instead
of blocking a merge. This amendment requires independent architecture review
before it may close B0.

## 1. Summary

This RFC creates one trustworthy hosted bootstrap gate after S1:

```text
Rust 1.91 × ubuntu-latest
Rust 1.91 × macos-latest
Rust 1.91 × windows-latest
                |
                v
           B0 / gate
```

Every platform cell runs the locked full workspace test suite with default test
parallelism. RFC 096's integration harness therefore exercises the current
Cargo-built CLI against an owned state sandbox and platform-shaped fallback
canaries on each hosted OS.

A single aggregate job named `B0 / gate` succeeds only when all three matrix
cells succeed. It is the only status proposed as the B0 branch-protection
candidate. Existing format, Clippy, security, i18n, visual-status, docs, GUI
build, release, and MSIX surfaces are not relabeled as B0.

During the proposed owner-only single-branch continuity mode, the same result
is fail-closed operationally: an unsuccessful or unobserved result for the
latest pushed `main` SHA stops unrelated continuation, downstream gate use,
and release operations. Before the repository adopts concurrent integration,
`B0 / gate` must become an observed required check on the actual merge-control
path.

The new B0 workflow is separate from the existing CI workflow. Existing
format, Clippy, RustSec, docs, i18n, visual-status, and GUI jobs retain their
current triggers, permissions, and conclusions. They do not prevent the
separate B0 matrix from starting, and B0 does not relabel them or claim the
entire CI system is green. C0, C2, and C1 own later security, quality, and
complete-workflow convergence.

This RFC does not authorize implementation, workflow dispatch, a push, a
required-check change, or any release operation.

## 2. Problem and observed baseline

### 2.1 The current hosted matrix cannot test the workspace

`.github/workflows/ci.yaml` names the engine package `aaai-core` in:

- the three-OS build step;
- the engine unit-test step;
- the Linux-only MSRV check.

The current package is `aaai`. Cargo therefore rejects those commands before
they can provide build or test evidence.

### 2.2 Stateful CLI tests are serialized and predate S1

The current integration step is:

```sh
cargo test -p aaai-cli -- --test-threads=1
```

RFC 096 deliberately made the suite default-parallel-safe and moved black-box
coverage to the `cli` integration target. Serial execution would conceal
parallel isolation regressions and is no longer the accepted gate.

The hosted workflow has not yet executed the accepted S1 harness on Windows,
where ordinary home/config environment variables cannot authoritatively
redirect the Known Folder API.

### 2.3 Format and Clippy currently prevent platform evidence

The existing three-OS `test` job runs repository-wide format and Clippy before
build/test. RFC 096 implementation review independently confirmed that
`cargo fmt --all -- --check` is red because of pre-existing formatting across
unrelated files. Clippy cleanliness is likewise assigned to WS-08/C2 and is not
established by S1.

B0 must not waive or describe those checks as passing. A separate B0 workflow
keeps them observable and unchanged while removing them from the B0 dependency
path.

### 2.4 MSRV is Linux-only and incomplete

The workspace declares Rust 1.91, but the current `check-msrv` job:

- runs only on Ubuntu;
- uses nonexistent `aaai-core`;
- performs `cargo check`, not the isolated full test suite;
- has no stable aggregate context for branch protection.

B0 requires declared-MSRV execution on all three hosted operating systems.

### 2.5 Gate identity and operator evidence are ambiguous

The repository has no separate B0 workflow with a least-privilege permission
declaration, concurrency policy, per-job timeout, or aggregate. A new workflow
also needs an executable initial event route: `workflow_dispatch` cannot
bootstrap a workflow that is not yet present on the default branch.

## 3. Goals and non-goals

### 3.1 Goals

- Run the exact locked workspace on hosted Linux, macOS, and Windows with Rust
  1.91.
- Exercise all engine, CLI unit/integration, GUI unit, and doctest targets
  reached by `cargo test --workspace --locked`.
- Preserve RFC 096 default parallelism and current-binary discovery.
- Make every platform cell blocking for B0 and allow all cells to finish when
  one fails.
- Produce one stable `B0 / gate` result suitable for owner-configured required
  checks.
- Record enough runner/toolchain/job evidence for independent review without
  dumping inherited environment or secrets.
- Leave the existing CI workflow, including its known-red or otherwise
  unverified jobs and token authority, unchanged.
- Define operations-owner-assigned PR, dispatch, and repository-rule actions
  separately from developer implementation without claiming technical
  owner-only enforcement.
- Define an objective owner-only serial mode in which B0 failure blocks
  continuation, and mandatory triggers for activating merge-time enforcement.

### 3.2 Non-goals

- Editing or making every job in `ci.yaml` green.
- Repairing repository-wide formatting or Clippy findings.
- Resolving dependency advisories or action supply-chain pinning.
- Repairing mdBook, release, versioning, packaging, MSIX, or publishing
  workflows.
- Testing release builds or producing artifacts.
- Changing the declared MSRV.
- Adding self-hosted runners or platform-specific product behavior.
- Mutating product code to accommodate a CI-only failure before diagnosis and
  architecture review.
- Reading or fingerprinting actual user state on a hosted runner.
- Automatically changing branch protection or repository rules.

## 4. Authority and safety invariants

1. B0 is satisfied only by a GitHub-hosted three-OS run, never local evidence.
2. Every B0 cell installs Rust 1.91 and reports the observed compiler and Cargo
   versions before testing.
3. Every B0 cell checks out the same commit and uses `--locked`.
4. Every B0 cell runs `cargo test --workspace --locked` without
   `--test-threads=1`.
5. The RFC 096 helper remains the only raw CLI subprocess constructor in the
   integration target.
6. No workflow-level `AAAI_TEST_STATE_DIR`, `HOME`, `APPDATA`, or equivalent
   override replaces the per-command owned sandbox.
7. `fail-fast: false` allows all OS results to complete for diagnosis.
8. No B0 matrix cell uses `continue-on-error`.
9. `B0 / gate` fails or remains non-success when any required matrix job
   fails, is cancelled, or is skipped.
10. B0 jobs receive only read access to repository contents. They use no custom
    repository or environment secrets; GitHub still provisions its automatic
    read-only `GITHUB_TOKEN`.
11. The workflow does not use `pull_request_target`, privileged forks, release
    events, deployments, or write permissions.
12. Cache restoration is an optimization, not acceptance evidence. A cache
    outage may be non-blocking only at the cache step; compilation/tests remain
    blocking.
13. Logs and evidence may include synthetic RFC 096 labels, runner metadata,
    compiler versions, test names/counts, and failure text. They must not dump
    the inherited environment, token values, or operator data.
14. Existing format, Clippy, RustSec, docs, i18n, visual-status, and GUI jobs
    remain outside `b0.yaml` and cannot be mistaken for B0 cells or aggregate
    dependencies.
15. A manually rerun deterministic failure remains a failure. At most one
    infrastructure-only rerun is allowed and both run IDs are retained.
16. Required-check configuration is an owner operation after a successful
    reviewed run; workflow code cannot silently modify repository rules.
17. In single-branch continuity mode, an unsuccessful or unobserved B0 result
    for the latest pushed `main` SHA blocks unrelated implementation pushes,
    downstream gate consumption, and release, tag, or publish operations.
18. Merge-time enforcement is mandatory before any second writer, concurrent
    integration branch, pull-request integration flow, merge queue, or
    independently integrating bot/automation is introduced.

## 5. Selected workflow design

### 5.1 Triggers, permissions, and concurrency

Create `.github/workflows/b0.yaml` with:

```yaml
name: B0 Hosted Bootstrap

on:
  push:
    branches: [main]
  pull_request:
    branches: [main]
  workflow_dispatch:

permissions:
  contents: read

concurrency:
  group: b0-${{ github.workflow }}-${{ github.event.pull_request.number || github.ref }}
  cancel-in-progress: true
```

The designed initial hosted-validation route was an
operations-owner-authorized, same-repository pull request targeting `main`.
Its `pull_request` event could execute the proposed workflow before merge.
Creating the review commit, branch push, and pull request were external
operations requiring separate explicit owner authorization; they were
provisional review transport, not approval to merge, close B0, or release.

Manual dispatch is unavailable until a `workflow_dispatch` form of `b0.yaml`
exists on the default branch. After that default-branch adoption only, a user
with repository write permission may use `--ref` to select a ref. The handoff
assigns that procedural operation to nabbisen, but RFC 097 does not claim
GitHub technically restricts all manual dispatches to the repository owner.

The workflow prevents obsolete runs on the same change from consuming all
runner capacity and grants its automatic `GITHUB_TOKEN` read-only contents
authority. It references no custom repository or environment secret.
Cancellation of an obsolete run is not B0 evidence.

The implementation must validate the expression against GitHub's workflow
parser. If the pull-request-number fallback expression is rejected, use the
documented, simpler `${{ github.workflow }}-${{ github.ref }}` grouping rather
than inventing a script.

### 5.2 Required platform matrix

Add this authority only to the new `b0.yaml`; do not replace or edit jobs in
the existing `ci.yaml`:

```yaml
b0-platform:
  name: B0 / Rust 1.91 / ${{ matrix.label }}
  timeout-minutes: 90
  strategy:
    fail-fast: false
    matrix:
      include:
        - os: ubuntu-latest
          label: Linux
        - os: macos-latest
          label: macOS
        - os: windows-latest
          label: Windows
  runs-on: ${{ matrix.os }}
```

The labels intentionally select GitHub's hosted current images rather than
claiming a fixed OS release. Evidence must record the exact image/version
reported by each job.

Each cell performs, in order:

1. checkout;
2. install Rust 1.91 with a minimal profile;
3. restore a cache differentiated by OS and toolchain;
4. print `rustc -Vv` and `cargo -V`;
5. run `cargo metadata --locked --no-deps --format-version 1`;
6. run `cargo test --workspace --locked`.

The Cargo metadata output is synthetic repository structure, not user state.
It proves the job sees packages `aaai`, `aaai-cli`, and `aaai-gui` before the
test command.

Use the existing action families during B0:

```yaml
- uses: actions/checkout@v6
- uses: dtolnay/rust-toolchain@1.91
- uses: Swatinem/rust-cache@v2
  continue-on-error: true
  with:
    shared-key: b0-${{ runner.os }}-rust-1.91
```

Full commit-SHA pinning and dependency-action policy belong to WS-07/C0. B0
must not opportunistically choose new action vendors or upgrade unrelated
actions. The selected toolchain action installs with rustup's minimal profile
internally; RFC 097 does not pass an unsupported `profile` input.

### 5.3 Stable aggregate result

Add an Ubuntu aggregation job:

```yaml
b0-gate:
  name: B0 / gate
  if: ${{ always() }}
  needs: [b0-platform]
  runs-on: ubuntu-latest
  timeout-minutes: 5
```

Its only operation is to report `needs.b0-platform.result` and exit nonzero
unless it equals `success`. It does not check out code, restore a cache, access
secrets, or reinterpret individual test failures.

The owner may configure `B0 / gate` as a required check only after:

- the implementation review accepts the workflow;
- a current run satisfying Section 6.1's reviewed-head/base/tested-SHA
  relationship is green on all three cells;
- the exact visible status context is confirmed in GitHub's UI/API;
- repository rules permit the change without weakening other owner-selected
  checks.

The RFC does not pre-authorize that external change.

### 5.4 Existing CI and non-B0 authority

Do not edit `.github/workflows/ci.yaml`. Its jobs remain separate:

- existing stable test/format/Clippy job, including its known defects;
- Windows GUI build;
- security audit;
- i18n key audit;
- visual-verification status;
- docs build.

In particular, workflow-level `contents: read` in `b0.yaml` does not alter the
automatic token or documented RustSec check/issue permissions in `ci.yaml`.
RFC 097 neither accepts a read-only RustSec fallback nor adds job-level write
authority. C0/C1 must later disposition that job and the legacy workflow.

The existing Linux `check-msrv` remains a known broken legacy job, not B0
authority. B0 authority is unambiguously the new workflow's three-OS matrix and
aggregate. No result from `ci.yaml` is included in `B0 / gate`.

### 5.5 Blocking modes and enforcement activation

| Mode | Preconditions | B0 trigger and barrier | Closure evidence |
|---|---|---|---|
| Owner-only single-branch continuity | nabbisen is the sole writer/integrator; exactly one serial `main` line; no integration-bound concurrent branch, outside contributor/PR flow, merge queue, or independently integrating automation | Every pushed `main` SHA triggers B0. A latest SHA without successful `B0 / gate` stops unrelated pushes, downstream gate use, and release operations | Accepted workflow implementation, retained failed attempts, accepted governance amendment, a valid closure entry in `continuity-mode.md`, RFC 000-consistent lifecycle records, and a green three-OS run for their exact resulting final `main` SHA |
| Concurrent integration | Any serial-mode precondition is false or is about to become false | A pull request or merge queue must be unable to integrate without successful `B0 / gate` | Exact context required by the repository rule, rule snapshot, successful verification on the actual merge-control path, and independent review |

Single-branch continuity mode accepts a bounded post-push risk: `main` can be
temporarily red. While it is red, only diagnosis, correction, and evidence work
may proceed. A deterministic failure is repaired at a new SHA; its failed run
is retained and is not rerun unchanged to seek green.

Enforcement activation occurs before a second writer, concurrent branch
intended for integration, pull-request integration flow, bot or automation
with integration authority, merge queue, or other departure from owner-only
serial operation. Work stops while the owner:

1. selects the actual merge-control path;
2. amends and re-reviews this RFC for a `merge_group` trigger if a merge queue
   is selected;
3. configures the exact observed `B0 / gate` context as required without
   weakening unrelated checks;
4. records the repository rule state without secrets;
5. verifies the check on the actual merge-control path; and
6. obtains independent review before concurrent work resumes.

The owner reconfirms the single-branch preconditions at entry to every later
workstream. A false condition blocks entry until enforcement activation is
complete. Neither mode waives a platform cell, test failure, read-only
permission, or the later C0/C2/C1 gates.

The durable serial-mode record is
`.git-exclude/evidence/097-safe-hosted-ci-bootstrap/continuity-mode.md`. It has
one entry for B0 closure and a new entry at every later workstream boundary.
Each entry contains:

- timestamp and workstream or lifecycle-transition identifier;
- exact current `main` SHA, B0 run ID, attempt, event, run `headSha`, and
  `B0 / gate` conclusion;
- owner attestation that nabbisen is the sole writer and integrator and only
  one integration line exists;
- observed state or explicit owner attestation for the absence of an
  integration-bound concurrent branch, outside-contributor/PR integration
  flow, merge queue, and independently integrating bot/automation;
- observed repository-rule and enforcement state, explicitly classifying the
  barrier as procedural or technical; and
- the decision that serial mode remains valid, or that enforcement activation
  blocks entry.

When a read-only API cannot prove a condition, the entry labels it as owner
attestation. It does not record credentials, tokens, secrets, or secret
configuration.

## 6. B0 acceptance contract

### 6.1 Required green cells

The first hosted validation uses an owner-authorized same-repository pull
request targeting `main`. Evidence must distinguish:

- `H`, the reviewed PR head SHA containing `b0.yaml`;
- `B`, the target-branch base SHA used for the run; and
- `M`, GitHub's tested synthetic merge SHA when the `pull_request` run checks
  out `refs/pull/<number>/merge`.

The run's reported `headSha` is authoritative for what GitHub executed; for
the normal `pull_request` merge-ref path it is `M`, not `H`. Acceptance
requires one non-obsolete run whose recorded PR head is exactly reviewed `H`,
whose recorded base is `B`, and whose `M`/run `headSha` was produced from that
pair. All three cells and the aggregate must belong to that same run and
tested SHA. A run against an earlier head or base, a cancelled run, or a
superseded synthetic merge is not evidence.

The implemented workflow reached `main` through owner-performed direct pushes,
not the designed initial pull-request route. Final implementation review
accepted the workflow and hosted execution evidence but did not accept B0
lifecycle closure under the original merge-enforcement contract.

For single-branch continuity mode, a default-branch `push` run is authoritative
when its recorded `headSha` is the exact intended current `main` commit and all
cells belong to that run. Run `29798727537` satisfies this execution contract
for SHA `03e9a2d4e00600a8519c4cc704cd2174b840bca7`. A manual-dispatch run is
supplementary and does not replace the push result for the latest `main` SHA.
The PR `H`/`B`/`M` relationship above becomes mandatory evidence when
concurrent integration enforcement is activated.

For the accepted run:

| Cell | Toolchain | Required command | Required outcome |
|---|---|---|---|
| Linux | Rust 1.91 | `cargo test --workspace --locked` | success |
| macOS | Rust 1.91 | `cargo test --workspace --locked` | success |
| Windows | Rust 1.91 | `cargo test --workspace --locked` | success |
| Aggregate | N/A | inspect matrix result | `B0 / gate` success |

Every platform log must show packages `aaai`, `aaai-cli`, and `aaai-gui`, the
observed Rust/Cargo versions, the CLI integration target, and a successful
workspace result.

### 6.2 Isolation-specific evidence

The CLI integration log on every OS must show all RFC 096 focused cases pass,
including:

- empty history without root creation;
- five-to-three prune;
- history count/JSON/stats fixtures;
- status/output stdout/stderr disclosure rejection;
- relative-only fallback mutation reporting;
- success, non-zero, spawn failure, and unexecuted-wrapper paths;
- the three retained audit-write cases within the complete CLI suite.

A test count alone is insufficient if the integration target did not execute.
The operator records the test target and focused-case names from each job log.

### 6.3 Workflow and source scans

Implementation review must observe:

```sh
rg -n "aaai-core|test-threads=1" .github/workflows/b0.yaml
rg -n "AAAI_TEST_STATE_DIR|set_var|remove_var" .github/workflows crates -g "*.rs" -g "*.yaml"
rg -n "pull_request_target|permissions:|contents:|continue-on-error" .github/workflows/b0.yaml
rg -n "b0-platform|b0-gate|B0 / gate|fail-fast|rust-toolchain@1.91" .github/workflows/b0.yaml
git diff --exit-code -- .github/workflows/ci.yaml
git diff --check
```

Expected:

- no `aaai-core` or serialized-test match in the B0 workflow;
- the reserved variable appears only in the RFC 096 resolver and child helper,
  not as workflow environment;
- no `pull_request_target`;
- read-only contents permission;
- `continue-on-error` only on the cache step;
- one three-OS matrix, one Rust 1.91 installer, and one stable aggregate;
- no diff at all in the existing `ci.yaml`;
- no whitespace errors.

### 6.4 Evidence artifacts

Create:

```text
.git-exclude/evidence/097-safe-hosted-ci-bootstrap/
  continuity-mode.md
  environment.md
  local-preflight.log
  workflow-validation.md
  hosted-runs.md
  matrix-results.md
  isolation-results.md
  focused-scans.log
  scope.diffstat
```

`hosted-runs.md` records run ID, URL, event, branch/ref, run `headSha`,
attempt, conclusion, and whether a rerun occurred. For a pull-request run it
also records PR number/URL, `H`, `B`, and `M`, and whether the run was current
or obsolete when reviewed. Do not record tokens or inherited environment.

`matrix-results.md` records exact runner labels/images, observed toolchains,
package identities, command, duration, and conclusion per cell.

`isolation-results.md` records the RFC 096 focused case names and conclusions
per OS. It does not reproduce canary values.

`continuity-mode.md` follows Section 5.5. It is required for serial-mode B0
closure and for every later workstream entry; a missing, stale, or invalid-mode
entry blocks continuation.

## 7. Failure, retry, and rollback policy

### 7.1 Failure classification

| Failure | Classification | Action |
|---|---|---|
| Cargo package not found | deterministic workflow defect | block; repair and re-review |
| Rust 1.91 install rejected | toolchain/workflow defect or platform outage | diagnose; one rerun only with corroborated outage |
| compile/test failure on one OS | product or platform defect | block B0; do not exclude OS |
| RFC 096 isolation failure | security/data-safety defect | block immediately; preserve synthetic logs |
| cache restore/save failure | optimization failure | continue to uncached build |
| runner acquisition/network outage | infrastructure | record status evidence; one rerun allowed |
| cancelled or superseded run | no evidence | current PR head/base run required |
| non-B0 job red | later gate or separate defect | report accurately; do not call whole CI green |

The implementation review package includes every attempt. A rerun does not
erase the first result.

### 7.2 Workflow rollback

Before owner-required-check configuration, rollback is a normal revert of the
RFC 097 workflow patch.

After `B0 / gate` becomes required:

1. preserve the failing run ID and logs;
2. revert only the defective workflow change through normal review;
3. ensure the required context still reports rather than removing it to make a
   merge pass;
4. if GitHub no longer emits the context and repository work is deadlocked,
   the owner may temporarily remove only that newly added requirement, record
   the reason and timestamp, repair the workflow, and restore the requirement;
5. never disable all branch protection or mark a failing cell optional as
   rollback.

## 8. Implementation sequence

1. Owner confirms Codex implementation capacity, owner operations capacity,
   hosted Actions availability, and permission to use three hosted runners.
2. Independent architect accepts RFC 097 and the handoff.
3. Owner explicitly approves implementation.
4. Developer creates only the approved new B0 workflow and related lifecycle
   records; `.github/workflows/ci.yaml` remains byte-for-byte unchanged.
5. Run local structural scans and local Rust 1.91 preflight when available.
6. Prepare a draft implementation-review package before any external
   operation.
7. Owner separately and explicitly authorizes a provisional review commit,
   same-repository branch push, and pull request targeting `main`. This
   transports the patch for review; it does not authorize merge, B0 closure,
   or release.
8. The pull request event executes `b0.yaml`. Do not attempt initial
   `workflow_dispatch`. Operations records `H`, `B`, the run `headSha`/`M`,
   all attempts, all three cells, and the aggregate.
9. Independent implementation review evaluates the workflow diff, SHA
   relationship, all cells, and aggregate.
10. After implementation acceptance, the owner may merge through the
    repository's normal reviewed path.
11. The owner observes the resulting default-branch `push` run. Only after
    confirming the exact visible context may the owner configure `B0 / gate`
    as required.
12. B0 enforcement is claimed only after a subsequent pull request proves
    that `B0 / gate` is emitted and required on the repository's actual
    merge-control path.

If GitHub execution requires a product-code, dependency, MSRV, action-vendor,
release-workflow, or platform-exclusion change, stop and amend/re-review this
RFC before implementation continues.

### 8.1 Observed implementation disposition

The workflow implementation and corrections were integrated by the owner via
direct pushes to `main`; steps 7–12 above were therefore not followed as
designed. Independent final implementation review accepted the code and the
green three-OS push run, while leaving B0 open because neither the original PR
evidence nor required-check enforcement existed. This amendment makes that
deviation explicit and substitutes Section 5.5's single-branch continuity
contract only for the duration of the stated owner-only serial conditions.

### 8.2 Ongoing single-branch operation

For each pushed `main` SHA, the owner observes the corresponding B0 run before
unrelated continuation. Green preserves serial work eligibility. Failure,
cancellation, absence, or an obsolete result invokes stop-work until a new
corrected SHA succeeds. At every downstream workstream entry, the owner records
that Section 5.5's preconditions still hold or activates merge-time enforcement
before entry.

### 8.3 Lifecycle closure handback

RFC 097 remains Proposed until the governance amendment is accepted and its
integrated `main` SHA passes B0. The owner then performs a focused mechanical
lifecycle transition under RFC 000:

1. create the B0-closure entry in `continuity-mode.md` for the current green
   amendment SHA and record the accepted governance review;
2. move this RFC to `rfcs/done/097-safe-hosted-ci-bootstrap.md` and set its
   status to Implemented / B0;
3. move RFC 097 from Proposed to Done in `rfcs/README.md`;
4. update every affected tracked relative link, including this handoff's RFC
   link from `../../proposed/` to `../../done/`;
5. update `ROADMAP.md` so its planning status, M1C/B0 milestone and gate row,
   and WS-03 state say complete without claiming merge-time enforcement;
6. record the accepted governance-review and continuity-evidence paths in the
   lifecycle records; and
7. independently review the mechanical closure boundary before owner
   integration.

That lifecycle transition creates a new `main` SHA. It enters stop-work until
the corresponding push run succeeds on Linux, macOS, Windows, and `B0 / gate`
with run `headSha` equal to that exact final SHA. B0, M1C, and WS-03 close only
after that result is recorded in `continuity-mode.md`. WS-04 and all other
unrelated continuation remain blocked until then. A failed lifecycle SHA is
corrected at a new SHA and does not inherit the earlier green result.

This two-stage handback keeps the tracked lifecycle transition mechanical and
auditable: amendment acceptance and its green SHA precede the lifecycle move;
the lifecycle move's own exact SHA then must be green before its completion
claims become operative downstream.

## 9. Compatibility and downstream impact

There is no product compatibility change. B0 supplies later workstreams with:

- a current-package, locked workspace command;
- hosted Rust 1.91 evidence on Linux, macOS, and Windows;
- safe default-parallel execution of RFC 096;
- one stable platform aggregate for later workflow composition;
- a documented separation between B0 and later C0/C2/C1 gates.

WS-04 and WS-05 may use this matrix for adversarial platform cases only after
B0 passes. WS-06 may use it for persistence tests. WS-08 owns restoring
blocking format/Clippy policy. WS-09 owns full release/workflow convergence.

## 10. Alternatives considered

| Option | Decision |
|---|---|
| Fix only `aaai-core` names | Rejected: leaves serialization, Linux-only MSRV, and no stable gate identity |
| Keep stable three-OS tests plus Linux-only MSRV | Rejected: B0 requires declared-MSRV platform evidence |
| Run stable and MSRV on all three OSes | Deferred: six cells add cost without being required for bootstrap; later CI policy may add stable coverage |
| Keep format/Clippy before tests | Rejected: known-red later-gate debt would prevent B0 evidence |
| Delete format/Clippy entirely | Rejected: hides debt and weakens the transition to C2 |
| Make format/Clippy non-blocking observations in `ci.yaml` | Rejected: unnecessarily changes non-B0 conclusions and the existing workflow's token authority boundary |
| Claim the full CI workflow green | Rejected: docs, audit, quality, and release convergence have later owners |
| Use a separate least-privileged B0 workflow | Selected: isolates B0 permissions and conclusions while leaving `ci.yaml` unchanged |
| Set workflow-wide state/home variables | Rejected: bypasses RFC 096's owned per-command contract and is not authoritative on Windows |
| Exclude Windows if the GUI or Known Folder path fails | Rejected: the failure is exactly the evidence B0 exists to surface |
| Auto-edit branch protection through a token | Rejected: unnecessary write authority and outside developer scope |

## 11. Risks and mitigations

| Risk | Mitigation |
|---|---|
| A green aggregate hides a skipped cell | `always()` aggregate rejects every result other than `success`; implementation review inspects all cells |
| Matrix job names drift and break required checks | Require only stable `B0 / gate`, confirmed from an observed run |
| Hosted `-latest` image changes | Record exact image metadata per run; B0 is evidence for that run, not a permanent image claim |
| Cache corruption masks behavior | Cache is non-authoritative; uncached compile/test remains blocking |
| Known-red quality debt is mistaken for fixed | Leave `ci.yaml` unchanged and state that B0 does not accept or relabel its results |
| Full workflow remains red | State B0 scope precisely; C0/C2/C1 remain open |
| Pull request obtains excess authority | Same-repository initial PR, read-only automatic token, no custom secrets, no `pull_request_target` |
| Initial dispatch silently cannot run | Bootstrap only through the `pull_request` event; permit manual dispatch only after default-branch adoption |
| PR head SHA is confused with tested merge SHA | Record PR head/base and run `headSha`/synthetic merge together; reject obsolete runs |
| Rerun laundering | One infrastructure-only rerun; retain all run IDs and attempts |
| Owner required-check change deadlocks merges | Verify visible context first and use narrow rollback procedure |
| Action tags are mutable | Retain current action families for B0; WS-07/C0 owns pin policy |
| Local preflight is mistaken for B0 | Lifecycle and evidence require GitHub-hosted three-OS results |
| `main` is temporarily red in serial mode | Stop all unrelated continuation, downstream gate use, and release operations until a corrected SHA is green |
| The repository silently drifts into concurrent integration | Reconfirm serial preconditions at every workstream entry and require reviewed enforcement activation before any trigger condition |

## 12. Review questions

1. Is Rust 1.91 on all three hosted OSes the correct minimum B0 matrix?
2. Is `cargo test --workspace --locked` sufficient to exercise current engine,
   CLI integration, GUI unit, and doctest surfaces without separate serialized
   commands?
3. Does the aggregate correctly create one stable, fail-closed required-check
   candidate?
4. Does the separate `b0.yaml` adequately preserve the existing RustSec,
   format, Clippy, and other non-B0 jobs and their token authority unchanged?
5. Are permissions, fork behavior, cache treatment, evidence, retry, and
   rollback rules sufficiently fail-closed?
6. Does the handoff make the initial PR route, PR-head/base/synthetic-merge
   evidence, and procedural operations-owner assignment precise?
7. Are hosted stable-toolchain coverage, action SHA pinning, docs/security
   repair, and full-workflow green status correctly deferred?
8. May the owner confirm the proposed implementer/operations assignments and
   hosted-runner capacity after design acceptance?
9. Are the single-branch continuity preconditions objective and narrow enough
   to make post-push stop-work equivalent to a blocking B0 gate for this serial
   operating model?
10. Are the enforcement activation triggers and transition evidence complete
    enough to prevent concurrent work from beginning without merge-time B0?
11. May M1C/B0 and WS-03 close without current branch protection after this
    amendment is accepted and the amendment's resulting current `main` SHA is
    green, using run `29798727537` as accepted implementation evidence?
12. Is the temporary-red-`main` limitation sufficiently explicit for later
    workstreams to consume B0 safely?
13. Does `continuity-mode.md` make the serial prerequisites and enforcement
    classification sufficiently durable and auditable?
14. Is Section 8.3's two-stage RFC 000 lifecycle handback truthful and
    fail-closed through the final lifecycle SHA?
15. Is required-check activation unambiguously mandatory before concurrency
    while still requiring separate owner authorization for repository-rule
    mutation?

## 13. Sources

- `ROADMAP.md`, M1C / WS-03 / B0
- RFC 095 / D0
- RFC 096 / S1
- GitHub Actions event reference, `workflow_dispatch` default-branch
  limitation and `pull_request` merge-ref behavior:
  <https://docs.github.com/en/actions/reference/workflows-and-actions/events-that-trigger-workflows>
- GitHub Actions workflow syntax, permission defaults:
  <https://docs.github.com/en/actions/reference/workflows-and-actions/workflow-syntax>
- Existing workflow baseline: `.github/workflows/ci.yaml`
- Final implementation review:
  `.git-exclude/reviewed/016-rfc097-safe-hosted-ci-bootstrap-final-implementation-review-2026-07-21.md`
- Hosted evidence: `.git-exclude/evidence/097-safe-hosted-ci-bootstrap/`
- [GitHub Actions matrix jobs](https://docs.github.com/en/actions/how-tos/write-workflows/choose-what-workflows-do/run-job-variations)
- [dtolnay/rust-toolchain usage](https://github.com/dtolnay/rust-toolchain)
- [Swatinem/rust-cache usage](https://github.com/Swatinem/rust-cache)
