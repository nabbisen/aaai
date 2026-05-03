# FAQ

---

## Why does aaai require a reason for every entry?

The central value proposition of aaai is **explainability**. A diff without a
reason tells you *what* changed but not *why* it was accepted. The mandatory
reason transforms each entry from a technical fact into a decision record that
future maintainers can understand.

---

## How do I exclude files from the diff?

Add patterns to an `.aaaiignore` file (gitignore syntax):

```
# Build artefacts
target/**
dist/**
*.lock

# OS files
.DS_Store
Thumbs.db
```

When left blank, `Before/.aaaiignore` is searched automatically.
Use `--ignore` on the CLI or the dedicated field in the GUI Opening screen.

---

## Can I use glob patterns in audit rules?

Yes. The `path` field accepts glob patterns:

```yaml
- path: "logs/*.log"
  diff_type: Modified
  reason: "Log rotation on every deploy"
  strategy:
    type: None
```

Exact-path entries always take priority over glob entries when both match.

---

## What happens when an entry expires?

Expired entries (`expires_at` in the past) still produce OK verdicts — expiry
is a *reminder*, not an enforcement mechanism. Both the CLI and GUI display
warnings so you know to re-review them.

---

## How do I merge definitions from two teams?

```sh
aaai merge base.yaml overlay.yaml --out merged.yaml
```

The overlay wins on conflicts. Use `--detect-conflicts` first to preview what
would be overwritten.

---

## How do I embed the actual diff text in a report?

```sh
aaai report --left ./before --right ./after \
            --config ./audit.yaml --out report.md \
            --include-diff
```

This embeds a `diff`-format block for every Modified text file.

---

## What is SARIF output used for?

SARIF (Static Analysis Results Interchange Format) is a JSON standard that
GitHub, GitLab, and Azure DevOps use to show per-file, per-line annotations
on pull requests. Generate SARIF with `--format sarif` and upload it via your
CI provider's upload action.

---

## How do I prevent secrets from appearing in reports?

```sh
aaai audit --mask-secrets ...
```

Or set it permanently in `.aaai.yaml`:

```yaml
mask_secrets: true
```

Nine built-in patterns cover API keys, passwords, AWS keys, GitHub tokens,
Bearer tokens, private key headers, and more. Add custom patterns with
`custom_mask_patterns`.

---

## How do I suppress a specific AuditWarning kind?

In `.aaai.yaml`:

```yaml
suppress_warnings:
  - no-approver
  - no-strategy
```

Or via the CLI flag:

```sh
aaai audit --suppress-warnings no-approver,no-strategy ...
```

---

## Does aaai modify the files it compares?

No. aaai reads files but never modifies the compared directories. The only
writes are:

- The audit definition file (when saving approvals)
- `~/.aaai/history.jsonl` (disable with `--no-history`)
- `~/.aaai/prefs.yaml` (GUI theme preference)

---

## The history file is growing large. How do I prune it?

```sh
# Keep only the most recent 100 entries
aaai history --prune 100
```
