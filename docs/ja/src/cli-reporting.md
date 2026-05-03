# レポート・エクスポートコマンド

レポート生成とデータエクスポートに使うコマンドです。

## aaai report

監査結果をレポートファイルに出力します。

```sh
aaai report --left <BEFORE> --right <AFTER> --config <FILE> \
            --out <FILE> [--format markdown|json|html|sarif]
```

| フラグ | 説明 |
|---|---|
| `--format` | markdown（デフォルト）/ json / html / sarif |
| `--include-diff` | 実差分テキストを Markdown / HTML に埋め込み |
| `--mask-secrets` | 機密値をマスク |

**例:**

```sh
aaai report --left ./before --right ./after --config audit.yaml \
            --format html --out report.html

# GitHub Actions 向け SARIF
aaai report --left ./before --right ./after --config audit.yaml \
            --format sarif --out results.sarif
```

---

## aaai diff

定義ファイルなしで純粋な差分を表示します。

```sh
aaai diff --left <BEFORE> --right <AFTER> [OPTIONS]
```

| フラグ | 説明 |
|---|---|
| `--content` | 変更行の実テキストを表示 |
| `--all` | Unchanged ファイルも表示 |
| `--json-output` | JSON 形式で出力 |

---

## aaai export

承認済みエントリを CSV または TSV に出力します。

```sh
aaai export --left <BEFORE> --right <AFTER> --config <FILE> \
            [--out <FILE>] [--format csv|tsv] [--all]
```

出力カラム: `path` / `diff_type` / `status` / `reason` / `strategy` / `ticket` /  
`approved_by` / `approved_at` / `expires_at` / `enabled` / `note` / `created_at` / `updated_at`

---
