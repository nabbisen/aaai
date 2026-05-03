# Changelog

## [0.10.0] — Phase 10: GUI Polish

### テスト (92 unit + 30 CLI = 122 テスト全通過)
- `profile/prefs.rs` の round_trip / default / display テスト追加（core unit 92 件）

### GUI 新機能

#### ペインリサイズ (PaneGrid)
- `main_view.rs` を **`iced::widget::pane_grid::PaneGrid`** ベースに全面書き換え
- ファイルツリー / 差分ビュー / インスペクターの 3 ペインをドラッグハンドルでリサイズ可能
- `DiffReady` 時に初期比率（30% / 45% / 25%）で自動初期化
- `Message::PaneResized(ResizeEvent)` で比率をライブ更新

#### ダーク / ライトテーマ
- `profile/prefs.rs` — `Theme` (Light / Dark / System) + `UserPrefs` を `~/.aaai/prefs.yaml` に永続化
- `main.rs` に `.theme(|app| ...)` を追加して iced アプリに反映
- フッターに **テーマピッカー** (Light / Dark) を追加
- 起動時に前回のテーマを自動復元

#### ディレクトリ折りたたみ
- ファイルツリーにディレクトリヘッダー（▼ / ▶ アイコン）を追加
- ヘッダークリックで配下エントリを折りたたみ / 展開
- `App.collapsed_dirs: HashSet<String>` で状態を保持
- `Message::ToggleDir(String)` で切り替え

### コード品質
- **全クレート警告ゼロ** — `cargo fix` 適用 + 不要インポート整理

## [0.9.0] — Phase 9: Documentation & Test Completeness

### ドキュメント充実
- **`gui.md`** — 10 行 → **136 行**: Opening 画面・3 ペイン操作・バッジ一覧・キーボードショートカット・フッター・典型ワークフローを完全記述
- **`cli.md`** — 27 行 → **307 行**: 全 15 コマンドのフラグ・終了コード・使用例を網羅
- **`getting-started.md`** — 17 行 → **129 行**: `aaai init` 起点フロー・手動セットアップ・`.aaai.yaml`・シェル補完インストールを追記

### テストカバレッジ拡大 (89 unit + 30 integration = 119 テスト全通過)
以下の未カバーコマンドにテストを追加:
- `completions bash/zsh` — 出力が空でない・"aaai" を含む
- `config --init` — `.aaai.yaml` 生成・既存ファイル検出（バグ修正: `--dir` 指定時も既存チェックするよう修正）
- `dashboard` — exit 0 確認
- `init --non-interactive` — `.aaai.yaml` 生成
- `lint --json-output` — 有効 JSON 出力・`empty-linematch` エラー検出
- `version --json-output` — `version` / `license` フィールド確認

### 新機能
- **`AuditWarning` 抑制** — `.aaai.yaml` に `suppress_warnings: [no-approver, no-strategy]` を追加可能。`aaai audit --suppress-warnings <kind,...>` フラグでも抑制可能
- **`AuditEngine::evaluate_with_options()`** — `AuditOptions` 構造体 (suppress_warnings) を受け取る新オーバーロード
- **`aaai history --prune <N>`** — 履歴を最新 N 件に刈り込む（history/store.rs に `prune()` を実装）
- **`aaai audit --warn-only`** — 意図を明示するフラグ（警告は元々 exit code に影響しないため実質ドキュメント的追加）

### バグ修正
- **`config --init` の既存チェック** — `--dir` 指定時でも `.aaai.yaml` の存在チェックをスキップしていたバグを修正

## [0.8.0] — UI/UX テスト前 課題修正

### バグ修正 (UI/UX テスト前に必須)

#### 修正1・2: GUI — `ignore_path` → `IgnoreRules` 未接続
- **`StartAudit`** の非同期 diff 実行に `ignore_path` フィールドの値を接続
  - 空欄の場合は `<Before>/.aaaiignore` を自動検索（CLI と同動作に統一）
  - 指定がある場合はそのパスから `IgnoreRules` を構築
- **`rerun_audit()`** も同じ `IgnoreRules` で再スキャンするよう修正
  - `App` に `active_ignore: IgnoreRules` を追加し `DiffReady` メッセージ経由で保存
  - 「監査再実行」ボタンが以前のスキャンと異なるファイルを返す問題を解消
- **`DiffReady` メッセージのシグネチャ変更**: `IgnoreRules` を第3引数として追加

#### 修正3: GUI — `AuditWarning` 未表示
- **インスペクターパネル** の divider 直下に警告セクションを追加
  - `large-file` → 黄色背景ブロック ＋ `⚠` アイコン
  - `no-strategy` → 青色系 `ℹ` アイコン
  - `no-approver` → グレー系 `ℹ` アイコン
  - 警告なしの場合はセクション自体を非表示（レイアウトに影響なし）

#### 修正4: GUI — ファイルツリーの `AuditWarning` バッジ未表示
- ファイルツリーの各行に `⚠N`（N = 警告数）バッジを追加
  - 黄色系の小型バッジ（サイズ 9px）
  - 警告ゼロのエントリには表示なし

#### 修正5: GUI — ツールバーの `warning_count` 未表示
- ツールバーの verdict バッジ横に `⚠ N warning(s)` テキストを追加
  - `AuditSummary.warning_count > 0` の場合のみ表示

#### 修正6: GUI — キーボードショートカット凡例未表示
- フッターに `Ctrl+S: 保存  Ctrl+R: 再実行  Ctrl+Z: Undo  ↑↓: 移動` を表示
  - Main 画面のみ表示（Opening 画面では非表示）

#### 修正7: GUI — Opening 画面の UX 改善
- `.aaaiignore` フィールドのプレースホルダーを「省略時: Before/.aaaiignore を自動検索」に更新
- ローディングスピナーに Before/After フォルダ名を表示してスキャン対象を明示

### バージョン
- Cargo.toml のバージョンを `0.8.0` に設定（v1.0 は UI/UX テスト後）

## [1.0.0] — Phase 8: v1.0 Release

### テスト (109 テスト全通過)
- core unit tests: **89 件** (proptest プロパティベーステスト 8 件追加: masking idempotency, ignore rules)
- CLI integration tests: **20 件**

### Core 追加
- **プロパティベーステスト** — `proptest` によるランダム入力への堅牢性検証
  - `masking/prop_tests.rs`: マスキングの非パニック保証・冪等性・`mask_if_needed` 一貫性
  - `diff/prop_tests.rs`: IgnoreRules の非パニック保証・空ルール・グロブマッチング特性
- **ベンチマーク** — `criterion` による性能計測 (`cargo bench -p aaai-core`)
  - `diff_bench`: 100 / 1000 ファイルの並列差分エンジン
  - `masking_bench`: 5 種の混合テキストに対するマスキング処理

### CLI 追加
- **`aaai version`** — 詳細バージョン情報 (version, authors, license, OS/arch, build profile)。`--json-output` 対応
- **`aaai lint <FILE>`** — 定義ファイルのベストプラクティス全チェック
  - duplicate-path: 同一パスの重複定義 (error)
  - short-reason: 理由が短すぎる (warning, default 10 文字)
  - missing-ticket: チケット未設定 (warning, `--require-ticket` 時)
  - missing-approver: 承認者未設定 (warning, `--require-approver` 時)
  - expired: 有効期限切れ (warning)
  - empty-linematch / empty-line-rule: LineMatch ルール不備 (error)
  - strategy-mismatch: Added/Removed に LineMatch (info)
  - disabled: 無効化エントリ (info)
  - `--json-output` 対応・error 存在時は exit 1
- **`aaai snap --approver <NAME>`** — 生成エントリの `approved_by` を設定。プロジェクト設定の `approver_name` にフォールバック
- **`aaai snap --suggest-glob`** — 同一ディレクトリのエントリをグロブパターンにまとめる提案を表示

### GUI 追加 (Phase 8)
- **非同期 diff** — `tokio::task::spawn_blocking` でフォルダ比較をバックグラウンドスレッドで実行。GUI は比較中もレスポンシブを維持
- **ローディングスピナー** — Opening 画面にスキャン中の進捗メッセージを表示

### v1.0 リリース準備
- **LICENSE** — Apache-2.0 全文を記載
- **AUTHORS** — 著者ファイル追加
- **README バッジ** — version / tests / CI / license シールドを追加
- **ROADMAP 更新** — Phase 8 を記録し、全フェーズの実績を確定

## [0.7.0] — Phase 7: v1.0 Quality

### テスト (101 テスト全通過)
- core unit tests: **81 件** (AuditWarning 7 件、SARIF 2 件、lockfile 2 件追加)
- CLI integration tests: **20 件** (export CSV/TSV, merge conflict, SARIF format, history stats, diff JSON)

### コード品質
- **警告ゼロ達成** — `cargo fix` + 全 unused variable/dead_code を `_prefix` / `#[allow]` で抑制。`cargo check` が全クレートで警告ゼロ

### Core 追加
- **`audit/warning.rs`** — `AuditWarning` システム: `LargeFileStrategy` (>1MB に Exact/LineMatch 適用)、`NoStrategyOnModified`、`NoApprover` の 3 種類
- **`FileAuditResult.warnings`** — 各エントリに advisory 警告リストを付与
- **`AuditSummary.warning_count`** — 全体の警告件数を集計
- **`config/lock.rs`** — `.lock` ファイルによる書き込みロック。60 秒 TTL でステールロックを自動削除。`config/io.rs` に統合済み

### CLI 追加
- **`aaai export`** — 承認済みエントリを CSV / TSV に出力。13 カラム: path, diff_type, status, reason, strategy, ticket, approved_by, approved_at, expires_at, enabled, note, created_at, updated_at
- **`aaai init`** — 対話型プロジェクト初期設定ウィザード。Before/After パス・定義ファイル・承認者名を対話入力し `.aaai.yaml` を生成。`--non-interactive` フラグ対応
- **`aaai history --stats`** — 全実行履歴のトレンド分析: 合格率・平均 OK/Pending/Failed 件数・直近 5 回 vs 前 5 回の傾向 (↑改善 / ↓低下 / →安定)

## [0.6.0] — Phase 6: Production Readiness

### テスト (85 テスト全通過)
- core unit tests: 73 件（SARIF テスト 2 件追加）
- CLI integration tests: 12 件

### Core 追加
- **エントリバージョニング** — `AuditEntry` に `created_at` / `updated_at` フィールドを追加。`stamp_now()` メソッドで承認時に自動スタンプ
- **`report/sarif.rs`** — SARIF v2.1.0 レポート生成。Failed → error、Pending → warning にマッピング
- **`ReportGenerator::build_markdown_string(include_diff: bool)`** — 差分テキスト埋め込みオプション付き Markdown 生成

### CLI 追加
- **`aaai diff`** — 定義ファイル不要の純粋差分表示。`--content` で実差分テキスト、`--json-output` で JSON 出力
- **`aaai merge <BASE> <OVERLAY>`** — 2つの定義ファイルをマージ。`--detect-conflicts` で競合チェックのみ実行
- **`aaai report --format sarif`** — SARIF v2.1.0 出力（GitHub Actions `upload-sarif` で PR アノテーション対応）
- **`aaai report --include-diff`** — Markdown/HTML レポートに実差分テキストを埋め込み

### GitHub Actions CI/CD
- **`.github/workflows/ci.yaml`** — テスト (Ubuntu/macOS/Windows)・フォーマットチェック・Clippy・MSRV 確認・セキュリティ監査
- **`.github/workflows/release.yaml`** — タグプッシュ時にクロスコンパイルビルド + GitHub Release 自動作成

### GUI 追加
- **Undo 機能** — `Message::UndoApproval` で最後の承認を取り消し (最大 20 件スタック)
- **キーボードショートカット** — Ctrl+S (保存)、Ctrl+R (再実行)、Ctrl+Z (Undo)、↑↓ (エントリ移動)

### ドキュメント完成
- `docs/src/strategies.md` — 全 5 戦略の詳細解説・使い分けガイド
- `docs/src/ci-integration.md` — GitHub Actions 例・SARIF アップロード・Watch モード・シェル補完インストール
- `docs/src/faq.md` — 13 件の FAQ（理由必須の理由・glob ルール・マージ・SARIF 活用など）
- `docs/src/SUMMARY.md` — mdBook 目次更新 (8 章)

## [0.5.0] — Phase 5: Polish

### テスト (83 テスト全通過)
- **core unit tests**: 71 件 (diff/audit/config/masking/project/templates/profile/history)
- **CLI integration tests**: 12 件 (実バイナリを使った end-to-end テスト)
  - exit code 検証 (0/1/2)、JSON 出力の妥当性、glob ルール、HTML レポート生成、dry-run 動作など

### CLI 追加
- **`aaai completions <shell>`** — bash / zsh / fish / powershell 向けシェル補完スクリプト生成 (clap_complete)
- **`aaai watch`** — before・after・定義ファイルの変更を監視し、変更検出時に自動で監査を再実行 (notify crate、500ms デバウンス)
- **`aaai dashboard`** — カラーコードの統計カード＋要注意エントリ一覧。`--detail` フラグで全変更エントリを表示
- **`aaai audit --progress`** — indicatif プログレスバーで大規模フォルダの比較進捗を表示
- **`aaai snap --dry-run`** — ファイルを書き込まずに生成内容をプレビュー
- **`aaai report --format html`** — スタイル付き HTML レポートを出力（summary カード・ステータス色分け・チケット表示・差分統計）

### Core 追加
- **`diff/progress.rs`** — `DiffProgress` イベント + `ProgressSink` トレイト + `ChannelProgress` / `NullProgress` 実装
- **`DiffEngine::compare_with_progress()`** — 進捗シンクを受け取るオーバーロード
- **`report/html.rs`** — セルフコンテイン HTML レポート生成 (BootstrapなしのCSSインライン)

### GUI 追加
- **ダッシュボードビュー** — ファイル未選択時にサマリーカード (OK/Pending/Failed/Error/Ignored 件数) + 結果バナー + 要注意エントリ一覧を表示
- **ファイルツリー検索バー** — パス名インクリメンタルフィルター (フィルターバーの下に検索入力を配置)

## [0.4.0] — Phase 4: Advanced

### Core 新モジュール (69 テスト全通過)
- **`diff/entry.rs` 強化** — `is_binary` フラグ・`before_size`/`after_size`・`before_sha256`・`DiffStats`（lines_added/removed/unchanged）フィールドを追加
- **並列差分エンジン** — `rayon` による `par_iter` で大規模フォルダの並列ファイル比較を実現。ソート済み出力を保証
- **バイナリ検出** — ヌルバイト検査によるバイナリ判定。バイナリファイルは hash/size のみ追跡、テキスト戦略の適用を防止
- **`diff/entry::fmt_size()`** — バイト数を人間可読文字列 (B/KB/MB/GB) にフォーマット
- **`masking/`** — `MaskingEngine` + 9 種のビルトインパターン（API キー、パスワード、AWS キー、GitHub トークン、Slack トークン、Bearer トークン、接続文字列パスワード、秘密鍵ヘッダー）。カスタムパターン追加可能
- **`project/config.rs`** — `.aaai.yaml` の読み込み・保存・上位ディレクトリへのオートディスカバリー

### CLI 追加
- **`aaai config`** — `.aaai.yaml` を現在ディレクトリ付近から検索・表示。`--init` でスターターテンプレート生成
- **`aaai audit --mask-secrets`** — Verbose 出力の reason フィールドをマスキング。プロジェクト設定の `mask_secrets: true` でも有効化
- **`aaai audit --verbose`** — バイナリファイルの (binary file) 表示、差分統計 (+N -N lines)、サイズ変化を追加
- **レポートへのマスキング対応** — `write_markdown` / `write_json` に `Option<&MaskingEngine>` 引数を追加

### GUI 追加
- **バイナリファイルパネル** — バイナリ差分選択時に専用パネルを表示。ファイル種別・サイズ変化・before/after SHA-256 ハッシュ・一致/不一致の視覚的表示
- **差分統計バー** — テキスト差分ビューアの上部に `+N lines` / `−N lines` と サイズ変化を表示

## [0.3.0] — Phase 3: Integrations

### Core 追加
- **承認者トラッキング** — `approved_by` / `approved_at` フィールドをすべての `AuditEntry` に追加。承認操作時に自動スタンプ
- **有効期限** — `expires_at` (NaiveDate) フィールド。期限切れエントリを CLI / GUI で警告表示
- **チケット連携** — `ticket` フィールド (JIRA-123, INF-42 等) をレポートおよびインスペクターに表示
- **空理由 → Pending** — snap で生成された理由未入力エントリを Pending として扱うよう AuditEngine を修正
- **`.aaaiignore`** — `diff/ignore.rs`。gitignore スタイルのパターンで差分から除外。`!pattern` による否定ルール対応
- **監査履歴** — `history/store.rs`。`~/.aaai/history.jsonl` に実行ログを JSONL 形式で追記
- **ルールテンプレート** — `templates/library.rs`。8 種の定義済みテンプレート（バージョン番号、ポート変更、設定値変更など）
- **監査プロファイル** — `profile/store.rs`。`~/.aaai/profiles.yaml` に before/after/定義の組み合わせを保存

### CLI 追加
- **`aaai check`** — 定義ファイルの妥当性を差分実行なしで検証。期限切れエントリも報告。Config エラーで exit 4
- **`aaai history`** — `~/.aaai/history.jsonl` から最近の監査実行を一覧表示。`--json-output` 対応
- **`aaai snap --template <id>`** — 生成時にルールテンプレートを適用
- **`aaai snap --list-templates`** — テンプレート一覧表示
- **`aaai audit --ignore <FILE>`** — .aaaiignore ファイルを明示指定
- **詳細終了コード** — 0=PASSED, 1=FAILED, 2=PENDING, 3=ERROR, 4=CONFIG_ERROR

### GUI 追加
- **インスペクター Phase 3 フィールド** — ticket, approved_by, expires_at の表示・編集
- **有効期限バッジ** — `EXPIRED` / `Expiring soon` のカラーバッジをインスペクターヘッダーに表示
- **テンプレートピッカー** — インスペクターに "Apply template" ドロップダウンを追加（8 テンプレート対応）
- **プロファイルマネージャー** — Opening 画面にプロファイル保存・読み込み・削除 UI を追加
- **Opening: ignore path フィールド** — .aaaiignore ファイルのパスを Opening 画面で指定可能

### テスト
- Phase 3 動作カバー (51 テスト)：空理由 → Pending、Unchanged 自動 OK など

## [0.2.0] — Phase 2: Quality & Completeness

### 必須要件対応 (別紙)
- **tests.rs 分離**: `diff/tests.rs`, `audit/tests.rs`, `config/tests.rs` に unit/integration テスト 37 件を追加
- **GUI 多言語対応**: rust-i18n v3 で日英 locale ファイル (`en.yaml` / `ja.yaml`) を実装。フッターのロケールピッカーで切り替え可能

### Core 機能追加
- **Glob パターンマッチング**: `path` フィールドに `logs/*.log` や `build/**` 形式の glob ルールを使用可能。完全一致ルールが優先
- **Unchanged エントリの自動 OK**: 差分のないエントリは監査ルールなしで自動 OK 判定
- **tests.rs**: config の glob マッチテストを含む 37 件のテストが全通過

### GUI 機能追加 (iced + snora)
- **フィルターバー**: Changed Only / All / Pending / Failed・Error の 4 モードで差分一覧を絞り込み
- **バッチ承認**: 複数エントリを選択（チェックボックス）し、共通理由で一括承認。snora `Sheet` (端パネル) として表示
- **Toast subscription 修正**: `App::subscription` を iced アプリケーションに正しく接続し、TTL 自動削除が機能するよう修正
- **差分ビューアの改善**: 行番号表示、`iter_all_changes` ベースの安定したレンダリング
- **ロケールピッカー**: フッターに配置。`LANG` 環境変数でシステムロケールを自動検出

### CLI 機能追加
- `--verbose`: OK / Ignored エントリも表示し、reason を併記
- `--quiet`: サマリー行のみ出力
- `--json-output`: 監査結果を JSON で stdout に出力（CI/CD での機械処理向け）

## [Unreleased] — Phase 1

### Added
- Folder diff engine with seven diff types
- Audit definition YAML format (version 1)
- Content audit strategies: None, Checksum, LineMatch, Regex, Exact
- CLI subcommands: audit, snap, report
- GUI with snora/iced: opening screen and 3-pane main screen
- Approval flow requiring mandatory reason
- Markdown and JSON report generation
