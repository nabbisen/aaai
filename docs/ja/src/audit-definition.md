# 監査定義ファイル

監査定義ファイル（`audit.yaml`）は、期待される変更ごとに  
人間が読める理由と任意の内容監査戦略を記述した YAML ファイルです。

---

## ファイル構造

```yaml
version: "1"
meta:
  description: "Release 2.0.0 差分監査"

entries:
  - path: "config/server.toml"
    diff_type: Modified
    reason: "ポートを 80 から 8080 に変更 — INF-42"
    ticket: "INF-42"
    approved_by: "alice"
    approved_at: "2026-05-01T09:00:00Z"
    expires_at: "2026-12-31"
    strategy:
      type: LineMatch
      rules:
        - action: Removed
          line: "port = 80"
        - action: Added
          line: "port = 8080"
    enabled: true
```

---

## エントリの各フィールド

| フィールド | 必須 | 説明 |
|---|---|---|
| `path` | ✅ | ファイルパスまたはグロブパターン |
| `diff_type` | ✅ | Added / Removed / Modified / Unchanged / TypeChanged |
| `reason` | ✅ | 変更を許容する人間可読な根拠（空欄の場合は Pending 扱い） |
| `strategy` | — | 内容監査戦略（省略時: None） |
| `ticket` | — | チケット参照番号（JIRA-123、INF-42 など） |
| `approved_by` | — | 承認者の名前または ID |
| `approved_at` | — | 承認日時（ISO-8601 UTC） |
| `expires_at` | — | 再審査期日（YYYY-MM-DD） |
| `enabled` | — | `false` にするとこのエントリを Ignored として扱う |
| `note` | — | 補足情報（判定に影響しない） |

---

## グロブパターン

```yaml
entries:
  - path: "logs/*.log"
    diff_type: Modified
    reason: "デプロイのたびにログがローテーションされる"
    strategy:
      type: None
```

完全パスのエントリは、グロブエントリより常に優先されます。

---

## 定義ファイルの生成

```sh
# 差分からテンプレートを生成
aaai snap --left ./before --right ./after --out audit.yaml

# テンプレートを適用して生成
aaai snap --left ./before --right ./after --out audit.yaml \
          --template version_bump --approver "alice"
```

生成直後は `reason` フィールドが空欄です。記入後に監査を実行してください。

---

## バリデーション

```sh
# 構文チェックのみ
aaai check audit.yaml

# ベストプラクティスチェック
aaai lint audit.yaml
aaai lint audit.yaml --require-ticket --require-approver
```

---

## 内容監査戦略

詳細は [内容監査戦略](strategies.md) を参照してください。

| 戦略 | 概要 |
|---|---|
| **None** | 差分種別のみ確認。内容検査なし |
| **Checksum** | SHA-256 ダイジェストを検証。バイナリファイル向け |
| **LineMatch** | 特定行の追加/削除を検証。設定値変更向け |
| **Regex** | 変更行が正規表現に一致することを検証 |
| **Exact** | ファイル全体の内容を完全一致で検証 |
