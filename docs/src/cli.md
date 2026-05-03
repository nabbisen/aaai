# CLI Reference

`aaai` provides 15 commands. Use `--help` on any command for full details.

```sh
aaai --help
aaai <command> --help
```

---

## Exit Codes (`aaai audit`)

| Code | Meaning |
|---|---|
| 0 | PASSED — all entries OK or Ignored |
| 1 | FAILED — one or more audit failures |
| 2 | PENDING — unresolved entries (`--allow-pending` to suppress) |
| 3 | ERROR — file-level read / compare errors |
| 4 | CONFIG_ERROR — definition file parse error |

---

## Commands by Category

### [Auditing](cli-auditing.md)

| Command | Description |
|---|---|
| [`aaai audit`](cli-auditing.md#aaai-audit) | Compare folders and audit against the definition |
| [`aaai snap`](cli-auditing.md#aaai-snap) | Generate a definition template from the current diff |
| [`aaai check`](cli-auditing.md#aaai-check) | Validate a definition file without running a diff |
| [`aaai lint`](cli-auditing.md#aaai-lint) | Best-practice linter for definition files |

### [Reporting & Export](cli-reporting.md)

| Command | Description |
|---|---|
| [`aaai report`](cli-reporting.md#aaai-report) | Write a Markdown / JSON / HTML / SARIF report |
| [`aaai diff`](cli-reporting.md#aaai-diff) | Raw folder diff without a definition |
| [`aaai export`](cli-reporting.md#aaai-export) | Export audit entries to CSV or TSV |

### [Workflow](cli-workflow.md)

| Command | Description |
|---|---|
| [`aaai merge`](cli-workflow.md#aaai-merge) | Merge two definition files |
| [`aaai history`](cli-workflow.md#aaai-history) | Show recent audit runs |
| [`aaai dashboard`](cli-workflow.md#aaai-dashboard) | Colour-coded statistics dashboard |
| [`aaai watch`](cli-workflow.md#aaai-watch) | Re-run audit on file changes |

### [Setup & Tooling](cli-setup.md)

| Command | Description |
|---|---|
| [`aaai init`](cli-setup.md#aaai-init) | Interactive project setup wizard |
| [`aaai config`](cli-setup.md#aaai-config) | Show or create `.aaai.yaml` project config |
| [`aaai version`](cli-setup.md#aaai-version) | Print version and build information |
| [`aaai completions`](cli-setup.md#aaai-completions) | Generate shell completion scripts |
