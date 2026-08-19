# RFC 107 — Formatting policy: developer handoff

**Historical — RFC 107 is implemented and in `main`** (reformat `706f8a7`,
policy record `b981f19`). Nothing here is outstanding.

Companion to [`RFC 107`](../../done/107-formatting-policy.md). The RFC
records what was decided and why; this records how to do it. It must not
override the RFC.

## 1. Authority and entry conditions

**Owner decision of record:** adopt rustfmt — nabbisen, 2026-08-10, in session.
**RFC approved by the owner 2026-08-10.**

Begin when `main` is green and the working tree is clean. **Do this before any
other work starts** — RFC 107 §3 schedules it now precisely because nothing is
in flight, and a 66-file reformat collides with anything that is.

## 2. Role split

| Role | Party | Scope |
|---|---|---|
| Implementer | mid-capability model | Both commits |
| Integrator | nabbisen | Reviews, commits, pushes, observes B0 |
| Architect | RFC 107 author | Owns any RFC change; consulted only if §6 fires |

## 3. The work — two commits, in this order

### Commit 1 — the reformat, and nothing else

```sh
cargo fmt --all
```

That is the entire change. **Do not touch anything by hand afterwards** — not
to tidy an odd wrap, not to fix a pre-existing typo you notice, not to reorder
an import rustfmt left alone.

This is not fussiness. RFC 107 §5 item 3 requires the commit be
**regenerable**: a reviewer checks out its parent, runs `cargo fmt --all`, and
must get a byte-identical result. That is the only practical way to review 3700
changed lines, and **one manual edit destroys it** — the reviewer then has to
read the whole diff to find your one change, which nobody will do properly.

If you spot something that wants fixing, note it in the review request and
leave the code alone.

Expected scale: **~66 files, ~3717 insertions, ~1945 deletions.** Treat a large
deviation as a signal something is wrong, not as a surprise to accept.

### Commit 2 — the policy record

Create **`CONTRIBUTING.md`** at the repository root — not under `docs/`, which
is the user manual. `RELEASING.md` is the existing pattern for a root-level,
English-only process document; this is its sibling.

Content, briefly:

- the project uses **rustfmt with default settings**;
- there is deliberately **no `rustfmt.toml`** — defaults, so there is nothing to
  maintain or argue about;
- run `cargo fmt --all` before committing;
- `cargo +1.91 fmt --check --all` is part of every RFC's verification list;
- the hand-alignment convention that preceded this was retired by RFC 107 on
  2026-08-10; do not reintroduce aligned `=` or aligned trailing comments.

Also create **`.git-blame-ignore-revs`** containing commit 1's SHA with a
one-line comment saying what it was. That keeps `git blame` useful:

```
# Repository-wide rustfmt adoption (RFC 107). Formatting only, no behaviour change.
<commit-1-sha>
```

## 4. Verification

After commit 1, before commit 2:

```sh
cargo +1.91 fmt --check --all                     # must exit 0
cargo +1.91 test --workspace --locked             # 146 / 13 / 97 / 27 / 3
cargo +1.91 check --target x86_64-pc-windows-gnu -p aaai --tests --locked
git diff --check
grep -rE "\.size\([0-9]" crates/aaai-gui/src/     # nothing
grep -rn "Color::from_rgb" crates/aaai-gui/src/   # nothing outside design_tokens.rs
```

**Prove regenerability yourself** rather than leaving it for review:

```sh
git stash list                       # must be empty; a stash here hides a manual edit
git checkout -B fmt-check HEAD~1
cargo fmt --all
git diff main -- . ':!*.md'          # must be EMPTY
git checkout - && git branch -D fmt-check
```

An empty diff there is the single most valuable line in your review request.

Hosted B0 confirms the other platforms. **Expect no count movement anywhere** —
146 Linux and macOS, 134 Windows.

## 5. Must not

- Edit any `.rs` file by hand in commit 1.
- Add `rustfmt.toml`.
- Combine the two commits, or add anything else to either.
- Use nightly rustfmt or any nightly-only option — MSRV is 1.91 **stable**.
- Fix Clippy findings. Explicitly out of scope (RFC 107 §8); C2's Clippy half
  is a separate decision.
- Reformat generated or vendored files if any exist outside the workspace
  members.

## 6. Stop and escalate

- `cargo fmt --all` changes a file outside the three workspace crates.
- Any test count moves, or any test fails. **A formatting change cannot alter
  behaviour** — if one appears to, stop; that is a finding, not something to
  work around.
- The regenerability check in §4 produces a non-empty diff and you cannot
  account for it.
- The diff is wildly off the expected ~66 files.
- rustfmt's version differs from the pinned 1.91 toolchain's, producing output
  the next contributor cannot reproduce.

## 7. Evidence

`.git-exclude/evidence/107-formatting-policy/`:

```
scale.diffstat        commit 1's git diff --stat
regenerable.md        the §4 reproduction, showing the empty diff
local-results.md      fmt --check, test counts, windows check, V1 greps
hosted-runs.md        the B0 run for the integrated SHA
```

`regenerable.md` is the one that matters. Everything else is routine; that one
is what makes a 3700-line diff reviewable at all.

## 8. Rollback

Commit 1 reverts cleanly on its own — it is mechanical and touches no
behaviour. Commit 2 likewise. Reverting commit 1 without commit 2 would leave
`CONTRIBUTING.md` describing a policy the tree no longer follows, so revert
both or neither.
