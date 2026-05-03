# CLI リファレンス

`aaai` は 15 のコマンドを提供します。各コマンドの詳細は `--help` で確認できます。

```sh
aaai --help
aaai <command> --help
```

---

## 終了コード (`aaai audit`)

| コード | 意味 |
|---|---|
| 0 | PASSED — 全エントリ OK または Ignored |
| 1 | FAILED — 1 件以上の監査失敗 |
| 2 | PENDING — 未承認エントリあり（`--allow-pending` で 0 に） |
| 3 | ERROR — ファイル読み取りエラー |
| 4 | CONFIG_ERROR — 定義ファイルの構文エラー |

---

## カテゴリ別コマンド

### [監査](cli-auditing.md)

| コマンド | 説明 |
|---|---|
| [`aaai audit`](cli-auditing.md#aaai-audit) | フォルダ比較と監査の実行 |
| [`aaai snap`](cli-auditing.md#aaai-snap) | 差分から定義テンプレートを生成 |
| [`aaai check`](cli-auditing.md#aaai-check) | 差分なしで定義ファイルを検証 |
| [`aaai lint`](cli-auditing.md#aaai-lint) | 定義ファイルのベストプラクティスチェック |

### [レポート・エクスポート](cli-reporting.md)

| コマンド | 説明 |
|---|---|
| [`aaai report`](cli-reporting.md#aaai-report) | Markdown / JSON / HTML / SARIF レポートを出力 |
| [`aaai diff`](cli-reporting.md#aaai-diff) | 定義なしの純粋な差分表示 |
| [`aaai export`](cli-reporting.md#aaai-export) | 監査エントリを CSV / TSV にエクスポート |

### [ワークフロー](cli-workflow.md)

| コマンド | 説明 |
|---|---|
| [`aaai merge`](cli-workflow.md#aaai-merge) | 2 つの定義ファイルをマージ |
| [`aaai history`](cli-workflow.md#aaai-history) | 過去の監査実行履歴を表示 |
| [`aaai dashboard`](cli-workflow.md#aaai-dashboard) | カラーコードの統計ダッシュボード |
| [`aaai watch`](cli-workflow.md#aaai-watch) | ファイル変更を監視して自動再実行 |

### [セットアップ・ツール](cli-setup.md)

| コマンド | 説明 |
|---|---|
| [`aaai init`](cli-setup.md#aaai-init) | 対話型プロジェクト初期設定ウィザード |
| [`aaai config`](cli-setup.md#aaai-config) | `.aaai.yaml` プロジェクト設定の表示・生成 |
| [`aaai version`](cli-setup.md#aaai-version) | バージョン・ビルド情報を表示 |
| [`aaai completions`](cli-setup.md#aaai-completions) | シェル補完スクリプトを生成 |
