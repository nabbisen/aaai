# RFC 098 platform developer and adversarial-QA handoff

## 1. Purpose and authority

This is the required WS-04 execution companion to
[`RFC 098`](../../proposed/098-selected-folder-and-symlink-policy.md). The RFC
decides what the selected-folder boundary means; this handoff defines how a
developer and platform QA operator implement and prove it.

This handoff cannot override the RFC. A conflict, public API break, skipped
required platform case, link-following behavior, or ambient-path fallback stops
work for RFC amendment and independent re-review.

Implementation entry requires:

1. accepted independent architecture/security review of the RFC and handoff;
2. explicit project-owner approval to implement;
3. confirmed implementation, maintainer, security-review, and adversarial-QA
   capacity; and
4. confirmed GitHub-hosted Linux/macOS/Windows fixture capacity.

No entry condition authorizes commit, push, branch/rule mutation, release, or
publication.

## 2. Role split

| Role | Proposed party | Responsibility |
|---|---|---|
| Primary developer | Codex | Focused local implementation, tests, evidence, review package |
| Maintainer/integration authority | nabbisen | Scope review, authorization, serialized integration, B0 observation |
| Independent design reviewer | Independent architect/security reviewer | Design acceptance and focused rereview |
| Independent implementation reviewer | Independently assigned reviewer; may differ from the design reviewer | Implementation/evidence acceptance using the self-contained implementation review package |
| Adversarial QA owner | nabbisen | Confirm fixture feasibility and hosted evidence completeness |
| Runner provider | GitHub-hosted Actions | Required Linux/macOS/Windows execution |

The owner confirmed these assignments and capacities on 2026-07-22. Actual
runner/fixture behavior must still be observed; an unavailable required case
stops implementation acceptance rather than weakening the assignment.

## 3. Mandatory implementation boundary

Expected tracked files include:

- workspace/engine `Cargo.toml` entries and `Cargo.lock` for reviewed
  capability dependencies;
- `crates/aaai/src/diff/engine.rs` and new focused private modules as needed;
- `crates/aaai/src/diff/tests.rs` plus focused platform fixture modules;
- affected CLI integration tests;
- narrowly affected GUI state/error tests and locale keys if actionable root
  presentation changes;
- RFC 098/handoff implementation checklists only after observed evidence; and
- `rfcs/README.md` only at the separately reviewed lifecycle transition.

Do not change without RFC amendment/re-review:

- public `DiffEntry` fields or `DiffEngine::compare*` signatures;
- CLI command inventory or add a follow-links flag;
- audit-definition schema;
- report/export encoding or masking policy owned by WS-05;
- B0 workflow, legacy CI workflow, MSRV, release/package files;
- audited-root contents; or
- project no-`unsafe` policy.

Keep unrelated dirty worktree changes intact. Do not run a repository-wide
mutating formatter.

## 4. Developer sequence

### 4.1 Baseline and dependency checkpoint

1. Record `HEAD`, `main`, `origin/main`, worktree state, and exact accepted RFC
   review.
2. Reconfirm current B0 continuity before implementation begins.
3. Resolve the reviewed capability dependency versions without broad updates.
4. Record direct/transitive delta, licenses, sources, MSRV behavior, duplicate
   versions, advisories, and `cargo deny` findings.
5. Re-run the reviewed compile-only probe under Rust 1.91 for Windows, then
   prove the exact root/child/file handle-and-attribute mechanism at runtime on
   the hosted Windows fixture set.

Stop if the dependency cannot supply sandboxed directory-relative resolution,
single-component no-follow final opens, Windows
`FILE_FLAG_OPEN_REPARSE_POINT` plus all-reparse attribute rejection, or
supported-platform behavior without project `unsafe`.

### 4.2 Engine migration

1. Introduce a private `SelectedRoot`-equivalent wrapper.
2. Open/reject roots according to RFC 098 Section 6.1, using an atomic
   no-follow directory open for the final component rather than a metadata/
   ordinary-open pair.
3. Replace `WalkDir`/ambient descendant `PathBuf` collection with retained
   parent/child directory capabilities and single-component enumeration/open.
4. Key observations by native relative identity.
5. Open each child directory as one final name with no-follow, reject Windows
   reparse attributes and Unix xdev before conversion/enumeration, and retain
   the accepted child capability.
6. Store each regular observation with its retained parent capability, one
   final native name, and stable identity. Open only that single name with
   no-follow/nonblocking where applicable; reject reparse/special/identity
   change and read/hash only from the validated returned handle.
7. Map link, reparse, xdev, special, race, metadata, and read outcomes to the
   stable issue contract.
8. Preserve ordinary-file sorting, hashes, text/binary classification,
   progress, and ignore semantics.

On Unix, root and child-directory opens must use a directory-required
no-follow primitive such as `open_dir_nofollow` (`O_DIRECTORY | O_NOFOLLOW`)
or an equivalent sequence that cannot block on a raced FIFO before type
classification. Do not use a plain blocking read-open followed only by a
metadata check.

On Windows, capability confinement, `FILE_FLAG_OPEN_REPARSE_POINT`, returned-
handle reparse rejection, and same-handle reads are the security controls.
`MetadataExt::{dev, ino}` is diagnostic/race evidence only: the abstraction's
64-bit inode does not represent every possible 128-bit ReFS identifier. Do not
claim complete Windows identity from it. If adequate identity is unavailable,
fail closed or stop under the RFC amendment rule; hosted NTFS evidence does not
establish ReFS behavior.

Focused scans must show no production descendant use of `std::fs::read`,
`Path::is_dir`, `Path::canonicalize`, or a later absolute join/open inside the
new traversal path.

### 4.3 Frontend convergence

- Exercise every `DiffEngine` caller inventory.
- Map audit root failures to Error/exit 3.
- Ensure CLI human output and GUI guided/expert surfaces expose the same path,
  `Incomparable`/`Unreadable` classification, and safe detail where those
  surfaces already support it. Existing structured surfaces retain their
  current fields; do not add an unversioned raw-diff issue field before WS-05.
- A failed GUI run retains prior valid data only if clearly stale and never
  writes history or a report as a new success.
- No frontend reopens a result path.

## 5. Fixture construction

Every test owns a temporary before root, after root, and sibling deny/canary
root. Canary names and content are synthetic unique markers. Snapshot all three
before and after.

Required fixture families:

| Family | Required variants |
|---|---|
| File links | absolute/relative, inside/outside, broken, Added/Removed/both |
| Directory links | inside/outside, descendant canary, ignored, cycle/self-cycle |
| Windows reparse | file symlink, directory symlink, junction, and mount-point or equivalent name-surrogate; every reparse attribute rejects through the production handle path. Non-name-surrogate/unknown is deferred — no constructible hosted fixture (RFC §9.1 platform constraint; WOF is client-only) — and is a named S2 exit obligation, not a waiver |
| Unix special | FIFO and socket; classification must not block/open |
| Root | direct directory/anchor succeeds; final-component link rejects with no trailing-slash or `/.` bypass; terminal `..` rejects |
| Mutation | barrier replaces file and directory after classification with outside link; Unix directory-to-FIFO replacement must fail without blocking before classification |
| Names | `%`, literal Unix backslash, controls; invalid Unix bytes in the Unix formatter and Linux filesystem runtime (APFS rejects their creation); unpaired UTF-16 formatter unit |
| Collision | two filesystem-native names whose former lossy renderings collide on Unix at filesystem level; Unix uses literal `back\slash` versus nested `back/slash`. On Windows, collision-freedom is evidenced through the unpaired-UTF-16 formatter unit (see RFC §9.1 platform constraint); a filesystem-level pair is not constructible there |
| XDEV | mandatory real production-path differing-device child on Linux and macOS; actual returned-handle metadata, `AAAI-PATH-XDEV`, and zero child enumeration; no mocked substitute/skip |

The test-only barrier is injected at the production classification/open
boundary. No sleep-based race test is accepted. A runner's inability to create
a required link/reparse fixture is evidence of a blocked platform case, not a
pass or silent skip.

## 6. Required assertions

For each applicable case, record:

- root-relative escaped identifier;
- `DiffType` and stable issue code;
- audit status and CLI exit;
- whether the path was ignored;
- descendant enumeration count;
- outside canary non-disclosure scan over structured results and captured
  stdout/stderr;
- before/after stable-field snapshots: bytes, namespace/entry set, object kind,
  size, permissions, modification time, and identity where supported; exclude
  access time and documented volatile filesystem metadata; and
- platform/toolchain identity.

Never put real user paths, environment dumps, link targets, tokens, or secrets
in evidence. Synthetic canary values may be recorded only as absence-check
identifiers, not as examples of real secret material.

## 7. Validation commands

Run and preserve exits/output for the accepted implementation boundary:

```sh
cargo +1.91 test -p aaai --lib diff
cargo +1.91 test -p aaai-cli --test cli
cargo +1.91 test --workspace --locked
cargo +1.91 check --workspace --all-targets --locked
cargo tree --locked
cargo audit
cargo deny check
git diff --check
```

Also run focused source scans for ambient descendant access, link-follow flags,
unsafe code, ignored/skipped platform tests, and changed public signatures.
Known C0/C2 findings remain later work unless this dependency delta introduces
a new finding; new findings require disposition before WS-04 acceptance.

After owner-controlled integration, B0 must pass on the exact current `main`
SHA. All three matrix cells and `B0 / gate` belong to one run. A red/missing or
different-SHA B0 stops downstream continuation under the current serialized
integration contract.

## 8. Evidence package

Populate `.git-exclude/evidence/098-selected-folder-and-symlink-policy/` with:

- `environment.md` — baseline, toolchains, runner identities, fixture
  capabilities;
- `dependency-delta.md` — versions, licenses, transitive delta, MSRV and audit
  disposition;
- `threat-model-matrix.md` — case-to-invariant-to-test map;
- `local-results.md` — commands, exits, counts, limitations;
- `platform-results.md` — Linux/macOS/Windows cases and outcomes;
- `race-results.md` — deterministic barrier cases;
- `non-disclosure-results.md` — canary/path/hash absence checks;
- `non-mutation-results.md` — before/after snapshots;
- `focused-scans.log` — raw scoped source scans;
- `hosted-runs.md` — B0 run/SHA/job identity and every attempt;
- `continuity-reference.md` — identifies the WS-04 entry appended to the
  single governing record at
  `.git-exclude/evidence/097-safe-hosted-ci-bootstrap/continuity-mode.md`; do
  not create an RFC 098 `continuity-mode.md`; and
- `scope.diffstat` — tracked change boundary.

The implementation review request is the architect's entry point. It must
reference RFC 098, this handoff, the accepted design review(s), the exact
implementation diff, and every evidence file needed to reproduce the claim.
The implementation reviewer may differ from the design reviewer and must not
need prior conversation context.

## 9. Stop and escalation conditions

Stop and request an RFC amendment when:

- a link/reparse target must be followed;
- capability containment or final no-follow cannot be proved on any supported
  OS;
- project `unsafe` or native FFI appears necessary;
- the public engine shape or stable CLI contract must break;
- a required platform fixture would be skipped/ignored;
- outside content/path/hash appears anywhere in results;
- ignore negation behavior changes;
- a new dependency finding lacks an accepted disposition; or
- implementation expands into WS-05, WS-10, persistence, CI, or release work.

Rollback before integration is the focused patch reversal by the owner. After
integration, a deterministic failure is corrected at a new SHA and re-reviewed;
do not rerun an unchanged deterministic failure as acceptance evidence.
