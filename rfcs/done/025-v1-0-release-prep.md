# RFC 025 — v1.0.0 Release Preparation

**Status.** Implemented partial (v0.20.0) — docs groundwork only (compatibility.md en/ja); full release-prep at v1.0.0
**Priority.** v1.0 ゲート
**Tracks.** PLAN.md Rev. 4 Phase 16 / 設計書全体（p.1 全体像 〜 p.10 バックログ）
**Touches.** `ROADMAP.md` · `CHANGELOG.md` · `README.md` · `docs/src/{intro,testing,gui}.md` · `docs/ja/src/` 全般 · `rfcs/README.md` · `rfcs/proposed/` → `rfcs/done/` への移動 · `Cargo.toml` (workspace.package.version) · GitHub Release タグ

---

## 1. 要件定義

### 1.1 目的

v0.19.0 から v1.0.0 へと至る道筋を **「設計書 (`aaai_uiux_design.pdf`) を満たし、視覚検証
を通過し、ABDD 監査を通過した v0.x 系列の総決算」** として明示する。本 RFC は
新規機能を提案するものではなく、**リリース判定基準とリリース作業手順** を確定する
プロセス RFC である。

### 1.2 解決すべき問題

| 問題 | 影響 |
|---|---|
| v1.0.0 の達成条件が ROADMAP.md に書かれていない | リリース判定が属人化 |
| `proposed/` の RFC が done に移される基準が個別 RFC ごとにあいまい | 完了管理が一貫しない |
| `docs/src/testing.md` の受け入れ基準が v0.19.0 時点で全項目通過しているか未確認 | 「テストは通っている」だけでは v1.0 を名乗れない |
| CHANGELOG に v1.0.0 のリリースノート構造が用意されていない | リリース直前にフォーマット議論が発生 |
| v1.0.0 公開後の API/CLI 互換性方針 (SemVer 解釈) が宣言されていない | 後方互換破壊の判断が場当たり的になる |

### 1.3 機能要件

| ID | 要件 | 必須/任意 |
|---|---|---|
| FR-1 | `ROADMAP.md` に「v1.0.0 リリース判定ゲート」セクションを追加し、満たすべき条件を列挙 | 必須 |
| FR-2 | `proposed/017–024` がすべて Implemented となり、`done/` に移動済みであること | 必須 |
| FR-3 | `docs/src/testing.md` の受け入れ基準を v1.0.0-rc に対して実行し、結果を `verification/v1.0.0-rc/` に記録 | 必須 |
| FR-4 | `CHANGELOG.md` に `## [1.0.0] — YYYY-MM-DD` セクションを v0.19.0 〜 v0.23.0 の差分の総まとめとして作成 | 必須 |
| FR-5 | `README.md` の「Status / 開発ステータス」表記を `pre-1.0 (active development)` から `1.0.0 (stable)` に更新 | 必須 |
| FR-6 | v1.0.0 以降の SemVer 解釈（CLI 引数 / 終了コード / 設定ファイル / RFC 化されたキーボードショートカット の互換性方針）を `docs/src/compatibility.md` に明文化 | 必須 |
| FR-7 | v1.0.0-rc.1 タグを切り、最低 1 週間のソークテスト期間（ユーザー検証フィードバックの受付）を設ける | 必須 |
| FR-8 | v1.0.0 公開後、`rfcs/README.md` の冒頭に「これより以降の RFC は v1.x の運用 RFC」セクションを追加 | 任意 |

### 1.4 非機能要件

| ID | 要件 |
|---|---|
| NFR-1 | 本 RFC 自体は新規コードを書かない（既存ドキュメント / メタファイル更新のみ） |
| NFR-2 | リリース判定の各ゲートに対し「合格 / 不合格 / 例外 (理由付き)」が記録される |
| NFR-3 | v1.0.0 のリリース判定後、次の minor / major リリース方針は別 RFC（v1.x 系）で議論する |

---

## 2. v1.0.0 リリース判定ゲート

ROADMAP.md に追記する判定基準。各ゲートが「合格」となるまで v1.0.0 タグは切らない。

### 2.1 機能ゲート

| ゲート | 条件 | 関連 RFC |
|---|---|---|
| G1. 視覚検証 | 設計書 p.1 〜 p.10 の各要素について、`verification/` にスクリーンショット + 設計書ページ番号紐付けが存在 | 017 |
| G2. i18n | en / ja 双方で literal key (`title.opening` のような未翻訳表示) が一切表示されない | 016, 018 |
| G3. ABDD 監査 | `docs/src/abdd-audit.md` の全項目が「合格」または「v1.0 例外 (記録)」 | 020 |
| G4. エラー文 | `failed` / `invalid` / `error` 単独表示の error key が 0 件 | 020 |
| G5. 画面リレーション | Audit 定義変更後の再実行誘導バナーと、Save / Report の完了マークが想定通り動作する | 021 |
| G6. 空状態 | Opening / file_tree / diff_panel / inspector のすべてに有意な空状態テキストが表示される | 022 |
| G7. Opening | DnD 受け入れ + Recent 並び順（last_used_at 降順） + relative time 表示が動作 | 023 |
| G8. CLI | 全サブコマンドの `--help` に Next-action hint があり、`aaai dashboard` が設計書 p.4 風の出力を返す | 024 |

### 2.2 品質ゲート

| ゲート | 条件 |
|---|---|
| Q1. テスト | `cargo test --workspace` がすべて通過。core / cli の最低テスト数を CHANGELOG に記録 |
| Q2. Clippy | `cargo clippy --workspace -- -D warnings` が通過 |
| Q3. Fmt | `cargo fmt --check` が通過 |
| Q4. ドキュメント | `mdbook build docs/` と `mdbook build docs/ja/` がエラーなく完了 |
| Q5. ビルド | `cargo build --workspace --release` が Linux / macOS / Windows でクリーンに通過（CI 確認） |
| Q6. パッケージング | release tarball が GitHub Release artifact として添付される |

### 2.3 ドキュメントゲート

| ゲート | 条件 |
|---|---|
| D1. README.md | スクリーンショット + GUI / CLI 双方の Quick Start が現バージョンで動く |
| D2. ROADMAP.md | v1.0.0 セクションが「達成済み」として固定され、v1.1+ の方向性が別セクションに分離 |
| D3. CHANGELOG.md | v1.0.0 セクションが v0.19.0 → v1.0.0 の累積差分を網羅 |
| D4. compatibility.md | 互換性方針が読みやすく説明されている（後述 §4） |

---

## 3. リリース作業手順

### 3.1 v1.0.0-rc.1 への遷移

```
1. proposed/017–024 を done/ に移動（各 RFC の Status を Implemented に更新）
2. Cargo.toml workspace.package.version = "1.0.0-rc.1" にバンプ
3. CHANGELOG.md に [1.0.0-rc.1] エントリを追加
4. git tag v1.0.0-rc.1 → push
5. GitHub Release (pre-release) として artifact を添付
6. verification/v1.0.0-rc.1/ ディレクトリに G1〜G8 のスクリーンショット / ログを記録
```

### 3.2 ソーク期間

- 最低 7 日。重大な不具合報告があれば v1.0.0-rc.2 を切る。
- ソーク期間中の修正は **既存 RFC の範囲内** に限定する。新規 RFC（016 以降の系列）は v1.0.0 後の v1.1.0 で扱う。

### 3.3 v1.0.0 への昇格

```
1. verification/v1.0.0/ で全ゲート (G1〜G8, Q1〜Q6, D1〜D4) を再確認
2. Cargo.toml version = "1.0.0"
3. CHANGELOG.md の [1.0.0-rc.1] を [1.0.0] にプロモート、リリース日確定
4. README.md の status バッジ更新
5. git tag v1.0.0 → push
6. GitHub Release (latest) として公開
7. rfcs/README.md に「v1.x 系の運用 RFC」セクション準備（FR-8）
```

---

## 4. v1.0.0 以降の互換性方針（`docs/src/compatibility.md`）

v1.0.0 で固定する互換性契約を以下に列挙する。これらは v1.x の間維持される。

### 4.1 CLI

| 対象 | 互換性 | 例 |
|---|---|---|
| サブコマンド名 | 維持 | `aaai audit`, `aaai snap`, `aaai report` 等 |
| 主要オプション (long form) | 維持 | `--from`, `--to`, `--strategy`, `--reason` |
| short option | best-effort（重複時は long form を正とする） | `-f`, `-t` |
| 終了コード値 | 維持 | 0/1/2/3/4 (RFC 001 で確定済み) |
| ヘルプ文言 | 変更可（要 CHANGELOG 記載） | wording 修正、`after_help` の追加 |

### 4.2 設定ファイル

| 対象 | 互換性 | 備考 |
|---|---|---|
| `~/.aaai/profiles.yaml` のキー | 後方互換維持（追加は可、削除/改名は major） | `AuditProfile` |
| `~/.aaai/prefs.yaml` のキー | 同上 | `UserPrefs` |
| `~/.aaai/history.jsonl` の行構造 | 追加フィールドは可、削除/改名は major | reader 側は unknown field を無視する |

### 4.3 GUI

| 対象 | 互換性 |
|---|---|
| キーボードショートカット (Ctrl+R / Ctrl+S 等) | RFC 化されたものは維持 |
| ペイン構造 (3 ペイン / 上下 2 段) | 維持 |
| i18n キー (`error.*` / `banner.*` / `empty_state.*` / `relative.*` ほか) | 削除/改名は major |
| テーマ (Light/Dark) | 維持 |

### 4.4 ライブラリ API

`aaai-core` を外部から `crate` として依存することは現時点で想定していない。
そのため v1.x の間、`aaai-core` の public API 互換は **best-effort** に留め、
変更時は CHANGELOG に Breaking として明記する（major bump は要求しない）。
将来 `aaai-core` を独立 crate として publish する判断が出た時点で別 RFC で扱う。

### 4.5 互換性を破る変更が必要になった場合

- 新しい挙動を **opt-in フラグ** または **新サブコマンド** で導入する（v1.x 中）
- 旧挙動の deprecation 警告を最低 1 minor バージョン出す
- 削除は v2.0.0 まで待つ

---

## 5. ROADMAP.md への追記イメージ

```markdown
## v1.0.0 — UI/UX 設計書完全実装 + 視覚検証通過

設計書 `aaai_uiux_design.pdf` の全要素を実装し、視覚検証 / ABDD 監査 /
行動可能エラー文 / 画面リレーション継続性 / 空状態誘導 を通過した
最初の安定版リリース。

### 達成条件 (RFC 025 §2 を参照)

- 機能ゲート G1〜G8: ✅
- 品質ゲート Q1〜Q6: ✅
- ドキュメントゲート D1〜D4: ✅

### 互換性宣言

v1.0.0 から v1.x の間、`docs/src/compatibility.md` に列挙した
契約を維持する。

## v1.1.0 以降

(別 RFC で計画。v1.0.0 ソーク後に着手)
```

---

## 6. CHANGELOG.md への追記イメージ

```markdown
## [1.0.0] — YYYY-MM-DD

### Highlights

- **v0.x 系の総決算リリース**。設計書 `aaai_uiux_design.pdf` の全要素を
  実装し、視覚検証 / ABDD 監査 / 行動可能エラー文 を通過。
- 互換性契約を v1.x 系として正式に宣言（`docs/compatibility.md`）。

### Added (v0.19.0 → v1.0.0 累積)

- Visual verification harness (RFC 017)
- i18n fallback strategy documentation (RFC 018)
- ABDD audit sheet + action-oriented error messages (RFC 020)
- Audit dirty banner + relative timestamp for Save/Report marks (RFC 021)
- Empty-state guidance for Opening / file_tree / diff_panel / inspector (RFC 022)
- Drag-and-drop folder selection + Recent profiles sorted by last_used_at (RFC 023)
- `--help` Next-action hints for every subcommand; `aaai dashboard` redesign (RFC 024)

### Changed

- (列挙)

### Fixed

- (列挙)

### Documentation

- `docs/src/gui.md` / `docs/ja/src/gui.md` を v0.15〜v0.19 + 視覚検証結果に合わせて書き直し (RFC 019)
- `docs/src/compatibility.md` を新設 (RFC 025 §4)
```

---

## 7. 受け入れ基準

| 項目 | 基準 |
|---|---|
| AC-1 | ROADMAP.md に「v1.0.0 リリース判定ゲート」セクションが存在 |
| AC-2 | `docs/src/compatibility.md` が存在し、§4 の方針を網羅 |
| AC-3 | RFC 017〜024 がすべて `done/` 配下にあり、Status: Implemented |
| AC-4 | `verification/v1.0.0/` に G1〜G8 のエビデンスが揃っている |
| AC-5 | `cargo test --workspace` / `cargo clippy --workspace -- -D warnings` / `cargo fmt --check` がすべて通過 |
| AC-6 | `mdbook build docs/` と `mdbook build docs/ja/` がエラーなしで完了 |
| AC-7 | git tag `v1.0.0` が push され、GitHub Release が "latest" 扱いになっている |

---

## 8. 例外と除外事項

- **スクリーンリーダー対応**: iced 0.14 の制約により v1.0 では実現しない（RFC 020 §で明記）。
  v1.x で iced 側に accessibility 機構が入った時点で別 RFC として扱う。
- **国際化の追加言語**: en / ja のみ。zh / ko などは v1.x 以降。
- **macOS / Windows ビルドの公式配布物**: v1.0.0 では Linux x86_64 を主、macOS / Windows は best-effort。CI で build がグリーンであれば artifact 添付に含めるが、署名 / Notarization は v1.x 以降。

---

## 9. 関連 RFC

- RFC 000 — RFC lifecycle policy
- RFC 017 — Visual Verification Harness & Protocol（G1 の根拠）
- RFC 018 — i18n Locale Fallback Strategies (B/C)（G2 の根拠）
- RFC 019 — Documentation Refresh for v0.15–v0.19 Realities（D1, D3 の根拠）
- RFC 020 — ABDD Accessibility Audit & Action-oriented Errors（G3, G4 の根拠）
- RFC 021 — Screen Navigation Continuity（G5 の根拠）
- RFC 022 — Empty States and First-run Guidance（G6 の根拠）
- RFC 023 — Opening Drag-and-Drop and Recent Polish（G7 の根拠）
- RFC 024 — CLI Dashboard & Help Discoverability Polish（G8 の根拠）

## 10. 未解決事項

- v1.1.0 で扱う最初の機能を何にするか（候補: スクリーンリーダー検討 / 追加言語 / `aaai-core` の独立 crate 化）は v1.0.0 ソーク中に別 RFC で議論。
- `verification/` を git 管理下に置くか gitignore に留めるかは RFC 017 § で議論済み（gitignore 推奨、別途共有用に `verification-summary.md` を残す）。
