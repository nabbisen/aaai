# GUI UI/UX Production-Ready Plan — Rev. 4

## 改訂履歴

| Rev. | 日付 | 内容 |
|---|---|---|
| 1 | — | 初版（Sprint A〜C / RFC 001〜006） |
| 2 | — | 設計書との差分照合追補（Sprint D-1〜D-4 / RFC 007〜014） |
| 3 | 2026-05 | Opening 画面根本問題発覚（Sprint D-5/D-6 / RFC 015〜016） |
| **4** | **本リビジョン** | **v0.19.0 視覚検証未了問題を起点に v0.20.0 以降の中期計画を策定（RFC 017〜025）** |

---

## 1. 背景

v0.15.0 → v0.19.0 にかけて RFC 007〜016 を導入し、設計書（`aaai_uiux_design.pdf`）の
要求要素は **コード上は揃った**。一方、HANDOVER-v0.19.0.md が明確に警告する通り、
以下が未消化となっている：

1. **視覚的検証が一度も行われていない** — 過去 2 回の silent failure
   (RFC 007/009 view 側、v0.18.0 Opening) と同種のリスクが再発する可能性
2. **ドキュメント（docs/src/gui.md および docs/ja/src/gui.md）が陳腐化**している。
   ツールバー記述・「Batch Approve」「Export MD/JSON」「承認して適用」など、
   RFC 007/008/014 で除去された要素が残存
3. **RFC 016 のパターン B/C** が未試行のまま残されている
4. 設計書 p.6 の **Re-run / Save / Report の循環** が GUI 上で視覚的に意識されにくい
5. 設計書 p.8 ABDD チェックリストの **「エラー文の行動可能性」「色以外の状態表現の全数監査」**
   が体系的に未実施
6. 設計書 p.10 バックログ A 末尾「**最近の組み合わせ再利用**」の利便性面（並び順・DnD）が未完
7. 設計書 p.4 CLI の **「結論先出し」のリンタービュー** （`aaai dashboard` / `--help`）の整備余地

本 Rev. 4 は、これらを v1.0.0 までに段階的に解消する計画である。

---

## 2. 設計書バックログ（p.10）との対応 — 現状の達成度

| バックログ | 主な担当 RFC | 達成度（コード） | 達成度（視覚検証） |
|---|---|---|---|
| A. 初期画面 | 004, 015 | ✅ 完了 | ❌ 未確認 |
| B. メイン画面（3 ペイン責務分離） | 003, 005, 007, 008, 011, 013, 014 | ✅ 完了 | ❌ 未確認 |
| C. インスペクター（reason 必須・戦略別フォーム・リアルタイム検証） | 002, 009, 012, 014 | ✅ 完了 | ❌ 未確認 |
| D. CLI 出力（結論先出し・安定終了コード） | 001 | ✅ 完了 | ⚠ 部分検証 |
| E. レポート | 006 | ✅ 完了 | ⚠ 部分検証 |
| F. アクセシビリティ | 003, 005, 014 | 🟡 タップ領域・キーボードは ✅、エラー文・スクリーンリーダー対応は ❌ | ❌ 未確認 |

**結論**: コード差分は埋まったが、品質ゲート (視覚検証 / a11y 監査 / 行動可能エラー文) が未通過。

---

## 3. 中期計画 — Phase 12 〜 v1.0.0

### 3.1 全体像

```
[v0.19.0 現在] ──[Phase 12]──> [v0.20.0]
                  視覚検証 / i18n 仕上げ / docs 刷新

  ──[Phase 13]──> [v0.21.0]
                  ABDD 監査 + 行動可能エラー文

  ──[Phase 14]──> [v0.22.0]
                  画面リレーション可視化 + 空状態誘導

  ──[Phase 15]──> [v0.23.0]
                  Opening DnD + Recent 並び順 + CLI 仕上げ

  ──[Phase 16]──> [v1.0.0-rc → v1.0.0]
                  最終検証 + リリース準備
```

### 3.2 Phase 別の小計画

#### Phase 12 — v0.19.0 の答え合わせ ── 視覚検証 / i18n / docs 刷新 (v0.20.0)

**目的**: 「書いたコードが設計書通り動く」を保証してから先に進む。

| RFC | タイトル | 種類 | 工数感 |
|---|---|---|---|
| [017](proposed/017-visual-verification-harness.md) | Visual Verification Harness & Protocol | プロセス＋実装 | 中 |
| [018](proposed/018-i18n-fallback-strategies.md) | i18n Locale Fallback Strategies (B/C) | 実装（必要時） | 小 |
| [019](proposed/019-documentation-refresh.md) | Documentation Refresh for v0.15–v0.19 Realities | ドキュメント | 中 |

成功条件:
- すべての v0.15〜v0.19 RFC について、スクリーンショットつきで「設計書 p.X 一致」を表明できる
- `docs/src/gui.md` および `docs/ja/src/gui.md` が現実と一致する
- i18n の最終確定パターン (A/B/C) と運用手順が記録される

#### Phase 13 — ABDD 監査と行動可能エラー文 (v0.21.0)

**目的**: 設計書 p.8 のチェックリストを **網羅的かつ機械的に** 通過させる。

| RFC | タイトル | 種類 | 工数感 |
|---|---|---|---|
| [020](proposed/020-abdd-audit-and-error-messages.md) | ABDD Accessibility Audit & Action-oriented Errors | 監査＋実装 | 中 |

成功条件:
- ABDD チェックリスト全項目に対し「合格 / 例外 (記録)」のいずれかが付与される
- `failed` / `invalid` / `error` といった一語型エラー文がコードから削除される
- 「何が」「どこで」「どうすればよいか」が含まれない人間向けエラー文が残らない

#### Phase 14 — 画面リレーション可視化と空状態誘導 (v0.22.0)

**目的**: 設計書 p.6 の循環フロー (Opening → Audit → Review → Save/Report → Re-run) を
ユーザーが画面上で「今どこにいるか」「次にどこへ行くか」を把握できるようにする。

| RFC | タイトル | 種類 | 工数感 |
|---|---|---|---|
| [021](proposed/021-screen-navigation-continuity.md) | Screen Navigation Continuity & Re-run Visibility | 実装 | 中 |
| [022](proposed/022-empty-states-and-first-run.md) | Empty State Guidance & First-Run UX | 実装 | 小 |

成功条件:
- 「監査未実行 (audit_result 不在)」状態で、ユーザーが次にすべきことが画面上に明示される
- Re-run が「ボタンを押せ」ではなく「再監査の意義」として伝わる
- 未保存／監査再実行待ちのバナーが永続的に画面に残る

#### Phase 15 — Opening DnD + Recent 並び順 + CLI 仕上げ (v0.23.0)

**目的**: 設計書 p.10 バックログ A／D の積み残し品質を埋める。

| RFC | タイトル | 種類 | 工数感 |
|---|---|---|---|
| [023](proposed/023-opening-dnd-and-recent.md) | Opening Drag-and-Drop & Recent Projects Polish | 実装 | 中 |
| [024](proposed/024-cli-dashboard-and-help.md) | CLI Dashboard & Help Discoverability Polish | 実装 | 小 |

成功条件:
- フォルダのドラッグ＆ドロップで Opening のフォルダカードに即時反映
- 「最近使った」一覧が最終使用日時で並び替えされる
- `aaai dashboard` の表示が設計書 p.4 のスタットカード構造と一致
- 主要サブコマンドの `--help` 末尾に「次の操作」ヒントが添えられる

#### Phase 16 — v1.0.0-rc / v1.0.0 (リリース準備)

**目的**: v1.0.0 として外部公開できる品質ゲートを通過させる。

| RFC | タイトル | 種類 | 工数感 |
|---|---|---|---|
| [025](proposed/025-v1-0-release-prep.md) | v1.0.0 Release Preparation | プロセス＋ドキュメント | 中 |

成功条件:
- ROADMAP.md の v1.0.0 セクションが具体化される
- すべての提案中 RFC が done または archive に移動済み
- `docs/src/testing.md` の受け入れ基準が現バージョンで通過
- CHANGELOG が `v1.0.0` エントリを持つ

---

## 4. 実装スプリント計画

| Sprint | RFC | 内容 | 予想バージョン |
|---|---|---|---|
| E-1 | 017 | 視覚検証プロトコルの確立と v0.19.0 全項目の検証実行 | v0.20.0 |
| E-2 | 018 | (条件付き) i18n パターン B/C 実装 | v0.20.0 |
| E-3 | 019 | docs/src/gui.md・docs/ja/src/gui.md・README の現実同期 | v0.20.0 |
| F-1 | 020 | ABDD 監査 + エラー文一斉書き換え | v0.21.0 |
| G-1 | 021 | 画面リレーション可視化 (バナー / 未保存表示 / Re-run cue) | v0.22.0 |
| G-2 | 022 | 空状態誘導 (audit_result 不在時) | v0.22.0 |
| H-1 | 023 | Opening DnD + Recent 並び順 | v0.23.0 |
| H-2 | 024 | CLI dashboard + --help ヒント | v0.23.0 |
| I-1 | 025 | v1.0.0-rc → v1.0.0 リリース準備 | v1.0.0 |

---

## 5. 依存関係

```
RFC 017 (verification) ─┬─> RFC 018 (i18n fallback if needed)
                       └─> RFC 019 (docs refresh)
                              │
                              ▼
                       RFC 020 (ABDD audit & error messages)
                              │
                              ▼
                       RFC 021 (screen navigation continuity)
                       RFC 022 (empty states)
                              │
                              ▼
                       RFC 023 (opening DnD & recent polish)
                       RFC 024 (CLI dashboard & help)
                              │
                              ▼
                       RFC 025 (v1.0.0 release prep)
```

RFC 017 は前提。RFC 018/019 は並行可能。RFC 020 以降は前段の完了を待つ。

---

## 6. 設計書原則の再確認 (本計画の評価尺度)

中期計画の各 RFC は、以下の設計書原則のいずれかに紐づくこと:

1. **p.2 中心体験** — 「選ぶ → 見る → 理由を書く → 承認する → 保存する → 確認する」
2. **p.2 三原則** — 監査は人間の判断を残す / YAML は成果物 / 状態は常に見える
3. **p.5 ペイン責務分離** — 一つの場所で一つのこと
4. **p.6 画面リレーション** — 分岐を少なく、戻れる構造
5. **p.7 インスペクター** — 承認前に判断材料を揃える
6. **p.8 ABDD** — 視認性 / キーボード / スクリーンリーダー / エラー文
7. **p.9 状態モデル** — CLI と GUI で同じ語彙

---

## 7. 過去 RFC との関係

本 Rev. 4 で追加される RFC 017〜025 は、RFC 001〜016 を **置換するものではない**。
それらの成果物の上に、品質保証層 (Phase 12) ・整備層 (Phase 13〜15) ・確定層 (Phase 16) を
段階的に重ねるものである。

RFC 014 のような「過去 RFC の実装が想定通り反映されているかを再確認する」差分実装が
必要になった場合は、本計画とは別個に挿入する (RFC 番号は連続採番)。
