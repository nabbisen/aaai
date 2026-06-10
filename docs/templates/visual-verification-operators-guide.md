# Visual Verification — Operator's Guide

Companion to `visual-verification-template.md`. This file lists the
specific design-document anchors and observable surfaces to check for each
RFC currently in `rfcs/done/`. It exists so an operator running the GUI
on a real display does not have to re-derive what to look at — only
report what they see.

The priorities `P0` / `P1` / `P2` follow RFC 017 §2.4.

## Setup

```sh
# from repository root
cargo build -p aaai-gui --release
mkdir -p /tmp/aaai-test/{before,after}
echo 'port = 80'   > /tmp/aaai-test/before/config.toml
echo 'port = 8080' > /tmp/aaai-test/after/config.toml
echo 'v1'          > /tmp/aaai-test/before/version.txt
./target/release/aaai-gui
```

Have `aaai_uiux_design.pdf` open side-by-side; verification is a comparison
exercise, not from memory.

For each RFC below, the workflow is:

1. Drive the GUI to the screen the RFC affects.
2. Run through the row-by-row checks.
3. (Optional) save screenshots under `rfcs/verification/<NNN>/`.
4. Copy the card from `docs/templates/visual-verification-template.md`
   into the end of `rfcs/done/<NNN>-<slug>.md` and fill it in.

---

## P0 — must-have before v0.20.0

### RFC 015 — Opening Screen Redesign (v0.18.0)

**Design anchors.** PDF p.2 (中心体験「選ぶ → 見る → ...」), p.5
(GUI 主画面の周辺), HANDOVER §1.3.

**Open the app fresh** (no preselected profile). Inspect:

| 設計書参照 | 期待 | 確認方法 |
|---|---|---|
| p.2 / p.5 | ヘッダーに「aaai」タイトル + サブタイトル | 起動直後の最上段 |
| p.2 ガイド文 | 必須2フォルダの説明文がある | タイトル直下のリード文 |
| p.5 | 「Before / After」のフォルダカードが横並び（または縦に視認可能） | 中央領域 |
| ABDD | 「フォルダを選ぶ」ボタンが OS ネイティブダイアログを開く | クリックして確認 |
| p.5 | 任意設定 (audit.yaml / .aaaiignore) が折りたたまれている | 詳細トグル |
| 折りたたみ展開 | 任意設定が展開されると 2 ピッカーが現れる | トグル操作 |
| p.5 | 「監査を開始」ボタンが中央に大きく配置 | 下部 |
| p.5 | 「最近使ったプロジェクト」リストが存在する | 下部または横 |

**典型的な不合格パターン.** ボタン padding が 44px 未満 / DnD 未対応の旨が
表示されない（RFC 023 で扱うので例外記録で可）/ Recent が登録順のまま
（同上）。

### RFC 016 — i18n Repair (v0.19.0)

**Design anchors.** HANDOVER §4.2, §5.1（rust-i18n v4 挙動分析）.

**Run twice**: `LANG=en_US.UTF-8 ./target/release/aaai-gui` then
`LANG=ja_JP.UTF-8 ./target/release/aaai-gui`. For each locale:

| 期待 | 確認方法 |
|---|---|
| `opening.title` のようなリテラルキーが画面のどこにも出ない | 全画面をスクロール、メニュー / インスペクター / バナー / ボタン / placeholder を確認 |
| en では英語、ja では日本語が表示される | 同じ画面の文字列を見比べる |
| フォルダピッカーの「Pick the Before folder」(en) / 「Before フォルダを選択」(ja) など、サブシステムのダイアログタイトルも国際化済み | フォルダピッカーを開いて確認 |
| 翻訳の抜けがあれば「en.opening.title」のような <locale>.<key> 形式で見える | 出たキーをすべて記録（RFC 018 が必要かの判断材料） |

**重要.** リテラルキーが 1 件でも残れば、`判定` は ❌ にしてキー名を列挙する。
これは即 RFC 018（パターン B または C）に進む合図。

---

## P1 — silent failure 履歴があるもの

### RFC 014 — View Fixes (v0.17.0)

**Design anchors.** RFC 007 / RFC 009 の view 側再適用、p.5 主画面、
ABDD タップ領域 ≥ 44px。

| 確認 | 期待 |
|---|---|
| インスペクターの理由入力が **複数行** textarea になっている | 1 行ではなく、複数行入力 + スクロール可 |
| ツールバーのボタンが **44px 以上** のタップ領域 | hover / click 領域が指でも狙える大きさ |
| すべてのボタンの padding がほぼ同じ（10.0, 14.0 以上） | 視覚的に揃っている |

### RFC 007 — Toolbar & Navigation (v0.15.0)

**Design anchors.** PDF p.5 上部ツールバー、Batch Approve 削除。

| 確認 | 期待 |
|---|---|
| ツールバー左端の並びが **「開く / 保存 / 監査実行 / レポート出力」** | p.5 と一致 |
| Batch Approve ボタンが **無い** | ツールバー上に出ていない |
| 右端に **監査ステータス: OK / Pending / Failed** が表示 | テキスト + 色 |

### RFC 008 — Bottom Action Bar (v0.15.0)

**Design anchors.** PDF p.5 下部、主操作の固定。

| 確認 | 期待 |
|---|---|
| 画面下部に **「承認して保存」** ボタンが固定表示 | 中央寄せ・大きめ |
| 「選択中: <path>」ラベルが左寄せで表示 | 選択した差分のパス |
| 「N 件の差分中 M 件未対応」のような件数表示が右側にある | 数値が更新される |

---

## P2 — 設計書記載項目、視覚仕上げ系

### RFC 009 — Reason Field Textarea (v0.15.0)

| 確認 | 期待 |
|---|---|
| インスペクターの「理由 (必須)」が **複数行 textarea** | 改行可、最低 3 行視認 |
| 空欄では「承認して保存」が disabled | ボタンの状態を切り替えて確認 |
| placeholder に入力例が出る | 例文 or ヒント |

### RFC 010 — Diff View Legend (v0.15.0)

| 確認 | 期待 |
|---|---|
| 差分ビューア下部または右に **凡例** が出る | 「削除」「追加」など色 + テキスト |
| 色だけでなく **文字 + 記号** で識別可 | ABDD 準拠 |

### RFC 011 — Diff View Tabs (v0.16.0)

| 確認 | 期待 |
|---|---|
| 差分ビューア上部に **「左右差分 / 統合 / 変更のみ」** タブ | 3 つ存在 |
| クリックで表示が切り替わる | 3 つすべて挙動確認 |
| 初期表示は「左右差分」 | 起動直後の状態 |

### RFC 012 — LineMatch Rule Color Blocks (v0.16.0)

| 確認 | 期待 |
|---|---|
| インスペクターで監査戦略 **LineMatch** を選ぶと、ルールが **色付きブロック** で表示 | 削除は赤系、追加は緑系 |
| 「+ ルールを追加」ボタンがあり、新規行を追加できる | クリック動作 |

### RFC 013 — File Tree Icon Unification (v0.17.0)

| 確認 | 期待 |
|---|---|
| ファイルツリーの **行頭にステータスアイコン**（✓ / ⚠ / ✗ / —） | 文字 + 色 |
| 右端に **diff-type 記号**（追加 / 削除 / 変更） | 統一感あり |
| 色だけに依存していない | ABDD 準拠 |

---

## P3 — 対象外（必要時のみ）

RFC 001〜006 は出荷済みで顕在化していない（RFC 017 §2.4）。
v1.0.0 リリース判定時に必要があれば別途検証する。

---

## Phase 12 — implementing RFCs (must verify before v0.20.0 cut-over)

RFC 017 自身がこのハーネスを定義した RFC。RFC 018 §3.4 と RFC 024
は CLI / 静的検査で自動化済みなので視覚検証不要。残りの実装 RFC
は新機能の正しい表示を operator が確認する必要がある。

### RFC 020 — ABDD + Action-oriented Errors (v0.20.0)

**Design anchors.** RFC 020 §3.3 「message + hint」2 行パターン、
`docs/src/abdd-audit.md` §1–§6、`docs/src/gui.md` §1 エラーバナー。

**駆動方法.** Opening 画面で意図的に失敗を起こす:
1. 存在しないフォルダパスをピックして「監査を開始」
2. 破損した audit.yaml を指定して開始
3. インスペクターで `[abc` のような不正な regex を入力

| 設計書参照 | 期待 | 確認方法 |
|---|---|---|
| RFC 020 §3.3 | 失敗時に Start ボタン上部にエラーバナーが出る | (1) を実行、バナーの有無を確認 |
| RFC 020 §3.3 | バナーは **2 行構成**（赤いメッセージ + グレーのヒント） | バナー内のテキストとスタイル |
| 「次に何をすればよいか」 | ヒント行が「フォルダを再選択してください」「audit.yaml の構文を確認してください」など、行動可能な文言 | テキスト内容を読む |
| RFC 020 (regex) | インスペクターの Regex pattern 欄に不正な式を入れると、欄直下に「Invalid regex: ...」+ regex101.com への誘導 | (3) を実行、ヒント文言を確認 |
| ABDD §1 | バナーが色だけでなくアイコン/記号でも識別可能 | グレースケールで見ても警告と分かるか |

**典型的な不合格パターン.** バナーが 1 行のみ（hint 行欠落）、文言が
「Error occurred」のような抽象表現、regex101 リンクが出ない。

### RFC 021 — Screen Navigation Continuity (v0.20.0 — partial)

**Design anchors.** RFC 021 §3.1 Save/Report freshness marks,
`docs/src/gui.md` §2 ツールバー。

**駆動方法.** メイン画面まで進めて Save と Export を実行:
1. 適当なエントリを承認して Ctrl+S（保存）
2. 同じ画面で Ctrl+E（レポート出力）
3. そのまま **数分待つ** か、`Message::RelativeTimeTick` の周期 (30 秒)
   を観察

| 設計書参照 | 期待 | 確認方法 |
|---|---|---|
| RFC 021 §3.1 | Save 直後、ツールバー Save ボタン横に `✓ Saved just now` | 保存時に見る |
| 同上 | 30 秒経過後、表示が `✓ Saved 1 min ago` に切り替わる | 待機して観察 |
| 同上 | Export 後、Report ボタン横にも同様のマーク | 同様の検証 |
| RFC 021 §3.4 | Ctrl+R で監査を再実行すると両マークがクリア | リランで消える |

**典型的な不合格パターン.** マークが出ない、相対時刻が更新されない
（subscription が停止している）、リラン後もマークが残る。

**Note.** audit_dirty バナーは Phase 12 で **deferred** された
（synchronous rerun アーキ下では発火しない）。検証不要。

### RFC 022 — Empty States & First-Run UX (v0.20.0)

**Design anchors.** RFC 022 §4 全 4 状態、`docs/src/gui.md` §1
onboarding、§2-2 file_tree, §2-3 diff, §2-4 inspector の空状態。

**駆動方法.** 4 つの空状態をそれぞれ表示させる:
1. **初回起動.** `~/.aaai/profiles.yaml` を一時退避してから起動
2. **監査未実行.** メイン画面に進んだ直後（Run audit を押す前）
3. **ファイル未選択.** インスペクターを見る
4. **差分なし.** 同じ内容の Before/After で監査

| 設計書参照 | 期待 | 確認方法 |
|---|---|---|
| RFC 022 §4.1 | (1) Opening で Recent の代わりに **「はじめての方へ」パネル** | 起動直後の下部 |
| 同上 | ① ② ③ の番号付きステップ説明 + audit.yaml 自動生成の注意書き | テキスト構造 |
| RFC 022 §4.2 | (2) ファイルツリーが「監査結果なし。ツールバーの ▶ から実行してください」 | プレースホルダー文言 |
| RFC 022 §4.3 | (3) インスペクターに「ファイルを選んでください」プレースホルダー | 中央配置 |
| RFC 022 §4.4 | (4) ダッシュボードに「変更なし」専用の状態が出る | 監査実行後 |
| 全般 | 4 つの状態すべてが「次に何をするか」を示している | 行動誘導の文言があるか |

**典型的な不合格パターン.** 空のパネルが何も言わない / 「No data」
だけで次の操作が分からない。

### RFC 023 — Opening DnD + Recent Polish (v0.20.0)

**Design anchors.** RFC 023 §3 DnD, §4 Recent 並び順, `docs/src/gui.md`
§1 ドラッグ&ドロップ + Recent。

**駆動方法.** ファイルマネージャから:
1. フォルダを Opening 画面にドラッグ（ドロップせず保持）
2. フォルダをドロップ → Before カードに入ること
3. もう 1 つフォルダをドロップ → After カードに入ること
4. **ファイル**（フォルダではない）をドロップ
5. プロジェクトを 3 つ保存して、古い順に Open → 並び順を観察

| 設計書参照 | 期待 | 確認方法 |
|---|---|---|
| RFC 023 §3 | (1) ドラッグ中、画面上部に「フォルダをドロップしてください」ヒントバナー | ドラッグ保持中 |
| 同上 | (2) Before カードが空ならそこに入る | ドロップ位置に関わらず |
| 同上 | (3) After カードも同様に埋まる | 2 つ目のドロップ |
| 同上 | (4) ファイルをドロップするとエラーバナー「フォルダのみ受け付けます」 | ドロップ後 |
| RFC 023 §4 | (5) Recent リストが **最終使用日時の降順** | 並び順 |
| 同上 | 各行に「3 min ago」「2 d ago」のような **相対時刻** | テキスト要素 |
| 同上 | 1 週間以上前は ISO 形式（2026-05-08）に切り替わる | 古い entry を 1 つ作って確認 |

**典型的な不合格パターン.** DnD ヒントが出ない / 並びが登録順のまま /
相対時刻が常に「just now」のままで更新されない。

### RFC 024 — CLI Dashboard & Help (verified by automation)

CLI integration tests で `Next steps:` ブロックの存在と
`aaai exit-codes` の表が確認済み（aaai-cli 70/70 green）。
GUI verification 不要。

### RFC 025 — v1.0.0 release prep (docs-only)

`docs/src/compatibility.md` / `docs/ja/src/compatibility.md`
が存在し、mdbook ビルドが clean に通れば OK。GUI verification 不要。

---

## カードの書き方の小例

例として RFC 016 を検証した結果が「合格」だった場合:

```markdown
## Visual Verification

**Verified.** 2026-05-15 by nabbisen
**Build.** v0.20.0-rc.1 (git: 7f8a12c)
**Platform.** Ubuntu 24.04
**Locale.** en + ja

### Checks

| 設計書参照 | 期待 | 実観測 | 判定 |
|---|---|---|---|
| HANDOVER §4.2 | リテラルキー (opening.title 等) が画面に出ない | en/ja 双方で日本語/英語のみ表示 | ✅ |
| 同上 | サブシステムのダイアログも国際化済み | フォルダピッカータイトルが切替 | ✅ |
| HANDOVER §5.1 | `<locale>.<key>` 形式が出ない | 全画面で確認、未検出 | ✅ |

### Notes

- RFC 018 (フォールバックパターン B/C) は不要と判明。
```

例として RFC 015 を検証した結果に **例外** が含まれる場合:

```markdown
| 設計書参照 | 期待 | 実観測 | 判定 |
|---|---|---|---|
| p.5 Recent | 最終使用日時順で並ぶ | 登録順のまま | 例外: RFC 023 でカバー予定 |
```
