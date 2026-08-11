# RFC 098 — Selected-folder and symlink policy

**Status.** Implemented — **released in `v0.41.0`** (2026-08-10, release
unit 1). §9.1's adversarial matrix is discharged on all three hosted platforms,
audited case by case in
`.git-exclude/reviewed/065-rfc098-disposition-audit-2026-08-10.md`. The
**cross-root-link case** — unowned since review 045 reassigned it from this RFC
to WS-05, where RFC 103 did not pick it up — closed 2026-08-10 and is proven
load-bearing.

**§14's ten confirmations** were satisfied by design re-review 025 ("Accept with
notes; Go for owner-authorized implementation") plus owner authorization and
release, not by enumerated sign-off. Item 3 — *no v1 follow-links option,
including for inside-root targets* — is a **live constraint**, not settled
history: it is what the cross-root test guards, and what RFC 101 would collide
with if it revisits the workspace.

**Outstanding, non-blocking:** a Windows twin of the cross-root test, for §9.1's
"every hosted OS" symmetry. No substantive gap — the Windows reparse decision is
tag-agnostic and never consults the link target — assigned to the dev team.

**Gate S2 does not close with this RFC.** Its threat-model half and RFC 104's
GUI export masking remain.

**Tracks.** `ROADMAP.md` M2 / WS-04 / gate S2

**Depends.** RFC 095/D0, RFC 096/S1, and RFC 097/B0

**Decision owner.** nabbisen, project owner

**Design agent.** Codex

**Required handoff.**
[`rfcs/handoffs/098-selected-folder-and-symlink-policy/README.md`](../handoffs/098-selected-folder-and-symlink-policy/README.md)

**Touches.** The shared diff engine's root acquisition, traversal, path
identity, file opening, path-local error semantics, CLI/GUI consumption, and
Linux/macOS/Windows adversarial evidence. It may add narrowly selected
capability-filesystem dependencies. It does not implement WS-05 output
encoding/masking, WS-10 scale work, or a follow-links mode.

## 1. Summary

aaai treats each user-selected folder as an explicit read capability, not as
an ambient path prefix. Descendant discovery and content reads must remain
relative to an already-open root directory handle. No descendant operation may
re-enter `std::fs` through an ambient absolute path.

For v1, symbolic links and equivalent link-like indirections are
**metadata-only and fail closed**:

- aaai identifies the link entry without following it;
- aaai never traverses a linked directory or reads a linked file target;
- any comparison involving a link-like entry is `Incomparable` and therefore
  an audit `Error`;
- ignore rules may omit the entry, but cannot make aaai follow it; and
- there is no CLI flag, GUI setting, project option, or definition field that
  enables following links.

The same fail-closed rule applies to special files and cross-filesystem
descent. Regular-file content is opened as one native final name through its
retained parent-directory capability; the returned handle is checked for
link/reparse, type, device, and identity before that same handle is read. This
closes both the currently observed cross-root read and the intermediate/final
replacement races that a simple `canonicalize`/`starts_with` repair or a
multi-component root-relative reopen would leave open.

The RFC preserves the current public `DiffEntry` shape and existing behavior
for ordinary Unicode files and directories. It adds internal native path
identity, deterministic escaped display for otherwise ambiguous paths, stable
path-issue codes, and one shared engine contract for CLI and both GUI surfaces.

## 2. Motivation and observed defect

The current engine:

1. walks each root with `WalkDir`;
2. converts every descendant to a lossy, slash-replaced `String` key;
3. stores an ambient `PathBuf`; and
4. later calls `std::fs::read(path)`.

`WalkDir` does not descend into directory symlinks by default, but it records
the symlink itself. The later `Path::is_dir` and `std::fs::read` calls follow
that entry. A link inside a selected folder can therefore read and hash a file
outside the selected folder. The content can then reach diff, GUI, CLI, audit,
report, or export flows.

Replacing this with `canonicalize(path)` followed by a lexical prefix check is
not sufficient. An attacker or concurrent producer can replace a checked
entry before the subsequent absolute-path read. Lossy string keys also permit
distinct non-Unicode names to collapse to the same displayed identity, while
replacing every backslash with `/` changes a valid Unix filename.

An asset-integrity auditor must not silently widen the input authority it was
given. An uninspectable entry is an explicit Error, not implicit permission to
read elsewhere or a path that disappears without explanation.

## 3. Authority and compatibility

RFC 095 supplies these controlling invariants:

- audited folders are read-only inputs;
- one engine supplies identical CLI and GUI verdict semantics;
- selected roots may not be escaped by input-selection behavior;
- folder comparison includes `Unreadable` and `Incomparable`; and
- actual CLI contracts and the public engine API are v1 compatibility
  surfaces.

RFC 097 supplies the hosted Linux/macOS/Windows Rust 1.91 B0 matrix. WS-04
must run its adversarial suite through that matrix before S2 evidence can be
consumed by WS-05.

This RFC does not change the fields of public `DiffEntry`, remove a public
method, or add a required argument to `DiffEngine::compare*`. Ordinary UTF-8
root-relative paths keep their current forward-slash spelling. New behavior is
limited to unsafe or previously ambiguous filesystem cases.

## 4. Goals and non-goals

### 4.1 Goals

- Make each selected root a capability boundary for discovery and reads.
- Define link, reparse-point, mount, special-file, root-link, and mutation
  behavior on Linux, macOS, and Windows.
- Prevent outside-root content, hashes, filenames, and OS error paths from
  entering results.
- Give CLI, guided GUI, and expert GUI the same path classification and audit
  result.
- Preserve lossless internal path identity and collision-free user-visible
  identifiers.
- Define stable, actionable path-local issue codes for downstream WS-05
  rendering.
- Provide deterministic adversarial tests and evidence ownership.

### 4.2 Non-goals

- Following links that remain inside the selected root.
- An opt-in unsafe or compatibility mode.
- Showing or hashing link target text.
- Contextual Markdown/HTML/JSON/SARIF/CSV/TSV encoding or secret masking;
  WS-05 consumes the safe path/error contract defined here.
- Bounded-memory or tens-of-thousands optimization; WS-10 owns scale.
- Atomic definition persistence, dependency-wide remediation, formatting,
  release automation, or GUI redesign.
- Detecting that a regular file is also hard-linked from another namespace.

## 5. Threat model and security invariants

### 5.1 In scope

The selected tree may contain:

- absolute or relative file and directory symlinks;
- broken links, cycles, and chains;
- Windows file/directory symlinks, junctions, mount-point reparse points,
  name-surrogate reparse indirections, and non-name-surrogate reparse-backed
  entries such as provider placeholders;
- FIFOs, sockets, devices, or other non-regular nodes;
- names containing `%`, backslash, control characters, invalid Unix bytes, or
  unpaired Windows UTF-16 units;
- unreadable directories or files;
- a different filesystem mounted below a Unix root; and
- entries replaced between enumeration, classification, and open.

The process may have ambient permission to read the outside target. Protection
must therefore come from the engine's authority boundary, not from an expected
permission failure.

### 5.2 Required invariants

1. Descendant operations use only handles derived from the selected root
   capability.
2. No descendant absolute path is passed to ambient filesystem APIs.
3. A link-like final component is never opened for content.
4. A link-like directory is never descended.
5. A race may produce an explicit path issue or a read of the already-opened
   regular file, but never an outside-root read.
6. Error/result text contains only the safe root label, lossless
   root-relative display path, stable issue code, and bounded generic action;
   it does not embed link targets or ambient OS paths.
7. An unignored incomplete traversal never produces a passing audit; an
   explicit ignore may exclude the link entry but still grants no traversal.
8. Audited roots and outside canaries retain content bytes, namespace/entry
   set, object kind, size, permissions, modification time, and stable identity
   fields where supported after every test. Access time and documented
   volatile filesystem metadata are excluded from the non-mutation claim.

### 5.3 Trust boundary and limitations

The owner-selected root itself is ambient input. aaai rejects a selected root
whose final component is link-like and asks the user to select the physical
directory directly. Once opened, the directory handle—not a later
canonicalized pathname—is the root identity for that comparison.

Hard links are treated as regular files. A hard link is already a directory
entry inside the selected namespace, and portable filesystems do not expose an
authoritative “original path.” This limitation must be documented; it does not
permit symlink following or justify a broader claim that all alternate names
are prevented. Unix mount descent is rejected by device-boundary
classification.

On Windows, aaai deliberately rejects **every** entry carrying
`FILE_ATTRIBUTE_REPARSE_POINT`, not only name-surrogate tags. This includes
symlinks, junctions, mount points, and legitimate non-name-surrogate
reparse-backed files such as cloud/provider placeholders. They are not opened
for content or traversed. This over-rejection is the selected fail-closed v1
compatibility policy; supporting a subset of reparse tags requires a later
owner-approved RFC and new platform evidence.

## 6. Normative selected-folder policy

### 6.1 Root acquisition

For both before and after roots, the engine must:

1. reject an empty path;
2. remove trailing separators and terminal `.` components for root-selection
   purposes, reject a terminal `..`, and treat an actual filesystem anchor or
   the current-directory anchor as a direct directory authority;
3. split any remaining supplied path into its parent and effective final
   component and obtain the parent authority;
4. open that single final component once through the platform mechanism in
   Section 8.1, before conversion to a directory capability;
5. reject a Unix link, any Windows reparse point, or a non-directory final
   component from the returned-handle metadata;
6. obtain root metadata from the accepted directory handle;
7. retain the caller-supplied path only as a user-facing root label; and
8. perform all later operations relative to the capability.

The caller-selected root path is trusted authority input, but its final
component is not trusted to remain the class observed by a separate metadata
check. A separate `symlink_metadata` followed by an ordinary open does not
satisfy this rule. A trailing separator or `/.` must not turn a link final
component into an ordinary directory open. Root acquisition failure is an
operation-level comparison error. `aaai audit`
maps it to `ERROR`/exit 3; other CLI commands return their existing nonzero
command-error result. The GUI keeps the user on the selection/workspace
surface, shows a repair action, does not replace a prior valid result, and does
not append history or emit a report for the failed attempt.

### 6.2 Entry classes

The internal walker classifies entries without following the final component:

| Class | Treatment |
|---|---|
| Regular file | Retain its opened parent-directory capability and final native name; later open that single name once through Section 8.1, validate returned-handle kind/identity, and read the same handle |
| Directory | Open a single child name through Section 8.1, validate returned-handle kind/reparse/xdev state, retain that child capability, then enumerate it |
| Unix symbolic link | Record metadata-only issue; never read target text, open target, or descend |
| Any Windows reparse-point entry | Record `AAAI-PATH-REPARSE`; do not convert to a directory capability, read content, or distinguish an allowed tag |
| Cross-filesystem directory | Record boundary issue; do not descend |
| FIFO/socket/device/other special node | Record special-node issue; never open content |
| Metadata/open/read failure | Record an unreadable or changed-during-scan issue without an ambient path |

The walker must not use `Path::is_dir`, `Path::metadata`, `Path::canonicalize`,
`std::fs::read`, or another ambient descendant operation.

### 6.3 Comparison classification

| Before / after observation | `DiffType` | Audit consequence |
|---|---|---|
| regular / regular | Existing content comparison | Existing verdict behavior |
| directory / directory | `Unchanged` | Existing behavior |
| regular / directory or directory / regular | `TypeChanged` | Existing behavior |
| either side link-like | `Incomparable` | Always `Error` |
| either side has Windows `FILE_ATTRIBUTE_REPARSE_POINT` | `Incomparable` | Always `Error` |
| either side special or cross-filesystem | `Incomparable` | Always `Error` |
| either side cannot be read safely | `Unreadable` | Always `Error` |
| entry changes class during scan/open | `Incomparable` | Always `Error` |

This applies when the problematic path is Added, Removed, or exists on both
sides. A definition cannot approve an `Incomparable` or `Unreadable` entry;
the existing audit engine's error-first rule remains authoritative.

### 6.4 Ignore rules

Ignore matching uses the safe normalized root-relative display identity.
Ignored paths do not produce diff entries. Ignoring a link or special node
only suppresses that entry; it never authorizes traversal or content access.

Directory traversal may not be pruned solely because a directory currently
matches an ignore rule when a later negation could re-include a descendant.
This RFC preserves existing ordered “last matching rule wins” behavior. A
future ignore optimizer must prove equivalent negation semantics.

## 7. Path identity and reporting contract

### 7.1 Internal identity

Collection maps are keyed by native root-relative `PathBuf`/`OsString`, not a
lossy `String`. Separator normalization occurs only when creating a display or
machine identifier. Two native names must never overwrite one another because
their lossy renderings collide.

Paths are sorted by their normalized display identifiers, with native identity
as a deterministic tie-breaker. Ordinary Unicode paths retain their current
forward-slash output.

### 7.2 Escaped identifier

Each component is rendered without loss:

- `/` separates components;
- printable Unicode is retained;
- literal `%`, Unix literal `\`, and control characters use UTF-8 `%HH`
  escapes;
- invalid Unix bytes use `%HH` per byte; and
- unpaired Windows UTF-16 units use `%uHHHH`.

Hex digits are uppercase. This representation is deterministic and
collision-free for a platform. WS-05 must contextually encode this already-safe
identifier for each output format; it must not decode escapes before display.

### 7.3 Stable path issues

Path-local issues use these stable ASCII codes in `error_detail` followed by a
short generic message:

| Code | Meaning |
|---|---|
| `AAAI-PATH-LINK` | Link-like entry was not followed |
| `AAAI-PATH-REPARSE` | Windows reparse-point entry was rejected without processing it |
| `AAAI-PATH-SPECIAL` | Non-regular filesystem object was not opened |
| `AAAI-PATH-XDEV` | Traversal stopped at a filesystem boundary |
| `AAAI-PATH-RACE` | Entry kind/identity changed during safe open |
| `AAAI-PATH-METADATA` | Entry metadata could not be read |
| `AAAI-PATH-READ` | Safely opened regular file could not be read completely |

Messages include the root-relative escaped identifier through the entry's
existing `path` field, not by repeating an absolute path in `error_detail`.
Existing public fields remain intact. Codes are stable inputs to WS-05 and
WS-12 compatibility tests; prose after the code may be clarified without
changing machine meaning.

## 8. Implementation architecture

### 8.1 Capability adapter

Implementation uses the Bytecode Alliance capability filesystem family,
`cap-std` and `cap-fs-ext` 4.0.2, plus a target-Windows direct
`windows-sys` 0.60.2 dependency for the documented
`FILE_ATTRIBUTE_REPARSE_POINT` constant. All versions are pinned by
`Cargo.lock`; changing the capability family or its Windows open mechanics
requires design amendment and rereview. These crates supply safe Rust call
sites; RFC 098 adds no project `unsafe` or direct FFI.

Every descendant operation is a **single-component operation on an already
opened parent `Dir`**. Traversal retains parent/child directory capabilities;
an observed regular file retains an `Arc<Dir>`-equivalent parent capability,
its one native final name, and its observed stable identity. Content
comparison may not reopen a multi-component path from `SelectedRoot`.

On Unix, a final component is opened with `FollowSymlinks::No`; regular-file
opens also request nonblocking behavior before handle metadata confirms a
regular file, preventing a raced FIFO from blocking. The returned handle is
checked for object kind, root device, and observed `(dev, ino)` identity before
any content read. Child directories are checked for device equality before
enumeration.

On Windows, root-final, child-directory, and regular-file opens all use the
same concrete sequence:

1. call `Dir::open_with` on exactly one native name with
   `FollowSymlinks::No`, an explicit `FILE_FLAG_OPEN_REPARSE_POINT`, and
   `OpenOptionsMaybeDirExt::maybe_dir(true)`;
2. `FollowSymlinks::No` already causes `cap-primitives` 4.0.2 to add
   `FILE_FLAG_OPEN_REPARSE_POINT` before the OS open, so the final reparse
   point itself is opened rather than processed; supplying the flag explicitly
   additionally suppresses that crate's own name-surrogate short-circuit, so
   every reparse kind — name surrogate or not — is returned as a handle and
   rejected by this project's single all-reparse check;
3. obtain metadata from that returned handle;
4. reject the handle when
   `OsMetadataExt::file_attributes() & FILE_ATTRIBUTE_REPARSE_POINT != 0`;
5. only then check regular-file/directory kind and collect available
   volume/file identity for diagnostics; and
6. convert an accepted directory handle to `Dir`, or read an accepted regular
   file through that same handle.

The attribute check occurs before directory conversion, enumeration, content
I/O, hashing, target inspection, or another open. It intentionally rejects all
reparse tags. The compile-only Rust 1.91 Windows probe and its limitations are
recorded under
`.git-exclude/evidence/098-selected-folder-and-symlink-policy/windows-api-feasibility/`.
Runtime junction/reparse behavior remains mandatory hosted implementation
evidence; the compile probe is not acceptance evidence by itself.

Windows identity values are diagnostic, not confinement controls. In
particular, the selected metadata abstraction's 64-bit inode cannot represent
every 128-bit ReFS identifier. Security claims rely on retained capabilities,
suppressed normal reparse processing, returned-handle all-reparse rejection,
and same-handle reads. Implementation must not claim complete ReFS identity
from `(dev, ino)` or treat hosted NTFS evidence as ReFS proof; an identity-
dependent requirement that cannot fail closed stops for amendment.

The implementation review must pin the resolved dependency versions in
`Cargo.lock`, prove Rust 1.91 compilation, record licenses and transitive
changes, and run the current advisory/license checks. A dependency failure
stops implementation for design amendment; it is not permission to fall back
to canonicalize-then-read.

### 8.2 Engine structure

The engine gains private components equivalent to:

```text
SelectedRoot
  ├── caller label
  ├── open directory capability
  └── root filesystem identity

ObservedPath
  ├── native relative identity
  ├── safe display identity
  ├── observed stable identity
  └── File { retained parent capability, final name } | Directory | PathIssue
```

Traversal is handle-relative and iterative/recursive over retained child
directory capabilities. Content comparison receives the retained parent
capability and one final name, opens once through Section 8.1, validates the
returned handle against its observed type/identity, and reads that same
handle. It never receives an ambient descendant `PathBuf` or reopens a
multi-component root-relative path.

Rayon may retain parallel content work only if capability handles and progress
events remain safe and deterministic. Otherwise implementation may make this
WS-04 path sequential and leave performance recovery to WS-10; security and
correctness outrank current parallel structure.

### 8.3 Public and frontend boundary

`DiffEngine::compare`, `compare_with_ignore`, and `compare_with_progress`
retain their signatures. Every existing engine caller therefore receives the
policy: CLI audit/diff/snap/report/dashboard/watch/export/init and guided/expert
GUI runs cannot diverge.

Frontend changes are limited to:

- actionable root-selection failure presentation;
- displaying the existing Error state and stable issue detail in current human
  surfaces that already support detail;
- ensuring failed comparisons do not persist history or stale success; and
- test snapshots needed to prove parity.

No frontend may re-walk or re-open a reported path.

This RFC does not add or version JSON/SARIF/CSV/TSV fields. Existing structured
surfaces that already carry `detail` receive the stable code through that
field; a surface such as raw `diff --json-output` that currently omits detail
continues to expose `Incomparable` until WS-05 defines the common machine
schema. WS-04 must not pre-empt WS-05 by inventing an unversioned `issue_code`
field. Human wording may expose the code safely, but contextual output encoding
remains an open M2 risk until WS-05 is implemented.

## 9. Adversarial acceptance matrix

### 9.1 Required cases on every hosted OS

- outside-root file link containing a unique secret canary;
- outside-root directory link containing a unique filename/content canary;
- inside-root file and directory link;
- broken link;
- two-link cycle and self-cycle;
- Added, Removed, and same-path link cases;
- link versus regular file and link versus directory;
- ignored link and ignored linked directory;
- root path whose final component is a link;
- regular, directory, link, and special-node classification seams;
- deterministic barrier race replacing a classified regular file or directory
  with an outside-root link before open;
- unreadable/removed-during-scan behavior;
- `%`, control, and platform-relevant separator-like names; and
- duplicate lossy-looking native names proving no map collision.

Linux and macOS must additionally execute a real production xdev case. The
test opens a bounded, known mounted-child candidate through the production
parent-capability/single-component function, obtains metadata from the actual
returned handle, observes a device mismatch, emits `AAAI-PATH-XDEV`, and proves
the child enumerator was never entered. Linux candidates include `/proc` and
`/dev/shm`; macOS candidates include `/dev` and mounted children below
`/System/Volumes` and `/Volumes`. The test must find at least one accessible
differing-device candidate and fails if none exists. A mocked device detector
or automatic skip is not equivalent evidence; an unavailable hosted fixture
stops acceptance for amendment.

Invalid Unix-byte encoding is exercised through the formatter on all Unix
targets and through actual filesystem entries on Linux. APFS rejects such
names, so macOS filesystem collision evidence instead uses both the literal
single-component name `back\slash` and the nested native path `back/slash`.
They collided under the former slash-replacement mapping and must remain two
distinct native-key entries with distinct escaped display identifiers.

Windows forbids `\` in a filename because it is the path separator, and Win32
rejects control characters below `0x20`, so a Windows filesystem-level
collision pair equivalent to the Unix case is not constructible. Windows
collision-freedom is instead evidenced by the unpaired-UTF-16 formatter unit
test (`display_preserves_unpaired_utf16_without_collision`); the engine-level
native-name collision test is `#[cfg(unix)]` by this reviewed platform
constraint, not by omission.

Windows must exercise file symlink, directory symlink, junction, and
mount-point or equivalent name-surrogate reparse rejection through the exact
production handle/attribute path.

The non-name-surrogate/unknown reparse case has no constructible hosted
fixture. WOF System Compression
(`compact.exe /exe:xpress4k|xpress8k|xpress16k|lzx`) is a Windows client
NTFS/WOF-driver capability and is absent from the Windows Server images GitHub
hosts; on `windows-2025-vs2026` `compact.exe` reports success while silently
applying ordinary NTFS compression, which sets `FILE_ATTRIBUTE_COMPRESSED` and
no reparse point. This was observed identically in runs `81038738047` and
`82177458147`. NTFS Data Deduplication requires feature installation and an
asynchronous job on every run and is not guaranteed on ephemeral system
volumes; the stock Windows compatibility reparse points are junctions and
therefore name surrogates.

This case was **deferred, not waived** in Part 1, on the same
reviewed-platform-constraint basis as the macOS APFS invalid-byte case and the
Windows name-collision case above. **It is now discharged by execution.**

`windows_non_name_surrogate_reparse_is_rejected` constructs a real
non-name-surrogate reparse point through `FSCTL_SET_REPARSE_POINT` with a
non-Microsoft tag — `0x00000042`, with both the Microsoft-owned bit 31 and the
name-surrogate bit 29 clear — supplying a `REPARSE_GUID_DATA_BUFFER` from an
external PowerShell helper. No `unsafe` or FFI enters any crate, no elevation
or service is required, and the tag is deleted explicitly before the temporary
file is removed.

The fixture asserts `FILE_ATTRIBUTE_REPARSE_POINT` is set while
`is_symlink()` is **false**, then asserts the production classifier rejects the
entry with `AAAI-PATH-REPARSE`. It is the only fixture in the suite where those
two conditions diverge: every other Windows reparse fixture uses a symlink,
directory symlink, or junction, all of which are name surrogates. A change
replacing the generic-attribute check at
`crates/aaai/src/diff/path_boundary.rs` with `is_symlink()` would leave every
other fixture passing and fail only here.

Verified on GitHub-hosted `windows-2025-vs2026`, run `30456899630`, `headSha`
`aa0e3aa5fb3f26248c6299468510208836d4b5f2`: Windows reported 132 tests passed,
including this one by name, with Linux and macOS at 144 and `B0 / gate`
successful. Evidence:
`.git-exclude/evidence/098-selected-folder-and-symlink-policy/hosted-runs.md`.

Rejection of non-name-surrogate reparse points is therefore evidenced by
execution, not solely by the tag-agnostic construction of the decision
procedure.

Unix must exercise FIFO/socket classification without opening or blocking.
Platform fixture creation failure is a failed required case, not an automatic
skip; amendment is required if the runner cannot supply a reviewed equivalent
fixture.

### 9.2 Assertions

Each adversarial test asserts:

- expected `DiffType` and stable issue code;
- audit status `Error` and audit exit 3 where applicable;
- identical engine/CLI/GUI classification through shared fixtures or focused
  parity tests;
- no outside canary content, filename, target path, digest, or ambient absolute
  path appears in captured results or output;
- no descendant behind a link is enumerated;
- ignored links remain untraversed;
- roots and all canaries retain content bytes, namespace/entry set, object
  kind, size, permissions, modification time, and stable identity where
  supported; access time and documented volatile metadata are excluded; and
- normal-file behavior remains unchanged.

The race test must use a test-only synchronization seam, not timing or sleep.

### 9.3 Commands and hosted gate

Minimum implementation evidence:

```sh
cargo +1.91 test --workspace --locked
cargo +1.91 test -p aaai --lib diff
cargo +1.91 test -p aaai-cli --test cli
cargo tree --locked
cargo audit
cargo deny check
git diff --check
```

Known repository-wide C0/C2 failures are recorded rather than relabeled. The
WS-04 implementation SHA must pass the existing B0 Linux/macOS/Windows matrix
and `B0 / gate`. WS-04's symlink/path contribution to S2 is not complete until
independent review accepts the adversarial evidence. M2/S2 as a whole remains
open until WS-05's output-encoding, spreadsheet-neutralization, and masking
contribution is also implemented and accepted. WS-05 may use the reporting
contract after WS-04 design approval, but implementation continuation remains
serialized through the current B0 authority.

## 10. Implementation and review sequence

1. Independent architecture/security review accepts this RFC and handoff.
2. The project owner approves implementation and confirms the named roles and
   runner capacity.
3. Prove the selected dependency set builds on Rust 1.91 and record its supply-
   chain delta before engine migration.
4. Add the private capability/path adapter and focused unit tests.
5. Replace ambient collection/read behavior without changing public method
   signatures.
6. Add adversarial engine, CLI, and GUI/parity tests.
7. Run local focused/full checks and prepare an implementation review package
   referencing this RFC, its handoff, and all evidence.
8. After accepted implementation review and owner-controlled integration,
   observe B0 for the exact current `main` SHA and append the WS-04 boundary to
   RFC 097's single durable record at
   `.git-exclude/evidence/097-safe-hosted-ci-bootstrap/continuity-mode.md`.
9. Move this RFC to `done/` and update the index only through an independently
   reviewed lifecycle patch.

Implementation stops for RFC amendment and re-review if it requires link
following, project `unsafe`, a public `DiffEntry`/`DiffEngine` break, a new CLI
option, weakened tests, skipped required platform cases, or an ambient-path
fallback.

## 11. Role and evidence ownership

| Responsibility | Proposed assignment | Required confirmation |
|---|---|---|
| Primary implementation | Codex | Owner approval after design review |
| Maintainer review and integration authority | nabbisen | Before implementation |
| Independent architecture/security design review | Independent architect | Design review and focused rereview |
| Independent implementation review | Independently assigned reviewer; may differ from design reviewer | Later implementation/evidence review through a self-contained package |
| Adversarial platform QA owner | nabbisen, using GitHub-hosted runners | Runner/fixture capacity before implementation |
| Linux/macOS/Windows execution | GitHub-hosted B0 runners | Observed on implementation SHA |

The owner confirmed the proposed roles and fixture capacities on 2026-07-22.
Actual required fixture behavior remains an observed implementation gate.
Unfilled replacement review capacity or unavailable required runner/fixture
behavior blocks implementation acceptance. Architecture acceptance alone does
not authorize commit, push, repository-rule mutation, or release.

## 12. Alternatives considered

| Alternative | Decision |
|---|---|
| Keep current behavior | Rejected: reads and hashes outside selected roots |
| `canonicalize` + `starts_with` before `std::fs::read` | Rejected: leaves a check/read race and ambient descendant authority |
| Follow only links whose resolved target is inside root | Rejected for v1: substantially larger identity, cycle, mutation, and reporting contract |
| Add `--follow-links` or a GUI checkbox | Rejected: unsafe mode fragments CLI/GUI semantics and weakens default assurance |
| Silently skip links | Rejected: an incomplete audit could appear passing |
| Abort the whole comparison on the first link | Rejected: safe collection of all path-local issues gives better remediation while still failing the audit |
| Metadata-only link classification as `Incomparable` | **Selected:** explicit, non-following, auditable, and compatible with the existing error-first model |
| Native OS code in project | Rejected: violates the no-project-`unsafe` rule and multiplies platform security code |
| Capability-rooted traversal with reviewed dependencies | **Selected:** closes ambient escape and race classes across supported OSes |

## 13. Risks and limitations

| Risk | Mitigation |
|---|---|
| Capability dependency does not compile at MSRV or changes advisories materially | Feasibility/supply-chain checkpoint before migration; amend rather than weaken |
| Windows all-reparse rejection affects legitimate provider placeholders | Explicit v1 compatibility choice, distinct stable code, exact hosted fixtures, later RFC required to narrow |
| Race seam passes while production path differs | Put seam immediately between production classification and production no-follow open |
| Path escaping changes edge-case machine identifiers | Preserve ordinary UTF-8 spelling; add exact compatibility fixtures and document encoded anomalies |
| Link-heavy legitimate projects become audit errors | Explicit v1 policy; user may ignore a link path, but cannot follow it |
| Hard-linked outside content is read | Documented namespace limitation; portable origin detection is unavailable |
| Cross-filesystem directories are common in some trees | Fail-closed issue identifies exact root-relative boundary; future opt-in needs a new RFC |
| WS-05 accidentally decodes or leaks targets | Link targets never enter the model; WS-05 consumes codes and escaped identifiers only |

## 14. Approval decisions requested

The owner and independent reviewer are asked to confirm:

1. selected roots are capability handles and final-component root links are
   atomically rejected rather than checked then opened;
2. all Unix links and all Windows `FILE_ATTRIBUTE_REPARSE_POINT` entries are
   metadata-only, non-traversed, and `Incomparable`, including legitimate
   non-name-surrogate reparse-backed files;
3. there is no v1 follow-links option, including for inside-root targets;
4. cross-filesystem and special-node descent fails closed;
5. retained parent capabilities plus single-component no-follow/all-reparse
   rejection and same-handle validation/read are required instead of
   canonicalize-then-read or multi-component root-relative reopen;
6. the public `DiffEntry`/`DiffEngine` shape is preserved;
7. native internal identity plus the specified escaped identifier is the WS-05
   path input;
8. stable issue codes and Error/exit semantics are sufficient and consistent;
9. the adversarial matrix, no-skip platform rule, and B0 requirement are
   adequate; and
10. the proposed assignments and capacity checkpoint are approval-ready.

Approval authorizes only the later implementation work described here after
the explicit owner instruction. It does not authorize commit, push, release,
publication, repository-rule changes, or unrelated remediation.

## 15. Governing and technical sources

- `ROADMAP.md`, M2 / WS-04 / S2
- RFC 095, especially Sections 4, 6, 8, 10, and 11
- RFC 096/S1 and RFC 097/B0
- the initial architecture review's selected-folder finding
- Rust [`std::fs::symlink_metadata`](https://doc.rust-lang.org/std/fs/fn.symlink_metadata.html)
  and [`FileType`](https://doc.rust-lang.org/std/fs/struct.FileType.html)
  documentation
- Bytecode Alliance [`cap-std`](https://docs.rs/cap-std/4.0.2/cap_std/fs/struct.Dir.html)
  and [`cap-fs-ext`](https://docs.rs/cap-fs-ext/4.0.2/cap_fs_ext/trait.OpenOptionsFollowExt.html)
  documentation
- Bytecode Alliance [`cap-primitives` 4.0.2 Windows open-option translation](https://docs.rs/cap-primitives/4.0.2/src/cap_primitives/windows/fs/oflags.rs.html)
- Microsoft [`FILE_ATTRIBUTE_REPARSE_POINT`](https://learn.microsoft.com/en-us/windows/win32/fileio/file-attribute-constants)
  and [`FILE_FLAG_OPEN_REPARSE_POINT`](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/ns-fileapi-createfile2_extended_parameters)
  documentation
