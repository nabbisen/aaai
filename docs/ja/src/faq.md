# よくある質問 (FAQ)

---

## なぜすべてのエントリに reason（理由）が必要なのですか？

aaai の中心的な価値は **説明可能性** です。  
reason のない差分は「何が変わったか」は示しますが「なぜ許容されたか」を示しません。  
reason を必須にすることで、各エントリは技術的な事実から将来の保守者が理解できる意思決定の記録へと変わります。

---

## 監査対象から除外したいファイルはどうすればいいですか？

`.aaaiignore` ファイルに gitignore スタイルのパターンを記述します。

```
# ビルド生成物
target/**
dist/**
*.lock

# OS ファイル
.DS_Store
Thumbs.db
```

省略時は `Before/.aaaiignore` が自動検索されます。  
CLI では `--ignore` フラグ、GUI では Opening 画面の専用フィールドで指定できます。

---

## 監査ルールにグロブパターンは使えますか？

はい。`path` フィールドにグロブパターンを使えます。

```yaml
- path: "logs/*.log"
  diff_type: Modified
  reason: "デプロイのたびにログがローテーションされる"
  strategy:
    type: None
```

完全パスのエントリは、グロブエントリより常に優先されます。

---

## エントリの有効期限が切れるとどうなりますか？

`expires_at` が過去の日付になっても、監査の verdict は変わりません（**再審査の促し**であって強制ではありません）。  
CLI とGUI の両方で警告表示されるので、再審査が必要なことがわかります。

---

## 2 つのチームの定義ファイルをマージするには？

```sh
aaai merge base.yaml overlay.yaml --out merged.yaml
```

競合（同一パスに異なる diff_type）がある場合は overlay が優先されます。  
`--detect-conflicts` で競合チェックのみ実行できます。

---

## レポートに実際の差分テキストを含めるには？

```sh
aaai report --left ./before --right ./after \
            --config ./audit.yaml --out report.md \
            --include-diff
```

変更のある全テキストファイルに `diff` 形式のブロックを埋め込みます。

---

## SARIF 出力は何のために使いますか？

SARIF（Static Analysis Results Interchange Format）は  
GitHub・GitLab・Azure DevOps がプルリクエストにインライン注釈を表示するために使う JSON 標準です。  
`--format sarif` で出力し、CI の upload-sarif アクションでアップロードすると  
ファイル・行ごとの注釈が表示されます。

---

## 機密情報をレポートに含めないようにするには？

```sh
aaai audit --mask-secrets --left ./before --right ./after --config ./audit.yaml
```

または `.aaai.yaml` に設定します。

```yaml
mask_secrets: true
```

API キー・パスワード・Bearer トークンなど、9 種類のビルトインパターンで自動マスクします。  
カスタムパターンは `custom_mask_patterns` で追加できます。

---

## AuditWarning の特定の種別を無効化するには？

`.aaai.yaml` に記述します。

```yaml
suppress_warnings:
  - no-approver
  - no-strategy
```

または CLI フラグで指定します。

```sh
aaai audit --suppress-warnings no-approver,no-strategy ...
```

---

## aaai は CI で実行したファイルを変更しますか？

aaai は比較対象のファイルを変更しません。書き込みが発生するのは以下のみです。

- `--config` で指定した監査定義ファイル（承認時）
- `~/.aaai/history.jsonl`（`--no-history` で無効化可能）
- `~/.aaai/prefs.yaml`（GUI テーマ設定）

---

## 履歴ファイルが大きくなりすぎた場合は？

```sh
# 最新 100 件のみ残す
aaai history --prune 100
```
