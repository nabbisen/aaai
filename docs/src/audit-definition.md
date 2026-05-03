# Audit Definition File

The audit definition (`audit.yaml`) lists every expected change alongside
a human-readable reason and an optional content strategy.
Without a reason, an entry is treated as **Pending** — the audit does not pass.

---

## File Structure

```yaml
version: "1"
meta:
  description: "Release 2.0.0 delta audit"

entries:
  - path: "config/server.toml"
    diff_type: Modified
    reason: "Port changed from 80 to 8080 — INF-42"
    ticket: "INF-42"
    approved_by: "alice"
    approved_at: "2026-05-01T09:00:00Z"
    expires_at: "2026-12-31"
    strategy:
      type: LineMatch
      rules:
        - action: Removed
          line: "port = 80"
        - action: Added
          line: "port = 8080"
    enabled: true
```

---

## Entry Fields

| Field | Required | Description |
|---|---|---|
| `path` | ✅ | File path or glob pattern (e.g. `logs/*.log`) |
| `diff_type` | ✅ | `Added` / `Removed` / `Modified` / `Unchanged` / `TypeChanged` |
| `reason` | ✅ | Human-readable justification — empty means Pending |
| `strategy` | — | Content audit method (default: `None`) |
| `ticket` | — | Issue reference (JIRA-123, INF-42, …) |
| `approved_by` | — | Approver name or ID |
| `approved_at` | — | ISO-8601 UTC timestamp of approval |
| `expires_at` | — | Re-review date `YYYY-MM-DD` |
| `enabled` | — | `false` makes this entry Ignored during audit |
| `note` | — | Free-form supplementary note (not used in judgement) |
| `created_at` | — | Auto-stamped when first approved |
| `updated_at` | — | Auto-stamped on every approval |

---

## Glob Patterns

Exact-path entries always take priority. Globs are matched when no exact
entry is found.

```yaml
entries:
  - path: "logs/*.log"
    diff_type: Modified
    reason: "Log rotation on every deploy"
    strategy:
      type: None
```

---

## Generating a Definition

```sh
# Snapshot the current diff into a starter definition
aaai snap --left ./before --right ./after --out audit.yaml

# Apply a named template (e.g. version_bump)
aaai snap --left ./before --right ./after --out audit.yaml \
          --template version_bump --approver "alice"
```

Generated entries have an empty `reason` — fill them in before auditing.

---

## Validating a Definition

```sh
# Syntax check only
aaai check audit.yaml

# Best-practice lint (duplicate paths, empty rules, short reasons, …)
aaai lint audit.yaml
aaai lint audit.yaml --require-ticket --require-approver
```

---

## Content Audit Strategies

See [Content Audit Strategies](strategies.md) for full details.

| Strategy | Summary |
|---|---|
| `None` | Verify diff type only — no content inspection |
| `Checksum` | SHA-256 digest match — best for binary files |
| `LineMatch` | Specific lines added/removed — best for config changes |
| `Regex` | Changed lines match a pattern — best for version numbers, dates |
| `Exact` | Full file content match — best for small, stable files |

---

## AuditWarning Advisories

aaai emits non-fatal advisories alongside results:

| ID | Trigger |
|---|---|
| `large-file` | `Exact` or `LineMatch` applied to a file > 1 MB |
| `no-strategy` | `None` strategy on a `Modified` file |
| `no-approver` | Entry has a reason but no `approved_by` field |

Suppress specific kinds via `.aaai.yaml`:

```yaml
suppress_warnings:
  - no-approver
```
