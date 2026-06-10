# RFC 017 — Visual Verification Harness & Protocol

**Status.** Implemented (v0.20.0)
**Priority.** v1.0 blocker (前提条件)
**Tracks.** HANDOVER-v0.19.0.md §4.1「コード書いた ≠ 動く」問題 / 設計書全ページの最終確認
**Touches.** `docs/src/testing.md` · `docs/ja/src/testing.md` · `crates/aaai-gui/tests/` · 新規ディレクトリ `verification/` （スクリーンショット保管） · `.github/workflows/ci.yaml`

---

## 1. 要件定義

### 1.1 目的

「コードを書いた」と「設計書通りに動く」の間に再び silent failure を発生させないために、
**視覚検証を制度として埋め込む**。RFC 007〜016 の実装結果が、それぞれ設計書 p.X の
意図と一致していることを、スクリーンショットと文書による表明として残せる状態にする。

### 1.2 解決すべき問題

| 問題 | 出典 |
|---|---|
| Sprint D-2 で RFC 007/009 の view 側差し替えが silent failure。v0.18.0 で RFC 014 として再適用が必要に | HANDOVER §1.3 |
| v0.18.0 リリース後の Opening 画面 i18n 表示・テキスト入力 UX 問題をユーザーが発見 | HANDOVER §4.1 |
| v0.19.0 の RFC 015/016 も `cargo check` 通過のみで検証完了扱いされている | HANDOVER §2.1 R-1〜R-3 |
| docs/src/testing.md は受け入れ基準を列挙するが、誰がどう実施するかの運用が無い | docs/src/testing.md §9 |

### 1.3 機能要件

| ID | 要件 | 必須/任意 |
|---|---|---|
| FR-1 | RFC 単位で「設計書の対応ページ」「期待スクリーンショット」「比較観点」を1ファイルで保持できる | 必須 |
| FR-2 | 過去の RFC 001〜016 すべてに対し、検証カードを発行できる | 必須 |
| FR-3 | 視覚検証の **欠如** が CI 上で検出できる（少なくとも警告） | 必須 |
| FR-4 | 検証スクリーンショットは Git に含めず、`verification/` 配下を `.gitignore` で除外する | 必須 |
| FR-5 | 設計書 PDF を直接参照する代わりに、抜粋画像（または ASCII 図）を検証カードに添付できる | 任意 |
| FR-6 | 検証カードに「合格 / 不合格 / 例外 (理由)」の 3 値を記録できる | 必須 |
| FR-7 | 検証カードが未記入の RFC を一覧で抽出できる | 必須 |

### 1.4 非機能要件

| ID | 要件 |
|---|---|
| NFR-1 | 検証作業者は Rust 開発者でなくても運用できる（チェックリスト追従のみで完結） |
| NFR-2 | 1 RFC あたりの検証時間が現実的（目安 5〜10 分） |
| NFR-3 | 検証結果が Git に残らない場合でも、欠落していることが CI で把握できる |

---

## 2. 外部設計（基本設計）

### 2.1 検証カードの形式

各 RFC に対し、`rfcs/done/<NNN>-<slug>.md` 末尾または `rfcs/verification/<NNN>.md` 別ファイルに
次のセクションを追加する：

```markdown
## Visual Verification

**Verified.** YYYY-MM-DD by <name>
**Build.** vX.Y.Z (git: <short-sha>)
**Platform.** macOS 14 / Ubuntu 24.04 / Windows 11

### Checks

| 設計書参照 | 期待 | 実観測 | 判定 |
|---|---|---|---|
| p.5 ツールバー左端 | 「開く / 保存 / 監査実行 / レポート出力」の順 | (実観測) | ✅ / ❌ / 例外 |
| ... | ... | ... | ... |

### Screenshots (optional, not committed)

`verification/<NNN>/main-toolbar.png` で保管。
```

各 RFC 末尾の `## Visual Verification` セクションは **欠落していてもファイル自体は有効** とする
（過去 RFC を後付けで検証する場合に備える）。

### 2.2 「未検証 RFC 一覧」スクリプト

```bash
$ scripts/list-unverified-rfcs.sh
RFC 001 (cli-output-ux): UNVERIFIED
RFC 002 (inspector-validation): UNVERIFIED
...
RFC 015 (opening-screen-redesign): UNVERIFIED  ← v0.19.0 重大
RFC 016 (i18n-repair): UNVERIFIED  ← v0.19.0 重大

Total: 16 / 16 unverified.
```

### 2.3 CI への組み込み（最小）

`ci.yaml` に「未検証 RFC 件数」をログとして表示する step を追加する。**fail させない**。
これは「視認できる」状態を保つためのもの。

### 2.4 検証の優先順位

| 優先度 | 範囲 |
|---|---|
| P0 | RFC 015 (Opening 再設計) / RFC 016 (i18n) — v0.19.0 の核 |
| P1 | RFC 014 (View 修復) / RFC 007 (Toolbar) / RFC 008 (Bottom bar) — 過去に silent failure を起こした周辺 |
| P2 | RFC 009 (Reason textarea) / RFC 010 (Diff legend) / RFC 011 (Diff tabs) / RFC 012 (LineMatch blocks) / RFC 013 (File tree icons) |
| P3 | RFC 001 〜 006 — 既に出荷済みで顕在化していない |

優先度 P0 を Phase 12 で必ず実施。P1〜P2 は同 Phase 内で完了が望ましい。P3 は本 RFC の対象外
（必要時のみ別途検証）。

---

## 3. 内部設計（詳細設計）

### 3.1 ディレクトリ構造

```
rfcs/
├── done/                    # 既存
├── proposed/
├── verification/            # 新規（.gitignore 対象）
│   ├── 015/
│   │   ├── opening-main.png
│   │   └── opening-optional-expanded.png
│   └── 016/
│       └── locale-en.png
└── PLAN.md
```

`.gitignore` 追記:
```
/rfcs/verification/
```

### 3.2 検証カードのテンプレート

`docs/templates/visual-verification-template.md` を新規作成し、新規 RFC の Verified
セクションをすぐに追加できるようにする。

### 3.3 `scripts/list-unverified-rfcs.sh` の実装

```bash
#!/usr/bin/env bash
# scripts/list-unverified-rfcs.sh
set -euo pipefail

cd "$(dirname "$0")/.."
unverified=0
total=0
for f in rfcs/done/[0-9]*.md; do
    total=$((total + 1))
    if ! grep -q "^## Visual Verification" "$f"; then
        echo "RFC $(basename "$f" .md): UNVERIFIED"
        unverified=$((unverified + 1))
    fi
done
echo
echo "Total: $unverified / $total unverified."
```

`chmod +x` でコミット。

### 3.4 testing.md との関係

`docs/src/testing.md` は受け入れ基準 (acceptance criteria) を持っている。
RFC 017 の検証カードは、testing.md の各テストケースが「設計書のどの記述に対応するか」を
裏付けるリンク先として機能する。両者は冗長ではなく相補的とする。

### 3.5 v0.19.0 のカードを先行作成

RFC 015 と RFC 016 のスクリーンショットを実機で取得し、当該 RFC の `done/` ファイル末尾に
Visual Verification セクションを追加する。これが本 RFC の最初の実運用例となる。

---

## 4. プログラム設計

### 4.1 実装手順

| Step | 作業 | 検証 |
|---|---|---|
| 1 | `scripts/list-unverified-rfcs.sh` を作成 | 手動実行で出力確認 |
| 2 | `.gitignore` に `/rfcs/verification/` を追加 | `git status` で除外確認 |
| 3 | `docs/templates/visual-verification-template.md` を作成 | レビュー |
| 4 | RFC 015 を実機で起動し検証カードを記入 | 設計書 p.2 / p.5 と一致 |
| 5 | RFC 016 を検証 (en/ja 切替・全画面文字列) | リテラルキー残存ゼロ |
| 6 | RFC 014, 007, 008 を検証 (silent failure 履歴) | 一致 |
| 7 | RFC 009〜013 を検証 | 一致 |
| 8 | `ci.yaml` に未検証件数表示 step を追加 | CI ログに出力されることを確認 |

### 4.2 影響範囲

- Rust ソースコード: なし
- ドキュメント: テンプレート 1 件、`docs/src/testing.md` への参照リンク 1 件
- スクリプト: 1 件追加
- CI: 1 step 追加

### 4.3 リスク

| リスク | 対策 |
|---|---|
| スクリーンショットが OS / フォント差で揺れて検証時に紛糾する | 「主要要素の配置・順序・有無」を判定基準とする。pixel-perfect は要求しない |
| 検証作業が形骸化する (チェックだけ入って未確認) | 検証日時 + 担当者署名を必須とし、欠落は弱い警告にとどめるが目立たせる |
| 視覚検証で iced の OS 差異 (フォント描画など) が顕在化する | 各プラットフォームで揃った時のみ「合格」とし、片方しか確認していない場合は「例外: 〇〇 未検証」と記録 |

---

## 5. 完了条件

- [ ] `scripts/list-unverified-rfcs.sh` が実行可能で、未検証 RFC を列挙する
- [ ] `.gitignore` に `verification/` が追加されている
- [ ] `docs/templates/visual-verification-template.md` が存在する
- [ ] RFC 015 / RFC 016 / RFC 014 / RFC 007 / RFC 008 の Visual Verification セクションが
      v0.20.0 リリース時点で記入済み（P0 + P1 完了）
- [ ] RFC 009 / RFC 010 / RFC 011 / RFC 012 / RFC 013 についても、v0.20.0 出荷前に
      P2 として記入済みであることが望ましい
- [ ] CI ログに未検証件数が出力される

## 6. 依存

- なし（本 RFC は基盤プロトコル）

## 7. 後続 RFC への影響

- RFC 018, 019, 020 以降の RFC は、`done/` 移動時に Visual Verification セクション記入を
  必須とする
