# RFC 096 — Test Environment and User-State Isolation

**Status.** Implemented (S1 local Linux — 2026-07-17)

**Tracks.** `ROADMAP.md` M1 / WS-02 / gate S1

**Depends.** RFC 095 / D0 and the accepted v1 trustworthiness-remediation
roadmap

**Design owner.** Codex

**Decision owner and maintainer reviewer.** nabbisen, project owner

**Owner decision.** Approved as proposed after independent architecture
re-review on 2026-07-17; scoped WS-02 implementation authorized

**Implementation acceptance.** Independent architecture/security review
accepted the implementation with only explicitly non-blocking notes on
2026-07-17. S1 is complete on the reviewed local Linux boundary; declared-MSRV
and hosted Linux/macOS/Windows execution remain WS-03/B0.

**Primary implementer.** Codex, only after independent design review and
explicit owner approval (conditions satisfied 2026-07-17)

**Independent reviewers.** Architecture/security reviewer for design and
implementation; no separate specialist co-approver is required

**Capacity and environment.** One primary implementer and one maintainer
reviewer within the M1 serial window; local Linux with Rust 1.91 is available
for S1. macOS and Windows hosted execution belongs to WS-03/B0, while this RFC
must make the isolation mechanism platform-independent and test its resolution
contract without changing process-global environment.

**Evidence location.** `.git-exclude/evidence/096-test-state-isolation/`

**Touches.** Shared user-state path resolution in `aaai`; history, profile, and
preferences stores; the CLI subprocess test harness and history tests. No
persisted schema, default production location, CLI option, GUI workflow, or
release workflow changes.

**Handoff.** No separate developer handoff. Sections 7–11 contain the complete
subprocess-helper migration table, failure cases, sequencing, and verification
checklist required by WS-02.

**Revision.** Closes architecture-review blockers B1 and B2: every child stream
is captured and scanned before return, and `OsString` presence semantics plus
pure resolver tests prove that no present override invokes platform fallback.

## 1. Summary

This RFC makes the test suite incapable of reading, printing, pruning, or
otherwise mutating an operator's real aaai state.

The design has three reinforcing layers:

1. Centralize resolution of the aaai user-state root used by
   `prefs.yaml`, `profiles.yaml`, and `history.jsonl`.
2. Give every binary an authoritative, absolute `AAAI_TEST_STATE_DIR` override
   that is checked before any platform directory API. The variable is reserved
   for repository tests and automation; when it is unset, production binaries
   retain the existing OS-standard path.
3. Replace the raw CLI `Command` helper with an owned per-command sandbox that
   injects the authoritative root plus platform home/config fallbacks, seeds
   those fallbacks with canaries, captures every child byte stream, and verifies
   no fallback resolution, observable disclosure, or mutation.

The destructive history regression uses more records than its prune limit and
proves that only the authoritative temporary state root changes. S1 is reached
only when the complete local suite passes and the recorded evidence satisfies
the negative-access contract in Section 10.

Implementation began only after independent architecture review and explicit
owner approval. The completed implementation then passed independent review
and the local Linux S1 gate on 2026-07-17.

## 2. Problem and observed baseline

### 2.1 Operator-state exposure

`crates/aaai-cli/src/tests.rs` currently defines:

```rust
fn aaai() -> Command {
    // locate target/debug/aaai
    Command::new(path)
}
```

The helper inherits the contributor's real environment. It has 89 observed
invocation sites. Most commands do not intentionally use global user state,
but a helper-level omission makes every subprocess capable of reaching it.
It also hard-codes `target/debug/aaai`, so `cargo test -p aaai-cli` can execute
a missing or stale binary unless a separate build happened first. A stale
pre-isolation binary could ignore the new state-root contract entirely.

The immediate unsafe cases are:

- `history_stats_exits_0`;
- `history_prune_exits_0`;
- `history_n_flag_limits_output`;
- `history_json_output_is_array`;
- the three RFC 024 audit-output tests that omit `--no-history`;
- any future subprocess test that reads or writes preferences, profiles, or
  history without remembering an ad hoc override.

The initial architecture review observed real history in test output.
`history_prune_exits_0` avoided deletion only because the operator store
contained fewer than its limit of 100 entries. A larger real store could have
been rewritten and truncated by an ordinary test run.

Two later RFC 062 history tests set `HOME` and `XDG_CONFIG_HOME` manually.
Those tests demonstrate intent but do not make the shared helper safe and do
not cover Windows.

### 2.2 User-state inventory

The complete current OS-level aaai state inventory is:

| File | Resolver/caller | Read behavior | Write/destructive behavior |
|---|---|---|---|
| `history.jsonl` | `history::store` | `history`, `history --stats`, GUI/CLI history consumers | CLI/GUI audit append; `history --prune N` rewrites and truncates |
| `profiles.yaml` | `profile::store` | GUI startup and recent projects | GUI save, auto-profile, touch, and delete |
| `prefs.yaml` | `profile::prefs` | GUI startup and theme/locale/settings load | GUI theme and settings save |

All three currently resolve independently through
`dirs::config_dir().join("aaai")` and create the parent directory during path
resolution. Project-local `.aaai.yaml`, `.aaaiignore`, audit definitions,
reports, and exports are explicit working/input/output paths, not OS-level
operator state; their tests still use temporary project directories.

### 2.3 Platform limitation of environment-only isolation

Setting only standard home/config variables is insufficient:

| Platform | `dirs` 6 behavior | Environment-only result |
|---|---|---|
| Linux | `XDG_CONFIG_HOME`, falling back to `HOME/.config` | Redirectable when both variables are set correctly |
| macOS | `HOME/Library/Application Support` | Redirectable when `HOME` is set |
| Windows | `SHGetKnownFolderPath(FOLDERID_RoamingAppData)` | `APPDATA`/`USERPROFILE` do not authoritatively redirect the Known Folder API |

Therefore, a helper that sets `HOME`, `XDG_CONFIG_HOME`, `APPDATA`, and
`USERPROFILE` can still reach the real Windows roaming-app-data directory.
The test seam must take precedence before `dirs::config_dir()` is called.

## 3. Goals and non-goals

### 3.1 Goals

- Ensure every aaai CLI subprocess test receives a unique temporary user-state
  root and replacement platform home/config variables.
- Ensure Cargo builds the exact binary under test and supplies its path; never
  execute a manually located or stale `target/debug/aaai`.
- Ensure the temporary root remains alive for the complete child-process
  lifetime.
- Prove platform fallback resolution is not attempted and fallback canary
  content is neither emitted nor changed.
- Exercise a real destructive history fixture above the prune threshold.
- Centralize the three user-state paths so a new store cannot silently invent
  a fourth resolution rule.
- Keep unit tests parallel-safe by avoiding mutation of the parent process
  environment.
- Preserve production state locations and all persisted file formats.
- Define auditable S1 commands and evidence artifacts.

### 3.2 Non-goals

- Changing `prefs.yaml`, `profiles.yaml`, or `history.jsonl` schemas or
  compatibility rules.
- Migrating historical `~/.aaai` data.
- Redesigning history retention or making history writes atomic; WS-06/P1 owns
  persistence atomicity and recovery.
- Changing project-local config discovery, selected-root handling, reports,
  masking, or symlink policy.
- Repairing hosted CI crate names or creating the platform matrix; WS-03/B0
  owns those changes.
- Running GUI visual tests or changing GUI behavior.
- Adding an end-user `--state-dir` option or a supported production
  configuration feature.
- Reading, hashing, copying, or otherwise inspecting a contributor's actual
  aaai state as part of the proof.

## 4. Safety invariants

Implementation and future tests must preserve all of these invariants:

1. A CLI subprocess created by the test helper always has an absolute,
   per-sandbox `AAAI_TEST_STATE_DIR`.
2. In every build profile, a present `AAAI_TEST_STATE_DIR` is authoritative.
   Invalid values fail closed and never fall back to the OS config directory.
3. When the reserved variable is absent, every build profile uses
   `dirs::config_dir()/aaai`.
4. Merely resolving or reading a missing store does not create an OS-level
   directory. Directory creation occurs only immediately before a write.
5. Every stateful store derives its path from the one shared resolver.
6. Test code does not call `std::env::set_var` or `remove_var`; child-specific
   `Command::env` calls are the only environment mutation.
7. The black-box CLI suite is a Cargo integration-test target and obtains the
   current binary from `CARGO_BIN_EXE_aaai`.
8. Every raw CLI binary spawn in that target is owned by the isolation helper.
   Direct `Command::new` is confined to that helper.
9. A fallback canary's recursive path/type/content snapshot is identical
   before and after child execution.
10. Every child execution captures both stdout and stderr before returning to
    the test, including status-like calls.
11. Both captured streams are scanned byte-for-byte for every canary token
    before an exit status or output is returned.
12. A destructive test modifies only its authoritative temporary state root
    and uses a source fixture whose record count is strictly greater than the
    requested prune limit.
13. Non-zero exits and spawn errors still run every applicable output/snapshot
    verification before the sandbox is destroyed.
14. No evidence step reads or fingerprints the operator's real files.

## 5. External contract

### 5.1 Production behavior

There is no documented end-user behavior or persisted-format change. Normal
production execution, where the reserved test variable is absent, is
unchanged.

Default resolution in every build profile:

```text
Linux:  $XDG_CONFIG_HOME/aaai, else $HOME/.config/aaai
macOS:  $HOME/Library/Application Support/aaai
Windows: FOLDERID_RoamingAppData/aaai
```

The existing files remain:

```text
<aaai-state-root>/
  prefs.yaml
  profiles.yaml
  history.jsonl
```

### 5.2 Reserved test state-root override

Every build profile recognizes:

```text
AAAI_TEST_STATE_DIR=<absolute directory containing the three state files>
```

Contract:

- presence is read with `std::env::var_os`, preserving `OsString` values that
  are not valid Unicode;
- a present value is converted directly to `PathBuf` and accepted only when it
  is non-empty and absolute;
- the directory need not exist for a read;
- writers create the directory as needed;
- if the variable is present but invalid, resolution returns an actionable
  error and does not consult `dirs`;
- a present non-Unicode absolute path is valid on platforms that support it;
  a present non-Unicode relative path is invalid, and neither case is ever
  treated as absence;
- the variable is read before any OS directory lookup;
- it is reserved for repository tests and automation;
- it is not a v1 CLI, GUI, environment, or compatibility promise;
- it remains intentionally undocumented in end-user help and guides;
- recognizing it in every profile is required so a normal, release-profile, or
  otherwise custom-built subprocess cannot silently become unsafe.

The override points directly at the aaai application directory. The resolver
must not append a second `aaai` component.

This narrow internal seam is required because platform home/config variables
cannot override the Windows Known Folder API. It is not permission to add other
test-only product behavior. Removal or semantic change requires an equally
strong replacement for S1, even though the variable is not a public
compatibility promise.

## 6. Internal design

### 6.1 Shared resolver

Add one internal module under the `aaai` crate, for example
`crates/aaai/src/user_state.rs`, with a small value object:

```rust
struct UserStatePaths {
    root: PathBuf,
}
```

Responsibilities:

- resolve the reserved test override before platform fallback in every build;
- otherwise resolve `dirs::config_dir()?.join("aaai")`;
- validate that an override is absolute;
- expose `prefs()`, `profiles()`, and `history()` path methods;
- create the root only through an explicit `ensure_for_write()` operation.

The module remains crate-private. Existing public store APIs retain their
signatures. This avoids introducing a new public engine compatibility surface.

Resolution is split into:

1. a production adapter that reads the process environment and calls `dirs`;
2. a pure decision function that accepts supplied override/default values.

The adapter uses `var_os`, not `var(...).ok()`. The pure decision function
accepts `Option<OsString>` plus a lazy default resolver. Unit tests exercise
the pure function with a panic/counting default closure. They must not change
process-global environment and therefore remain safe under the default
parallel test runner.

Required pure cases:

| Override input | Result | Default resolver calls |
|---|---|---:|
| Absent | Use supplied platform default | 1 |
| Present valid absolute | Use override | 0 |
| Present empty | Invalid-override error | 0 |
| Present relative | Invalid-override error | 0 |
| Present non-Unicode absolute (`#[cfg(unix)]`) | Use override | 0 |
| Present non-Unicode relative (`#[cfg(unix)]`) | Invalid-override error | 0 |

The Unix cases construct `OsString` from bytes through
`std::os::unix::ffi::OsStringExt`; they do not mutate the process environment.

### 6.2 Store migration

`history::store`, `profile::store`, and `profile::prefs` stop calling
`dirs::config_dir()` directly. Each operation resolves `UserStatePaths` once
and obtains its file path from that value.

Read operations:

- return the existing empty/default result when the file is absent;
- do not create the root directory;
- preserve current malformed-data behavior;
- preserve current public return types and logging behavior.

Write operations:

- call `ensure_for_write()` before opening the file;
- preserve current serialization and file contents;
- do not absorb WS-06 atomicity/locking work.

The implementation review must show that the only remaining project call to
`dirs::config_dir()` is inside the shared resolver.

### 6.3 Cargo integration target and owned CLI test sandbox

Move the black-box subprocess suite from the binary's internal
`crates/aaai-cli/src/tests.rs` module to Cargo's standard integration-test
target:

```text
crates/aaai-cli/tests/cli.rs
crates/aaai-cli/tests/cli/support.rs
crates/aaai-cli/tests/cli/state_isolation.rs
```

Remove `#[cfg(test)] mod tests;` from `src/main.rs`. The integration target
locates the current package binary with `env!("CARGO_BIN_EXE_aaai")`. Cargo
therefore builds the exact binary for the test target; the helper must not
construct `target/debug/aaai` manually or depend on a preceding `cargo build`.

Subprocess construction moves into `tests/cli/support.rs`.

The helper owns:

- a `tempfile::TempDir`;
- an allowed state root;
- fallback home/config roots;
- unique high-entropy canary tokens generated without real secrets;
- recursive before-snapshots of every fallback root;
- the child `Command`.

The temporary directory must remain owned until the common execution primitive
returns and verification completes. Returning a bare `Command` from a function
that owns a `TempDir` is forbidden because the directory would be deleted
before the child runs.

The wrapper supports only the command operations currently needed by the test
suite: `arg`, `args`, `current_dir`, a status-like execution method, and an
output-returning execution method. Name them `run_status()` and `run_output()`
so a focused scan can distinguish them from raw `Command` methods. Both
delegate to one internal primitive that calls `Command::output()` and therefore
captures both stdout and stderr. `run_status()` returns only the captured
`ExitStatus` after verification; it must never delegate to
`Command::status()`.

The wrapper also exposes the allowed state path and narrow synthetic-fixture
helpers so state-isolation tests can seed and inspect their own
`history.jsonl`, `profiles.yaml`, or `prefs.yaml`. It does not expose an
unrestricted method that can replace the reserved isolation variables.

Existing one-shot call sites may continue to use a temporary wrapper in a
chain. A test that must inspect state after execution binds the wrapper to a
local variable before executing; the owned `TempDir` then remains alive through
fixture inspection. The destructive and audit-write regressions must use this
bound form. The command execution methods borrow the wrapper rather than
consume it.

The common execution primitive performs this order:

1. call `Command::output()` so both streams are drained without pipe deadlock;
2. scan stdout and stderr as bytes for every canary byte sequence;
3. verify every fallback recursive snapshot;
4. return the verified `Output`, or only its status for the status-like API.

The primitive records a scan failure but does not return early: snapshot
verification always runs, and the structured error reports all applicable
synthetic failure classes without including captured bytes.

The default policy does not replay captured child output. A future explicit
diagnostic replay option may replay bytes only after the scan succeeds. If a
canary is detected, neither stream is replayed or included in the error; the
error identifies only the synthetic canary ID and stream.

The helper returns a structured harness error for disclosure, mutation, or
spawn failure. Snapshot verification still runs after a non-zero child exit
and, where a child was not created, after a spawn error. Existing `.unwrap()`
call patterns may remain after method-name migration.

Every child receives:

```text
AAAI_TEST_STATE_DIR=<sandbox>/state
HOME=<sandbox>/fallback/home
XDG_CONFIG_HOME=<sandbox>/fallback/xdg
USERPROFILE=<sandbox>/fallback/profile
APPDATA=<sandbox>/fallback/roaming
LOCALAPPDATA=<sandbox>/fallback/local
```

On Windows the helper may also set consistent `HOMEDRIVE` and `HOMEPATH`
values derived from the temporary profile path. These platform variables are
defense in depth and canary surfaces; `AAAI_TEST_STATE_DIR` remains the
authoritative cross-platform control.

The helper must preserve unrelated inherited variables required to execute the
binary and load its runtime dependencies. It must not print inherited
environment values into evidence.

### 6.4 Canary layout and verification

Each fallback root contains plausible aaai state at the exact platform-relative
locations a bad fallback could select:

```text
fallback/home/.config/aaai/
fallback/home/Library/Application Support/aaai/
fallback/xdg/aaai/
fallback/roaming/aaai/
fallback/local/aaai/
```

Every applicable directory contains:

- a valid `history.jsonl` containing a unique canary marker in record fields;
- a valid `profiles.yaml` containing a unique canary marker;
- a valid `prefs.yaml` containing a unique canary marker;
- an unrelated sentinel file to detect directory replacement or broad writes.

Before spawning, the helper records an exact recursive snapshot of relative
paths, entry types, and bytes. These are synthetic fixtures, so reading their
contents does not expose operator data.

After spawning:

- compare the exact snapshot for addition, removal, replacement, truncation, or
  content change;
- unconditionally capture and scan stdout and stderr for every canary token
  before either public execution API returns;
- run snapshot verification after success, non-zero exit, and spawn error;
- report only the synthetic canary identifier and relative sandbox path on
  failure.

The test must not chmod the fallback roots to unreadable as its primary
mechanism. Cross-platform permission semantics differ, and access-denied alone
cannot distinguish a correct resolver from a silently swallowed read error.
The proof is deliberately composite:

- pure resolver tests plus the centralized-`dirs` scan prove that a present
  override does not resolve a fallback path;
- unconditional byte-stream scans prove no observable canary disclosure;
- exact snapshots prove no fallback mutation.

A snapshot alone does not prove that a file was unread, and the S1 evidence
must not make that claim.

Helper self-tests must exercise four disclosure cases: canary bytes on stdout
and stderr through both the status-like and output-returning APIs. Safe
synthetic CLI arguments/paths may cause the current Cargo-built binary to emit
the token; no fallback file needs to be read for these scanner tests. Each case
asserts a disclosure error, no replay, and unchanged snapshots. Additional
cases cover success, non-zero exit, and an internally injected nonexistent
executable for spawn-error snapshot verification.

## 7. Complete subprocess-helper migration table

The following table is the WS-02 handoff-equivalent migration record.

| Existing surface | Observed risk | Required migration | Required regression |
|---|---|---|---|
| `src/tests.rs::aaai()` and all 89 observed invocation sites | Raw `Command` inherits operator state and manually locates a potentially stale binary | Move the suite to `tests/cli.rs`; construct only `env!("CARGO_BIN_EXE_aaai")` through the owned helper in `tests/cli/support.rs`; every execution mode uses the common capture/scan primitive; no call site supplies state variables manually | Static scan: `target/debug/aaai` is absent, raw `Command::new` exists only in support, and raw `.status()` use is absent |
| Existing `.status()` callers | Child streams currently inherit the terminal/CI log | Migrate to `run_status()`, which internally calls `Command::output()`, scans both streams, verifies snapshots, and returns only `ExitStatus` | Status-like stdout and stderr disclosure self-tests both fail closed without replay |
| Existing `.output()` callers | Capture exists but has no universal scanner | Migrate to `run_output()`, backed by the same primitive | Output-returning stdout and stderr disclosure self-tests both fail closed without replay |
| `help_stdout` and all help/completion/version/exit-code calls | Currently read-only but can become stateful without caller changes | Use the same isolated helper; no “safe command” exception | A representative captured help command leaves canaries unchanged |
| Audit tests already using `--no-history` | Flag reduces writes but does not isolate fallback reads or future state access | Keep the behavior flag where it is part of the scenario, and still run through the isolated helper | Allowed state root remains empty; canaries unchanged |
| `rfc024_audit_zone4_hint_appears_for_pending` | Omits `--no-history`; appends to real history | Run in owned sandbox; preserve command semantics and assert history appears only in allowed state | Allowed history contains one record; canaries unchanged and absent from output |
| `rfc024_quiet_audit_suppresses_zone4_hint` | Omits `--no-history`; appends to real history | Same as above; capture output through helper | Quiet behavior remains correct; isolated history only |
| `rfc024_json_output_audit_suppresses_zone4_hint` | Omits `--no-history`; appends to real history | Same as above; captured JSON must remain valid | JSON contains no hint/canary; isolated history only |
| `history_stats_exits_0` | Reads and prints real history | Move to `tests/cli/state_isolation.rs`, seed only allowed state, capture output | Output reflects allowed fixture and no canary |
| `history_prune_exits_0` | Can rewrite/truncate real history; current empty/below-limit fixture proves nothing | Replace with five-record allowed fixture and `--prune 3` | Exactly two removed; newest three retained; all fallback snapshots unchanged |
| `history_n_flag_limits_output` | Reads real history | Move and seed at least five allowed records | Exactly the requested allowed records are emitted; no canary |
| `history_json_output_is_array` | Reads real history | Move and seed allowed records | Valid JSON array from allowed fixture only |
| `rfc062_history_empty_exits_zero` | Ad hoc Linux-only `HOME`/`XDG_CONFIG_HOME` isolation | Remove manual `.env` calls and use shared sandbox | Empty allowed state returns success; resolver proof, stream scans, and snapshots all pass |
| `rfc062_history_stats_empty` | Same ad hoc isolation | Same migration | Empty stats success; canaries unchanged |
| `config_show_when_no_config_found` and project-local config/init tests | Could discover repository files if working directory is not explicit | Preserve explicit temporary `current_dir`/`--dir`; also use isolated state helper | Project output remains inside its temp directory |
| Future CLI subprocess tests | Caller can forget state isolation | Test-module policy requires the shared helper; implementation review scans for bypass | New direct spawn or reserved env override fails review/gate |
| Direct store unit tests | Process-global environment mutation would race parallel tests | Test the pure resolver and explicit temporary paths; never call `set_var` | Default-parallel engine unit suite passes |
| GUI unit tests | Current tests do not construct `App::default`, but future construction would load profiles/prefs | Any future stateful GUI test must use an explicit test root or isolated child process before constructing the app | Static inventory and targeted test when such construction is added |

No test is exempt because a command happens to be read-only today. The helper
boundary protects future changes as well as the known history defect.

## 8. File and module plan

Expected implementation boundary:

| File | Planned change |
|---|---|
| `crates/aaai/src/lib.rs` | Declare the crate-private shared user-state module |
| `crates/aaai/src/user_state.rs` | Resolution, validation, filenames, and write-time directory creation |
| `crates/aaai/src/user_state/tests.rs` | Pure resolution precedence/fail-closed tests; no global env mutation |
| `crates/aaai/src/history/store.rs` | Use shared paths; separate reads from directory creation |
| `crates/aaai/src/history/store/tests.rs` | Add explicit-root/absence behavior tests where required |
| `crates/aaai/src/profile/store.rs` | Use shared paths |
| `crates/aaai/src/profile/store/tests.rs` | Exercise persistence in a temporary explicit root, including real `touch()` |
| `crates/aaai/src/profile/prefs.rs` | Use shared paths |
| `crates/aaai/src/profile/prefs/tests.rs` | Exercise load/save in a temporary explicit root |
| `crates/aaai-cli/src/main.rs` | Remove the internal black-box test-module declaration |
| `crates/aaai-cli/src/tests.rs` | Move existing black-box cases to the Cargo integration target |
| `crates/aaai-cli/tests/cli.rs` | Integration-test root using Cargo's current binary path; retain unrelated CLI behavior cases |
| `crates/aaai-cli/tests/cli/support.rs` | Owned command sandbox, environment injection, canaries, snapshots, and verification |
| `crates/aaai-cli/tests/cli/state_isolation.rs` | Destructive fixture and read/write/non-disclosure regressions |

Exact filenames may change during implementation only if the logical
boundaries and acceptance contract remain the same.

### 8.1 Oversized-file boundary assessment

`crates/aaai-cli/src/tests.rs` is already over 1,700 lines and exceeds the
project's 500-ELOC split threshold. RFC 096 moves it to the standard black-box
integration-test location and must not add the harness or new state-isolation
cases to the moved monolith.

The stable split is:

- generic subprocess mechanics in `tests/cli/support.rs`;
- all history/user-state isolation cases in
  `tests/cli/state_isolation.rs`;
- existing unrelated CLI behavior stays temporarily in `tests/cli.rs`.

Moving the existing history tests reduces concentration without forcing an
unrelated full-suite reorganization. A broader CLI-test split belongs to
WS-08 unless implementation discovers a small additional stable boundary that
can be moved without semantic churn.

No touched engine file currently exceeds 500 ELOC. Implementation review must
re-run the ELOC assessment rather than relying on this design-time observation.

## 9. Implementation sequence

Implementation remains one serial, reviewable change:

1. Add the shared resolver and pure precedence/error tests.
2. Migrate history, profile, and preference stores without format changes.
3. Add explicit-root store tests and confirm reads no longer create directories.
4. Move the black-box suite to Cargo's integration-test target and switch
   binary discovery to `CARGO_BIN_EXE_aaai`.
5. Add the owned CLI sandbox and its canary snapshot verifier.
6. Move the existing history tests into `tests/cli/state_isolation.rs`.
7. Migrate all 89 subprocess invocation sites.
8. Add the above-threshold prune, audit-write, disclosure, and fail-closed
   regressions.
9. Run formatting once after implementation, per project policy.
10. Run the S1 evidence commands and package an implementation review.

If implementation requires a public API break, persisted-format change,
process-global environment mutation, release-workflow change, or separate
platform-specific product behavior, stop and amend/re-review this RFC.

## 10. S1 acceptance and evidence contract

### 10.1 Functional acceptance

- All current CLI subprocess tests run through the owned helper.
- Existing command/exit-code/output assertions remain unchanged in substance.
- History read/stats/limit/JSON cases use only the allowed temporary fixture.
- Pruning five records to three reports two removals and retains the newest
  three.
- The three audit cases that intentionally write history create records only
  under their allowed state roots.
- Preferences and profiles round-trip through explicit temporary roots.
- `ProfileStore::touch()` is exercised through its real persistence path.
- Both public helper execution APIs reject canary bytes on stdout and stderr
  without replaying either stream.
- Snapshot verification runs after a successful exit, non-zero exit, and spawn
  error.

### 10.2 Negative safety acceptance

- The environment adapter uses `var_os` and never collapses a present
  non-Unicode value into absence.
- Pure tests cover absent, valid absolute, empty, relative, non-Unicode
  absolute, and non-Unicode relative inputs.
- Valid and every invalid present value invoke a panic/counting default
  resolver zero times; only the absent case invokes it once.
- All fallback recursive snapshots match before/after.
- Every child captures and scans both stdout and stderr before returning,
  including status-like calls.
- Neither stream is replayed before a successful scan.
- A read from an absent allowed store does not create the state directory.
- No CLI test directly constructs the binary command outside support.
- No support execution path calls `Command::status()`.
- No test mutates the parent process environment.
- No store outside the shared resolver calls `dirs::config_dir()`.
- Evidence contains no actual operator path, filename, record, profile,
  preference, or secret value.

### 10.3 Observed commands required at implementation review

Commands may be adjusted only for actual target names; every result must be
reported rather than assumed:

```sh
cargo fmt --all
cargo test -p aaai --lib
cargo test -p aaai-cli --test cli
cargo test -p aaai-gui --bin aaai-gui
cargo test --workspace
git diff --check
```

Run the CLI suite with its default parallelism. The design avoids parent
environment mutation and gives each command a unique root, so serial execution
must not be required for safety. A targeted serial run may additionally make
destructive evidence easier to read, but cannot substitute for the default
parallel run.

Required focused scans:

```sh
rg -n "Command::new|\\.status\\(|target/debug/aaai" crates/aaai-cli/tests crates/aaai-cli/src
rg -n "set_var|remove_var" crates -g "*.rs"
rg -n "dirs::config_dir" crates -g "*.rs"
rg -n "AAAI_TEST_STATE_DIR" crates -g "*.rs"
```

### 10.4 Evidence artifacts

Store only synthetic evidence under
`.git-exclude/evidence/096-test-state-isolation/`:

| Artifact | Required contents |
|---|---|
| `environment.md` | Commit/baseline, OS, architecture, `rustc -V`, `cargo -V`; no inherited environment dump |
| `commands.log` | Commands, exit codes, and summarized counts |
| `canary-contract.md` | Synthetic layout and token-generation method; all-child stdout/stderr capture policy; status-like/output-returning × stdout/stderr scanner cases; success/non-zero/spawn-error snapshot results |
| `resolver-contract.md` | `var_os`/`OsString` presence rule; absent/absolute/empty/relative/non-Unicode cases; observed default-resolver call counts |
| `destructive-history.md` | Fixture count 5, prune limit 3, expected/observed removal 2, retained-record identity using synthetic labels |
| `focused-scans.log` | Raw-spawn, global-env-mutation, resolver-centralization, and test-override scans |
| `scope.diffstat` | Changed-file boundary and ELOC assessment |

The evidence directory is ignored and is review input, not a release artifact.
The durable RFC and implementation-review package summarize every required
result.

## 11. Regression checklist

Before requesting implementation review:

- [x] The owner approved this RFC after independent design review.
- [x] Default production paths are unchanged on Linux, macOS, and Windows when
      the reserved variable is absent.
- [x] Every build profile recognizes and fail-closes on
      `AAAI_TEST_STATE_DIR`.
- [x] The reserved override is validated before any `dirs` call.
- [x] The adapter uses `var_os`; present non-Unicode values cannot become
      absence.
- [x] Valid and invalid present override tests prove the fallback closure was
      not invoked; only absence invokes it.
- [x] Reads do not create a missing state root.
- [x] Writes create only the allowed temporary root.
- [x] All three store modules use the shared resolver.
- [x] All CLI subprocesses use the owned helper.
- [x] The suite uses `CARGO_BIN_EXE_aaai` and has no manual
      `target/debug/aaai` lookup or separate-build prerequisite.
- [x] No test changes the parent process environment.
- [x] Fallback canaries cover Linux, macOS, and Windows path shapes.
- [x] Canary snapshots are exact and unchanged.
- [x] Every child execution captures and scans stdout and stderr.
- [x] Status-like and output-returning APIs each detect stdout and stderr
      disclosure without replay.
- [x] Success, non-zero, and spawn-error paths verify snapshots.
- [x] No helper execution path uses `Command::status()`.
- [x] The prune fixture has five records and limit three.
- [x] Audit history-write cases are asserted inside allowed state roots.
- [x] Existing ad hoc RFC 062 environment overrides are removed.
- [x] History tests and helper code are split out of the oversized test file.
- [x] Required format/build/test/scan commands are observed and recorded.
- [x] Failures are reported; no red gate is described as passed.
- [x] No real operator state or secrets are read into evidence.
- [x] The implementation review request is itself the architect review entry
      point.

## 12. Compatibility and downstream impact

### 12.1 RFC 095 compatibility matrix

This RFC advances these RFC 095 rows:

| RFC 095 surface | WS-02 contribution | Remaining owner |
|---|---|---|
| All 16 CLI commands and exit codes | Safe isolated subprocess foundation and stateful-command cases | WS-03/B0, WS-09/C1, WS-12/D1 |
| Profiles, recent projects, preferences, and append-only history | Operator-state isolation and real temporary persistence fixtures | WS-12/D1 for final compatibility acceptance |
| Watch, completions, and progress | Isolated CLI helper used by present/future command tests | WS-03/B0, WS-10/E1, WS-12/D1 |

No persisted-format promise is changed. If implementation discovers that safe
isolation requires a format or default-location change, the compatibility row
must be amended and reviewed before coding continues.

### 12.2 WS-03/B0 handoff

S1 supplies WS-03 with:

- one platform-independent test-root precedence contract;
- a CLI helper that does not rely on Windows Known Folder redirection;
- safe default-parallel tests suitable for the hosted matrix;
- explicit canary and destructive-test cases to execute on Linux, macOS, and
  Windows after CI bootstrap repair.

S1 local acceptance does not claim macOS/Windows hosted evidence. B0 must run
the isolated suite on those platforms and treat failures as blocking.

### 12.3 WS-06/P1 boundary

This RFC may separate read-time path resolution from write-time directory
creation, but it does not claim atomic writes, locking, recovery, or
concurrency safety. The current serialization/write mechanics remain inputs to
WS-06.

## 13. Alternatives considered

| Option | Decision |
|---|---|
| Add `--no-history` to every audit test | Rejected. It does not protect history read/prune tests, profiles, preferences, or future state access. |
| Set only `HOME` and `XDG_CONFIG_HOME` | Rejected. Linux-focused and does not redirect the Windows Known Folder API. |
| Also set `APPDATA` and `USERPROFILE`, with no product seam | Rejected. `dirs` 6 uses `SHGetKnownFolderPath` on Windows. |
| Mutate the parent test process environment around each test | Rejected. Parallel tests race, Rust 2024 makes environment mutation explicitly unsafe, and cleanup after panic is fragile. |
| Run every test serially | Rejected as a safety mechanism. Serial execution does not prevent access to real state. |
| Fingerprint the operator's real config before and after | Rejected. The proof must not read operator data or infer secrets. Synthetic deny/canary roots give stronger controlled evidence. |
| Dependency injection only, with no subprocess override | Rejected. A separately spawned CLI cannot receive an in-process Rust value. |
| Public `--state-dir` CLI option | Rejected. It expands the v1 external contract and is unnecessary for test isolation. |
| Always-supported `AAAI_STATE_DIR` environment feature | Rejected for this RFC. It would create a production configuration surface under feature freeze. |
| All-profile reserved test state root plus platform canaries | **Selected.** Cross-platform, fail-closed, narrow, safe under ordinary or release-profile test builds, and leaves default production behavior unchanged. |

## 14. Risks and mitigations

| Risk | Mitigation |
|---|---|
| The internal variable becomes an accidental end-user surface | Keep it out of CLI/help/guides, name it explicitly for tests, make no v1 compatibility promise, and require an RFC-reviewed replacement before removal while S1 depends on it. |
| A helper bypass reintroduces exposure | Confine `Command::new` to integration-test support, scan at review, and require the helper for every command without read-only exceptions. |
| Tests execute a stale binary that lacks the seam | Use Cargo's integration-test target and `CARGO_BIN_EXE_aaai`; forbid manual target-path construction. |
| Status-like calls leak inherited output before scanning | Implement both public APIs over `Command::output()`, scan both byte streams first, and forbid `Command::status()` in support. |
| Captured pipes deadlock | Use `Command::output()`, which drains both streams, rather than configuring piped streams around `status()`. |
| Temp directory drops before child execution | The owned wrapper retains `TempDir` through execution and verification. |
| Canary verification is skipped on child error | `status`/`output` wrappers verify on every return path, with a drop-time guard for unconsumed commands. |
| Canary token appears in logs from the harness itself | Do not replay before scan; failure messages use synthetic identifiers, not token content; evidence records pass/fail and relative paths. |
| Platform fallback coverage is incomplete | Seed Linux, macOS, roaming Windows, and local Windows path shapes in every sandbox. |
| A non-Unicode or invalid variable silently exposes real state | Read with `var_os`, validate `OsString` as `PathBuf` before calling `dirs`, and prove zero default-resolver calls for every present case. |
| Scope absorbs persistence redesign | Preserve formats and write mechanics; defer atomicity, locks, and recovery to WS-06/P1. |
| Oversized CLI test file grows | Put support and state cases in submodules and move existing history cases out. |

## 15. Review and approval

### 15.1 Design-review questions

The independent architect should decide:

1. Does the reserved state-root seam close the Windows Known Folder gap while
   remaining a justified internal safety control rather than a documented
   production compatibility surface?
2. Does the resolver preserve `OsString` presence and fail closed without a
   default lookup for every present valid or invalid override?
3. Does universal child-stream capture prevent both status-like and
   output-returning APIs from leaking canaries?
4. Does the composite resolver/scan/snapshot contract prove no fallback
   resolution, observable disclosure, and mutation without
   inspecting real operator files?
5. Is the complete 89-invocation migration adequately enforced at the helper
   boundary?
6. Are all current stateful CLI cases represented in Section 7?
7. Is the above-threshold prune fixture destructive enough to exercise the
   original risk safely?
8. Are production paths, schemas, and the WS-06 persistence boundary preserved?
9. Is the M1 staffing/environment record sufficient to enter implementation
   after owner approval?

### 15.2 Owner decision requested after review

After an `Accept` or explicitly non-blocking `Accept with notes`, the owner is
asked to approve:

- the all-profile, reserved `AAAI_TEST_STATE_DIR` safety seam;
- the shared internal state-path resolver;
- the subprocess migration and module split;
- the canary/destructive evidence contract;
- Codex as primary implementer for this serial workstream;
- nabbisen as maintainer reviewer;
- local Linux as S1 execution, with hosted macOS/Windows evidence deferred to
  WS-03/B0.

Approval authorizes RFC 096 implementation only. It does not authorize a
commit, push, release, WS-03 implementation, or changes outside the reviewed
boundary.

## 16. Design evidence and limitations

Observed design inputs:

- accepted `ROADMAP.md` M1/WS-02/S1 contract;
- RFC 095 Sections 6.4, 10, and 11;
- the initial architecture finding that real history was printed and prune
  could rewrite it;
- current `aaai()` helper and its 89 observed invocation sites;
- current history tests, including two ad hoc Linux-only overrides;
- current `dirs` 6 Linux, macOS, and Windows resolution implementations;
- current history, profile, and preference stores;
- current GUI state callers and CLI audit/history callers;
- project RFC lifecycle and Rust/test module rules.

Design limitations:

- No unsafe test command was executed while preparing this RFC.
- No operator state was inspected.
- No macOS or Windows runner result is claimed.
- No product test, build, format, lint, hosted workflow, or release gate is
  claimed by this design-only proposal.
- Exact implementation line numbers may move; review should use symbols and
  the migration table rather than treating baseline line numbers as stable.
