# Changelog

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
