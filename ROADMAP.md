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
- **Shell completion** — `aaai completions <shell>` for bash/zsh/fish/powershell via clap_complete
- **Watch mode** — `aaai watch` re-runs audit when before/after/definition files change (notify crate)
- **Progress tracking** — channel-based DiffProgress events; CLI progress bar via indicatif
- **HTML report** — third output format alongside Markdown and JSON
- **`aaai snap --dry-run`** — preview what would be generated without writing
- **`aaai dashboard`** — CLI summary dashboard with colour-coded stat cards
- **GUI: dashboard view** — summary cards as the default landing pane before selecting a file
- **GUI: file tree search** — incremental path filter input above the file tree
- **CLI integration tests** — end-to-end tests driving the real binary
- **Image diff (basic)** — detect image type for binary panel (PNG/JPEG/GIF/WebP) and show metadata

## Phase 6 — Production Readiness (v0.6.0)
- **Entry versioning** — created_at / updated_at auto-stamped on AuditEntry at approval time
- **`aaai diff`** — raw folder diff without an audit definition (plain change listing)
- **`aaai merge`** — merge two audit definition files with conflict detection
- **SARIF output** — SARIF v2.1.0 report format for GitHub/GitLab CI annotations
- **Report --include-diff** — optionally embed actual diff text in Markdown / HTML reports
- **GitHub Actions CI/CD** — .github/workflows/ci.yaml and release.yaml
- **GUI keyboard shortcuts** — Ctrl+S (save), Ctrl+R (re-run), Escape (deselect), / (search)
- **Undo last approval** — revert the most recently upserted entry
- **Complete documentation** — all docs/src/ chapters populated (strategies, CI integration, FAQ)
- **`aaai completions --install`** — auto-install completion script to shell config

## Phase 7 — v1.0 Quality (v0.7.0)
- **Non-blocking async diff** — tokio::spawn-based diff engine; GUI remains interactive during large scans
- **GUI theme support** — light / dark / system-follow theme selection stored in ~/.aaai/prefs.yaml
- **`aaai init` wizard** — interactive project setup: define before/after paths, create .aaai.yaml, generate starter definition
- **History analytics** — `aaai history --stats` showing trend graphs (pass rate, pending counts over time)
- **Large file warnings** — emit AuditWarning when Exact or LineMatch strategy applied to files >1 MB
- **`aaai export`** — export audit entries to CSV / TSV for external review in spreadsheets
- **Audit write locking** — `.aaai.lock` prevents concurrent definition file writes
- **Zero-warning build** — all cargo fix suggestions applied; Clippy clean at -D warnings level
- **`aaai snap --approver`** — pre-fill approver_name from project config or flag when generating entries
- **GUI: resizable panes** — drag handles between file tree / diff view / inspector

## Phase 8 — v1.0 comes closer (v0.8.0)
- **`aaai snap --approver`** — pre-fill approved_by from project config or --approver flag
- **Async GUI diff** — tokio-based non-blocking diff in aaai-gui; spinner during scan
- **`aaai version`** — detailed version info: crate version, build profile, Rust toolchain
- **`aaai lint`** — best-practice linter for definition files; emits categorized warnings
- **Property-based tests** — proptest fuzzing on masking engine, ignore rules, glob matching
- **Benchmarks** — criterion benchmarks for diff engine and masking engine
- **`aaai snap --suggest-glob`** — detect common path prefixes and suggest glob rules
- **README badges** — test count, version, license, CI status shields
- **v1.0.0 release prep** — full Apache-2.0 LICENSE text, AUTHORS, Cargo.lock finalised

## Phase 9 — Documentation & Test Completeness (v0.9.0)
- **Complete docs** — gui.md (full 3-pane walkthrough), cli.md (all 15 commands with examples), getting-started.md updated with aaai init
- **CLI test coverage** — integration tests for: completions, config --init, dashboard, init --non-interactive, lint --json-output, version --json-output
- **AuditWarning suppression** — suppress_warnings: list in .aaai.yaml; CLI --suppress-warnings flag on audit
- **History rotation** — --max-entries on history command; history store prune() method
- **report --no-history** — --no-history flag for report command (already on audit)
- **aaai audit --warn-only** — exit 0 even with warnings (do not fail on warning-only conditions)

