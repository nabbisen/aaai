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

## Phase 3 — Integrations ✅ (v0.3.0)
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

## Phase 5 — Polish ✅ (v0.5.0)
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

## Phase 6 — Production Readiness ✅ (v0.6.0)
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

## Phase 7 — v1.0 Quality ✅ (v0.7.0)
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

## Phase 8 — v1.0 comes closer ✅ (v0.8.0)
- **`aaai snap --approver`** — pre-fill approved_by from project config or --approver flag
- **Async GUI diff** — tokio-based non-blocking diff in aaai-gui; spinner during scan
- **`aaai version`** — detailed version info: crate version, build profile, Rust toolchain
- **`aaai lint`** — best-practice linter for definition files; emits categorized warnings
- **Property-based tests** — proptest fuzzing on masking engine, ignore rules, glob matching
- **Benchmarks** — criterion benchmarks for diff engine and masking engine
- **`aaai snap --suggest-glob`** — detect common path prefixes and suggest glob rules
- **README badges** — test count, version, license, CI status shields
- **v1.0.0 release prep** — full Apache-2.0 LICENSE text, AUTHORS, Cargo.lock finalised

## Phase 9 — Documentation & Test Completeness ✅ (v0.9.0)
- **Complete docs** — gui.md (full 3-pane walkthrough), cli.md (all 15 commands with examples), getting-started.md updated with aaai init
- **CLI test coverage** — integration tests for: completions, config --init, dashboard, init --non-interactive, lint --json-output, version --json-output
- **AuditWarning suppression** — suppress_warnings: list in .aaai.yaml; CLI --suppress-warnings flag on audit
- **History rotation** — --max-entries on history command; history store prune() method
- **report --no-history** — --no-history flag for report command (already on audit)
- **aaai audit --warn-only** — exit 0 even with warnings (do not fail on warning-only conditions)

## Phase 10 — GUI Polish ✅ (v0.10.0)
- **Resizable panes** — replace fixed widths with iced PaneGrid; user can drag dividers between file tree / diff view / inspector
- **Dark / Light theme** — toggle via footer picker; persisted to ~/.aaai/prefs.yaml
- **Directory collapse** — click directory entries in file tree to fold/unfold children
- **Unchanged files toggle** — show/hide Unchanged entries separately from the filter bar
- **Inspector: auto-focus reason field** — focus the reason text input when an entry is selected

## Phase 11 — GUI UI/UX Production Ready ✅ (v0.11.0 – v0.12.0)

### RFC 001 — CLI Output UX Consistency (v0.11.0)
- Result-first 4-zone layout; symbol-based status `✓ ⚠ ✗ ! —`; next-action hint

### RFC 004 — Opening Screen Input Validation (v0.11.0)
- `OpeningValidation`: inline real-time feedback; Start button disabled when paths invalid

### RFC 002 — Inspector Validation & Primary Action (v0.12.0)
- `InspectorValidation`: per-field Checksum/LineMatch/Regex/Exact checks; `ApproveAndSave` primary action

### RFC 003 — ABDD Status Display (v0.12.0)
- `status_badge()`: symbol + text per row; neutral diff badge; ABDD compliance

### RFC 005 — Keyboard Navigation & Focus (v0.12.0)
- `FocusTarget`; Tab/`/`/Enter/Ctrl+E/Escape shortcuts; complete keyboard flow


## Phase 12 — v0.19.0 の答え合わせ：視覚検証 / i18n / docs 刷新 + Phase 13〜16 統合 ✅ (v0.20.0)

**目的**: v0.15.0 〜 v0.19.0 で実装した UI/UX が「設計書通りに動くこと」を視覚的に保証する。

**実績**: 当初 Phase 12 は RFC 017〜019 のみを対象としていたが、開発の流れで Phase 13〜16 のスコープ（RFC 020〜025）も v0.20.0 で先行実装した。結果、Phase 12 は当初計画の 3 倍超のスコープで完了。

### RFC 017 — Visual Verification Harness & Protocol ✅
- 設計書 `aaai_uiux_design.pdf` の各画面要素に対してスクリーンショット + チェック項目のエビデンスを `verification/` に保存する手順を確立
- `scripts/list-unverified-rfcs.sh` で未検証 RFC を一覧化
- CI に「未検証 RFC 数」のステップを追加
- `docs/templates/visual-verification-operators-guide.md` に RFC 020〜023 まで含む包括チェックリストを整備

### RFC 018 — i18n Locale Fallback Strategies ✅ (partial — §3.4 only)
- §3.4 i18n キー監査スクリプト `scripts/check-i18n-keys.py` を新規実装
- 動的 `t!()` 呼び出しサイト（`make_btn` パターン）への false-positive 対策
- B/C 対策は RFC 016 視覚検証で問題が残った場合に限り着手（条件付き）

### RFC 019 — Documentation Refresh for v0.15–v0.19 Realities ✅
- `docs/src/gui.md` / `docs/ja/src/gui.md` を v0.20.0 の実装に合わせて全面刷新
- 加えて `cli-auditing.md` / `cli-reporting.md` / `cli-setup.md` / `cli-workflow.md` / `getting-started.md` が「英語パスに日本語が放置」状態だったため、6 ファイルを真の英語化
- 結果として `docs/src/*.md` 全 15 章が真の英語、`docs/ja/src/*.md` 全 15 章が真の日本語に到達

### RFC 020 — ABDD Audit & Action-oriented Errors ✅ (originally Phase 13)
- `docs/src/abdd-audit.md` / `docs/ja/src/abdd-audit.md` 新規（合格 / 例外 (記録) 付きのチェックシート）
- `UserError { message, hint }` 構造体導入、`error.<context>.<short_id>.{message,hint}` 形式の新 i18n キー
- Opening 画面のエラーバナー（赤いメッセージ + グレーのヒントの 2 行構成）
- Regex 入力時の regex101.com 誘導

### RFC 021 — Screen Navigation Continuity ✅ partial (originally Phase 14)
- Save / Report の完了マーク `✓ Saved Nm ago` + 30 秒 tick refresh
- `last_saved_at`, `last_reported_at`, `audit_dirty` を `App` に追加
- 新 i18n キー `banner.*`, `relative.*`
- **deferred**: `audit_dirty` バナー — sync rerun アーキ下では発火しないため、アーキ decoupling 後に再起 RFC で対応

### RFC 022 — Empty States & First-run Guidance ✅ (originally Phase 14)
- Opening プロファイル空のときの ① ② ③ オンボーディングパネル
- `audit_result` が None のときの file_tree / diff_panel / inspector それぞれの空状態
- 新しい i18n キー `empty_state.*`

### RFC 023 — Opening Drag-and-Drop & Recent Polish ✅ (originally Phase 15)
- iced 0.14 の `Event::Window::FileDropped` を用いた DnD 受け入れ
- `AuditProfile.last_used_at: Option<DateTime<Utc>>` (`#[serde(default)]`) で前方互換性確保
- `sort_by_recent` と `humanize_since` 相対時刻フォーマッタ（7 ユニットテスト付き）
- 新しい i18n キー `relative.*`, `error.opening.drop_invalid_kind.*`, `opening.drop_here`

### RFC 024 — CLI Dashboard & Help Discoverability ✅ (originally Phase 15)
- 全 16 サブコマンドの clap `after_help` に「Next steps」ブロックを追加
- `next_action_hint()` ヘルパを `audit.rs` と `dashboard.rs` で共有
- 新規 `aaai exit-codes` サブコマンド（5 値の終了コード表を印刷、v1.x 安定）
- aaai-cli tests 54 → 70 (+16)

### RFC 025 — v1.0.0 Release Preparation ✅ partial (originally Phase 16)
- `docs/src/compatibility.md` / `docs/ja/src/compatibility.md` 新規（v1.x 互換性契約）
- CLI 16 コマンド名 / 5 終了コード / 7 キーボードショートカット / 設定ファイル `#[serde(default)]` ポリシーを SemVer 解釈で明示
- 破壊的変更パイプライン（opt-in → ≥1 minor deprecation → 次 major で削除）
- **deferred**: 実際の v1.0.0 リリース判定ゲート通過とリリースタールボール cut-over は Phase 16 で

### 19 件の pre-existing バグも併せて修正
Phase 12 実装中の体系的な spot-check で v0.19.0 までに紛れ込んでいた 19 件の既存バグが判明し、すべて修正:
- GUI: `open_error` silent failure、regex error の hint 欠落、toolbar.* リテラル表示、`abdd-audit.md` mdbook ナビ孤立、23 件の dead i18n キー、`make_btn` false-positive
- ドキュメント: gui.md が日本語のまま英語パスに置かれていた、5 つの追加 CLI ガイドも同様、`aaai exit-codes` 未文書化、`compatibility.md` の crates.io 公開状態誤記、`overview.md` のコマンド数 15、`faq.md` の書き込み対象不足、`ci-integration.md` の終了コード安定性宣言欠落
- Toolchain: `Cargo.toml` に rust-version 不在、CI MSRV check が edition 2024 と非互換な 1.81 を指定（プロジェクト指示書通り 1.91 に統一）

## Phases 13〜16 — Phase 12 に統合済み ✅

当初計画では Phase 13〜16 で個別にリリース予定だった内容は、すべて Phase 12 / v0.20.0 で先行実装した:

| 当初予定 | 当初 RFC | 実際の着地 |
|---|---|---|
| ~~Phase 13 (v0.21.0)~~ | RFC 020 | v0.20.0 |
| ~~Phase 14 (v0.22.0)~~ | RFC 021, 022 | v0.20.0（RFC 021 は partial） |
| ~~Phase 15 (v0.23.0)~~ | RFC 023, 024 | v0.20.0 |
| ~~Phase 16 (v1.0.0)~~ | RFC 025 | v0.20.0（docs groundwork のみ。実リリースゲート通過は別途） |

---

## Phase 13 (v0.21.0 想定) — Post-v0.20.0 followup

v0.20.0 では 9 件の RFC が同時並行で実装され、いくつかの deferred 項目と新たに発覚した課題が残った。これらを Phase 13 で順次クローズする:

### deferred from v0.20.0

- **RFC 021 audit-dirty バナー**: 現在の同期 rerun アーキでは発火しないため deferred。Update tick を非同期化するアーキ decoupling を別途設計する RFC を切る必要あり
- **RFC 018 メイン作業 (B/C)**: RFC 016 の視覚検証で literal key 露出が残った場合に限り着手する条件付き作業
- **FieldError / toast subtitle refactor**: i18n キー `error.inspector.invalid_regex.*` / `error.save.failed.*` を再導入できるように、`Toast` 構造に subtitle (hint) フィールドを追加するリファクタ

### v0.20.0 視覚検証の結果次第

- **RFC 020 ABDD 監査シート埋め込み作業**: operator の検証結果を `docs/src/abdd-audit.md` に転記
- **RFC 022/023 視覚検証で判明した不整合の修正**: 個別 fix RFC として切る

### CI / 開発体験

- **CI に `mdbook build` ジョブ追加**: 現在は手動 smoke test のみ
- **`mdbook test` で code sample をテスト**: 現状 0 件、追加する価値ありか検討

## Phase 16 — v1.0.0 リリース判定 (改めて)

v0.20.0 で RFC 025 docs groundwork は landed したが、**実リリースゲート通過 → tag/publish → GitHub Release** は別個に行う。

### v1.0.0 リリース判定ゲート

| グループ | ゲート | 関連 RFC |
|---|---|---|
| 機能 | G1 視覚検証 / G2 i18n / G3 ABDD / G4 エラー文 / G5 画面リレーション / G6 空状態 / G7 Opening / G8 CLI | 017〜024 |
| 品質 | Q1 cargo test / Q2 clippy / Q3 fmt / Q4 mdbook / Q5 release ビルド / Q6 packaging | — |
| ドキュメント | D1 README / D2 ROADMAP / D3 CHANGELOG / D4 compatibility.md | 019 |

すべての G ゲート は v0.20.0 のコード/ドキュメントレベルでは満たされている。**残るは operator の視覚検証エビデンス**。

### v1.0.0 リリース手順

`docs/release-prep-v0.20.0.md` の 11 ステップを v0.20.0 ベースとして利用し、`v1.0.0` への version bump と CHANGELOG プロモートを行うだけで足りる見込み。

### v1.0.0 以降の互換性宣言 (`docs/src/compatibility.md`)

v0.20.0 で landed 済み。詳細は当該ドキュメント参照。

## v1.1.0 以降（暫定）

v1.0.0 ソーク中に別 RFC で議論する候補:
- スクリーンリーダー対応（iced 側の a11y サポート待ち）
- 追加言語（zh / ko）
- `aaai-core` の独立 crate 化
