# RFC 102 — B0 trigger scope: developer handoff

Companion to [`RFC 102`](../../proposed/102-b0-trigger-scope-and-ci-disposition.md).
The RFC records what was decided and why; this records how to implement and
verify it. It must not override the RFC.

## 1. Authority and entry conditions

Begin only after design review accepts RFC 102 **and** the owner explicitly
approves implementation. `main` green, working tree clean.

This changes CI behaviour, so it is verified by running it, not by inspection.
Expect two hosted pushes: one touching only docs, one touching `crates/**`.

## 2. Role split

| Role | Party | Scope |
|---|---|---|
| Implementer | mid-capability model | T1–T4 |
| Integrator | nabbisen | Commits, pushes, observes both verification runs |
| Architect | RFC 102 author | Consulted if a path's classification is unclear |

## 3. Mandatory boundary

**Only** `.github/workflows/b0.yaml` and `.github/workflows/ci.yaml`.

**Must not:** change the B0 command, matrix, toolchain, permissions,
concurrency, or timeouts; add a marketplace action (RFC 097 §5.2 defers pin
policy to WS-07/C0 — use `git`); touch product code, tests, manifests, or
`release.yaml` / `windows-msix.yaml`; delete any `ci.yaml` job.

## 4. T1 — change-detection job

Add a `changes` job to `b0.yaml`, before `b0-platform`, running on
`ubuntu-latest` with a short timeout. It outputs a single boolean, e.g.
`code`, meaning "this push touched something outside the inert set."

Determine the diff with `git`, not an action. Checkout needs enough history to
compare — `fetch-depth: 0` or an explicit fetch of the base commit.

**Inert paths** (from RFC 102 §5.1 — skip permitted only if *every* changed file
matches):

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

**This is a denylist and the direction is load-bearing.** Anything not listed —
including any new file at any new path — must yield `code=true`. Never invert
this into a match list: a future root `build.rs` or new crate directory would
then silently skip B0.

`crates/**`, `Cargo.toml`, `Cargo.lock`, and `b0.yaml` itself must never be
ignorable. Note `crates/aaai-gui/locales/*.yaml` are build inputs — `rust-i18n`
is a compile-time macro — and are already covered by `crates/**`.

**Fail safe on every ambiguity.** Set `code=true` when: the event is
`workflow_dispatch` or `schedule`; `github.event.before` is absent or all
zeroes; the before-commit is not an ancestor of the head; or the diff command
fails for any reason. **An unresolvable diff must never produce a skip.**

## 5. T2 — make the matrix conditional, keep the gate unconditional

Gate `b0-platform` on `needs.changes.outputs.code == 'true'`.

`b0-gate` keeps `if: ${{ always() }}` and `needs: [changes, b0-platform]`. Its
check becomes: succeed when the platform result is `success`, **or** when it is
`skipped` **and** `changes.outputs.code == 'false'`. Fail otherwise.

It must still fail for `failure`, for `cancelled`, and for `skipped` arising
from any other cause — a matrix skipped because a dependency failed is not a
pass. Log which branch was taken so the run is self-explaining.

Keep the job name exactly **`B0 / gate`**. It is the required-check candidate
and RFC 097 §5.3 requires a stable status context.

## 6. T3 — weekly schedule

Add a `schedule:` trigger (weekly) to `b0.yaml` for hosted-image drift, per
RFC 102 §5.3. Scheduled runs always execute the full matrix — §4 already forces
`code=true` for `schedule`.

## 7. T4 — park `ci.yaml`

Reduce its triggers to `workflow_dispatch` only. Add a header comment recording
that automatic triggers are parked under RFC 102 pending C2, and that C2 owns
restoring them.

**Do not delete, disable, or edit any job.** C2 needs them intact.

## 8. Verification — this is behavioural, so run it

Static checks first:

```sh
git diff --check
git diff --stat        # exactly two files
```

Then two hosted pushes, in this order:

**V1 — docs-only.** Push a commit touching only a `.md` file.
Expect: `changes` success with `code=false`; **`b0-platform` skipped**;
**`B0 / gate` success**. Record the billable-equivalent minutes and compare
against the ~15 min baseline in RFC 102 §2.

**V2 — code.** Push a commit touching `crates/**`.
Expect: `changes` success with `code=true`; full three-OS matrix; gate reflects
it. Counts unchanged — 144 / 8 / 91 / 27 / 3 on Linux and macOS, 132 / 8 / 89 /
27 / 3 on Windows.

Also confirm no CI workflow run appears for either push.

**If V1 shows `B0 / gate` as anything other than success, stop.** That is the
invariant RFC 097 depends on, and a skipped-but-not-green gate is a stop-work
condition rather than a cost saving.

## 9. Evidence

Create `.git-exclude/evidence/102-b0-trigger-scope/`:

```
workflow-diff.md      the two files, before and after
verification-runs.md  V1 and V2 — run IDs, per-job conclusions, changes output
cost-comparison.md    billable-equivalent minutes before and after, docs-only push
```

## 10. Stop and escalate

- V1's gate is not green.
- A path's classification is unclear — ask rather than guess; a wrong entry here
  silently disables the gate.
- Change detection cannot be done without a marketplace action.
- Any `ci.yaml` job would have to change to park its triggers.
- The skip path cannot be distinguished from a dependency-failure skip.

## 11. Rollback

Both files revert independently. Reverting `b0.yaml` restores
run-on-every-push; reverting `ci.yaml` restores its automatic triggers. Neither
affects product code, so rollback carries no test risk.
