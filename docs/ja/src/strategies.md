# 内容監査戦略

aaai は 5 種類の内容監査戦略を提供します。  
戦略は監査定義ファイルの各エントリで指定します。

---

## None

差分種別（Added / Modified など）が一致することだけを確認します。  
内容の検査は行いません。

```yaml
strategy:
  type: None
```

**使いどころ:** ファイルの追加/削除で内容確認が不要な場合、  
またはより具体的なルールを後で追加するためのプレースホルダーとして使います。

**注意:** 重要な設定ファイルに対しては、後でより具体的な戦略を設定することを検討してください。

---

## Checksum

ファイルの SHA-256 ダイジェストが期待値と一致することを確認します。

```yaml
strategy:
  type: Checksum
  expected_sha256: "abc123...（64 文字の 16 進数）"
```

**使いどころ:** バイナリファイル（画像・アーカイブ・コンパイル済みアセットなど）、  
またはバイト単位での同一性を保証したい任意のファイル。

**ハッシュの取得方法:**

```sh
sha256sum myfile.bin   # Linux
shasum -a 256 myfile   # macOS
```

---

## LineMatch

指定した行が追加・削除されていることを確認します。

```yaml
strategy:
  type: LineMatch
  rules:
    - action: Removed
      line: "port = 80"
    - action: Added
      line: "port = 8080"
```

**使いどころ:** 設定値の変更確認。TOML・YAML・`.env`・INI ファイルに最もよく使う戦略です。

**ルール:** 各ルールは 1 行を検証します。  
順序は関係ありません。空白（先頭・末尾）は区別されます。

---

## Regex

変更行が正規表現パターンに一致することを確認します。

```yaml
strategy:
  type: Regex
  pattern: "^version = \"\\d+\\.\\d+\\.\\d+\""
  target: AddedLines   # AddedLines | RemovedLines | AllChangedLines
```

**使いどころ:** バージョン番号・日付など、値は変わるが形式は固定のケース。

**`target` の選択:**

| 値 | 説明 |
|---|---|
| `AddedLines` | 追加された行にのみ適用（デフォルト） |
| `RemovedLines` | 削除された行にのみ適用 |
| `AllChangedLines` | 追加・削除の全変更行に適用 |

---

## Exact

ファイル全体の内容が期待値と完全に一致することを確認します。

```yaml
strategy:
  type: Exact
  expected_content: |
    [server]
    host = "0.0.0.0"
    port = 8080
```

**使いどころ:** 小さく安定したファイルで、いかなる変化も許容しない場合。

**注意:** 数 KB を超えるファイルへの適用は避けてください。  
頻繁に変更されるファイルでは保守が困難になります。

---

## 戦略の選び方

| ファイルの種類 | 推奨戦略 |
|---|---|
| バイナリ・画像・アーカイブ | Checksum |
| 設定ファイル（キー=値の変更） | LineMatch |
| バージョン番号・日付の変更 | Regex |
| 追加/削除のみ（内容は不要） | None |
| 小さく安定したテキストファイル | Exact |

---

## 大容量ファイルの警告

1 MB を超えるファイルに **Exact** または **LineMatch** を適用すると、  
`AuditWarning::LargeFileStrategy` 警告が発生します。  
このような場合は **Checksum** の使用を検討してください。
