# CLI リファレンス

aaai の全コマンドとフラグを説明します。

すべてのコマンドで `--help` が使えます。

```sh
aaai --help
aaai <command> --help
```

---

## aaai audit

2 つのフォルダを比較し、監査定義ファイルに照らして審査します。

```sh
aaai audit --left <BEFORE> --right <AFTER> --config <FILE> [OPTIONS]
```

| フラグ | 説明 |
|---|---|
| `-l, --left <PATH>` | 変更前フォルダ |
| `-r, --right <PATH>` | 変更後フォルダ |
| `-c, --config <FILE>` | 監査定義ファイル（YAML） |
| `--ignore <FILE>` | .aaaiignore ファイルのパス |
| `--verbose` | OK / Ignored エントリと reason も表示 |
| `--quiet` | サマリー行のみ出力 |
| `--json-output` | JSON 形式で stdout に出力 |
| `--allow-pending` | Pending エントリを許容（exit 0） |
| `--mask-secrets` | reason などの機密値をマスク |
| `--suppress-warnings <KIND,...>` | 指定した種別の警告を抑制 |
| `--progress` | プログレスバーを表示 |
| `--no-history` | 実行履歴を記録しない |

**終了コード:**

| コード | 意味 |
|---|---|
| 0 | PASSED — 全エントリ OK または Ignored |
| 1 | FAILED — 1 件以上の監査失敗 |
| 2 | PENDING — 未承認エントリあり（`--allow-pending` で 0 に） |
| 3 | ERROR — ファイル読み取りエラー |
| 4 | CONFIG_ERROR — 定義ファイルの構文エラー |

**例:**

```sh
# 基本的な監査
aaai audit --left ./before --right ./after --config audit.yaml

# CI/CD 向け（JSON 出力 + 履歴なし）
aaai audit --left ./before --right ./after --config audit.yaml \
           --json-output --no-history

# 除外パターン + 機密値マスク
aaai audit --left ./before --right ./after --config audit.yaml \
           --ignore .aaaiignore --mask-secrets
```

---

## aaai snap

現在の差分から監査定義テンプレートを生成します。

```sh
aaai snap --left <BEFORE> --right <AFTER> --out <FILE> [OPTIONS]
```

| フラグ | 説明 |
|---|---|
| `--merge` | 既存ファイルに新エントリをマージ |
| `--template <ID>` | ルールテンプレートを適用 |
| `--list-templates` | テンプレート一覧を表示して終了 |
| `--ignore <FILE>` | .aaaiignore ファイルのパス |
| `--approver <NAME>` | 生成エントリの `approved_by` を設定 |
| `--suggest-glob` | グロブパターン化の提案を表示 |
| `--dry-run` | ファイルを書かずにプレビュー |

**例:**

```sh
# 初回スナップショット
aaai snap --left ./before --right ./after --out audit.yaml

# バージョン番号変更パターンのテンプレートを適用
aaai snap --left ./before --right ./after --out audit.yaml \
          --template version_bump

# 承認者を自動設定
aaai snap --left ./before --right ./after --out audit.yaml \
          --approver "alice" --suggest-glob
```

---

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

## aaai check

差分を実行せずに定義ファイルの妥当性を検証します。

```sh
aaai check <FILE> [--all]
```

| フラグ | 説明 |
|---|---|
| `--all` | 正常エントリも表示 |

---

## aaai lint

定義ファイルのベストプラクティスをチェックします。

```sh
aaai lint <FILE> [OPTIONS]
```

| フラグ | 説明 |
|---|---|
| `--require-ticket` | 全エントリにチケットを必須化 |
| `--require-approver` | 全エントリに承認者を必須化 |
| `--min-reason-len <N>` | 理由の最小文字数（デフォルト: 10） |
| `--json-output` | JSON 形式で出力 |

**検出項目:**

| ID | 深刻度 | 内容 |
|---|---|---|
| `duplicate-path` | error | 同一パスの重複定義 |
| `empty-linematch` | error | LineMatch にルールなし |
| `empty-line-rule` | error | LineMatch の行が空 |
| `short-reason` | warning | 理由が短すぎる |
| `missing-ticket` | warning | チケット未設定（`--require-ticket` 時） |
| `missing-approver` | warning | 承認者未設定（`--require-approver` 時） |
| `expired` | warning | 有効期限切れ |
| `strategy-mismatch` | info | 追加/削除エントリに LineMatch |
| `disabled` | info | 無効化エントリ |

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

## aaai merge

2 つの定義ファイルをマージします。

```sh
aaai merge <BASE> <OVERLAY> [--out <FILE>] [OPTIONS]
```

| フラグ | 説明 |
|---|---|
| `--out <FILE>` | 出力先（省略時: BASE を上書き） |
| `--detect-conflicts` | 競合検出のみ（マージしない） |
| `--dry-run` | プレビューのみ |

---

## aaai history

過去の監査実行履歴を表示します。

```sh
aaai history [-n <N>] [--stats] [--prune <N>] [--json-output]
```

| フラグ | 説明 |
|---|---|
| `-n <N>` | 表示件数（デフォルト: 10） |
| `--stats` | 合格率・平均件数・トレンド分析を表示 |
| `--prune <N>` | 最新 N 件のみ残して刈り込む |
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

## aaai dashboard

カラーコードの統計カードを表示します。

```sh
aaai dashboard --left <BEFORE> --right <AFTER> --config <FILE> [--detail]
```

| フラグ | 説明 |
|---|---|
| `--detail` | 全変更エントリを一覧表示 |

---

## aaai watch

ファイル変更を監視して自動再実行します。

```sh
aaai watch --left <BEFORE> --right <AFTER> --config <FILE> \
           [--debounce-ms <MS>]
```

Ctrl+C で停止します。

---

## aaai init

対話型プロジェクト初期設定ウィザードです。

```sh
aaai init [--dir <DIR>] [--non-interactive]
```

`--non-interactive` でプロンプトをスキップしてデフォルト値で `.aaai.yaml` を生成します。

---

## aaai config

プロジェクト設定ファイル `.aaai.yaml` を表示または生成します。

```sh
aaai config [--init] [--dir <DIR>] [--show]
```

---

## aaai version

バージョン情報を表示します。

```sh
aaai version [--json-output]
```

---

## aaai completions

シェル補完スクリプトを生成します。

```sh
aaai completions <bash|zsh|fish|powershell>
```

**インストール例:**

```sh
# Bash
aaai completions bash >> ~/.bash_completion

# Zsh
aaai completions zsh > ~/.zfunc/_aaai
echo 'fpath=(~/.zfunc $fpath)' >> ~/.zshrc

# Fish
aaai completions fish > ~/.config/fish/completions/aaai.fish
```
