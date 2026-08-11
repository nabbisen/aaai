# RFC 098 Part 2 — behavioural non-name-surrogate reparse fixture: developer handoff

Companion to [`RFC 098`](../../done/098-selected-folder-and-symlink-policy.md)
§9.1, which specifies the mechanism. This handoff translates it into
implementation-ready work. It must not override the RFC — if execution
uncovers a design conflict, stop and escalate (§8).

**Owner decision of record:** 2026-07-29, option **B1-first** — attempt the
behavioural fixture now rather than building the interim structural guard.
Rationale: `main` is green, so a failed attempt is an ordinary corrective cycle
under RFC 097 §5.5 rather than a program-wide block, and a working fixture
means the guard and its `include_str!` coupling never need to exist.

## 1. Why this fixture is worth building

RFC 098 §9.1 currently records the case as **deferred, not waived**, discharged
only by "an argument from source, not from execution."

The gap that argument leaves is specific. Review
`.git-exclude/reviewed/032-rfc098-windows-fixture-amendment-owner-directed-review-2026-07-28.md`
§4 established that if the production decision at
`crates/aaai/src/diff/path_boundary.rs:486` were ever changed from
`Ok(windows_reparse(&metadata))` to `Ok(metadata.file_type().is_symlink())`,
**every existing hosted fixture would still pass** — because Rust's
`is_symlink()` is true for all Windows name surrogates, which is what symlinks
and junctions are. Only a non-name-surrogate reparse point exposes it.

This fixture is therefore the **only** behavioural control for that regression.
That is what makes it worth a corrective cycle.

## 2. Authority and entry conditions

Begin only after **all** of:

- the owner explicitly approves implementation;
- `main` is green on hosted B0 and the working tree is clean;
- the two unpushed GUI commits (`f1ffa38`, `6d7fe2c`) have been pushed and
  their B0 run is green — do not stack a Windows-fixture cycle on an
  unverified tree.

No display is required. No GUI involvement.

## 3. Role split

| Role | Party | Scope |
|---|---|---|
| Implementer | mid-capability model | S1–S5 |
| Architect | high-capability model | §6 escalation; RFC 098 §9.1 amendment once the fixture passes |
| Integrator | nabbisen | Reviews, commits, pushes, observes B0 |

## 4. Slice A — inline-test extraction (independent, may be skipped)

`crates/aaai/src/diff/path_boundary.rs` is 877 lines / 796 total ELOC, of which
**499 is production** and the rest is an inline `#[cfg(test)] mod tests` block
starting at `:552`. It is the only file in `crates/aaai/src/diff/` that does
this — `ignore.rs:80` and `progress.rs:51` both declare `mod tests;` against a
sibling directory. This violates DEC-003 and the project's testing rules.

**This slice is independent of Slice B.** Under the B1-first decision it is no
longer a prerequisite — that argument applied to the structural guard, which is
not being built. Ship it, defer it, or drop it as the owner prefers.

**A1.** Move the inline `mod tests { … }` body to
`crates/aaai/src/diff/path_boundary/tests.rs`, removing one indentation level.
Open the new file with `use super::*;`, matching
`crates/aaai/src/diff/ignore/tests.rs:1`.

**A2.** Replace the inline block in `path_boundary.rs` with:

```rust
#[cfg(test)]
mod tests;
```

Anchor on the `#[cfg(test)]` attribute preceding `mod tests {`, not on line
numbers.

**A3 — verify.** Test counts **unchanged**: aaai 144, CLI unit 8, CLI
integration 91, GUI 27, doctests 3. Tests move; none is added, removed, or
renamed. `path_boundary.rs` should land at ~551 lines / 499 ELOC.

**Must not:** change any test name, body, or `#[cfg]` attribute; touch
production code; combine with Slice B in one commit.

## 5. Slice B — the behavioural fixture

Separate commit. `crates/aaai/src/diff/tests.rs` only; **no production code
change**.

### B1 — Construct the reparse point

An external helper process, invoked exactly as the existing Windows fixtures
invoke `cmd.exe /C mklink /J`. PowerShell `Add-Type` with a C# `DllImport` is
the expected form. **This is not project `unsafe` and not FFI in `crates/`** —
SEC-1 and DEC-012 are unaffected, because no unsafe code enters the crate.

The tag must satisfy two bit constraints simultaneously:

| Bit | Meaning | Required |
|---|---|---|
| 31 (`0x80000000`) | Microsoft-owned tag | **clear** — a non-Microsoft tag is what makes `REPARSE_GUID_DATA_BUFFER` the correct form and lets user mode set it |
| 29 (`0x20000000`) | Name surrogate | **clear** — this is the whole point of the fixture |

A tag of `0x00000042` satisfies both. Use `REPARSE_GUID_DATA_BUFFER`:
`ReparseTag` (4 bytes) + `ReparseDataLength` (2) + `Reserved` (2) +
`ReparseGuid` (16) + payload — a 24-byte header. Open the target with write
access and issue `DeviceIoControl` with `FSCTL_SET_REPARSE_POINT`.

Create the target as an **empty** file first.

If the helper fails, the test must **fail with the Win32 error surfaced**, not
with a generic assertion. A silent or generic failure here costs another
hosted cycle to diagnose.

### B2 — Assert, reusing the accepted helper

`assert_windows_reparse(after, name)` in `crates/aaai/src/diff/tests.rs:571`
already asserts the full contract: reparse attribute present, then
`DiffType::Incomparable`, then exactly
`"[AAAI-PATH-REPARSE] Windows reparse points are not read."`, then no SHA on
either side. Reuse it — do not write a parallel assertion.

**Add one assertion this fixture uniquely enables**, before calling the helper:

```rust
assert!(metadata.file_attributes() & FILE_ATTRIBUTE_REPARSE_POINT != 0);
assert!(!metadata.file_type().is_symlink(),
        "fixture must be a NON-name-surrogate reparse point");
```

Those two lines together are the entire value of this slice. `is_symlink()` is
true for every other Windows reparse fixture in the suite; this is the only one
where the two disagree. A future change basing the decision on `is_symlink()`
fails **here and nowhere else**.

Name the test `windows_non_name_surrogate_reparse_is_rejected` — the name Part 1
deleted. It returns with a fixture that works.

### B3 — Cleanup

Delete the reparse point explicitly at the end of the test rather than relying
on `TempDir` drop. Removal of a file carrying an unknown tag is an untested
path in this project, and a cleanup failure must surface as a test failure, not
as a leaked temp directory.

## 6. Verification

```sh
cargo +1.91 test --workspace --locked
cargo +1.91 check --target x86_64-pc-windows-gnu -p aaai --tests --locked
cargo +1.91 fmt --check -p aaai
cargo +1.91 clippy -p aaai --all-targets -- -D warnings
git diff --check
git diff --stat
```

Expected after both slices: Linux/macOS aaai **144** (the new test is
`#[cfg(windows)]`), Windows aaai **132** (131 + 1), CLI 8 / 91, GUI 27,
doctests 3. Diff confined to `crates/aaai/src/diff/`.

Windows behaviour is provable only on the hosted runner. Local evidence is
compile-only, exactly as for every prior Windows fixture in this RFC.

## 7. Evidence

Append to `.git-exclude/evidence/098-selected-folder-and-symlink-policy/`:

- `hosted-runs.md` — the B0 run for the integrated SHA, in the existing format,
  retaining all prior attempts;
- `platform-results.md` — replace the Windows "deferred" paragraph with the
  observed result, including the reparse tag actually set and confirmation that
  `is_symlink()` was false while the reparse attribute was set;
- `scope.diffstat` — the Part 2 boundary.

## 8. Stop and escalate

Stop and report rather than working around, when:

- `FSCTL_SET_REPARSE_POINT` is rejected on `windows-2025-vs2026` — capture the
  exact Win32 error. **Do not** substitute a structural text-scan guard on your
  own initiative; that is an architect decision and the fallback is recorded in
  `.git-exclude/reviewed/036-rfc098-part2-preparation-2026-07-28.md`;
- the helper requires elevation, a service, or a daemon;
- cleanup cannot remove the reparse point;
- the fixture would need production code to change;
- test counts move other than the single added Windows test;
- `is_symlink()` returns **true** for the constructed tag — that would mean the
  tag was not what it appears and the fixture proves nothing.

## 9. Rollback

Slices A and B are independently revertible by design. Before integration:
discard the working tree. After integration: revert the offending commit
through the normal reviewed path and correct at a new SHA. A deterministic
hosted failure is never rerun unchanged as acceptance evidence.

## 10. What the architect does after this lands

Once the fixture passes on hosted Windows, I amend RFC 098 §9.1 to replace the
"deferred, not waived" paragraphs with the discharged result, and update
`ROADMAP.md`'s S2 gate-evidence row, which currently names this fixture as an
outstanding S2 obligation. **Do not edit either document as part of this
work** — the RFC records the decision, and amending it is the architect's
responsibility under the project's governance model.
