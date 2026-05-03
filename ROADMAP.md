# aaai ROADMAP

## Phase 1 — Core (current)
- Folder diff detection (Added / Removed / Modified / Unchanged / TypeChanged / Unreadable)
- Audit definition YAML (version 1)
- Content audit strategies: None, Checksum, LineMatch, Regex, Exact
- CLI: `aaai audit`, `aaai snap`, `aaai report`
- GUI: Opening screen, 3-pane main screen (file tree / diff viewer / inspector)
- Approval flow with mandatory reason
- Markdown and JSON report output

## Phase 2 — Usability
- Batch approval with shared reason
- Rule templates
- Audit profiles
- Path pattern matching

## Phase 3 — Integrations
- CI/CD detailed exit codes
- Audit history
- Approver tracking
- Ticket number linkage
