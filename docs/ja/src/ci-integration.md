# CI/CD Integration

aaai is designed to run in CI/CD pipelines with predictable exit codes and
machine-readable output.

## Exit codes

| Code | Meaning |
|------|---------|
| 0 | PASSED — all entries OK or Ignored |
| 1 | FAILED — one or more audit failures |
| 2 | PENDING — unresolved entries (use `--allow-pending` to suppress) |
| 3 | ERROR — file-level read / compare errors |
| 4 | CONFIG_ERROR — definition file parse error |

## GitHub Actions example

```yaml
- name: Audit release artifacts
  run: |
    aaai audit \
      --left ./dist-before \
      --right ./dist-after \
      --config ./audit/release.yaml \
      --no-history
```

## SARIF annotations

Generate SARIF output to get inline file annotations in GitHub pull requests:

```yaml
- name: Run aaai audit
  run: |
    aaai report \
      --left ./before \
      --right ./after \
      --config ./audit.yaml \
      --format sarif \
      --out results.sarif

- name: Upload SARIF
  uses: github/codeql-action/upload-sarif@v3
  with:
    sarif_file: results.sarif
```

## Allowing pending entries in draft mode

During initial setup, you may want CI to pass even when entries are pending:

```sh
aaai audit --left ./before --right ./after \
           --config ./audit.yaml \
           --allow-pending --no-history
```

## Watch mode for local development

```sh
aaai watch --left ./before --right ./after --config ./audit.yaml
```

## Project-level defaults (.aaai.yaml)

Place `.aaai.yaml` at the repository root to avoid repeating flags:

```yaml
version: "1"
default_definition: "audit/audit.yaml"
default_ignore: "audit/.aaaiignore"
approver_name: "ci-bot"
mask_secrets: true
```

## Shell completion

Install completions for faster CLI use:

```sh
# Bash
aaai completions bash >> ~/.bash_completion

# Zsh
aaai completions zsh > ~/.zfunc/_aaai

# Fish
aaai completions fish > ~/.config/fish/completions/aaai.fish
```
