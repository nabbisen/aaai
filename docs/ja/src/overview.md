# aaai 概要

**aaai** (audit for asset integrity) は、2つのフォルダの差分を比較し、
検出されたすべての変更に対して人間が読める「理由」の記載を必須とする監査ツールです。

## 構成

| コンポーネント | 説明 |
|---|---|
| `aaai-core` | コアエンジン — diff, audit, report, masking |
| `aaai-cli` | `aaai` コマンドラインバイナリ |
| `aaai-gui` | `aaai-gui` デスクトップアプリ (iced) |

## 特徴

- **フォルダ差分** — rayon による並列比較
- **監査定義** — 変更ごとに reason を必須とする YAML ファイル
- **内容監査戦略** — None / Checksum / LineMatch / Regex / Exact
- **レポート出力** — Markdown, JSON, HTML, SARIF
- **機密値マスキング** — API キー・パスワード等の自動マスク
- **CLI** — 15 コマンド (audit, snap, lint, diff, merge, watch …)
- **GUI** — 3 ペインリサイズ可能なデスクトップ UI

## クイックスタート

```sh
# 差分から監査定義テンプレートを生成
aaai snap --left ./before --right ./after --out audit.yaml

# reason フィールドを記入後、監査を実行
aaai audit --left ./before --right ./after --config audit.yaml
```
