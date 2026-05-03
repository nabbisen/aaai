# aaai 概要

**aaai** (audit for asset integrity) は、2 つのフォルダの差分を比較し、  
検出されたすべての変更に対して人間が読める「理由」の記載を必須とするフォルダ差分監査ツールです。

---

## 構成

| コンポーネント | 説明 |
|---|---|
| [`aaai-core`](https://crates.io/crates/aaai-core) | コアエンジン — diff・audit・report・masking |
| [`aaai-cli`](https://crates.io/crates/aaai-cli) | `aaai` コマンドラインバイナリ |
| [`aaai-gui`](https://crates.io/crates/aaai-gui) | `aaai-gui` デスクトップアプリ（iced） |

---

## 主な特徴

| 機能 | 説明 |
|---|---|
| フォルダ差分 | rayon による並列比較 |
| 監査定義 | 変更ごとに reason を必須とする YAML ファイル |
| 内容監査戦略 | None / Checksum / LineMatch / Regex / Exact |
| レポート出力 | Markdown・JSON・HTML・SARIF（GitHub Actions 向け） |
| 機密値マスキング | API キー・パスワード等の正規表現ベース自動マスク |
| 監査警告 | 大容量ファイルへの戦略適用・承認者未設定などの advisory |
| CLI | 15 コマンド（audit・snap・lint・diff・merge・watch など） |
| GUI | 3 ペインリサイズ可能なデスクトップ UI。ダーク/ライトテーマ対応 |
| CI/CD | 詳細な終了コード（0=PASSED … 4=CONFIG_ERROR）、SARIF 出力 |

---

## クイックスタート

```sh
# 差分からテンプレートを生成
aaai snap --left ./before --right ./after --out audit.yaml

# reason フィールドを記入後、監査を実行
aaai audit --left ./before --right ./after --config audit.yaml
```

詳細は [はじめに](getting-started.md) を参照してください。

---

## ライセンス

Apache-2.0 — [LICENSE](https://github.com/nabbisen/aaai/blob/main/LICENSE) および  
[NOTICE](https://github.com/nabbisen/aaai/blob/main/NOTICE) を参照してください。
