# RFC 117 — Replace the Engine's Unmaintained Format Dependencies

**Status.** **Draft** — opened 2026-11-18 on the owner's decision that this be
*"recorded and managed as an issue in progress"*. Tracked as **RISK-007** in
`.git-exclude/specs/decision-log/decision-log.md`. Lives in `rfcs/draft/`;
moves to `proposed/` when its design is complete (§6).

**Tracks.** Gate C0 (dependency audit clean, or every exception formally
accepted). `docs/src/threat-model.md` §6.1 and §6.3.

**Design owner.** high-capability model

**Decision owner.** nabbisen, project owner

**Scheduled.** Design completes and goes to `proposed/` **when RFC 115 §S3
starts**. Implementation follows the GUI programme — RFC 106, then RFC 115
§S2b–§S4 — unless an escalation trigger in §5 fires first.

---

## 1. The problem

Two dependencies **we chose ourselves**, not ones a GUI crate brings in, are
unmaintained, and both sit on the code that reads and writes our saved files:

| Advisory | Crate | What it does for us | Since |
|---|---|---|---|
| `RUSTSEC-2026-0543` | `chrono` 0.4 | every timestamp and date in the engine, CLI and GUI — 19 source files | published 2026-09-14 |
| `RUSTSEC-2024-0449` | `unsafe-libyaml`, via `serde_yaml 0.9.34+deprecated` | parsing and writing every YAML file aaai reads or writes | carried since the threat model was published |

Neither is a vulnerability today, so `cargo audit` exits 0 and both are accepted
by ID. Neither will ever be cleared by an upgrade, because neither project is
releasing. **Accepting them by ID is a holding position, not a resolution.**
This RFC is the resolution.

They are in one RFC because they share their hardest part: **both reach file
formats other people store**, so both need the same proof that existing files
still load and that re-saving one does not rewrite it.

## 2. The persisted surface — measured

`chrono` types in `serde`-derived, persisted structs (2026-11-18):

| Format | Field | Type | Source |
|---|---|---|---|
| **Audit definition** | `approved_at` | `Option<DateTime<Utc>>` | `crates/aaai/src/config/definition.rs:129` |
| **Audit definition** | `expires_at` | `Option<NaiveDate>` | `crates/aaai/src/config/definition.rs:133` |
| **Audit definition** | `created_at` | `Option<DateTime<Utc>>` | `crates/aaai/src/config/definition.rs:142` |
| **Audit definition** | `updated_at` | `Option<DateTime<Utc>>` | `crates/aaai/src/config/definition.rs:146` |
| Profile store (`profiles.yaml`) | `last_used_at` | `Option<DateTime<Utc>>` | `crates/aaai/src/profile/store.rs:24` |
| History | `run_at` | `DateTime<Utc>` | `crates/aaai/src/history/record.rs:10` |

**The audit definition is the one that matters most.** Users commit it to their
own repositories, and it is the file whose every entry carries a human-written
reason. If a replacement serializes `2026-11-18T09:00:00Z` as
`2026-11-18T09:00:00+00:00`, or adds or drops fractional seconds, then the first
save after upgrading rewrites every timestamp in a file the user did not mean to
change, and their review history fills with noise. **Byte-stable round-tripping
of existing files is therefore a requirement, not a nicety.**

`serde_yaml`'s surface is wider: every YAML file — the definition, `prefs.yaml`,
`profiles.yaml`, history — plus `rust-i18n`'s build-time locale loading, which
pulls `serde_yaml` in separately (`docs/src/threat-model.md` §6.1). Measuring it
is §6's first task.

## 3. Candidates — not yet chosen

- **`chrono` → `jiff`.** The advisory recommends it, and it is **already in our
  dependency graph** (0.2.28, transitively), so it adds no new supply-chain
  party. `jiff::Timestamp` and `jiff::civil::Date` map to `DateTime<Utc>` and
  `NaiveDate`. Whether its `serde` output matches chrono's byte for byte, for
  every value our files can hold, is unverified and is the deciding question.
  `time` is the other recommended alternative.
- **`serde_yaml` → not yet assessed.** The successor question was raised at
  the threat model's publication and never decided.

## 4. Requirements any design must meet

1. **Every existing file loads.** A corpus of real definition, profile and
   history files from every released format version, round-tripped in CI.
2. **Re-saving an unchanged file changes no byte.** Same corpus.
3. **No new supply-chain party without a reason recorded.** Prefer what is
   already in the graph (`jiff` is).
4. **Gate C0 closes for both** — no accepted-by-ID row remains for either.
5. **CLI and GUI agree** on every timestamp they display, before and after.

## 5. While this waits — escalation triggers

Waiting is safe only while both stay *unmaintained* rather than *vulnerable*.
**Any of these moves this RFC ahead of the GUI programme immediately:**

- a **vulnerability** (not an unmaintained notice) published against `chrono`,
  `serde_yaml` or `unsafe-libyaml`;
- a Rust or MSRV change that stops either building;
- a downstream request to read or write our formats with another tool, which
  would make format stability a published commitment.

`cargo audit` runs on every CI push against the live advisory database, so the
first trigger surfaces as a red `Security audit` job. **That job is watched to
completion after every push.**

## 6. To complete the design (next steps)

1. Measure `serde_yaml`'s full surface, including `rust-i18n`'s.
2. Build the round-trip corpus: one file per released format version, from the
   repository's tags.
3. Prove or disprove `jiff`'s byte stability against that corpus, in a
   throwaway worktree, before choosing.
4. Assess a `serde_yaml` successor the same way.
5. Write the design, the acceptance contract and the migration order, and move
   this to `proposed/`.
