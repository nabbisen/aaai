# Changelog

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
