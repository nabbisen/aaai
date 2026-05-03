# Workflow Commands

Commands for managing history, merging definitions, and automating workflows.

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
aaai history [-n <N>] [--stats] [--json-output]
```

| フラグ | 説明 |
|---|---|
| `-n <N>` | 表示件数（デフォルト: 10） |
| `--stats` | 合格率・平均件数・トレンド分析を表示 |
| `--json-output` | JSON 形式で出力 |

---

## aaai dashboard

カラーコードの統計カードを表示します。

```sh
aaai dashboard --left <BEFORE> --right <AFTER> --config <FILE> [--detail]
```

---

## aaai watch

ファイル変更を監視して自動再実行します。

```sh
aaai watch --left <BEFORE> --right <AFTER> --config <FILE> \
           [--debounce-ms <MS>]
```

Ctrl+C で停止します。

---
