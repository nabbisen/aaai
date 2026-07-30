# RFC 103 — Safe output surfaces: developer handoff

Companion to [`RFC 103`](../../proposed/103-safe-output-surfaces.md). The RFC
records what was decided and why; this records how to implement and verify it.
It must not override the RFC.

## 1. Authority and entry conditions

**Owner decision of record:** approved for implementation by nabbisen on
2026-07-29, in session, together with RFC 102. RFC 103 §10's open question
on whether `Masking::Disabled` should exist was left as specified — the
variant stays, and every use must carry a written justification.

**Design review of record:**
`.git-exclude/reviewed/047-rfc102-rfc103-design-review-2026-07-29.md`
— accepted as corrected; two defects found and fixed before implementation.

Begin only after design review accepts RFC 103 **and** the owner explicitly
approves implementation. `main` green, working tree clean.

Independent of the GUI work — touches no `aaai-gui` file — so it can run
alongside RFC 099 T6/T7.

## 2. Role split

| Role | Party | Scope |
|---|---|---|
| Implementer | mid-capability model | S1–S6 |
| Integrator | nabbisen | Commits, pushes, observes B0 |
| Architect | RFC 103 author | Consulted on §4 field-set questions; owns any RFC change |

## 3. Order — guard first, and it must fail

**S1 comes first and must fail when written.** A guard authored after the fixes
tends to encode what the code already does. Written first, it encodes the
requirement.

| Slice | Content | Commit |
|---|---|---|
| S1 | Cross-surface guard (RFC §5.4) — **expected red** | 1 |
| S2 | F2 — escape `ticket` in HTML | 2 |
| S3 | F1 — CSV/TSV formula neutralisation | 3 |
| S4 | `Masking` type + thread through all four `write_*`; enable at CLI report call sites (fixes F3 **and F5**) | 4 |
| S5 | F4 — masking in `export.rs` | 5 |
| S6 | Evidence and revert-demonstration | — |

After S5 the guard must be green. Record its output at each stage — that
progression is the evidence that it is load-bearing.

## 4. S1 — the guard

Build one `AuditResult` with an adversarial value in **every** RFC §4 field:

| Field | Suggested adversarial value |
|---|---|
| `path` | `=cmd\|'/c calc'!A1` — a legal Linux filename |
| `reason` | text embedding a canary matching a masking pattern |
| `ticket` | `<img src=x onerror=alert(1)>` |
| `error_detail` | a path containing `<` and `\|` |
| root paths | a path containing `"` and a leading `-` |

Render through **every** surface — Markdown, HTML, JSON, SARIF, CSV, TSV — and
assert per surface:

- the canary secret appears **nowhere** in any output;
- HTML contains no unescaped `<`, `>`, or `"` originating from field content;
- no CSV/TSV cell begins with `=`, `+`, `-`, `@`, tab, or CR;
- no Markdown table cell contains an unescaped `|`.

Structure it as field × surface, so adding either dimension fails until
handled. Place tests per DEC-003 in a `tests.rs` sibling, never inline.

**Do not weaken an assertion to make S1 pass.** Red at S1 is the point.

## 5. S2 — F2, HTML `ticket`

`crates/aaai/src/report/html.rs`:

```rust
.map(|t| format!("<span class=\"ticket\">[{t}]</span> "))
```

Wrap `{t}` in `html_escape`, matching every sibling field. `html_escape`
already covers `&`, `<`, `>`, `"`, `'` — do not modify it.

While here, audit every other interpolation in the file for the same omission
and report what you find, even if the answer is none.

## 6. S3 — F1, CSV/TSV

`crates/aaai-cli/src/cmd/export.rs`, `csv_escape`:

- Prefix a single apostrophe when the field's first character is `=`, `+`, `-`,
  `@`, tab, or CR.
- Add `\r` to the quoting trigger alongside `\n`.
- **Keep** RFC 4180 quoting. It is not a substitute — the CSV parser consumes
  the quotes before the spreadsheet evaluates the cell — and removing it breaks
  fields containing separators.

Apply neutralisation before quoting, so the apostrophe is inside the quoted
field.

## 7. S4 — the `Masking` type

Add to the masking module:

```rust
pub enum Masking<'a> {
    Enabled(&'a MaskingEngine),
    /// Caller asserts the sink is trusted. Justify at the call site.
    Disabled,
}
```

Replace `masker: Option<&MaskingEngine>` on `write_markdown`, `write_json`,
`write_html`, and their `build_*` helpers. **Add the parameter to
`write_sarif`**, which never had one, and thread it into `build_sarif` — that
is F3.

**No source-compatibility shim.** No `Default` impl, no
`From<Option<&MaskingEngine>>`, no `Into`-based signature. Any of those would
let existing `None` call sites keep compiling, preserving the silent default
that caused F3, F4, and F5. Breaking them is the mechanism.

**F5 — `build_json` ignores its masker.** Its parameter is `_masker` and
`"reason"` is emitted raw. Use it, exactly as `md_entry` does.

**Call-site decisions are already made. Do not re-derive them.**

| Call site | Value |
|---|---|
| `cmd/report.rs` — all four formats (`:55`, `:67`, `:73`, `:78`) | **`Enabled`** — build the engine as `cmd/audit.rs:98` does, so custom patterns are honoured |
| `aaai-gui/src/app.rs` `:1139`, `:1143` | **`Disabled`**, justified as "GUI report export is out of RFC 103 scope; carries the same gap, tracked as follow-up" |

The `report.rs` conversion is a **behaviour change approved by the owner on
2026-07-29**: `aaai report` will now redact where a secret pattern matches. It
has never masked any file output, on any format. See RFC 103 §5.1a.

The GUI justification is honest because it states a limitation rather than
asserting the sink is trusted — `App` constructs no `MaskingEngine`, so
enabling it there is more than a mechanical change and is deliberately deferred.

## 8. S5 — F4, export masking

`export.rs` reads `entry.map(|e| e.reason.as_str())` raw and never masks.
Route it through the same masking decision as the report surfaces rather than
adding a second mechanism.

## 9. S6 — evidence

Create `.git-exclude/evidence/103-safe-output-surfaces/`:

```
guard-progression.md   guard output after S1 (red) through S5 (green)
revert-demo.md         each fix reverted in turn, showing the guard fails
findings-map.md        F1-F4 mapped to test names and commits
local-results.md       fmt, clippy, test, i18n, diff --check
scope.diffstat
hosted-runs.md         the B0 run for the integrated SHA
```

`revert-demo.md` is the important one. RFC §6.3 requires proof the guard fails
when any single fix is reverted — otherwise it may be testing the
implementation rather than the property.

## 10. Verification

```sh
cargo +1.91 test --workspace --locked
cargo +1.91 check --target x86_64-pc-windows-gnu -p aaai --tests --locked
cargo +1.91 fmt --check -p aaai -p aaai-cli
cargo +1.91 clippy -p aaai -p aaai-cli --all-targets -- -D warnings
python3 scripts/check-i18n-keys.py
git diff --check

# RFC 099 V1 must not regress — app.rs is touched
grep -rE "\.size\([0-9]" crates/aaai-gui/src/          # must return nothing
grep -rn "Color::from_rgb" crates/aaai-gui/src/          # nothing outside design_tokens.rs

# masking must now actually be enabled on the CLI report path
grep -rn "Masking::Enabled" crates/aaai-cli/src/cmd/report.rs   # at least one
# Presence, not a count: binding `Masking::Enabled(&masker)` once and reusing
# it across the four call sites is preferred to inlining it four times, so a
# count assertion would penalise the better shape. The four end-to-end tests
# in crates/aaai-cli/tests/cli.rs are the real proof that each format masks.
```

Counts grow only by the named new tests; report the new totals explicitly. No
i18n key delta. No diff outside the files in RFC 103's Touches line.

## 11. Must not

- Weaken a guard assertion to obtain a pass.
- Change masking patterns — the defect is coverage, not patterns.
- Mask inside the engine before surfaces see the data; that corrupts the audit
  result and breaks diffing.
- Add a dependency, CLI flag, or persisted-format change.
- Touch `crates/aaai-gui/` **beyond the two masker arguments at `app.rs:1139`
  and `:1143`**. That narrow exception exists only because the §5.1 signature
  change makes those lines non-compiling; it was added on 2026-07-29 after the
  implementer correctly flagged the conflict. Nothing else in the file may
  change, and RFC 099's V1 greps must still pass — run them and include the
  output.
- Change report content or layout beyond the required escaping.

## 12. Stop and escalate

- An RFC §4 field cannot be masked without corrupting output.
- The apostrophe prefix breaks a legitimate value.
- Removing `Option` forces a public API change beyond the four `write_*`
  signatures.
- The guard cannot distinguish escaped from unescaped content for a surface.
- A fifth defect of this class appears — report it; do not fix it silently
  under this RFC.

## 13. Rollback

Five independent commits, revertible in reverse order. S1 alone is inert — a
failing test with no production change — so it can land and be reverted without
risk if the remainder is deferred.
