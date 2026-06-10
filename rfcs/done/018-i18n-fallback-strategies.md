# RFC 018 — i18n Locale Fallback Strategies (Patterns B/C)

**Status.** Implemented partial (v0.20.0) — §3.4 i18n key audit only; main B/C work conditional on RFC 016 visual verification
**Priority.** v1.0 blocker (条件付き — RFC 017 の検証結果による)
**Tracks.** RFC 016 §3.3 でパターン A のみ採用、B/C の未試行のままだったタスクを完了する
**Touches.** `crates/aaai-gui/locales/en.yaml` · `ja.yaml` · `src/main.rs` (`rust_i18n::i18n!` マクロ) · 場合により `locales/` ディレクトリ構造

---

## 1. 要件定義

### 1.1 目的

RFC 017 の視覚検証で、`t!()` マクロが依然としてリテラルキーを返すことが判明した場合、
RFC 016 §3.3 で予備案として置かれていたパターン B または C を実行し、確実に翻訳済み
文字列が表示される状態にする。

### 1.2 背景

RFC 016 で 3 つの仮説 A/B/C を立てて A (`_version: 1` 行削除) のみ実装し v0.19.0 を出荷した。
HANDOVER-v0.19.0.md §2.2 はこれが解決しなかった場合の判断基準を残している:

| 観察される文字列 | 原因仮説 |
|---|---|
| `"opening.title"` がそのまま表示 | 翻訳辞書未読込 → パターン B |
| `"en.opening.title"` が表示 | ロケール未設定 → `detect_locale()` の問題 |

### 1.3 機能要件

| ID | 要件 | 必須/任意 |
|---|---|---|
| FR-1 | `t!()` が翻訳済み文字列を返す | 必須 |
| FR-2 | en / ja 両方で動作する | 必須 |
| FR-3 | フォールバックは英語 (既存仕様) | 必須 |
| FR-4 | 採用したパターン (A/B/C) と判定経緯を記録する | 必須 |
| FR-5 | 全 `t!()` キーが両ロケールに存在することを CI で検証する | 任意 (推奨) |

### 1.4 非機能要件

| ID | 要件 |
|---|---|
| NFR-1 | YAML 構文として妥当 (yamllint 通過) |
| NFR-2 | `cargo clean` 実行を必須化する手順をドキュメント化 |
| NFR-3 | 設計書の他要件 (例: 言語切替) を破壊しない |

---

## 2. 外部設計（基本設計）

### 2.1 採用判断フロー

```
RFC 017 で v0.19.0 GUI 起動
         │
         ▼
  「t!()」 はキーをそのまま返している？
       ┌─── いいえ ──→ パターン A で確定 → 終了（本 RFC は実装不要）
       │
       はい
       ▼
  返り値の形式は？
       ┌─── "opening.title"          → パターン B 採用
       └─── "en.opening.title"       → detect_locale() の修正 + パターン A 維持
```

### 2.2 パターン B: ロケール識別子を YAML ルートに追加

`locales/en.yaml`:
```yaml
en:
  app:
    title: aaai — audit for asset integrity
  ...
```

`locales/ja.yaml`:
```yaml
ja:
  app:
    title: aaai — 資産整合性監査
  ...
```

### 2.3 パターン C: 名前空間別ファイル分割

```
locales/
├── en/
│   ├── app.yaml
│   ├── opening.yaml
│   ├── inspector.yaml
│   └── ...
└── ja/
    ├── app.yaml
    ├── opening.yaml
    ├── inspector.yaml
    └── ...
```

`rust_i18n::i18n!("locales", fallback = "en")` のままで動作することを確認する。

### 2.4 採否の記録

採用したパターンを `docs/src/i18n.md` (新規) に「採用パターン: X / 採用日: YYYY-MM-DD /
試行ログ」として記録する。

---

## 3. 内部設計（詳細設計）

### 3.1 rust-i18n v4 の挙動再確認

HANDOVER-v0.19.0.md §5.1 が分析している通り、`t!()` マクロは以下の順で展開される:

```
1. _rust_i18n_try_translate(locale, key) を呼ぶ
2. 翻訳ヒットすればその文字列を返す
3. ヒットしなければ msg_val (= リテラルキー文字列) を返す
```

つまり「キー文字列がそのまま表示される」 = 翻訳辞書がそのキーを保持していない。
これは「YAML が読まれていない」または「キー階層が読み込み時の期待と不一致」のいずれか。

### 3.2 パターン B 採用時の差分

```diff
- # locales/en.yaml (v0.19.0)
- app:
-   title: aaai — audit for asset integrity
+ # locales/en.yaml (v0.20.0 パターン B)
+ en:
+   app:
+     title: aaai — audit for asset integrity
```

`rust_i18n::i18n!("locales", fallback = "en")` 呼び出しはそのまま。
`set_locale("en")` を呼ぶと "en" 配下のキーが解決対象になる。

### 3.3 パターン C 採用時の差分

ディレクトリ再編に加え、`i18n!` の引数解釈に変更がない場合、そのまま動作するはず。
`rust-i18n` v4 のソースコード調査が必要 (パターン C の妥当性確認)。

### 3.4 キー漏れチェッカー

`scripts/check-i18n-keys.sh` (RFC 016 §3.4 で言及済み・未実装):

```bash
#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/.."

# 1. コード中の t!() キー一覧を抽出
keys_in_code=$(rg -No "t!\(\"([^\"]+)\"\)" -r '$1' crates/aaai-gui/src \
    | sort -u)

# 2. en/ja YAML に存在するキーと照合
for locale in en ja; do
    missing=()
    while IFS= read -r key; do
        # YAML 階層 (a.b.c) を確認
        if ! python3 -c "
import yaml, sys
with open('crates/aaai-gui/locales/$locale.yaml') as f:
    data = yaml.safe_load(f) or {}
parts = '$key'.split('.')
cur = data
for p in parts:
    if not isinstance(cur, dict) or p not in cur:
        sys.exit(1)
    cur = cur[p]
" 2>/dev/null; then
            missing+=("$key")
        fi
    done <<< "$keys_in_code"

    if [ ${#missing[@]} -gt 0 ]; then
        echo "$locale: missing keys:"
        printf '  - %s\n' "${missing[@]}"
        exit 1
    fi
done

echo "All keys are present in en + ja."
```

### 3.5 ロケール切替の再描画

`Message::LocaleChanged(String)` は既存。`update()` 内で `rust_i18n::set_locale(&code)` を
呼べば、次の `view()` 呼び出しで全文字列が更新される。

---

## 4. プログラム設計

### 4.1 実装手順 (パターン B 採用想定)

| Step | 作業 | 検証 |
|---|---|---|
| 1 | `cargo clean -p aaai-gui` | キャッシュ完全削除 |
| 2 | `locales/en.yaml` の全キーを `en:` 配下にインデント | YAML 再パース |
| 3 | `locales/ja.yaml` の全キーを `ja:` 配下にインデント | YAML 再パース |
| 4 | `cargo build -p aaai-gui` | 警告ゼロ |
| 5 | GUI 起動 (`LANG=ja` / `LANG=en`) | 全文字列が翻訳表示 |
| 6 | フッターから言語切替 | 再描画される |
| 7 | `scripts/check-i18n-keys.sh` を実行 | exit 0 |
| 8 | `docs/src/i18n.md` に採用結果を記録 | レビュー |

### 4.2 影響範囲

- `locales/en.yaml`, `locales/ja.yaml`: ルート構造に変更
- `src/main.rs`: 変更なし (`i18n!()` マクロのまま)
- `src/i18n.rs`: 変更なし
- 全 `t!("...")` 呼び出し: コード変更なし (キー文字列はそのまま)

### 4.3 リスク

| リスク | 対策 |
|---|---|
| パターン B でも解決しない (パターン C へ移行) | 試行ログを残し再順序で実施 |
| YAML インデントエラーで build 失敗 | `yamllint` で構文確認後にコミット |
| ロケール切替で再描画されない iced 0.14 のバグ | 既存の `Message::LocaleChanged` パスを確認 |

---

## 5. 完了条件

- [ ] `cargo build -p aaai-gui` が警告なしで通過する
- [ ] `aaai-gui` 起動時、Opening 画面で `t!("opening.title")` が `"aaai"` を表示する (リテラルキーなし)
- [ ] `LANG=en` → 英語表示、`LANG=ja` → 日本語表示が機械的に切り替わる
- [ ] フッターのロケールピッカーで即時再描画される
- [ ] `scripts/check-i18n-keys.sh` が exit 0 を返す
- [ ] 採用パターンと判定経緯が `docs/src/i18n.md` に記録される
- [ ] RFC 017 の Visual Verification セクションで「i18n 表示 正常」と裏付けられる

## 6. 依存

- **RFC 017**: 視覚検証で本 RFC の発動可否を判断するため、RFC 017 のスクリーンショット
  プロトコルが先に走っている必要がある

## 7. 条件付き実行の判断基準

| 判定 | RFC 018 の実装 |
|---|---|
| RFC 017 検証で「全ロケールが正常表示」と確認 | **不要** (RFC 018 は最初から withdrawn 扱いとして `archive/` へ) |
| Opening 画面でリテラルキーが表示される | **必須** (本書のパターン B / C のいずれかを実行) |
| 中途半端 (一部画面のみリテラル表示) | 当該キーの YAML 階層を点検しパターン A を堅牢化 |
