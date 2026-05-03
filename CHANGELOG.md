# Changelog

All notable changes to this project are documented in this file.

Format: `## [version] — description`

## [0.10.1] — Project structure and documentation update

### Cargo / Publish
- Added `version = "0.10.1"` to `aaai-core` dependency in `aaai-cli` and `aaai-gui` Cargo.toml — `cargo publish` now works correctly
- Removed `path` from `snora` in workspace `Cargo.toml` (version-only specification)
- Added `readme`, `documentation`, and `homepage` metadata to each crate's `Cargo.toml`

### Dependency updates
- `similar` v2 → **v3**
- `indicatif` v0.17 → **v0.18**

### Repository hygiene
- Replaced `.gitignore` with a clean, well-commented version
- Updated `NOTICE` copyright year to 2026
- Removed redundant `AUTHORS` file (information already in `Cargo.toml`, `LICENSE`, `NOTICE`)
- Replaced `README.md` with a concise English-only version (removed version-specific test badge — maintenance overhead)

### GitHub Actions
- `actions/checkout` v4 → **v6**
- `dtolnay/rust-toolchain@stable` — already correct, confirmed
- `actions/upload-artifact` — v4 (confirmed)
- `actions/download-artifact` — v4 (confirmed)

### Per-crate README files (for `cargo publish`)
- `crates/aaai-core/README.md` — references top-level README
- `crates/aaai-cli/README.md` — CLI-focused quick reference
- `crates/aaai-gui/README.md` — GUI-focused quick reference

### CHANGELOG
- Fixed `[0.8.0]` heading: "Phase 8 — v1.0 comes closer (v0.8.0)"
- Translated CHANGELOG to English (older phases retain some Japanese in detailed bullet points)

### Documentation (docs/)
- `book.toml` updated with HTML output settings and multilingual note
- Japanese docs structure added: `docs/ja/` with `book.toml` + `src/` (mirroring English sources)
- Created `docs/src/overview.md` and `docs/src/audit-definition.md` (previously stubs)
- `docs/src/SUMMARY.md` updated to include all 8 chapters

## [0.10.0] — Phase 10: GUI Polish

### Tests (92 unit + 30 CLI = 122 passing)
- Added `profile/prefs.rs` round-trip / default / display tests (core unit: 92 tests)

### GUI features

#### Resizable panes (PaneGrid)
- Rewrote `main_view.rs` using **`iced::widget::pane_grid::PaneGrid`**
- All 3 panes (file tree / diff view / inspector) are resizable via drag handles
- Auto-initialised at 30% / 45% / 25% on `DiffReady`
- `Message::PaneResized(ResizeEvent)` updates ratio live

#### Dark / Light theme
- `profile/prefs.rs` — `Theme` (Light / Dark / System) + `UserPrefs` persisted to `~/.aaai/prefs.yaml`
- Added `.theme(|app| ...)` in `main.rs` to connect theme to iced application
- Added **theme picker** (Light / Dark) in the footer
- Theme is automatically restored on next launch

#### Directory collapse
- Added directory headers (▼ / ▶ icons) to the file tree
- Click header to collapse / expand child entries
- State stored in `App.collapsed_dirs: HashSet<String>`
- `Message::ToggleDir(String)` toggles the state

### Code quality
- **Zero warnings across all crates** — `cargo fix` applied + unnecessary imports cleaned up

## [0.9.0] — Phase 9: Documentation & Test Completeness

### Documentation improvements
- **`gui.md`** — 10 lines → **136 lines**: Opening screen, 3-pane operations, badge reference, keyboard shortcuts, footer, and typical workflow fully documented
- **`cli.md`** — 27 lines → **307 lines**: All 15 commands with flags, exit codes, and examples
- **`getting-started.md`** — 17 lines → **129 lines**: `aaai init` flow, manual setup, `.aaai.yaml`, and shell completion install

### Test coverage expansion (89 unit + 30 integration = 119 passing)
Added tests for the following previously untested commands:
- `completions bash/zsh` — output is non-empty and contains "aaai"
- `config --init` — `.aaai.yaml` creation and existing file detection (bug fix: now checks existence even when `--dir` is specified)
- `dashboard` — exit 0 verified
- `init --non-interactive` — `.aaai.yaml` creation
- `lint --json-output` — valid JSON output and `empty-linematch` error detection
- `version --json-output` — `version` / `license` field validation

### New features
- **`AuditWarning` suppression** — `suppress_warnings: [no-approver, no-strategy]` in `.aaai.yaml`; also `aaai audit --suppress-warnings <kind,...>`
- **`AuditEngine::evaluate_with_options()`** — new overload accepting `AuditOptions` (suppress_warnings)
- **`aaai history --prune <N>`** — prune history to the most recent N entries (`prune()` implemented in `history/store.rs`)
- **`aaai audit --warn-only`** — explicit intent flag (warnings do not affect exit codes by design)

### Bug fixes
- **`config --init` existence check** — fixed a bug where `.aaai.yaml` existence was not checked when `--dir` was specified

## [0.8.0] — Phase 8 — v1.0 comes closer (v0.8.0)

### Bug fixes (UI/UX  test前に必須)

#### Fix 1&2: GUI — `ignore_path` not connected to `IgnoreRules`
- Connected `ignore_path` field value to the async diff execution in **`StartAudit`**
  - Falls back to `<Before>/.aaaiignore` auto-discovery when blank (consistent with CLI behaviour)
  - When specified, builds `IgnoreRules` from the given path
- Fixed **`rerun_audit()`** to re-scan with the same `IgnoreRules`
  - Added `active_ignore: IgnoreRules` to `App`, saved via `DiffReady` message
  - Fixed: "Re-run audit" button was returning different files from the original scan
- **`DiffReady` message signature changed**: `IgnoreRules` added as the third argument

#### Fix 3: GUI — `AuditWarning` not displayed
- Added warning section immediately below the divider in the **inspector panel**
  - `large-file` → yellow background block + `⚠` icon
  - `no-strategy` → blue-tinted `ℹ` icon
  - `no-approver` → grey-tinted `ℹ` icon
  - Section hidden entirely when no warnings (no layout impact)

#### Fix 4: GUI — `AuditWarning` badge missing in file tree
- Added `⚠N` badge (N = warning count) to each row in the file tree
  - Small yellow-tinted badge (9px)
  - Not shown for entries with zero warnings

#### Fix 5: GUI — `warning_count` not shown in toolbar
- Added `⚠ N warning(s)` text next to the verdict badge in the toolbar
  - Only shown when `AuditSummary.warning_count > 0`

#### Fix 6: GUI — keyboard shortcut legend missing
- Added `Ctrl+S: Save  Ctrl+R: Re-run  Ctrl+Z: Undo  ↑↓: Navigate` to the footer
  - Shown on Main screen only (hidden on Opening screen)

#### Fix 7: GUI — Opening screen UX improvements
- Updated `.aaaiignore` field placeholder to indicate auto-discovery behaviour
- Loading spinner now shows Before/After folder names to clarify what is being scanned

### Version
- Version set to `0.8.0` in Cargo.toml (v1.0 pending UI/UX testing)

## [1.0.0] — Phase 8: v1.0 Release

### Tests (109  passing)
- core unit tests: **89 件** (proptest プロパティベース test 8  added: masking idempotency, ignore rules)
- CLI integration tests: **20 件**

### Core additions
- **プロパティベース test** — `proptest` によるランダム入力への堅牢性検証
  - `masking/prop_tests.rs`: マスキングの非パニック保証・冪等性・`mask_if_needed` 一貫性
  - `diff/prop_tests.rs`: IgnoreRules の非パニック保証・空ルール・グロブマッチング特性
- **ベンチマーク** — `criterion` による性能計測 (`cargo bench -p aaai-core`)
  - `diff_bench`: 100 / 1000  fileの並列差分エンジン
  - `masking_bench`: 5 種の混合テキストに対するマスキング processing

### CLI additions
- **`aaai version`** — 詳細 version情報 (version, authors, license, OS/arch, build profile)。`--json-output`  support
- **`aaai lint <FILE>`** — 定義 fileのベストプラクティス全チェック
  - duplicate-path: 同一パスの重複定義 (error)
  - short-reason: 理由が短すぎる (warning, default 10 文字)
  - missing-ticket: チケット未 config (warning, `--require-ticket` 時)
  - missing-approver: 承認者未 config (warning, `--require-approver` 時)
  - expired: 有効期限切れ (warning)
  - empty-linematch / empty-line-rule: LineMatch ルール不備 (error)
  - strategy-mismatch: Added/Removed に LineMatch (info)
  - disabled: 無効化エントリ (info)
  - `--json-output`  support・error 存在時は exit 1
- **`aaai snap --approver <NAME>`** —  generationエントリの `approved_by` を config。プロジェクト configの `approver_name` にフォールバック
- **`aaai snap --suggest-glob`** — 同一ディレクトリのエントリをグロブパターンにまとめる提案を display

### GUI additions (Phase 8)
- **非同期 diff** — `tokio::task::spawn_blocking` で folder比較をバックグラウンドスレッドで実行。GUI は比較中もレスポンシブを維持
- **ローディングスピナー** — Opening 画面にスキャン中の進捗メッセージを display

### v1.0 リリース準備
- **LICENSE** — Apache-2.0 全文を記載
- **AUTHORS** — 著者 file added
- **README バッジ** — version / tests / CI / license シールドを added
- **ROADMAP  updated** — Phase 8 を記録し、全Phaseの実績を確定

## [0.7.0] — Phase 7: v1.0 Quality

### Tests (101  passing)
- core unit tests: **81 件** (AuditWarning 7 件、SARIF 2 件、lockfile 2  added)
- CLI integration tests: **20** (export CSV/TSV, merge conflict, SARIF format, history stats, diff JSON)

### Code quality
- **Zero warnings** — `cargo fix` + 全 unused variable/dead_code を `_prefix` / `#[allow]` で抑制。`cargo check` が全クレートで warningゼロ

### Core additions
- **`audit/warning.rs`** — `AuditWarning` システム: `LargeFileStrategy` (>1MB に Exact/LineMatch 適用)、`NoStrategyOnModified`、`NoApprover` の 3 種類
- **`FileAuditResult.warnings`** — 各エントリに advisory  warningリストを付与
- **`AuditSummary.warning_count`** — 全体の warning件数を集計
- **`config/lock.rs`** — `.lock`  fileによる書き込みロック。60 秒 TTL でステールロックを自動 removed。`config/io.rs` に統合済み

### CLI additions
- **`aaai export`** — 承認済みエントリを CSV / TSV に出力。13 カラム: path, diff_type, status, reason, strategy, ticket, approved_by, approved_at, expires_at, enabled, note, created_at, updated_at
- **`aaai init`** — 対話型プロジェクト初期 configウィザード。Before/After パス・定義 file・承認者名を対話入力し `.aaai.yaml` を generation。`--non-interactive` フラグ support
- **`aaai history --stats`** — 全実行履歴のトレンド分析: 合格率・平均 OK/Pending/Failed 件数・直近 5 回 vs 前 5 回の傾向 (↑ improvement / ↓低下 / →安定)

## [0.6.0] — Phase 6: Production Readiness

### Tests (85  passing)
- core unit tests: 73 件（SARIF  test 2  added）
- CLI integration tests: 12 件

### Core additions
- **Entry versioning** — `created_at` / `updated_at` fields added to `AuditEntry`; `stamp_now()` auto-stamps on approval
- **`report/sarif.rs`** — SARIF v2.1.0 レポート generation。Failed → error、Pending → warning にマッピング
- **`ReportGenerator::build_markdown_string(include_diff: bool)`** — 差分テキスト埋め込みオプション付き Markdown  generation

### CLI additions
- **`aaai diff`** — 定義 file不要の純粋差分 display。`--content` で実差分テキスト、`--json-output` で JSON 出力
- **`aaai merge <BASE> <OVERLAY>`** — 2つの定義 fileをマージ。`--detect-conflicts` で競合チェックのみ実行
- **`aaai report --format sarif`** — SARIF v2.1.0 出力（GitHub Actions `upload-sarif` で PR アノテーション support）
- **`aaai report --include-diff`** — Markdown/HTML レポートに実差分テキストを埋め込み

### GitHub Actions CI/CD
- **`.github/workflows/ci.yaml`** —  test (Ubuntu/macOS/Windows)・フォーマットチェック・Clippy・MSRV  verified・セキュリティ監査
- **`.github/workflows/release.yaml`** — タグプッシュ時にクロスコンパイルビルド + GitHub Release 自動作成

### GUI additions
- **Undo  feature** — `Message::UndoApproval` で最後の承認を取り消し (最大 20 件スタック)
- **Keyboard shortcuts** — Ctrl+S (save), Ctrl+R (re-run), Ctrl+Z (undo), ↑↓ (navigate)

### Documentation完成
- `docs/src/strategies.md` — 全 5 戦略の詳細解説・使い分けガイド
- `docs/src/ci-integration.md` — GitHub Actions 例・SARIF アップロード・Watch モード・シェル補完インストール
- `docs/src/faq.md` — 13 件の FAQ（理由必須の理由・glob ルール・マージ・SARIF 活用など）
- `docs/src/SUMMARY.md` — mdBook 目次 updated (8 章)

## [0.5.0] — Phase 5: Polish

### Tests (83  passing)
- **core unit tests**: 71 件 (diff/audit/config/masking/project/templates/profile/history)
- **CLI integration tests**: 12 件 (実バイナリを使った end-to-end  test)
  - exit code 検証 (0/1/2)、JSON 出力の妥当性、glob ルール、HTML レポート generation、dry-run 動作など

### CLI additions
- **`aaai completions <shell>`** — bash / zsh / fish / powershell 向けシェル補完スクリプト generation (clap_complete)
- **`aaai watch`** — before・after・定義 fileの changedを監視し、 changed検出時に自動で監査を再実行 (notify crate、500ms デバウンス)
- **`aaai dashboard`** — colour-coded stat cards + attention list; `--detail` flag shows all changed entries
- **`aaai audit --progress`** — indicatif プログレスバーで大規模 folderの比較進捗を display
- **`aaai snap --dry-run`** —  fileを書き込まずに generation内容をプレビュー
- **`aaai report --format html`** — スタイル付き HTML レポートを出力（summary カード・ステータス色分け・チケット display・差分統計）

### Core additions
- **`diff/progress.rs`** — `DiffProgress` イベント + `ProgressSink` トレイト + `ChannelProgress` / `NullProgress`  implementation
- **`DiffEngine::compare_with_progress()`** — 進捗シンクを受け取るオーバーロード
- **`report/html.rs`** — セルフコンテイン HTML レポート generation (BootstrapなしのCSSインライン)

### GUI additions
- **Dashboard view** — shows summary cards (OK/Pending/Failed/Error/Ignored counts) + result banner + attention list when no file is selected
- **File tree search bar** — incremental path filter (search input placed below filter bar)

## [0.4.0] — Phase 4: Advanced

### Core modules (69  passing)
- **`diff/entry.rs` 強化** — `is_binary` フラグ・`before_size`/`after_size`・`before_sha256`・`DiffStats`（lines_added/removed/unchanged）フィールドを added
- **Parallel diff engine** — `rayon` `par_iter` for large folder comparison; sorted output guaranteed
- **Binary detection** — null-byte heuristic; binary files tracked by hash/size only, text strategies not applied
- **`diff/entry::fmt_size()`** — formats byte counts as human-readable strings (B/KB/MB/GB)
- **`masking/`** — `MaskingEngine` + 9 種のビルトインパターン（API キー、パスワード、AWS キー、GitHub トークン、Slack トークン、Bearer トークン、接続文字列パスワード、秘密鍵ヘッダー）。カスタムパターン added可能
- **`project/config.rs`** — `.aaai.yaml` の loading・ saved・上位ディレクトリへのオートディスカバリー

### CLI additions
- **`aaai config`** — `.aaai.yaml` を現在ディレクトリ付近から検索・ display。`--init` でスターターテンプレート generation
- **`aaai audit --mask-secrets`** — masks reason field in verbose output; also activated by `mask_secrets: true` in project config
- **`aaai audit --verbose`** — バイナリ fileの (binary file)  display、差分統計 (+N -N lines)、サイズ変化を added
- **レポートへのマスキング support** — `write_markdown` / `write_json` に `Option<&MaskingEngine>` 引数を added

### GUI additions
- **バイナリ fileパネル** — バイナリ差分選択時に専用パネルを display。 file種別・サイズ変化・before/after SHA-256 ハッシュ・一致/不一致の視覚的 display
- **差分統計バー** — テキスト差分ビューアの上部に `+N lines` / `−N lines` と サイズ変化を display

## [0.3.0] — Phase 3: Integrations

### Core additions
- **Approver tracking** — `approved_by` / `approved_at` fields added to all `AuditEntry`; auto-stamped on approval
- **Expiry dates** — `expires_at` (NaiveDate) field; expired entries shown as warnings in CLI and GUI
- **Ticket linkage** — `ticket` field (JIRA-123, INF-42, etc.) shown in reports and inspector
- **Empty reason → Pending** — `AuditEngine` now treats snap-generated entries with no reason as Pending
- **`.aaaiignore`** — `diff/ignore.rs`; gitignore-style pattern exclusion from diff; negation rules (`!pattern`) supported
- **Audit history** — `history/store.rs`; run log appended to `~/.aaai/history.jsonl` in JSONL format
- **Rule templates** — `templates/library.rs`; 8 built-in templates (version bump, port change, config value change, etc.)
- **Audit profiles** — `profile/store.rs`; before/after/definition combos saved to `~/.aaai/profiles.yaml`

### CLI additions
- **`aaai check`** — 定義 fileの妥当性を差分実行なしで検証。期限切れエントリも報告。Config  errorで exit 4
- **`aaai history`** — `~/.aaai/history.jsonl` から最近の監査実行を一覧 display。`--json-output`  support
- **`aaai snap --template <id>`** —  generation時にルールテンプレートを適用
- **`aaai snap --list-templates`** — テンプレート一覧 display
- **`aaai audit --ignore <FILE>`** — .aaaiignore  fileを明示指定
- **詳細終了コード** — 0=PASSED, 1=FAILED, 2=PENDING, 3=ERROR, 4=CONFIG_ERROR

### GUI additions
- **インスペクター Phase 3 フィールド** — ticket, approved_by, expires_at の display・編集
- **有効期限バッジ** — `EXPIRED` / `Expiring soon` のカラーバッジをインスペクターヘッダーに display
- **テンプレートピッカー** — インスペクターに "Apply template" ドロップダウンを added（8 テンプレート support）
- **プロ fileマネージャー** — Opening 画面にプロ file saved・ loading・ removed UI を added
- **Opening: ignore path フィールド** — .aaaiignore  fileのパスを Opening 画面で指定可能

### Tests
- Phase 3 動作カバー (51  test)：空理由 → Pending、Unchanged 自動 OK など

## [0.2.0] — Phase 2: Quality & Completeness

### 必須要件 support (別紙)
- **tests.rs 分離**: `diff/tests.rs`, `audit/tests.rs`, `config/tests.rs` に unit/integration  test 37 件を added
- **GUI 多言語 support**: rust-i18n v3 で日英 locale  file (`en.yaml` / `ja.yaml`) を implementation。フッターのロケールピッカーで切り替え可能

### Core  feature added
- **Glob パターンマッチング**: `path` フィールドに `logs/*.log` や `build/**` 形式の glob ルールを使用可能。完全一致ルールが優先
- **Unchanged エントリの自動 OK**: 差分のないエントリは監査ルールなしで自動 OK 判定
- **tests.rs**: config の glob マッチ testを含む 37 件の testが passing

### GUI  feature added (iced + snora)
- **フィルターバー**: Changed Only / All / Pending / Failed・Error の 4 モードで差分一覧を絞り込み
- **バッチ承認**: 複数エントリを選択（チェックボックス）し、共通理由で一括承認。snora `Sheet` (端パネル) として display
- **Toast subscription  fixed**: `App::subscription` を iced アプリケーションに正しく接続し、TTL 自動 removedが featureするよう fixed
- **差分ビューアの improvement**: 行番号 display、`iter_all_changes` ベースの安定したレンダリング
- **ロケールピッカー**: フッターに配置。`LANG` 環境変数でシステムロケールを自動検出

### CLI  feature added
- `--verbose`: OK / Ignored エントリも displayし、reason を併記
- `--quiet`: サマリー行のみ出力
- `--json-output`: 監査結果を JSON で stdout に出力（CI/CD での機械 processing向け）

## [Unreleased] — Phase 1

### Added
- Folder diff engine with seven diff types
- Audit definition YAML format (version 1)
- Content audit strategies: None, Checksum, LineMatch, Regex, Exact
- CLI subcommands: audit, snap, report
- GUI with snora/iced: opening screen and 3-pane main screen
- Approval flow requiring mandatory reason
- Markdown and JSON report generation
