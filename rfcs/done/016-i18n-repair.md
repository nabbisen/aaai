# RFC 016 — i18n Locale File Repair

**Status.** Implemented (v0.19.0) — visual verification pending
**Priority.** v1.0 blocker
**Tracks.** RFC 015 の前提 / 全 GUI 画面の表示文字列正常化
**Touches.** `crates/aaai-gui/locales/en.yaml` · `ja.yaml` · `src/main.rs` (i18n!() マクロ)

---

## 1. 要件定義

### 1.1 目的

`t!()` マクロが翻訳済み文字列を返すようにする。
現状はキー名（例: `"opening.title"`）がそのまま画面に表示されている。

### 1.2 現状の症状

ユーザーから報告された画面：

```
opening.title              ← "aaai" であるべき
opening.subtitle           ← "audit for asset integrity" であるべき
opening.before_label       ← "比較元フォルダ" であるべき
…（全 t!() 呼び出しが同様）
```

### 1.3 機能要件

| ID | 要件 |
|---|---|
| FR-1 | 全 UI で表示される文字列がロケール（en / ja）に応じて表示される |
| FR-2 | ロケール切替が即座に反映される |
| FR-3 | キーが存在しない場合のフォールバックは英語（既存仕様） |

### 1.4 非機能要件

| ID | 要件 |
|---|---|
| NFR-1 | YAML ファイルが構文的に妥当（`yaml.safe_load` 通過） |
| NFR-2 | コード中の全 `t!()` 呼び出しに対応するキーが両ロケールに存在 |

---

## 2. 外部設計（基本設計）

外部仕様：ユーザーから見た期待動作は単純で「**画面上の文字列が選択した言語で表示される**」のみ。

| 操作 | 期待結果 |
|---|---|
| 起動（環境変数 LANG=ja） | 全文字列が日本語表示 |
| 起動（LANG=en） | 全文字列が英語表示 |
| フッターから言語切替 | 即座に全文字列が切替先言語で再描画 |

---

## 3. 内部設計（詳細設計）

### 3.1 根本原因の調査結果

```yaml
# 現状の en.yaml の冒頭
_version: 1            ← トップレベルに混入している不要キー

app:
  title: aaai — audit for asset integrity
batch:
  approve_button: Approve All
  …
opening:
  title: aaai
  …
```

**仮説 A**: `_version: 1` がトップレベルに存在することで `rust-i18n` v4 の YAML パーサが
ファイル全体の構造を誤認識している可能性。

**仮説 B**: `rust-i18n` v4 のマクロ `i18n!("locales", fallback = "en")` は
`build.rs` 経由で YAML を読み込むが、build キャッシュが古い状態のまま固定されている可能性。

**仮説 C**: `rust-i18n` v4 は YAML のルートに `en:` のようなロケール識別子を期待しており、
ファイル名による識別ではなく内容での識別を要する可能性。

### 3.2 検証手順

| ステップ | コマンド / 検査 | 期待結果 |
|---|---|---|
| 1 | `yamllint locales/en.yaml` / `yaml.safe_load` | 構文エラーなし |
| 2 | `rust-i18n` v4 の README で期待する YAML 構造を再確認 | ロケール識別子の要否を確定 |
| 3 | `crates/aaai-gui` で `cargo clean && cargo build` | build.rs キャッシュ再生成 |
| 4 | 最小再現コードでの動作確認 | `t!("app.title")` が `"aaai — audit for asset integrity"` を返す |

### 3.3 修復計画

判定結果に応じて 3 つの修復パターンのいずれかを採用：

#### パターン A: `_version` キーを削除するだけで解決

```yaml
# en.yaml (修復後)
app:
  title: aaai — audit for asset integrity
…
```

#### パターン B: ロケール識別子を追加

```yaml
# en.yaml (修復後)
en:
  app:
    title: aaai — audit for asset integrity
  …
```

#### パターン C: ファイル分割

`locales/en/app.yaml`, `locales/en/opening.yaml` のようにキー名前空間ごとに分割。

→ **検証手順 1〜4 を完了するまで採用パターンは確定しない**。RFC 段階では選択肢を保持する。

### 3.4 キー漏れ検証

修復後に、コード中で参照されているキー一覧を抽出し、両ロケールで欠落がないかチェックする：

```python
# scripts/check_i18n_keys.py（新規作成）
1. crates/aaai-gui/src/ 配下を再帰検索し
   t!("...") のキー文字列を全て抽出
2. en.yaml / ja.yaml のキー階層と照合
3. 欠落キーがあれば error 終了
```

CI に組み込むかは別途検討。

---

## 4. プログラム設計

### 4.1 修復作業の手順

| Step | 作業 | 検証 |
|---|---|---|
| 1 | `cargo clean` で build キャッシュ全削除 | — |
| 2 | `locales/en.yaml` の `_version: 1` を削除 | YAML 再パース |
| 3 | `cargo build -p aaai-gui` で再ビルド | 警告なし |
| 4 | GUI 起動し `app.title` が翻訳表示されるか目視確認 | 表示確認 |
| 5 | (Step 4 で解決しない場合) パターン B または C を試行 | 表示確認 |
| 6 | 確定したパターンで両 YAML を整形 | — |
| 7 | キー漏れチェッカーで全 `t!()` キー存在確認 | exit 0 |

### 4.2 影響範囲

- 純粋にデータ修正のため Rust ソースコード変更は最小（または不要）
- ただし build.rs キャッシュ問題なら `cargo clean` が必須

### 4.3 リスク

| リスク | 対策 |
|---|---|
| YAML 整形時に再度構造を壊す | `yamllint` 経由で必ず検証 |
| 一部キーが両ロケールで非対称 | キー漏れチェッカーで検出 |
| ロケール切替メッセージが再描画をトリガーしない | 既存の `LanguageChanged` ハンドラを確認 |

---

## 5. 完了条件

- [ ] `yamllint locales/*.yaml` がエラーなし
- [ ] GUI 起動時に `opening.title` などのキーが翻訳表示される
- [ ] `LANG=en` / `LANG=ja` で表示が切り替わる
- [ ] フッターの言語切替で即座に再描画
- [ ] キー漏れチェッカーで両ロケールに欠落なし

## 6. 依存

- RFC 015 の前提（Opening 画面再設計で新規 i18n キーを追加する前にこの修復を完了する必要がある）
