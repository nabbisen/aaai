# Changelog

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
