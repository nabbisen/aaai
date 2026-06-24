# Auditing Commands

Commands for running and inspecting audits.

## aaai audit

Compare two folders and audit the differences against an audit
definition file. This is the central command of `aaai`.

```sh
aaai audit --left <BEFORE> --right <AFTER> --config <FILE> [OPTIONS]
```

| Flag | Description |
|---|---|
| `-l, --left <PATH>` | The "Before" folder (baseline) |
| `-r, --right <PATH>` | The "After" folder (what you're auditing) |
| `-c, --config <FILE>` | The audit definition (YAML) |
| `--ignore <FILE>` | Path to a `.aaaiignore`-style exclusion file |
| `--verbose` | Also show OK / Ignored entries and their reasons |
| `--quiet` | Print only the summary line |
| `--json-output` | Emit results as JSON instead of human-readable text |
| `--allow-pending` | Treat Pending entries as success (exit 0) |
| `--mask-secrets` | Redact values that look like secrets in reasons and rule text |
| `--progress` | Show a progress bar while comparing folders |
| `--no-history` | Don't record this run in the history file |

**Exit codes:** see [`aaai exit-codes`](cli-setup.md#aaai-exit-codes)
for the canonical table.

**Examples:**

```sh
# A basic audit
aaai audit --left ./before --right ./after --config audit.yaml

# CI/CD friendly: JSON output, no history record
aaai audit --left ./before --right ./after --config audit.yaml \
           --json-output --no-history

# With ignore rules and secret masking
aaai audit --left ./before --right ./after --config audit.yaml \
           --ignore .aaaiignore --mask-secrets
```

After an audit run, `aaai audit` ends with a "Next steps:" hint
pointing to the most useful follow-up — for example, telling you
which entries are still Pending and need a reason. The hint adapts
to the verdict (Pending vs Failed vs all-clear).

---

## aaai snap

Generate an audit definition template from the current diff. This
is how you create your first `audit.yaml` — `aaai snap` looks at
what's changed and produces a YAML scaffold with one entry per
change, ready for you to fill in the `reason` field.

```sh
aaai snap --left <BEFORE> --right <AFTER> --out <FILE> [OPTIONS]
```

| Flag | Description |
|---|---|
| `--merge` | Merge new entries into the existing file (don't overwrite) |
| `--template <ID>` | Apply a rule template (see `--list-templates`) |
| `--list-templates` | List the available templates and exit |
| `--ignore <FILE>` | Path to a `.aaaiignore`-style exclusion file |
| `--approver <NAME>` | Pre-set `approved_by` on generated entries |
| `--suggest-glob` | Suggest glob patterns where multiple entries share a structure |
| `--dry-run` | Print what would be written, don't touch the file |

**Examples:**

```sh
# First snapshot — creates audit.yaml from scratch
aaai snap --left ./before --right ./after --out audit.yaml

# Apply the "version bump" template to detected version-number changes
aaai snap --left ./before --right ./after --out audit.yaml \
          --template version_bump

# Set the approver and ask for glob suggestions
aaai snap --left ./before --right ./after --out audit.yaml \
          --approver "alice" --suggest-glob
```

Note that `aaai snap` **never auto-approves**. Every generated entry
starts with an empty `reason`, so the next step is always to review
each entry and fill in why the change is allowed.

---

## aaai check

Validate that an audit definition file is well-formed, without
running an actual audit.

```sh
aaai check <FILE> [--all]
```

| Flag | Description |
|---|---|
| `--all` | Also show entries that passed validation |

By default `aaai check` prints only the problems. Useful as a
pre-commit hook or a CI gate to catch malformed audit files before
they cause a confusing audit failure downstream.

---

## aaai lint

Apply best-practice checks to an audit definition file.

```sh
aaai lint <FILE> [OPTIONS]
```

| Flag | Description |
|---|---|
| `--require-ticket` | Require every entry to have a `ticket` field |
| `--require-approver` | Require every entry to have an `approved_by` field |
| `--min-reason-len <N>` | Minimum length of the `reason` field (default: 10) |
| `--json-output` | Emit findings as JSON |

**Findings:**

| ID | Severity | Meaning |
|---|---|---|
| `duplicate-path` | error | Two or more entries share the same `path` |
| `empty-linematch` | error | A LineMatch strategy has no rules |
| `empty-line-rule` | error | A LineMatch rule has an empty `line:` |
| `short-reason` | warning | `reason` shorter than `--min-reason-len` |
| `missing-ticket` | warning | `ticket` missing when `--require-ticket` is set |
| `missing-approver` | warning | `approved_by` missing when `--require-approver` is set |
| `expired` | warning | `expires_at` is in the past |
| `strategy-mismatch` | info | LineMatch on an Added/Removed entry (LineMatch only makes sense for Modified) |
| `disabled` | info | An entry has `disabled: true` |

`aaai lint` exit codes mirror its findings: any `error` causes a
non-zero exit; warnings and info-level findings do not. Combine with
`--json-output` for structured CI consumption.
