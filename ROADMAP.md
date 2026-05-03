# aaai ROADMAP

## Phase 1 — Core ✅ (v0.1.0)
- Folder diff detection (Added / Removed / Modified / Unchanged / TypeChanged / Unreadable / Incomparable)
- Audit definition YAML (version 1)
- Content audit strategies: None, Checksum, LineMatch, Regex, Exact
- CLI: `aaai audit`, `aaai snap`, `aaai report`
- GUI: Opening screen, 3-pane main screen (file tree / diff viewer / inspector)
- Approval flow with mandatory reason
- Markdown and JSON report output

## Phase 2 — Quality & Completeness (v0.2.0)
- **tests.rs** — unit/integration tests separated per module (per 別紙 requirement)
- **i18n** — GUI multilingual support: Japanese (primary) + English (per 別紙 requirement)
- **Toast subscription** — properly wired TTL management in GUI
- **File tree filter** — filter by status (OK / Pending / Failed / Error) and diff type
- **Batch approval** — select multiple Pending entries and approve with shared reason
- **Path pattern matching** — glob-based audit rules that match multiple paths
- **CLI: --verbose / --json-output** — structured output for audit command
- **Subscription** — properly wired into iced application lifecycle

## Phase 3 — Integrations (v0.3.0)
- Audit history management
- Approver tracking (name / timestamp)
- Ticket number linkage
- CI/CD detailed exit code modes
- Rule templates
- Audit profiles

## Phase 4 — Advanced (v0.4.0)
- Image diff review
- Binary diff summary
- Secret masking in reports
- Expiry dates on permitted diffs
- Large folder progress display
- Project-level config presets
