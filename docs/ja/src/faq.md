# Frequently Asked Questions

## Why does aaai require a reason for every entry?

The central value of aaai is *explainability*. A diff without a reason is
just a change — it tells you *what* happened but not *why* it was accepted.
The mandatory reason transforms each entry from a technical fact into a
human-readable decision record that future maintainers (including your future
self) can understand.

## How do I handle files I never want to audit?

Add them to `.aaaiignore` using gitignore-style patterns:

```
# Build artifacts
target/**
dist/**
*.lock

# OS files
.DS_Store
Thumbs.db
```

## Can I use glob patterns in audit rules?

Yes. The `path` field accepts glob patterns:

```yaml
- path: "logs/*.log"
  diff_type: Modified
  reason: "Log files rotate on every deploy"
  strategy:
    type: None
```

Exact-path entries always take priority over glob entries when both match.

## What happens when an entry expires?

Expired entries (`expires_at` in the past) still produce OK verdicts during
audit — expiry is a *reminder*, not an enforcement mechanism. The CLI
and GUI both display warnings so you know to review them.

## How do I merge definitions from two teams?

```sh
aaai merge base.yaml overlay.yaml --out merged.yaml
```

The overlay wins on conflicts. Use `--detect-conflicts` first to see what
would be overwritten.

## Can I generate completions for my shell?

Yes — see [CI/CD Integration](ci-integration.md#shell-completion).

## How do I embed diff text in reports?

```sh
aaai report --left ./before --right ./after \
            --config ./audit.yaml --out report.md \
            --include-diff
```

This embeds a `diff`-formatted block for every Modified text file.

## Is aaai safe to run in CI on untrusted code?

aaai reads files but never executes them. It does write to:
- The audit definition file when saving.
- `~/.aaai/history.jsonl` (unless `--no-history` is passed).

It never modifies the files it is comparing.

## What is SARIF output used for?

SARIF (Static Analysis Results Interchange Format) is a JSON standard that
GitHub, GitLab, and Azure DevOps use to show inline annotations on pull
requests. Generate SARIF with `--format sarif` and upload it via your CI
provider's upload action to get per-file, per-line annotations.
