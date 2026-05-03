# aaai ROADMAP

## Phase 1 — Core ✅ (v0.1.0)
- Folder diff detection (Added / Removed / Modified / Unchanged / TypeChanged / Unreadable / Incomparable)
- Audit definition YAML (version 1)
- Content audit strategies: None, Checksum, LineMatch, Regex, Exact
- CLI: `aaai audit`, `aaai snap`, `aaai report`
- GUI: Opening screen, 3-pane main screen (file tree / diff viewer / inspector)
- Approval flow with mandatory reason
- Markdown and JSON report output

## Phase 2 — Quality & Completeness ✅ (v0.2.0)
- tests.rs — unit/integration tests separated per module (別紙 requirement)
- i18n — GUI multilingual: Japanese (primary) + English (別紙 requirement)
- Toast subscription — properly wired TTL management in GUI
- File tree filter — filter by status and diff type
- Batch approval — select multiple Pending entries and approve with shared reason
- Path pattern matching — glob-based audit rules (logs/*.log, build/**)
- CLI: --verbose / --quiet / --json-output

## Phase 3 — Integrations (v0.3.0)
- **Approver tracking** — approved_by / approved_at stamped on each AuditEntry at approval time
- **Expiry dates** — expires_at field; expired entries shown as warnings in CLI and GUI
- **Ticket linkage** — ticket field on AuditEntry (JIRA-123, INF-42, etc.) shown in reports
- **Ignore patterns** — .aaaiignore file (gitignore-style) to exclude paths from diff
- **Audit history** — ~/.aaai/history.jsonl records every audit run with summary counts
- **Rule templates** — built-in named templates for common audit patterns (version bump, port change…)
- **Audit profiles** — named before/after/definition combos saved to ~/.aaai/profiles.yaml
- **CLI: granular exit codes** — 0=PASSED, 1=FAILED, 2=PENDING, 3=ERROR, 4=CONFIG_ERROR
- **CLI: aaai check** — validate a definition file without running a diff
- **CLI: aaai history** — show recent audit runs from the history file
- **CLI: aaai snap --template** — apply a rule template when generating snapshots
- **GUI: inspector Phase 3 fields** — ticket, approved_by, expires_at display/edit
- **GUI: expiry warning badges** — visual indicator on expired / expiring-soon entries
- **GUI: rule template picker** — "Apply template" dropdown in inspector
- **GUI: profile manager** — Opening screen shows saved profiles for quick reload

## Phase 4 — Advanced ✅ (v0.4.0)
- **Binary diff summary** — is_binary flag + file size tracking; binary-specific diff viewer panel
- **Secret masking** — regex-based masking applied to CLI output, report generator, and diff viewer
- **Project-level config** — .aaai.yaml auto-discovery: default_definition, default_ignore, approver_name, mask_patterns
- **Parallel diff engine** — rayon-based parallel file reading for large folder performance
- **Diff statistics** — lines_added / lines_removed / lines_changed per DiffEntry, shown in CLI and GUI
- **Content size warnings** — warn when Exact / LineMatch strategies are applied to large files (>1 MB)
- **CLI: aaai config** — init / show .aaai.yaml project config
- **CLI: --mask-secrets** flag on audit and report commands
- **GUI: binary file info panel** — show type, size, before/after hash for binary diffs
- **GUI: diff statistics bar** — lines changed count shown in diff viewer header

## Phase 5 — Polish (v0.5.0)
- Image diff preview (side-by-side pixel diff)
- Interactive progress bar for large folder scans
- Audit dashboard / summary view
- Watch mode (re-run on file change)
- Shell completion (bash / zsh / fish)

