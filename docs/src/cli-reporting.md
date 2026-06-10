# Reporting & Export Commands

Commands for generating reports and exporting data from an audit run.

## aaai report

Emit audit results as a report file in one of several formats.

```sh
aaai report --left <BEFORE> --right <AFTER> --config <FILE> \
            --out <FILE> [--format markdown|json|html|sarif]
```

| Flag | Description |
|---|---|
| `--format` | markdown (default) / json / html / sarif |
| `--include-diff` | Embed the actual diff text into Markdown / HTML output |
| `--mask-secrets` | Mask values that look like secrets in the rendered report |

**Examples:**

```sh
aaai report --left ./before --right ./after --config audit.yaml \
            --format html --out report.html

# SARIF for GitHub Actions
aaai report --left ./before --right ./after --config audit.yaml \
            --format sarif --out results.sarif
```

The SARIF format makes the audit results appear inline in pull
request reviews on platforms that consume SARIF (GitHub, GitLab,
Azure DevOps, etc.).

---

## aaai diff

Show the raw diff between two folders, without consulting any audit
definition.

```sh
aaai diff --left <BEFORE> --right <AFTER> [OPTIONS]
```

| Flag | Description |
|---|---|
| `--content` | Show the actual changed-line content (not just the path summary) |
| `--all` | Include Unchanged entries (not only the differences) |
| `--json-output` | Emit results as JSON instead of human-readable text |

Useful for ad-hoc inspection or as a sanity check before running a
full audit.

---

## aaai export

Export approved audit entries as a CSV or TSV table.

```sh
aaai export --left <BEFORE> --right <AFTER> --config <FILE> \
            [--out <FILE>] [--format csv|tsv] [--all]
```

| Flag | Description |
|---|---|
| `--format` | csv (default) / tsv |
| `--out` | Output file path. If omitted, writes to stdout. |
| `--all` | Include all entries, not only Approved ones. |

The output schema is documented in the [Compatibility
Policy](compatibility.md). The columns are stable across v1.x.
