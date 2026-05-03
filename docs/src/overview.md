# Overview

**aaai** (audit for asset integrity) is a folder-diff auditor that requires
every detected change to carry a human-readable reason before it can be
marked as accepted.

It compares two directory trees, audits the differences against a YAML
definition of expected changes, and rejects anything that lacks an explicit
justification. This makes every accepted change traceable and explainable —
not just to the person who approved it, but to future maintainers months later.

---

## Components

| Component | Description |
|---|---|
| [`aaai-core`](https://crates.io/crates/aaai-core) | Core engine — diff, audit, report, masking |
| [`aaai-cli`](https://crates.io/crates/aaai-cli) | `aaai` command-line binary (15 commands) |
| [`aaai-gui`](https://crates.io/crates/aaai-gui) | `aaai-gui` desktop application (iced) |

---

## Key Concepts

| Term | Meaning |
|---|---|
| **Before** | The source / reference folder |
| **After** | The target / current folder (the version being audited) |
| **Audit definition** | YAML file listing expected changes with reasons |
| **Reason** | Mandatory human-readable justification for each change |
| **Strategy** | Content-audit method: None / Checksum / LineMatch / Regex / Exact |
| **Pending** | An entry exists but has no reason — human approval required |

---

## Features at a Glance

| Feature | Description |
|---|---|
| Folder diff | Parallel comparison powered by [rayon](https://crates.io/crates/rayon) |
| Audit definition | YAML with mandatory reason, ticket linkage, expiry date, approver |
| Content strategies | None / Checksum / LineMatch / Regex / Exact |
| Report formats | Markdown · JSON · HTML · [SARIF](https://sarifweb.azurewebsites.net/) |
| Secret masking | Auto-redaction of API keys, passwords, tokens (9 built-in patterns) |
| Advisory warnings | Large-file strategy, missing approver, no-strategy-on-modified |
| CLI | 15 commands — `audit` `snap` `lint` `diff` `merge` `watch` … |
| GUI | 3-pane resizable desktop UI with dark / light theme |
| CI/CD | Granular exit codes (0 = PASSED … 4 = CONFIG_ERROR), SARIF output |

---

## Quick Start

```sh
# 1. Generate a definition template from the current diff
aaai snap --left ./before --right ./after --out audit.yaml

# 2. Fill in the "reason" field for each entry, then audit
aaai audit --left ./before --right ./after --config audit.yaml
```

See [Getting Started](getting-started.md) for a full walkthrough.

---

## License

Apache-2.0 — [LICENSE](https://github.com/nabbisen/aaai/blob/main/LICENSE)
and [NOTICE](https://github.com/nabbisen/aaai/blob/main/NOTICE).
