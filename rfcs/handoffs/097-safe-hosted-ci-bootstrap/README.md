# RFC 097 CI developer and operations handoff

## 1. Authority and entry conditions

The workflow implementation governed by
[`RFC 097`](../../proposed/097-safe-hosted-ci-bootstrap.md) has passed final
implementation review. Sections 3–5 retain its reviewed implementation
boundary and designed sequence; Section 6 governs current operations.

The original implementation entry conditions were:

1. independent architecture review accepts the RFC and this handoff;
2. project owner nabbisen approves implementation;
3. the owner confirms GitHub Actions is enabled and hosted Linux, macOS, and
   Windows capacity is available;
4. Codex implementation capacity and owner operations/review capacity are
   confirmed.

Neither past implementation acceptance nor this governance amendment authorizes
a commit, push, pull request, workflow dispatch, required-check change, merge,
release, or repository-rule mutation.

## 2. Role split

| Role | Assigned party | Authority |
|---|---|---|
| Primary CI developer | Codex, pending owner confirmation | Edit the reviewed local workflow/RFC boundary; run read-only/local checks; prepare review packages |
| Operations owner | nabbisen | Authorize provisional review transport; observe hosted workflows; change repository required checks |
| Maintainer reviewer | nabbisen | Confirm scope, attempts, and lifecycle/commit decisions |
| Independent reviewer | Architecture/security reviewer | Review design and implementation evidence; no edits during review |
| Hosted runner provider | GitHub-hosted Actions | Ubuntu, macOS, Windows execution; availability must be observed |

The developer must not infer operations authority from implementation
approval. Assigning an operation to nabbisen is a procedural control in this
handoff. GitHub users with sufficient repository permission may technically
perform some of the same operations; no exclusive integration authority is
claimed to be technically enforced without separate evidence.

The current operational mode is serialized-integration continuity, pending
acceptance of RFC 097's authority-model refinement. All `main` updates pass
through one designated integration authority. This is a procedural stop-work
control, not technical merge enforcement.

## 3. Reviewed implementation boundary

The reviewed tracked implementation boundary was:

- new `.github/workflows/b0.yaml`
- `rfcs/proposed/097-safe-hosted-ci-bootstrap.md`
- `rfcs/handoffs/097-safe-hosted-ci-bootstrap/README.md`
- `rfcs/README.md`

The reviewed ignored review/evidence boundary was:

- `.git-exclude/evidence/097-safe-hosted-ci-bootstrap/`
- `.git-exclude/review-requests/...`

Do not change:

- `.github/workflows/ci.yaml`, including its triggers, jobs, permissions,
  conclusions, and RustSec token behavior;
- Rust product or test code;
- `Cargo.toml`, `Cargo.lock`, MSRV, or dependencies;
- release/MSIX workflows;
- documentation build configuration;
- branch protection through code or API;
- action vendors or supply-chain policy;
- product environment variables.

Any required expansion stops implementation for RFC amendment and re-review.

## 4. Reviewed developer change map

### 4.1 Workflow triggers and authority

The implementation created `.github/workflows/b0.yaml` with:

- name it `B0 Hosted Bootstrap`;
- trigger on `push` and `pull_request` for `main`, plus
  `workflow_dispatch`;
- declare workflow-level `permissions: contents: read`;
- add per-change concurrency with cancellation of obsolete runs;
- do not add secrets or write permission.

The `pull_request` event was the designed initial hosted-validation route.
Manual dispatch was unavailable until this workflow existed on the default
branch and could not bootstrap the implementation review. The actual
direct-push deviation is recorded in Section 5 and RFC 097 Section 8.1.

### 4.2 B0 matrix

The implementation added one `b0-platform` matrix to `b0.yaml`:

| Matrix label | Runner | Toolchain | Command |
|---|---|---|---|
| Linux | `ubuntu-latest` | 1.91 | `cargo test --workspace --locked` |
| macOS | `macos-latest` | 1.91 | `cargo test --workspace --locked` |
| Windows | `windows-latest` | 1.91 | `cargo test --workspace --locked` |

Required properties:

- job name `B0 / Rust 1.91 / <label>`;
- `timeout-minutes: 90`;
- `strategy.fail-fast: false`;
- no matrix/job `continue-on-error`;
- checkout at the triggering commit;
- minimal Rust 1.91 install;
- OS/toolchain-differentiated cache with cache step only allowed to continue on
  error;
- version and metadata observation before tests;
- no `--test-threads=1`;
- no workflow-level state/home redirection.

### 4.3 Aggregate

The implementation added `b0-gate`:

- visible name exactly `B0 / gate`;
- Ubuntu runner;
- `needs: [b0-platform]`;
- `if: always()`;
- five-minute timeout;
- no checkout/cache/secrets;
- exit success only when the matrix result is exactly `success`.

Do not aggregate unrelated jobs into B0.

### 4.4 Existing CI non-interference

Do not edit `.github/workflows/ci.yaml`. B0 does not change or accept its
stable test, format, Clippy, MSRV, Windows GUI, RustSec, i18n, visual-status,
or docs jobs. In particular, the read-only permission in `b0.yaml` has no
effect on the existing RustSec job's automatic-token permissions.

## 5. Designed developer sequence

This is the accepted design sequence retained for traceability. The owner used
direct pushes for the implementation and corrections, so steps 7–10 were not
performed through the designed pull-request route. RFC 097 Section 8.1 and the
final implementation review record that deviation.

1. Re-read the accepted RFC and review result.
2. Record the owner-approved baseline and preserve unrelated worktree changes.
3. Add `b0.yaml` in one focused patch; do not edit `ci.yaml`.
4. Check YAML structure with an available local parser/tool, noting that the
   GitHub workflow parser is authoritative.
5. Run focused scans from RFC 097 Section 6.3.
6. Observe `git diff --exit-code -- .github/workflows/ci.yaml` succeed.
7. If Rust 1.91 is locally installed, run:

   ```sh
   cargo +1.91 test --workspace --locked
   ```

   A local pass is preflight only.
8. Run `git diff --check`.
9. Create a draft implementation-review package before any commit, push, pull
   request, or dispatch.
10. Stop for the owner's separate review-transport authorization.

Do not run repository-wide mutating format for this YAML-only implementation.

## 6. Operations procedure

Only the operations owner performs or explicitly authorizes these steps.

**Independent update authority** means project authorization for a person,
bot, merge queue, or other mechanism to update `main` without routing that
update through the designated integration authority and its exact-current-SHA
B0 wait. Technical GitHub permission or capability alone does not grant that
project authority. A technically capable but independently unauthorized actor
must follow the designated serialized-integration procedure.

### 6.1 Independent-authority enforcement pull-request route

- Confirm Actions is enabled for the repository.
- Confirm hosted runner/minutes access for all three runner labels.
- Confirm no release, deployment, environment, or secret permission was added.
- Owner explicitly authorizes a provisional local review commit, a
  same-repository review-branch push, and a pull request targeting `main`.
- Record reviewed PR head SHA `H`, target base SHA `B`, branch, and PR
  number/URL.
- Treat this authorization only as review transport. It does not authorize
  merge, B0 closure, release, or a repository-rule change.

The `pull_request` event triggers the run automatically. This route was not
used for the already accepted implementation evidence. It becomes mandatory
before independently authorized integration begins. Do not use
`workflow_dispatch` as a substitute for evidence from the actual merge-control
path.

### 6.2 Observe an independent-authority enforcement run

Use the GitHub UI or, when the owner authorizes CLI use:

```sh
gh pr view <pr-number> \
  --json number,url,headRefName,headRefOid,baseRefName,baseRefOid,potentialMergeCommit
gh run list --event pull_request --branch <review-branch> \
  --json databaseId,workflowName,event,headBranch,headSha,status,conclusion,url
gh run watch <run-id> --exit-status
gh run view <run-id> \
  --json databaseId,url,event,headBranch,headSha,attempt,status,conclusion,jobs
```

These examples are operator commands, not pre-authorization for the agent to
push, dispatch, rerun, or change repository settings.

Record:

- run ID and URL;
- event and attempt;
- PR number/URL, review branch, PR head SHA `H`, and base SHA `B`;
- run `headSha`, which is synthetic merge SHA `M` when GitHub executes the
  normal `refs/pull/<number>/merge` path;
- confirmation that `H` and `B` were current for the PR when `M` was
  generated, and that the run was not cancelled or superseded;
- each matrix job's runner label/image, observed Rust/Cargo, duration, and
  conclusion;
- `B0 / gate` conclusion.

Do not paste tokens, full environment dumps, or canary values.

All B0 jobs and `B0 / gate` must belong to the same run and tested
`headSha`. A result for an older PR head/base pair is not acceptance evidence.
Do not infer the event-time `B` from a later `baseRefOid` after the target
branch has moved. Preserve the contemporaneous `potentialMergeCommit`,
workflow-run API association, or equivalent durable event evidence connecting
`M` to `H` and `B`.

### 6.3 After default-branch adoption

After accepted implementation review, the owner may merge through the normal
reviewed path. Observe the resulting default-branch `push` run and verify its
`headSha` is the merged default-branch commit.

Only after `b0.yaml` exists on the default branch may a sufficiently
permissioned user manually dispatch it. When the operations owner chooses that
route:

```sh
gh workflow run b0.yaml --ref <default-branch-or-reviewed-ref>
gh run list --workflow b0.yaml --event workflow_dispatch
```

These commands are examples, not standing authorization.

### 6.4 Required-check operation

Only after an accepted implementation review and a green current run:

1. confirm whether the repository's actual merge-control path uses ordinary
   pull requests or a merge queue;
2. if a merge queue is required, stop and amend/re-review RFC 097 for a
   `merge_group` trigger before configuring the check;
3. inspect the exact status context exposed by GitHub;
4. add exactly `B0 / gate` to the owner-selected required checks applying to
   `main`;
5. do not remove existing required checks without a separate owner decision;
6. record timestamp and resulting rule summary without secrets;
7. verify a subsequent pull-request run reports the context and that the
   repository's actual merge-control path requires it; and
8. obtain independent acceptance of the rule snapshot and merge-path evidence
   before independently authorized integration resumes.

The required-check change is not necessary to prove the workflow code works,
but its observed configuration is required before independently authorized
integration starts or B0 is claimed as an enforced merge control.
Independently authorized integration may resume only when the applicable rule
requires exactly `B0 / gate`, applies to `main` without weakening unrelated
checks, the actual merge-control path emits the context, and independent review
accepts that evidence. This procedure does not authorize repository-rule
mutation; the operation still requires separate explicit owner authorization.

An ordinary-PR verification branch and PR remain non-integrating review
transport. They do not authorize merge or independently authorized integration
before the evidence above is accepted.

### 6.5 Serialized-integration continuity

This mode is available while every update to `main` passes through one
designated integration authority and no person, bot, merge queue, or mechanism
is authorized to update `main` independently. Multiple developers, branches,
and pull requests are permitted when that authority integrates their updates
one at a time. Technical GitHub capability is governed as defined above; it is
not itself independent project authority.

For every pushed `main` SHA, the operations owner:

1. records the exact SHA and corresponding B0 run;
2. observes all three platform cells and `B0 / gate` before another unrelated
   integration or downstream continuation proceeds;
3. stops unrelated implementation pushes, downstream gate consumption, and
   release/tag/publish operations if the run fails, is cancelled, is missing,
   or tests another SHA;
4. preserves failures and corrects deterministic defects at a new SHA; and
5. records or updates `continuity-mode.md` as specified in Section 7.

Only diagnosis, correction, and evidence work may proceed during stop-work.
Before any independent updater is authorized, stop and complete Section 6.4.
If a merge queue is selected, first amend and re-review RFC 097 for
`merge_group`.

## 7. Evidence checklist

Populate `.git-exclude/evidence/097-safe-hosted-ci-bootstrap/`:

- `environment.md`: local baseline/tooling and hosted runner identities;
- `local-preflight.log`: commands, exits, counts, and limitations;
- `workflow-validation.md`: parser/trigger/permission/concurrency result;
- `hosted-runs.md`: every attempt and URL, event, PR head/base, run
  `headSha`/synthetic merge, and current-versus-obsolete disposition;
- `matrix-results.md`: exact three-cell results;
- `isolation-results.md`: RFC 096 focused case names per OS, without tokens;
- `continuity-mode.md`: B0-closure entry and a new entry at every later
  workstream boundary;
- `focused-scans.log`: raw requested scan output;
- `scope.diffstat`: tracked boundary and ELOC assessment.

The implementation-review package is the architect's entry point and links
these artifacts in review order.

Each `continuity-mode.md` entry records:

- timestamp and workstream or lifecycle-transition identifier;
- exact current `main` SHA, B0 run ID, attempt, event, run `headSha`, and
  `B0 / gate` conclusion;
- designated integration authority and confirmation that every update to
  `main` remains serialized through it;
- whether another person, bot, merge queue, or mechanism has independent
  authority to update `main`;
- observed repository-rule/enforcement state and whether the barrier is
  procedural or technical; and
- decision: serialized mode remains valid, or enforcement activation blocks
  entry.

Do not record or require developer count, branch count, or a sole-developer
disclaimer. Do not record credentials, tokens, or secret configuration. A
missing or stale entry blocks B0 closure or later workstream entry.

## 8. Failure and retry procedure

1. Preserve the first failing run.
2. Classify the failure using RFC 097 Section 7.
3. Do not rerun deterministic compile, test, package-name, YAML, or isolation
   failures merely to seek green.
4. For a corroborated GitHub runner/network incident, the owner may authorize
   one rerun.
5. Record both attempts and the corroborating GitHub status incident or runner
   message.
6. If any OS fails twice or fails deterministically, B0 remains red.
7. Never exclude an OS, add `continue-on-error`, serialize tests, update MSRV,
   or change dependencies as an unreviewed repair.

## 9. Rollback

Before required-check configuration, revert the focused workflow patch through
normal review.

After required-check configuration:

- preserve failure evidence;
- keep the context reporting while repairing;
- if context disappearance deadlocks all reviewed work, the owner may remove
  only the newly added `B0 / gate` requirement temporarily;
- record reason/timestamp and restore it after repair;
- never disable all protection, hide a failed cell, or call rollback a pass.

## 10. Stop and escalate conditions

Stop and request RFC amendment/re-review if:

- a product-code or dependency change appears necessary;
- Rust 1.91 cannot install or compile a workspace dependency;
- any hosted OS requires exclusion or special product behavior;
- RFC 096 isolation fails;
- a new action vendor, secret, write permission, self-hosted runner, or
  privileged event appears necessary;
- the aggregate cannot distinguish failed/cancelled/skipped cells;
- owner operations capacity or hosted-runner access is unavailable;
- repository-rule changes would weaken unrelated protections;
- implementation overlaps another workflow edit.

Also stop before another person, bot, merge queue, or mechanism gains
independent update authority. That transition requires the
enforcement evidence in Section 6.4 and independent review before independently
authorized integration resumes.

## 11. Completion handback

The developer hands back:

- the focused workflow diff;
- local validation and scan results;
- all hosted run references supplied by the operations owner;
- explicit red/non-B0 results;
- an implementation-review entry package.

For serialized-integration continuity closure, the handback also includes the
accepted governance amendment and exact green push evidence for the
then-current `main` SHA after amendment integration, plus its
`continuity-mode.md` closure entry.

The owner then performs RFC 097 Section 8.3's separately reviewed mechanical
lifecycle handback: move RFC 097 to `done/`, update its status, the RFC index,
affected links, and the roadmap completion state, and integrate those records.
The resulting final `main` SHA must pass all B0 cells and `B0 / gate`; that
result is added to `continuity-mode.md` before B0/WS-03 closes or unrelated
work continues.

For independent-authority enforcement handback, the record instead includes
the required-check rule snapshot and verification on the actual merge-control
path.

No commit message is requested until independent implementation review accepts
B0 and the owner approves the commit point.
