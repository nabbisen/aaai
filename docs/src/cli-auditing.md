# Auditing Commands

Commands for running and inspecting audits.

## aaai audit

2つのフォルダを比較し、監査定義ファイルに照らして審査します。

```sh
aaai audit --left <BEFORE> --right <AFTER> --config <FILE> [OPTIONS]
```

| フラグ | 説明 |
|---|---|
| `-l, --left <PATH>` | 変更前フォルダ |
| `-r, --right <PATH>` | 変更後フォルダ |
| `-c, --config <FILE>` | 監査定義ファイル（YAML） |
| `--ignore <FILE>` | .aaaiignore ファイルのパス |
| `--verbose` | OK/Ignored エントリと reason も表示 |
| `--quiet` | サマリー行のみ出力 |
| `--json-output` | JSON 形式で stdout に出力 |
| `--allow-pending` | Pending エントリを許容（exit 0） |
| `--mask-secrets` | reason などの機密値をマスク |
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
| `--approver <NAME>` | 生成エントリの approved_by を設定 |
| `--suggest-glob` | グロブパターン化の提案を表示 |
| `--dry-run` | ファイルを書かずにプレビュー |

**例:**

```sh
# 初回スナップショット
aaai snap --left ./before --right ./after --out audit.yaml

# テンプレート適用（バージョン番号変更パターン）
aaai snap --left ./before --right ./after --out audit.yaml \
          --template version_bump

# 承認者を自動設定
aaai snap --left ./before --right ./after --out audit.yaml \
          --approver "alice" --suggest-glob
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
