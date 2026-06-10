# Changelog

All notable changes to this project are documented in this file.

Format: `## [version] — description`

## [Unreleased]

(no entries yet)

## [0.20.0] — 2026-05-13 — UI/UX refresh, accessibility, doc hygiene

詳細は [rfcs/PLAN.md](rfcs/PLAN.md) Rev. 4 および [ROADMAP.md](ROADMAP.md) Phase 12〜16 を参照。

### Infrastructure (RFC 017 — Visual Verification Harness)

Phase 12 の足場として、視覚検証プロトコルの基盤が landed:

- `scripts/list-unverified-rfcs.sh` — `rfcs/done/` 配下で `## Visual Verification` セクションが未記入の RFC を列挙。`--strict` で exit 1、`--quiet` で要約のみ。shellcheck clean。
- `docs/templates/visual-verification-template.md` — 検証カードのコピー用テンプレート（Verified 日付 / Build / Platform / Locale / Checks 表 / Notes）。
- `.gitignore` に `/rfcs/verification/` を追加（スクリーンショット等は commit せず、カード本文のみ RFC 内に残す）。
- `.github/workflows/ci.yaml` に informational な `visual-verification-status` job を追加（fail させない、件数をログに出す）。
- `docs/src/testing.md` / `docs/ja/src/testing.md` 末尾に §10「Visual Verification (RFC 017)」を追加し、受け入れ基準と視覚検証カードの関係を明文化。

**現時点の未検証 RFC 件数: 16 / 17**（RFC 000 lifecycle policy を除く）。
P0 (RFC 015, 016) は v0.20.0 リリース前に必ず記入する。実機で GUI を起動できる
オペレーターが順次カードを埋めていく運用に切り替わる。

### Hygiene tooling (RFC 018 §3.4 — i18n key audit)

RFC 018 主部 (パターン B/C) は条件付きだが、§3.4 のキー監査スクリプトは
無条件に有用なため先行 landed:

- `scripts/check-i18n-keys.py` — Pure Python（macOS / Linux / Windows 完全互換）。`crates/aaai-gui/src/` 内の `t!()` 呼び出しキーを `locales/en.yaml` / `ja.yaml` と相互照合し、MISSING / DIVERGENT / UNUSED / SKIPPED の 4 カテゴリで報告。pyflakes clean。
- `.github/workflows/ci.yaml` に **blocking** な `i18n-key-audit` job を追加（RFC 017 の informational 検証とは別位置づけ）。
- `docs/src/testing.md` / `docs/ja/src/testing.md` に §11 を追加。

**初回実行で v0.19.0 のバグ 3 件を surface:**

| キー | 呼び出し箇所 | 状況 |
|---|---|---|
| `toolbar.passed` | `crates/aaai-gui/src/views/main_view.rs:79` | en/ja 双方の YAML から欠落 → GUI で literal `"toolbar.passed"` が表示される（RFC 016 と同パターンの追加事例） |
| `toolbar.failed` | `crates/aaai-gui/src/views/main_view.rs:81` | 同上 |
| `toolbar.undo` | `crates/aaai-gui/src/views/main_view.rs:129` | 同上 |

これらは v0.20.0 で wording を確定して両 YAML に追加する。RFC 017 の視覚検証で
トリアージされる前に静的検出できたのは RFC 018 §3.4 の正味効果。

**UNUSED 54 件**（en + ja 各 27 件）は RFC 019 が「docs と現実の乖離」として
記録した古い UI 要素と一致:

- `toolbar.batch_approve`（RFC 007 で削除）
- `toolbar.export_md` / `toolbar.export_json`（RFC 006 で単一の Report Output に統合）
- `opening.{before,after}_{label,placeholder,required}` 等（RFC 015 で text input → folder picker 化）
- `profile.*` / `filter.*` / `inspector.{title,approve_button,reason_placeholder}`（その他リネーム/廃止）

これらは RFC 019 のドキュメント刷新と同期して個別に削除する。

### CLI polish (RFC 024 — Dashboard & Help Discoverability)

CLI 表示の約束 (設計書 p.4) のうち「迷わないヘルプ」と「結論先出し」を全
サブコマンドに展開:

- **`crates/aaai-cli/src/cmd/next_hint.rs`** (新規) — `next_action_hint(&AuditSummary) -> Option<String>` を切り出し、`audit` の Zone 4 と `dashboard` で共有。Pending / Failed / Error / All-clean の 4 ブランチで一意の文言を返す純粋関数。単体テスト 8 件。
- **`crates/aaai-cli/src/cmd/exit_codes.rs`** (新規) — `aaai exit-codes` サブコマンドが正準な終了コード表 (0=PASSED / 1=FAILED / 2=PENDING / 3=ERROR / 4=CONFIG_ERROR) を出力。
- **`crates/aaai-cli/src/cmd/audit.rs`** — Zone 4 を新ヘルパーへリファクタ (FR-1 関連)。挙動同一。
- **`crates/aaai-cli/src/cmd/dashboard.rs`** — 出力末尾に Next-action hint を追加 (FR-1)。
- **`crates/aaai-cli/src/main.rs`** — トップレベル `aaai --help` に "Getting started:" ブロック (FR-3)。
- **全 15 サブコマンド** (`audit`, `snap`, `report`, `check`, `lint`, `diff`, `merge`, `init`, `watch`, `history`, `config`, `dashboard`, `completions`, `export`, `version`) に clap `after_help` を追加 (FR-2)。サブコマンドごとに「次の操作」のヒントを 2〜5 行で明示。
- **integration test 8 件** 追加: 各 `--help` に "Next steps:" が含まれること、`audit --help` が終了コード一覧を含むこと、`exit-codes` が正準表を出すこと、`--quiet` と `--json-output` で Zone 4 hint が抑制されること (NFR-1, NFR-2)、dashboard が hint を出すこと。

`aaai-cli` テスト数: 54 → **70 件**（unit 8 件、integration 8 件追加）。
全件 green、警告ゼロ、`cargo check --all-targets` 通過 (Rust 1.91)。

### Accessibility & error-message work (RFC 020 — partial landing)

設計書 p.8 ABDD チェック観点のうち、display を必要としない部分を先行 landed:

- **`docs/src/abdd-audit.md`** / **`docs/ja/src/abdd-audit.md`** (新規) — リリースごとに記入する ABDD チェックシート。6 セクション (Tab 順序 / 色なし判別 / 主操作と破壊的操作の距離 / 状態可視性 / 平易さ / クリック領域) + v1.0 制限事項としてスクリーンリーダーを明示。
- **`docs/src/testing.md`** / **`docs/ja/src/testing.md`** に §12「ABDD verification」を追加。
- **`crates/aaai-gui/src/error.rs`** (新規) — `pub struct UserError { message, hint }`。RFC 020 §3.3 の 2 行表示パターンの canonical キャリア。
- **`crates/aaai-gui/src/app.rs`** — `open_error: Option<String>` → `Option<UserError>` に型変更。4 サイトの `format!()` を `t!()` 経由の i18n 化:
  - StartAudit before-folder 不在 → `error.opening.before_not_found.{message,hint}`
  - StartAudit after-folder 不在 → `error.opening.after_not_found.{message,hint}`
  - 監査定義ロード失敗 → `error.opening.definition_load_failed.{message,hint}`
  - DiffFailed → `error.diff.failed.{message,hint}`
- **`crates/aaai-gui/src/views/opening.rs`** — `app.open_error` をレンダリングする 2 行バナー（message=赤、hint=ややグレー）を Start ボタン上に追加。**これにより v0.19.0 までの silent-failure バグが解消** — 「監査を開始」をクリックして無効パスが検出されても、これまでは画面に何も表示されなかった。
- **`crates/aaai-gui/src/app.rs`** — インスペクター正規表現エラーのインライン wording 改善 (`"Invalid regex: <e>. Tip: simplify the pattern or test it at regex101.com."`)。FieldError 構造体に `hint` を追加する大きい refactor は v1.1 へ。
- **i18n YAML** — `error:` namespace 8 leaf 追加 (4 contexts × `message` + `hint` × 2 locales)、`save.*` と `inspector.*` の aspirational 案は撤去 (toast / FieldError が hint フィールドをサポートするまで保留)。
- **`scripts/check-i18n-keys.py`** — 複数行に分かれた `t!(\n  "key.path",\n  arg = value,\n)` 形式を検出できるよう、全ファイルスキャン + line-comment ストリッピングに改修。pyflakes clean。

**RFC 020 §5 完了条件の進捗:**

| 完了条件 | 状況 |
|---|---|
| `docs/src/abdd-audit.md` 全 6 項目判定 | テンプレート完成、実機での記入は operator 待ち |
| 一語型エラー文 0 件 (人間向け文脈) | 主要 4 サイト書き換え済み；FieldError 一語型は v1.1 |
| 「何が・どこで・次にどうする」3 要素 | UserError 構造体で強制 + 4 主要サイトで実体験可能 |
| `docs/src/testing.md` ABDD 章 | §12 として追加済み |
| `scripts/check-i18n-keys.py` 新 error.* キーで exit 0 | 新 8 keys は USED、structure は preserved |
| RFC 017 Visual Verification で裏付け | operator 待ち |

**display 必要な残作業（operator 担当）:**
- ABDD シートの実機記入（Tab 順序、色なし判別、クリック領域測定、主操作 vs 破壊的操作の距離）
- 新エラーバナーが想定通り表示されることの視覚確認（無効 Before パスで「監査を開始」をクリック）

### Opening DnD + Recent polish (RFC 023 — partial landing)

設計書 p.10 A 「最近の組み合わせ再利用」と RFC 015 §1.3 FR-8（DnD 留保）の積み残しを解消:

**Data layer（フル単体テスト済）:**

- **`crates/aaai-core/src/profile/store.rs`** — `AuditProfile.last_used_at: Option<DateTime<Utc>>` フィールドを `#[serde(default, skip_serializing_if = "Option::is_none")]` で追加。レガシー `~/.aaai/profiles.yaml` は壊れずに読める (NFR-2/3 達成)。
- **`ProfileStore::touch(name) -> Result<bool>`** — 名前で検索して `last_used_at = Utc::now()` を立てて即時 save。
- **`ProfileStore::sorted_by_recent() -> Vec<&AuditProfile>`** — `Option` の自然順により `None` (レガシー) は最古として末尾へ。
- **新規 5 件のユニットテスト**（aaai-core 合計 92 → **97**）: `last_used_at_defaults_to_none`, `legacy_yaml_without_last_used_at_deserialises`, `sorted_by_recent_orders_most_recent_first`, `sorted_by_recent_pushes_none_to_end`, `touch_marks_profile_when_found`。

**Relative-time formatter（pure function、unit-testable）:**

- **`crates/aaai-gui/src/util.rs`** (新規) — `humanize_since(t)` と内部の `humanize_since_at(t, now)`。バケット境界: `<60s → just_now`, `<60min → N min ago`, `<24h → N h ago`, `<7d → N d ago`, それ以降は ISO 形式の絶対日付。7 件のユニットテスト（バケット遷移 + future-timestamp panic-safety）を埋め込み。
- **i18n キー** `relative.{just_now,minutes_ago,hours_ago,days_ago}` を en/ja 双方に追加。

**Opening view 統合:**

- `recent_projects_section` がプロファイルを `last_used_at` desc でソート表示（FR-5）。`LoadProfile(usize)` の wiring は維持（元のインデックスを保持してメッセージを発火）。
- 各プロファイル行の右上に humanize_since 結果（例「3 d ago」）を表示。レガシープロファイルでは省略。
- **`LoadProfile` ハンドラが `ProfileStore::touch` を呼ぶ**ようになり、開いたプロファイルは次回起動時に上位に並ぶ（FR-6）。

**DnD wiring（display-touching、コード commit 済・visual verification 待ち）:**

- **新規 Message variants**: `FileHoverEnter` / `FileHoverLeave` / `FileDropped(PathBuf)`。
- **新規 App field**: `file_hovering: bool`。
- **`dnd_sub()` 関数** — iced `Event::Window::{FileHovered, FilesHoveredLeft, FileDropped}` をリッスンする subscription source。既存の toast_sub + kb_sub と合成。
- **FileDropped ハンドラ** — Opening 画面のときのみ作動。フォルダなら最初に空のカード（Before → After の順）に格納、ファイルなら `open_error` で `error.opening.drop_invalid_kind.{message,hint}` をユーザーに提示（RFC 020 の 2 行パターンを再利用）。
- **opening.rs に drop-hint バナー** — `app.file_hovering` が true の間、画面上に「↓ Drop a folder anywhere on this screen」相当のヒントを表示（FR-2 必須要件）。
- **i18n キー**: `opening.drop_here` と `error.opening.drop_invalid_kind.{message,hint}` を en/ja 双方に追加。

**設計上の判断:**

- 「ドロップ先カードの判定」をレイアウト座標から hit-test するのは iced 0.14 の API では困難なため、シンプルな「最初に空のカードに割り当てる」ルールで初期実装。RFC 023 §3.1 が許容する簡易実装。
- `LoadProfile(usize)` のメッセージシグネチャは変えず、ソート後も元の Vec インデックスをペアで保持して発火することで wiring の変更を最小化。

**display 必要な残作業（operator 担当）:**
- フォルダ DnD で Before / After カードが実際に充填されることの視覚確認
- ファイル DnD でエラーバナーが表示されること
- Recent 一覧の並び順が last_used_at desc になっていること
- humanize_since の表示が読みやすいこと（en/ja 双方）

### Empty-state guidance & first-run UX (RFC 022 — complete except visual verification)

設計書 p.2「中心体験」前段 / p.8「初めての人が怖くない」/ p.10 A 初期画面 に対応:

- **`crates/aaai-gui/src/views/main_view.rs`** — Main 3 ペインそれぞれに RFC 022 §2 仕様の空状態パネルを実装:
  - `empty_state_file_tree()` — 「監査結果がまだありません / ▶ 監査実行で差分一覧がここに表示されます」
  - `empty_state_diff_panel()` — ① ▶ 監査実行  /  ② □ 開く から新しい監査を開始
  - `empty_state_inspector()` — 「ファイルを選んでください / ← 左のファイル一覧から選択」
- **`crates/aaai-gui/src/views/opening.rs`** — `recent_projects_section` を「profiles 空 → `onboarding_section()` / 非空 → 従来の Recent」分岐に置換。`onboarding_section` は ① ② ③ ステップ + audit.yaml 自動生成のヒントを 1 パネルに収める。NFR-1 達成（profiles が 1 件でも保存済みなら表示されない）。
- **`crates/aaai-gui/src/style.rs`** — `empty_state_panel_style` を追加。透明背景 + ソフトな 1px ボーダー + 8px 角丸。RFC 022 §3.3 「中間グレー」「色だけに依存しない」を満たす。
- **i18n キー** `empty_state.*` 12 件 × 2 locale を追加（onboarding_{title,step1..3,note} / file_tree_no_result_{title,hint} / diff_no_audit_{title,step1,step2} / inspector_no_selection{,_hint}）。
- **ABDD 適合**: 番号付け記号は `① ② ③` の Unicode 文字（カラーピクトに依存しない）。矢印・誘導は `←` `↑` `▶` `□` の文字記号。RFC 020 §3.3 のメッセージ + ヒントの 2 行パターンと統一感ある余白で構成。

**RFC 022 §5 完了条件の進捗:**

| 完了条件 | 状況 |
|---|---|
| Opening で profiles 空 → オンボーディング 3 ステップが表示 | コード commit 済（視覚確認待ち） |
| Opening で profiles 非空 → 既存 Recent | コード commit 済（既存挙動を維持） |
| Main で audit_result 不在 → 3 ペインに空状態 | コード commit 済 |
| audit_result 存在 → 空状態は出ない | NFR-1 の早期 return で保証 |
| en / ja 両方で正しく翻訳 | 構造対称（en=140, ja=140） |
| RFC 017 視覚検証で「初回起動者の動線」が判定 | operator 待ち |

### Pre-existing v0.19.0 i18n bug fixed

設計書 p.8 「色だけに依存しない」を支える toolbar 表示が、過去から 3 件の i18n キー欠落で
**dotted-key 文字列**（`"toolbar.passed"` 等）をそのまま表示していたバグを修正:

- `toolbar.passed` → "Passed" (en) / 「合格」(ja)
- `toolbar.failed` → "Failed" (en) / 「不合格」(ja)
- `toolbar.undo` → "Undo" (en) / 「元に戻す」(ja)

これにより `scripts/check-i18n-keys.py` の missing 件数は 6 → **0**、exit code 0 で CI 上の i18n
ゲートが strict モードなしでも通過するようになった。

### Screen navigation continuity — Save/Report freshness marks (RFC 021 — partial)

設計書 p.6 の「分岐を少なく、戻れる構造」を支えるため、Save と Report の完了マークを
toolbar に追加し、ユーザーが循環フロー（Opening → Audit → Review → Save/Report → Re-run）
の中で「いまどの段階にいるか」を視覚的に把握できるようにする:

**Data model（landed）:**

- **`crates/aaai-gui/src/app.rs`** — App に 3 つのフィールドを追加:
  - `audit_dirty: bool` — 監査結果が古くなった可能性のフラグ
  - `last_saved_at: Option<DateTime<Utc>>` — 最終 SaveDefinition 成功時刻
  - `last_reported_at: Option<DateTime<Utc>>` — 最終 ExportReport 成功時刻

**状態遷移（landed）:**

- `Message::SaveDefinition` 成功 → `last_saved_at = Some(now)`
- `Message::ExportReport` 成功 → `last_reported_at = Some(now)`
- `Message::ApproveEntry` / `Message::UndoApproval` → `audit_dirty = true`（直後の `rerun_audit()` が即座にクリア）
- `App::rerun_audit()` 完了 → `audit_dirty = false`

**toolbar 完了マーク（landed）:**

- **`crates/aaai-gui/src/views/main_view.rs`** — Save / Report ボタンの右隣に「✓ Saved Nm ago」「✓ Reported Nm ago」を表示。`last_saved_at` / `last_reported_at` が `None` の間は通常表示のまま。再描画は 30 秒 tick で humanize_since が更新される。
- **`crates/aaai-gui/locales/{en,ja}.yaml`** — `banner.saved_label` / `banner.reported_label` を追加。相対時刻は RFC 023 の `humanize_since` を再利用。

**Subscription（landed）:**

- 新 `Message::RelativeTimeTick` バリアント + 30 秒間隔の `iced::time::every` を `subscription()` に追加。**`last_saved_at` または `last_reported_at` が `Some` の時のみ enable** することで、新規起動時の CPU 無駄遣いを回避（needs_tick gating）。

**v0.20.0 で deferred したもの（FR-1 / FR-2 / FR-3 — 監査ダーティバナー）:**

設計書通りの "監査結果は古い可能性" バナー (FR-1) は v0.20.0 では出さない。現状のアーキテクチャでは、すべての definition mutation handler（ApproveEntry, UndoApproval）が同一 update tick 内で `rerun_audit()` を同期実行するため、`audit_dirty` フラグが視覚的に立つ瞬間がない。
データレイヤーは正しく実装してあり（flag set/clear が定義済み）、将来 mutation と rerun を非同期に decouple する RFC が出た時点で view 側を追加するだけで動作する。

**RFC 021 §5 完了条件の進捗:**

| 完了条件 | 状況 |
|---|---|
| 監査定義編集後にバナー表示 | ⏸ 視覚的には fires しない（同期 rerun アーキ）。フラグ自体は wired |
| 再監査ボタンが機能 | ✅ 既存 RFC 005 で実装済 |
| 再監査成功でバナー消える | ✅ `rerun_audit()` が `audit_dirty=false` を保証 |
| Save / Report 完了マークが toolbar に表示 | ✅ landed |
| 「N 分前」が 30 秒おきに更新 | ✅ `RelativeTimeTick` subscription |
| en / ja 両ロケールで自然な文言 | ✅ structure-symmetric |
| docs/src/gui.md に Re-run loop 図 | ⏸ RFC 019 のドキュメント refresh 範囲

### v1.0.0 release-prep groundwork (RFC 025 — partial, docs-only)

Phase 16 / v1.0.0 リリース準備のうち、display を必要としない docs 系を先行 landed:

- **`docs/src/compatibility.md`** (新規) — v1.x 系の互換性契約。`aaai` が v1.0.0 から v1.99.x の間維持する CLI / 設定ファイル / GUI / ライブラリ API の各サーフェスを SemVer 解釈で網羅。RFC 025 §4 全項目を反映:
  - CLI: 16 サブコマンド名・ロング/ショートオプション・終了コード 5 値・ヘルプ文言の policy
  - 設定: `~/.aaai/profiles.yaml` / `prefs.yaml` / `history.jsonl` / `audit.yaml` の field-policy（`#[serde(default)]` 経由の前方互換）
  - GUI: 7 件のキーボードショートカット、3 ペイン構造、i18n キー（8 名前空間）、テーマ、DnD
  - `aaai-core`: 独立 crate 公開判断は別 RFC で扱う旨を明記
  - 破壊的変更が必要な場合の運用フロー（opt-in → deprecation → 次 major で削除）
- **`docs/ja/src/compatibility.md`** (新規) — 日本語版。en と structure-symmetric。
- **mdbook ナビゲーション**: `docs/src/SUMMARY.md` と `docs/ja/src/SUMMARY.md` に `compatibility.md` を追加。あわせて、RFC 020 で作成済みだが mdbook ナビゲーションから孤立していた `abdd-audit.md` も両 SUMMARY に登録（orphan 解消）。
- **`README.md`** に Compatibility Policy へのリンクを追加。

**RFC 025 §7 受け入れ基準の進捗:**

| 項目 | 状況 |
|---|---|
| AC-1 `ROADMAP.md` 「v1.0.0 リリース判定ゲート」セクション | 既存 Phase 16 セクションで満たす（再記載不要） |
| AC-2 `docs/src/compatibility.md` 存在 + §4 方針網羅 | ✅ landed |
| AC-3 RFC 017〜024 が `done/` 配下 + Implemented | ⏸ Phase 16 リリース時点で実施 |
| AC-4 `verification/v1.0.0/` エビデンス | ⏸ Phase 16 リリース時点 |
| AC-5 cargo test / clippy / fmt 通過 | ✅ 現状 |
| AC-6 mdbook build エラーなし | ⏸ mdbook ローカル実行で要確認 |
| AC-7 git tag `v1.0.0` push + GitHub Release | ⏸ Phase 16 リリース時点 |

これにより `compatibility.md` 自体は v0.20.0 出荷物として利用可能になり、Phase 16 でのリリース準備時に文言調整のみで活用できる。

### i18n dead-key sweep — locale files trimmed from 142 → 119 keys

これまで 54 件あった「コードから参照されていない」i18n キーのうち、**23 件（× 2 locale = 46 件）を整理**し、関連する RFC で削除された機能の名残を locale ファイルから一掃した:

| 名前空間 | 削除キー | 削除理由 |
|---|---|---|
| `filter.*` | `failed` | 現コードは `filter.errors` で FailedAndError をカバー（RFC 002/003） |
| `inspector.*` | `approve_button`, `reason_placeholder`, `title` | Approve を ボトムバーへ移動（RFC 008）、placeholder はインライン化、panel タイトル廃止 |
| `opening.*` | `before/after_{label,placeholder,required}`, `definition_placeholder`, `not_a_directory`, `path_not_found` | RFC 015 で text input → file picker 化に伴い旧 UI の文言が不要に |
| `profile.*` | `delete`, `load`, `name_label`, `name_placeholder`, `save_current`, `title` | RFC 015 で独立 Profile パネル廃止、Opening の Recent リストに統合 |
| `toolbar.*` | `batch_approve`, `export_json`, `export_md`, `rerun` | RFC 007 でツールバー簡素化、`export_*` を単一 Export Report ボタンに統合（RFC 006）、`rerun` は `run_audit` にリネーム |

各キーは grep -r で 0 件のコード参照しか持たないことを個別検証してから削除した。

**audit script 改善（false-positive 対策）:** 削除作業中に判明したが、`make_btn("filter.all", FilterMode::All)` のように「dotted-key shape のリテラルを関数経由で `t!(label)` に渡す」パターンを静的解析が捕捉できておらず、`filter.{all,changed,errors,pending}` が誤って UNUSED として報告されていた。`scripts/check-i18n-keys.py` を改修し、dynamic `t!()` 呼び出しを含むファイル内の **dotted-key shape の文字列リテラルを保守的に「使用中」として収集**するよう変更。これにより:

- false-positive 「UNUSED」が消える（filter.* 4 件分）
- 本当に dead な key だけが UNUSED として残る
- 「dead と誤判定して削除 → 翻訳が壊れる」リスクが下がる
- pyflakes clean、既存テスト全件 green

**結果:**

```
i18n key audit: 0 missing, 0 divergent, 0 unused. (code: 119 keys; en: 119; ja: 119)
```

完全に均整の取れた状態。locale ファイルに死コードがなく、code 側の `t!()` 呼び出しと 1:1 対応する。新規翻訳者が unused キーに惑わされることもなくなった。

### Documentation refresh — GUI guide rewritten for v0.20.0 reality (RFC 019 — partial)

`docs/src/gui.md` と `docs/ja/src/gui.md` を v0.14 時代の記述から
**v0.20.0 の実装に合わせて完全に書き直し**:

**発見した深刻なドキュメント i18n バグ:** `docs/src/gui.md`（英語ドキュメントの想定パス）が実際には日本語で書かれていた — `docs/ja/src/gui.md` のほぼ複製で、英語の mdbook を読みに来た利用者には日本語が表示されていた。Phase 12 ですでに `compatibility.md` / `abdd-audit.md` / `testing.md` のような新規ドキュメントは正しく en/ja 2 系統で書かれているため、gui.md だけが過去の取り残しだった。

**書き直しの内容:**

- **§1 Opening 画面** — 4 つの text input フィールド前提の記述から、RFC 015 の 2 つのフォルダカード + 折りたたみオプション設定 + Recent / 初回オンボーディング分岐へ。DnD 受け入れ (RFC 023)、エラーバナー (RFC 020)、Recent の last_used_at 降順表示 + 相対時刻 (RFC 023)、はじめての方へオンボーディング (RFC 022) を追記。
- **§2 メイン画面** —
  - ツールバー: 旧 Export MD / Export JSON 並列ボタンを単一 Export Report に統一 (RFC 006/007)、Save / Report 完了マーク + 30 秒 tick で更新される `✓ Saved Nm ago` を記述 (RFC 021)、監査ステータスバッジ右側固定 (RFC 003/021)
  - ファイルツリー: バッチ選択 (RFC 007 で toolbar から削除) の記述を削除、空状態プレースホルダー (RFC 022) を追記
  - 差分ビューア: 3 タブ表示モード (RFC 011)、+ / − 行頭文字による色非依存表記 (RFC 010/012)、ダッシュボード代替表示 (RFC 022) を追記
  - インスペクター: 理由欄が textarea (RFC 009)、Regex のインライン validation + regex101 ヒント (RFC 002/020)、空状態プレースホルダー (RFC 022) を追記
  - ボトムアクションバー: 「承認して保存」単一主操作 (RFC 008) を独立節として追記
- **§3 キーボードショートカット** — Ctrl+E（エクスポート）、Enter（インスペクターフォーカス）、Escape（選択解除）を追加（既存 RFC 005）
- **§5 レポート出力** — 単一 Export Report ボタンの説明、Ctrl+E バインディング、JSON 出力は CLI 経由の旨を明記
- **§7 アクセシビリティ (ABDD)** — 新規セクション。ABDD 監査シートへのリンク、スクリーンリーダー v1.0 制限事項を明示

**en/ja 構造対称性検証:** 両ファイルとも 7 main セクション + 14 sub セクションが 1:1 対応する。en 303 行、ja 296 行 — 同等の情報密度。

**RFC 019 §5 完了条件の進捗:**

| 完了条件 | 状況 |
|---|---|
| `docs/src/gui.md` が v0.20.0 の Opening 再デザイン / Main 3 ペイン / インスペクター現状を正しく記述 | ✅ landed |
| `docs/ja/src/gui.md` が同等内容を日本語で記述 | ✅ landed |
| 削除済み機能（Batch Approve toolbar 入口、Export MD/JSON 分離、Profile 独立パネル）の言及を排除 | ✅ |
| 新機能（DnD、onboarding、空状態、エラーバナー、Save/Report freshness、Recent 相対時刻）を網羅 | ✅ |
| RFC 017 視覚検証で「ドキュメントと実画面が一致」と確認できる | ⏸ operator 検証時 |

`docs/src/testing.md` / `docs/ja/src/testing.md` への RFC 020 ABDD 検証セクション (§12) は既存ですでに更新済み。`docs/src/abdd-audit.md` / `docs/src/compatibility.md` も Phase 12 ですでに整備済み。

**第二弾の translation drift 修正（同じセッションで判明）:** gui.md だけでなく、`docs/src/` 配下の **CLI ガイド 4 ファイル + getting-started.md が同じ「日本語のまま放置」状態**であることが判明したため、以下も併せて修正した:

| ファイル | before | after |
|---|---|---|
| `docs/src/cli-auditing.md` | 日本語のまま | 英語化、`aaai audit` の `Next steps` ヒント (RFC 024) も明記 |
| `docs/src/cli-reporting.md` | 日本語のまま | 英語化、SARIF が GitHub PR レビューに統合される旨を補足 |
| `docs/src/cli-setup.md` | 日本語のまま | 英語化、**新規 `aaai exit-codes` セクション追加** (RFC 024)、Next-steps ブロックの説明を末尾に追記 |
| `docs/src/cli-workflow.md` | 日本語のまま | 英語化、`aaai dashboard` の Next-action hint (RFC 024) を明記 |
| `docs/src/getting-started.md` | 日本語のまま | 英語化、新規ユーザー向けに RFC 024 の Next-steps を case study に組み込み、`aaai-gui` の DnD UX (RFC 023) を反映 |
| `docs/src/cli.md` | 英語だが古い | Exit Codes セクションを「v1.x で安定」「`aaai exit-codes` で確認可能」と明記、Setup & Tooling コマンド表に `aaai exit-codes` を追加 |
| `docs/ja/src/cli.md` | 日本語 | 同等の更新（exit-codes 行 + 安定性宣言） |
| `docs/ja/src/cli-setup.md` | 日本語 | `aaai exit-codes` セクション + Next-steps の説明を追加（en と structure-symmetric） |

**結果: `docs/src/*.md` 全 15 章が真に英語、`docs/ja/src/*.md` 全 15 章が真に日本語。** バイリンガル mdbook が機能する状態に到達。

**RFC 024 のドキュメンテーション側完了条件 (§4):**

| 項目 | 状況 |
|---|---|
| `aaai exit-codes` サブコマンドが docs に記載 | ✅ landed（cli-setup.md と cli.md 両方） |
| `aaai audit` の Next-steps ヒントが説明されている | ✅ |
| `aaai dashboard` の Next-action hint が説明されている | ✅ |
| トップレベル `aaai --help` の Getting started ブロックが言及されている | ✅（cli-setup.md 末尾） |

### MSRV correction + docs spot-check (pre-existing bugs fixed)

`docs/src` の英語化スイープ完了後、残りの英語ドキュメントを spot-check したところ、ライフサイクル全体に影響する深刻な不整合が判明:

**MSRV 不整合 — CI が実行不能だったレベル:**

- `Cargo.toml` の `workspace.package` に `rust-version` フィールドが**存在しなかった**ため、cargo は MSRV を強制していなかった。
- プロジェクト指示書では開発環境として **Rust 1.91** を指定しているが、関連する記述すべてが古いまま放置されていた:
  - `.github/workflows/ci.yaml` の `check-msrv` job が `dtolnay/rust-toolchain@1.81` を指定（**毎回 fail していたはず** — 1.81 は edition 2024 を サポートしない）
  - `README.md` が「requires Rust 1.81+」と誤記
  - `docs/src/getting-started.md` も同じく「Rust 1.81 or later」
  - `docs/ja/src/getting-started.md` も同じ

**修正内容:**

- `Cargo.toml` の `[workspace.package]` に `rust-version = "1.91"` を明示追加（コメントで「プロジェクト指示書に従う」と明記）。
- `.github/workflows/ci.yaml` の `check-msrv` ジョブを `Rust 1.91` に統一。
- `README.md` / `docs/src/getting-started.md` / `docs/ja/src/getting-started.md` の MSRV 表記を `1.91+` に統一。

**aaai-core の crates.io 公開状態の誤記修正:**

`compatibility.md` を書いた際に「v1.0.0 時点で `aaai-core` は crates.io に独立 crate として公開されていない」と誤って記述していたが、CHANGELOG の過去エントリ (`docs.rs バッジ修正 — aaai → aaai-core`) から **すでに公開済み**であることが確認できたため:

- `docs/src/compatibility.md` Library API セクションを書き直し、crates.io / docs.rs リンクを明示、ベストエフォート安定の解釈を CLI / 設定ファイルと統一。
- `docs/ja/src/compatibility.md` も同じく修正。

**コマンド数 15 → 16:**

RFC 024 で `aaai exit-codes` を追加したことで CLI コマンド数は 15 → 16 になっているが、`docs/src/overview.md` の Components 表と Features 表で 15 のまま放置されていた:

- `docs/src/overview.md`: 「15 commands」→「16 commands」、Features 表の代表コマンド列挙に `exit-codes` を追加。
- `docs/ja/src/overview.md` 同じく。

**書き込み対象ファイルの不足:**

`docs/src/faq.md` の Q「aaai はファイルを変更するか？」の回答に列挙された「書き込み対象」リストから、RFC 015/023 で導入された `~/.aaai/profiles.yaml` が抜けていたため追加（en + ja）。

**`docs/src/ci-integration.md` の Exit Codes セクション:**

`cli.md` の Exit Codes と同じく v1.x 安定性宣言 + `aaai exit-codes` への参照を追加（en + ja symmetric）。

**統一感のある結果:**

| 場所 | MSRV 表記 |
|---|---|
| `Cargo.toml workspace.package.rust-version` | `1.91` |
| `.github/workflows/ci.yaml check-msrv` | `1.91` |
| `README.md` | `Rust 1.91+` |
| `docs/src/getting-started.md` | `Rust 1.91 or later` |
| `docs/ja/src/getting-started.md` | `Rust 1.91 以降が必要` |

これで cargo + CI + ドキュメントが 1 つの数値で揃い、`cargo check --workspace --all-targets` も問題なく通過する。

### Documentation infrastructure verified

`mdbook build` を両ロケールで実行し、英語版・日本語版ともに警告ゼロ・エラーゼロで rendered HTML が生成されることを確認。あわせて、独自の anchor-link 検証スクリプトで全ての `[text](file.md#anchor)` 形式リンクが正しく解決することも確認した（合計 0 件のリンク切れ）。これは Phase 12 で書き直した 6 ファイル + 既存ドキュメントの正常性を担保する。

`mdbook v0.4.52` を `cargo install mdbook` で取得し、CI 環境にもこのコマンドが入っていれば smoke test として組み込めるが、現状の CI には未組み込み（必要なら別 RFC で `docs-build` ジョブとして追加）。


### Operators' guide expanded for Phase 12 RFCs

`docs/templates/visual-verification-operators-guide.md` had detailed
per-RFC checklists for the legacy RFCs (014–016 as P0/P1, 007–013
as P1/P2) but **lacked any rows for the Phase 12 RFCs** (020/021/022/023).
The operator would have had to derive the verification rows from the
RFC specs themselves.

Added 5 new sections covering:

- **RFC 020** — error banner 2-line pattern, regex hint linkage to
  regex101.com, ABDD color-independence
- **RFC 021** — Save / Report freshness marks, 30-second tick refresh,
  clear-on-rerun behaviour
- **RFC 022** — all 4 empty states (Opening onboarding, file-tree no-result,
  inspector no-selection, dashboard no-changes) with «行動誘導の文言があるか»
  check
- **RFC 023** — DnD hint banner, folder-vs-file rule, Recent ordering by
  `last_used_at` with humanized relative timestamps
- **RFC 024 / 025** — explicitly marked as verified-by-automation / docs-only,
  so the operator doesn't waste time on them

Each section follows the same structure as the legacy rows: design-doc
anchor → expected → confirmation method, with «typical failure pattern»
notes at the end. The operator's verification work is now a guided
checklist rather than a derivation exercise.

### Release-prep checklist for v0.20.0

`docs/release-prep-v0.20.0.md` を新規追加。Phase 12 の operator 視覚検証完了後に行うべき機械的なリリース手順を 1 ファイルで walkthrough する形式:

1. Pre-flight checks (4 種類のスクリプトと mdbook smoke test)
2. Version bump 0.19.0 → 0.20.0
3. CHANGELOG の `[Unreleased]` → `[0.20.0]` プロモート
4. RFCs 017-025 を `proposed/` → `done/` へ移動
5. 各 RFC の Status フィールドを更新（部分実装の RFC 018/021/025 は特殊文言）
6. `rfcs/README.md` のインデックス更新
7. `cargo publish --dry-run` で 3 crate の整合性確認
8. リリースタールボール `aaai-v0.20.0.tar.gz` の作成
9. 抽出して動作確認（smoke test）
10. git tag + GitHub Release
11. ROADMAP.md の Phase 12 完了マーク

このファイルは `docs/src/` 配下に置かず、mdbook ナビゲーションには現れない（ユーザー向けドキュメントではなくリリースオペレーション用のドキュメントのため）。**operator が「次に何をすればよいか」が常時参照できる**ことを最重要視した構成。

### Planning

- **PLAN.md Rev. 4** — Phase 12 (視覚検証 / i18n / docs 刷新) から Phase 16 (v1.0.0 リリース準備) までの中期計画を整理。設計書 `aaai_uiux_design.pdf` の全要素はコード実装としては v0.19.0 で完了しているが、視覚検証 / ABDD 監査 / 行動可能エラー文 / 画面リレーション継続性 / 空状態誘導 等の品質ゲート未通過を v1.0.0 までに段階的に解消する。
- **新規 RFC 9 件 (Proposed)**:
  - RFC 017 — Visual Verification Harness & Protocol **(infrastructure landed)**
  - RFC 018 — i18n Locale Fallback Strategies (B/C) **(§3.4 key-audit script landed; B/C main work conditional on RFC 016 verification)**
  - RFC 019 — Documentation Refresh for v0.15–v0.19 Realities **(partial: GUI guide rewritten in proper en + ja for v0.20.0 reality; CLI/strategies/setup chapters and Re-run-loop diagram awaiting RFC 017 visual verification)**
  - RFC 020 — ABDD Accessibility Audit & Action-oriented Errors **(partial: error-message rewrites + checklist templates + open_error renderer landed; ABDD sheet filling awaits operator GUI run)**
  - RFC 021 — Screen Navigation Continuity **(partial: data layer + Save/Report toolbar marks + 30s tick subscription landed; audit-dirty banner deferred — non-visible under current synchronous-rerun architecture)**
  - RFC 022 — Empty States and First-run Guidance **(code-complete: 3 Main empty states + Opening onboarding + empty_state_panel_style; visual verification awaits operator)**
  - RFC 023 — Opening Drag-and-Drop and Recent Polish **(partial: data layer fully tested; DnD wiring committed; visual verification awaits operator)**
  - RFC 024 — CLI Dashboard & Help Discoverability Polish **(landed)**
  - RFC 025 — v1.0.0 Release Preparation **(groundwork: compatibility.md en/ja landed, mdbook nav fixed; final cut-over awaits Phase 16)**
- **rfcs/README.md 更新** — RFC 007〜016 を done/ に反映、新規 proposed 一覧を整理。
- **ROADMAP.md 更新** — Phase 12〜16 と v1.1.0 候補を追記。

## [0.19.0] — Sprint D-5/D-6: RFC 015 + RFC 016

> ⚠ **Visual verification status**: This release contains substantial GUI changes
> (Opening screen redesign, i18n repair) that have NOT yet been verified by
> running the binary. `cargo check` passes and tests are green, but the actual
> rendered output should be confirmed by launching `aaai-gui` and inspecting
> the Opening screen.

### RFC 016 — i18n Locale File Repair

- **Removed `_version: 1` line** from `crates/aaai-gui/locales/en.yaml` and
  `ja.yaml`. v1 is the default format for rust-i18n v4, so this line was
  redundant and possibly causing build cache confusion.
- Note: Root cause of literal-key display (e.g. `"opening.title"`) is not
  fully confirmed. The most likely fix is this YAML change plus a `cargo clean`
  during build. If literals still appear, RFC 016 §3.3 patterns B/C should be
  attempted in a follow-up release.

### RFC 015 — Opening Screen Redesign

#### Dependencies
- **Added `rfd = "0.17"`** to `crates/aaai-gui/Cargo.toml` for OS-native folder
  and file pickers.

#### Data model (`app.rs`)
- New field: `App.optional_settings_expanded: bool` (defaults to false)
- New `Message` variants (9 total):
  - `PickBeforeFolder` / `PickAfterFolder` / `PickDefinitionFile` / `PickIgnoreFile`
  - `BeforeFolderPicked(Option<PathBuf>)` / `AfterFolderPicked(...)` / `DefinitionFilePicked(...)` / `IgnoreFilePicked(...)`
  - `ToggleOptionalSettings`
- Picker handlers use `Task::perform(async { rfd::AsyncFileDialog::new().pick_folder().await }, ...)`

#### View (`opening.rs` — 306 lines, rewritten from scratch)
- **Welcome section**: large title + subtitle + guide line
  *「監査するための 2 つのフォルダを選んでください」*
- **Two required folder cards** (`folder_picker_card`):
  - 📁 icon + label
  - Status row: ✓/✗ + selected path OR "未選択"
  - "フォルダを選ぶ" / "フォルダを変更" button (≥ 44px tap target)
- **Optional settings section** (collapsible, default closed):
  - ▸/▾ toggle header
  - Hint text explaining "省略時は新規作成"
  - When expanded: audit.yaml + .aaaiignore file picker rows
- **Start audit button** (centered, large):
  - Active only when both Before+After are valid and not loading
- **Recent projects list** (up to 5 profiles):
  - Profile name + before→after summary + "開く" button

#### i18n keys (12 new keys, both `en` and `ja`)
- `opening.guide`, `opening.before_card`, `opening.after_card`, `opening.unselected`
- `opening.pick_folder`, `opening.change_folder`
- `opening.optional_section`, `opening.optional_hint`, `opening.ignore_label`, `opening.pick_file`
- `opening.recent_section`, `opening.open_recent`

### Verification checklist (for user testing)

- [ ] `cargo build -p aaai-gui` completes successfully
- [ ] Launching `aaai-gui` shows the Opening screen with translated text (no literal `opening.title`)
- [ ] Clicking "フォルダを選ぶ" opens the OS native folder dialog
- [ ] After selecting Before and After folders, "監査を開始" button becomes active
- [ ] Clicking "オプション設定" expands the audit.yaml / .aaaiignore fields
- [ ] Recent projects section shows previously saved profiles

## [0.18.1] — Critical i18n fix

### Bug fix — i18n keys were never resolving at runtime

**Symptom:** Every GUI label rendered as the raw translation key
(`opening.title`, `opening.subtitle`, `profile.save_current`, `app.version`, …)
instead of the translated text. The application was unusable in either locale.

**Root cause:** The `locales/en.yaml` and `locales/ja.yaml` files used
the wrong top-level structure for `rust-i18n` v4 with split-by-locale files:

```yaml
# WRONG — used since project inception
en:
  opening:
    title: aaai

# CORRECT — v4 split-file format
_version: 1
opening:
  title: aaai
```

The `en:` / `ja:` wrapper is only used in the "all locales in one file"
format with `_version: 2`. With split files (`en.yaml`, `ja.yaml`),
the locale is determined by the filename, so the wrapper makes every key
unreachable via `t!("foo.bar")`.

This bug was never noticed before because:
- Unit tests (`aaai-core`) and integration tests (`aaai-cli`) do not exercise the GUI
- Reported only after a user actually launched `aaai-gui` at v0.18.0

**Fix:**
- Removed `en:` / `ja:` wrappers from `locales/{en,ja}.yaml`
- Added `_version: 1` directive at the top of each file
- `app.version` key removed from YAML; now read at runtime from
  `env!("CARGO_PKG_VERSION")` so it always matches the actual crate version

## [0.18.0] — RFC 014: View layer fixes

### RFC 014 — View Layer Fixes (RFC 007/009 re-apply + ABDD tap areas)

#### Toolbar re-implemented (RFC 007 view side — previously failed to apply)
- Old toolbar (`save`, `rerun`, `Batch Approve`, `Export MD`, `Export JSON`, verdict badge) **removed**
- New toolbar: `[ □ 開く ] [ □ 保存 ] [ ▶ 監査実行 ] [ ↑ レポート出力 ]` with right-aligned `監査ステータス: PASSED / FAILED`
- `Message::BackToOpening` now correctly wired to "開く" button
- `colored_badge()` function removed (no longer used after toolbar + file tree changes)

#### Reason textarea applied (RFC 009 view side — previously failed to apply)
- Inspector reason field changed from single-line `text_input` to **multi-line `text_editor`** (height 72px)
- Reason label now shows `*` required marker: `理由(必須) *`
- `ins.reason_content` (text_editor::Content) properly rendered

#### ABDD tap area compliance (p.8)
- Toolbar buttons: padding `[4.0, 10.0]` → `[10.0, 16.0]` (≥ 44px)
- Filter bar buttons: padding `[3.0, 8.0]` → `[10.0, 14.0]`
- Bottom bar approve button: padding `[6.0, 18.0]` → `[10.0, 20.0]`
- Inspector add-rule button: padding `[4.0, 8.0]` → `[10.0, 14.0]`

## [0.17.1] — Documentation & README sync

### cargo outdated
- All dependencies up to date (v0.14.2 state maintained)
- `notify` 9.0.0-rc.4 (RC) — update deferred until stable
- `serde_yaml` 0.9.x (deprecated) — no stable successor yet; held

### README.md
- **License section removed** — per project policy (LICENSE file + badge is sufficient)
- **CI badge URL fixed** — `/.github/workflows/ci.yaml` → absolute GitHub Actions URL
- **docs.rs badge fixed** — `aaai` → `aaai-core` (the correct published crate name)
- **crates.io badge fixed** — `aaai` → `aaai-core`

### docs/src/gui.md + docs/ja/src/gui.md
- **"承認して適用"** updated to **"承認して保存"** (Approve & Save, bottom bar)
- **Batch Approve button** description updated (removed toolbar reference)
- **Export MD / Export JSON** → **レポート出力** (single button; JSON/HTML/SARIF via CLI)
- **Keyboard shortcuts** expanded: `Ctrl+E`, `Enter`, `/`, `Tab`, `Escape` added

### docs/src/testing.md + docs/ja/src/testing.md
- Section 4 (Inspector): "Approve" → "Approve & Save" (bottom bar)
- Section 5 (Save/Re-run): updated to reflect auto-save on approve
- Section 7 (Export): "Export MD" / "Export JSON" → "Export Report" / `aaai report`

### crates/aaai-gui/README.md
- Features list updated with: Bottom action bar, Diff view tabs,
  Reason textarea, LineMatch colour blocks, ABDD status icons,
  expanded keyboard shortcuts

## [0.17.0] — Sprint D-4: RFC 011 + RFC 012

### RFC 011 — Diff View Tabs
- `DiffViewMode` enum: `SideBySide` | `Unified` | `ChangedOnly` (default: SideBySide)
- `App.diff_view_mode` フィールドと `Message::SetDiffViewMode` を追加
- `diff_view::view()` にタブバーを追加: **左右差分 ｜ 統合 ｜ 変更のみ**
  - アクティブタブに青アンダーライン・太字
  - タブ切替で対応する差分ビューに即時反映
- `unified_view()`: unified diff 形式（- 赤 / + 緑 / 空白行）をスクロール可能な縦長ビューで表示
- `changed_only_view()`: Equal 行をスキップし `···` セパレーターで区切るビュー
- i18n キー追加: `diff.tab_side_by_side`, `diff.tab_unified`, `diff.tab_changed_only`, `diff.no_text_content`, `diff.no_changes`

### RFC 012 — LineMatch Rule Color Blocks
- `InspectorState.editing_rule: Option<usize>` フィールドを追加（クリックで編集モードに切替）
- `Message::EditRule(usize)` を追加（ルール行クリック → 編集フォーム展開 / 再クリックで閉じる）
- **表示モード（デフォルト）**: 色付きコードブロックで表示
  - Removed ルール: 薄い赤背景 + `- action: Removed / line: "..."` YAML 形式
  - Added ルール: 薄い緑背景 + `- action: Added / line: "..."` YAML 形式
  - ブロッククリックで編集フォームに展開
- **編集モード（クリックで展開）**: action ピックリスト + テキスト入力 + ✓ 閉じる / ✕ 削除
- `AddLineRule` 時は新しいルールを即座に編集モードで展開

## [0.16.0] — Sprint D-2: RFC 007 + RFC 008 + RFC 013

### RFC 007 — Toolbar & Navigation Restructure
- **「開く」ボタン追加**: Opening 画面に戻れるように（未保存時はトーストで警告）
- **ボタン再構成**: `[ □ 開く ]  [ □ 保存 ]  [ ▶ 監査実行 ]  [ ↑ レポート出力 ]`
- **「Batch Approve」削除**: ツールバーから除去（バッチ状態は内部保持）
- **「Export MD」「Export JSON」を「レポート出力」に統合**: Markdown をデフォルト出力
- **監査ステータス表示**: 件数バッジ → `監査ステータス: PASSED / FAILED` テキストに変更
- `Message::BackToOpening` 追加（未保存チェック付き）

### RFC 008 — Bottom Action Bar
- **ボトムバー新設**: 全ペインの外（最下部）に固定
  - `[ 承認して保存 ]` ← 選択中エントリが有効な場合のみ有効
  - `選択中: <filename>` ← 選択中ファイルパスを常時表示
  - `N件の差分中 M件が未解決` ← 右端に常時表示（未解決時は赤橙色）
- **インスペクターから承認ボタンを削除**: ボトムバーに一本化
- i18n キー追加: `bottombar.approve_and_save`, `bottombar.selected`, `toolbar.open`, `toolbar.run_audit`, `toolbar.report_output`, `toolbar.audit_status`

### RFC 013 — File Tree Icon Unification
- **行頭**: `diff-type バッジ（灰色）` → **`status_icon()`**（✓ ⚠ ✗ ! — の色付き記号のみ）
- **右端**: `status バッジ（テキストラベル付き）` → **`diff_type_tag()`**（+ − ~ T の控えめなグレー記号）
- `status_badge()` 関数を廃止し `status_icon()` / `diff_type_tag()` に置換
- `diff_icon` 計算・`diff_badge` を削除（不要になった）

## [0.15.0] — Sprint D-1: RFC 009 + RFC 010

### RFC 009 — Reason Field Multi-line Textarea
- `InspectorState` に `reason_content: text_editor::Content` フィールドを追加
- Inspector の理由フィールドを単行 `text_input` から **複数行 `text_editor`** (高さ 72px) に変更
- `Message::ReasonAction(text_editor::Action)` を追加
- `reason` String と `reason_content` を常に同期（`ReasonAction` ハンドラ内で `trim_end_matches('\n')` を適用）
- `SelectEntry` 時に `reason_content` を選択エントリの reason で初期化

### RFC 010 — Diff View Legend
- テキスト差分ビューア（side-by-side）の下部に「差分ハイライト: [■削除] [■追加]」凡例を追加
- `diff_legend()` 関数を `diff_view.rs` に追加
- i18n キー追加: `diff.legend_label`, `diff.legend_removed`, `diff.legend_added` (EN + JA)

## [0.14.2] — Dependency updates

### Updated
- `sha2` 0.10 → **0.11** (RustCrypto, API compatible)
- `clap_complete` 4.6.3 → **4.6.5** (patch, compatible)

### Investigated & held
- `notify` 8.2.0 — 9.0.0-rc.4 is a release candidate; update deferred until stable
- `serde_yaml` 0.9.x — deprecated upstream but no stable successor yet; held at 0.9
- `thiserror` — v2.0.18 is already the direct dependency; v1.0.69 is pulled in
  transitively by other crates (two-version coexistence is normal and harmless)

### Removed from roadmap
- "iced 0.15 / snora 0.9 migration" — neither version has been released;
  removed from the remaining issues list

## [0.14.1] — Consistency fixes & API documentation

### Bug fixes / Consistency
- `CHANGELOG.md`: removed stale `[1.0.0]` entry (was added prematurely in v0.8.x)
- `lib.rs`: removed outdated `//! # aaai-core v0.4.0` version tag from crate doc
- `report/html.rs`: "Generated by aaai v..." now shows the actual crate version
  via `env!("CARGO_PKG_VERSION")` instead of the hard-coded `v0.5.0`

### API documentation (`aaai-core`)
- `lib.rs`: expanded crate-level doc — module map, quick-start example,
  exit code contract table
- `audit/engine.rs`: `AuditEngine`, `evaluate()`, `evaluate_with_options()`
  now have full doc-comments with examples
- `diff/engine.rs`: `DiffEngine` now has a doc-comment with usage example

### Code quality
- `Message::ApproveEntry` in `aaai-gui`: documented as internal building block;
  `ApproveAndSave` is the preferred public action

## [0.14.0] — RFC 006: Report Output UX

### RFC 006 — Report Output UX

#### Markdown report restructured
- **Result symbol added** — `✓ PASSED` / `✗ FAILED` at the top
- **Summary table** — issues-first column order (Failed → Pending → Error → OK → Ignored)
- **New "⚠ Action Required" section** — consolidates Failed + Pending + Error entries
  at the top of the report, before OK entries
- **Passed entries** moved to a dedicated `## ✓ Passed Entries` section below
- **No-reason highlight** — empty `reason` shown as `*(no reason provided)*`
- **Audit detail as blockquote** — `> ✗ Entry has no reason...` for visibility
- **`md_entry()` helper extracted** — removes duplication across status sections

#### HTML report improved
- **`⚠ Action Required` banner** — shown above the entries table when audit fails;
  lists Failed/Pending/Error counts with a review reminder
- **`(no reason)` styling** — empty reason shown in red italic via `.no-reason` CSS class
- `html_escape()` function restored (was missing from the module)
- CSS additions: `.attention-banner`, `.no-reason`, `.detail-note`

## [0.13.1] — Technical debt cleanup

### Bug fixes

#### `ApproveAndSave` Task chain (GUI)
- `Message::ApproveAndSave` was calling `let _ = self.update(...)` on both
  `ApproveEntry` and `SaveDefinition`, silently discarding their `Task<Message>`
  return values.
- Fixed to use `Task::batch([t1, t2])` so that if either handler is ever made
  async (e.g. large-file save), the resulting Task is correctly propagated.

### Code quality

#### `CloseMenus` documented
- Added doc-comment to `Message::CloseMenus` explaining its role as the snora
  `ToastLayer` callback, and why it is kept distinct from `Noop`.

## [0.13.0] — Pre-release cleanup (UI/UX test pending)

### Note

v1.0.0 is pending completion of the UI/UX manual test checklist
defined in `docs/src/testing.md`. This release bundles the
post-RFC code cleanup only.

All 10 core development phases and the 5 UI/UX RFC improvements have shipped.
The acceptance criteria defined in `docs/src/testing.md` are met:

- **Zero warnings** across all targets (`cargo check --all-targets`)
- **92 unit tests** passing (`cargo test -p aaai-core --lib`)
- **54 integration tests** passing (`cargo test -p aaai-cli -- --test-threads=1`)
- All 5 RFC improvements implemented and moved to `rfcs/done/`

### What changed from v0.12.0

- Version set to `0.13.0` (v1.0.0 deferred until UI/UX test checklist passes)
- `CHANGELOG.md`: v0.11.0 and v0.12.0 entries backfilled
- `ROADMAP.md`: Phase 11 (GUI UI/UX Production Ready) added and marked ✅
- Minor code cleanup: `CloseMenus` → `Noop` (unified no-op message)

### Full feature set

See [CHANGELOG history](CHANGELOG.md) for the complete list of all changes
from v0.1.0 through v1.0.0, and [docs/](docs/src/SUMMARY.md) for
the full documentation.

## [0.12.0] — Sprint B+C: RFC 002 + RFC 003 + RFC 005

### RFC 002 — Inspector Validation & Primary Action
- `InspectorValidation` struct: per-field `reason_error`, `strategy_errors`, `expires_at_error`
- `FieldError { field, message }` for precise field-level attribution
- `validate_inspector()` rewritten with strategy-specific checks:
  - **Checksum**: must be exactly 64 hex characters
  - **LineMatch**: at least one rule required; no empty `line` fields
  - **Regex**: `regex::Regex::new()` compile-check on every keystroke
  - **Exact**: expected content cannot be empty
- `InspectorValidation::can_approve()` gates the approve button
- `Message::ApproveAndSave` — atomic approve + save as the primary inspector action
- `regex` added as `aaai-gui` dependency

### RFC 003 — ABDD Status Display
- `status_badge()`: symbol (`✓ ⚠ ✗ ! —`) + label text on coloured background, right-aligned per row
- Diff-type badge (`diff_badge`) uses neutral grey — status conveyed by `status_badge` alone
- Toolbar verdict updated to `✓ N  ⚠ N  ✗ N  ! N` (symbol + count, not color only)

### RFC 005 — Keyboard Navigation & Focus
- `FocusTarget` enum: `FileTree` | `Search` | `Inspector`; `focus_target` field on `App`
- New shortcuts: `Tab`/`Shift+Tab` (pane cycle), `/` (focus search), `Enter` (focus inspector reason),
  `Ctrl+E` (export Markdown report), `Escape` (deselect entry)
- New messages: `FocusNext`, `FocusPrev`, `FocusSearch`, `FocusInspectorReason`, `DeselectEntry`, `Noop`

### Other
- `docs/src/testing.md` + `docs/ja/src/testing.md`: CLI test count updated 30 → 54

## [0.11.0] — Sprint A: RFC 001 + RFC 004

### RFC 001 — CLI Output UX Consistency
- `aaai audit` output restructured: **result-first** layout with 4 zones
  - Zone 1 Header: `──────` separator + `Result: ✓ PASSED / ✗ FAILED` at the top, with Before/After/Config paths
  - Zone 2 Summary: `Total: N  ✓ OK: N  ⚠ Pending: N  ✗ Failed: N  ! Error: N`
  - Zone 3 Entries: symbol + label + path; Failed+Pending shown by default (max 20); `--verbose` for all
  - Zone 4 Next action: contextual hint ("fill in 'reason'…", "review Failed…", "run `aaai report`")
- Status symbols: `✓` OK · `⚠` Pending · `✗` Failed · `!` Error · `—` Ignored (color + symbol)
- `print_human_audit()` extracted as a standalone helper function

### RFC 004 — Opening Screen Input Validation
- `OpeningValidation` struct tracks before/after path errors
- `validate_opening()` called on every `BeforePathChanged` / `AfterPathChanged` event
- Inline error messages below each field (`✗ Folder not found.` / `✗ Path is not a directory.`)
- "Start Audit" button disabled when either required path is invalid or empty
- i18n keys added: `opening.before_required`, `after_required`, `path_not_found`, `not_a_directory`

## [0.10.3] — Fix unused import warnings in aaai-core

### Bug fixes
Removed 7 unused imports and 1 unnecessary `mut` across `aaai-core` test modules.
All warnings are now zero when running `cargo check -p aaai-core --tests`.

| File | Fix |
|---|---|
| `audit/warning.rs` | Removed unused `DiffStats` import |
| `audit/tests.rs` | Removed unused `std::path::Path` import; removed unnecessary `mut` |
| `config/lock.rs` | Removed unused `std::path::PathBuf` in test module |
| `diff/tests.rs` | Removed unused `std::path::PathBuf` import |
| `project/mod.rs` | Removed unused `std::path::Path` in test module |
| `report/sarif.rs` | Removed unused `PathBuf` and `DiffStats` imports in test module |

## [0.10.2] — Japanese docs, repository files update

### Repository files (replaced from upstream)
- `LICENSE` — updated to upstream version
- `README.md` — added crates.io / docs.rs / deps.rs badges
- `NOTICE` — copyright 2026 nabbisen
- `.github/CODE_OF_CONDUCT.md` — added
- `.github/CONTRIBUTING.md` — added
- `.github/SECURITY.md` — added (private reporting via GitHub security advisories)
- `.github/ISSUE_TEMPLATE/bug_report.yml` — structured bug report (YAML form)
- `.github/ISSUE_TEMPLATE/feature_request.yml` — structured feature request
- `.github/ISSUE_TEMPLATE/question.yml` — structured question
- `.github/ISSUE_TEMPLATE/config.yml` — blank issues enabled

### Documentation — Japanese translation complete (`docs/ja/`)
All 8 chapters are now fully written in Japanese:
- `overview.md` — 概要、構成、特徴
- `getting-started.md` — インストール、`aaai init`、手動セットアップ、GUI、`.aaai.yaml`
- `cli.md` — 全 15 コマンドのフラグ・終了コード・使用例
- `audit-definition.md` — YAML 構造、フィールド一覧、グロブ、バリデーション
- `strategies.md` — 5 戦略の詳細・使い分けガイド・大容量ファイル警告
- `gui.md` — 3 ペイン操作・キーボードショートカット・テーマ・ワークフロー
- `ci-integration.md` — 終了コード、GitHub Actions、SARIF、watch、設定
- `faq.md` — 13 件の Q&A（reason の必要性・グロブ・マージ・SARIF・マスキングなど）

## [0.10.1] — Project structure and documentation update

### Cargo / Publish
- Added `version = "0.10.1"` to `aaai-core` dependency in `aaai-cli` and `aaai-gui` Cargo.toml — `cargo publish` now works correctly
- Removed `path` from `snora` in workspace `Cargo.toml` (version-only specification)
- Added `readme`, `documentation`, and `homepage` metadata to each crate's `Cargo.toml`

### Dependency updates
- `similar` v2 → **v3**
- `indicatif` v0.17 → **v0.18**

### Repository hygiene
- Replaced `.gitignore` with a clean, well-commented version
- Updated `NOTICE` copyright year to 2026
- Removed redundant `AUTHORS` file (information already in `Cargo.toml`, `LICENSE`, `NOTICE`)
- Replaced `README.md` with a concise English-only version (removed version-specific test badge — maintenance overhead)

### GitHub Actions
- `actions/checkout` v4 → **v6**
- `dtolnay/rust-toolchain@stable` — already correct, confirmed
- `actions/upload-artifact` — v4 (confirmed)
- `actions/download-artifact` — v4 (confirmed)

### Per-crate README files (for `cargo publish`)
- `crates/aaai-core/README.md` — references top-level README
- `crates/aaai-cli/README.md` — CLI-focused quick reference
- `crates/aaai-gui/README.md` — GUI-focused quick reference

### CHANGELOG
- Fixed `[0.8.0]` heading: "Phase 8 — v1.0 comes closer (v0.8.0)"
- Translated CHANGELOG to English (older phases retain some Japanese in detailed bullet points)

### Documentation (docs/)
- `book.toml` updated with HTML output settings and multilingual note
- Japanese docs structure added: `docs/ja/` with `book.toml` + `src/` (mirroring English sources)
- Created `docs/src/overview.md` and `docs/src/audit-definition.md` (previously stubs)
- `docs/src/SUMMARY.md` updated to include all 8 chapters

## [0.10.0] — Phase 10: GUI Polish

### Tests (92 unit + 30 CLI = 122 passing)
- Added `profile/prefs.rs` round-trip / default / display tests (core unit: 92 tests)

### GUI features

#### Resizable panes (PaneGrid)
- Rewrote `main_view.rs` using **`iced::widget::pane_grid::PaneGrid`**
- All 3 panes (file tree / diff view / inspector) are resizable via drag handles
- Auto-initialised at 30% / 45% / 25% on `DiffReady`
- `Message::PaneResized(ResizeEvent)` updates ratio live

#### Dark / Light theme
- `profile/prefs.rs` — `Theme` (Light / Dark / System) + `UserPrefs` persisted to `~/.aaai/prefs.yaml`
- Added `.theme(|app| ...)` in `main.rs` to connect theme to iced application
- Added **theme picker** (Light / Dark) in the footer
- Theme is automatically restored on next launch

#### Directory collapse
- Added directory headers (▼ / ▶ icons) to the file tree
- Click header to collapse / expand child entries
- State stored in `App.collapsed_dirs: HashSet<String>`
- `Message::ToggleDir(String)` toggles the state

### Code quality
- **Zero warnings across all crates** — `cargo fix` applied + unnecessary imports cleaned up

## [0.9.0] — Phase 9: Documentation & Test Completeness

### Documentation improvements
- **`gui.md`** — 10 lines → **136 lines**: Opening screen, 3-pane operations, badge reference, keyboard shortcuts, footer, and typical workflow fully documented
- **`cli.md`** — 27 lines → **307 lines**: All 15 commands with flags, exit codes, and examples
- **`getting-started.md`** — 17 lines → **129 lines**: `aaai init` flow, manual setup, `.aaai.yaml`, and shell completion install

### Test coverage expansion (89 unit + 30 integration = 119 passing)
Added tests for the following previously untested commands:
- `completions bash/zsh` — output is non-empty and contains "aaai"
- `config --init` — `.aaai.yaml` creation and existing file detection (bug fix: now checks existence even when `--dir` is specified)
- `dashboard` — exit 0 verified
- `init --non-interactive` — `.aaai.yaml` creation
- `lint --json-output` — valid JSON output and `empty-linematch` error detection
- `version --json-output` — `version` / `license` field validation

### New features
- **`AuditWarning` suppression** — `suppress_warnings: [no-approver, no-strategy]` in `.aaai.yaml`; also `aaai audit --suppress-warnings <kind,...>`
- **`AuditEngine::evaluate_with_options()`** — new overload accepting `AuditOptions` (suppress_warnings)
- **`aaai history --prune <N>`** — prune history to the most recent N entries (`prune()` implemented in `history/store.rs`)
- **`aaai audit --warn-only`** — explicit intent flag (warnings do not affect exit codes by design)

### Bug fixes
- **`config --init` existence check** — fixed a bug where `.aaai.yaml` existence was not checked when `--dir` was specified

## [0.8.0] — Phase 8 — v1.0 comes closer (v0.8.0)

### Bug fixes (UI/UX  test前に必須)

#### Fix 1&2: GUI — `ignore_path` not connected to `IgnoreRules`
- Connected `ignore_path` field value to the async diff execution in **`StartAudit`**
  - Falls back to `<Before>/.aaaiignore` auto-discovery when blank (consistent with CLI behaviour)
  - When specified, builds `IgnoreRules` from the given path
- Fixed **`rerun_audit()`** to re-scan with the same `IgnoreRules`
  - Added `active_ignore: IgnoreRules` to `App`, saved via `DiffReady` message
  - Fixed: "Re-run audit" button was returning different files from the original scan
- **`DiffReady` message signature changed**: `IgnoreRules` added as the third argument

#### Fix 3: GUI — `AuditWarning` not displayed
- Added warning section immediately below the divider in the **inspector panel**
  - `large-file` → yellow background block + `⚠` icon
  - `no-strategy` → blue-tinted `ℹ` icon
  - `no-approver` → grey-tinted `ℹ` icon
  - Section hidden entirely when no warnings (no layout impact)

#### Fix 4: GUI — `AuditWarning` badge missing in file tree
- Added `⚠N` badge (N = warning count) to each row in the file tree
  - Small yellow-tinted badge (9px)
  - Not shown for entries with zero warnings

#### Fix 5: GUI — `warning_count` not shown in toolbar
- Added `⚠ N warning(s)` text next to the verdict badge in the toolbar
  - Only shown when `AuditSummary.warning_count > 0`

#### Fix 6: GUI — keyboard shortcut legend missing
- Added `Ctrl+S: Save  Ctrl+R: Re-run  Ctrl+Z: Undo  ↑↓: Navigate` to the footer
  - Shown on Main screen only (hidden on Opening screen)

#### Fix 7: GUI — Opening screen UX improvements
- Updated `.aaaiignore` field placeholder to indicate auto-discovery behaviour
- Loading spinner now shows Before/After folder names to clarify what is being scanned

### Version
- Version set to `0.8.0` in Cargo.toml (v1.0 pending UI/UX testing)

## [0.7.0] — Phase 7: v1.0 Quality

### Tests (101  passing)
- core unit tests: **81 件** (AuditWarning 7 件、SARIF 2 件、lockfile 2  added)
- CLI integration tests: **20** (export CSV/TSV, merge conflict, SARIF format, history stats, diff JSON)

### Code quality
- **Zero warnings** — `cargo fix` + 全 unused variable/dead_code を `_prefix` / `#[allow]` で抑制。`cargo check` が全クレートで warningゼロ

### Core additions
- **`audit/warning.rs`** — `AuditWarning` システム: `LargeFileStrategy` (>1MB に Exact/LineMatch 適用)、`NoStrategyOnModified`、`NoApprover` の 3 種類
- **`FileAuditResult.warnings`** — 各エントリに advisory  warningリストを付与
- **`AuditSummary.warning_count`** — 全体の warning件数を集計
- **`config/lock.rs`** — `.lock`  fileによる書き込みロック。60 秒 TTL でステールロックを自動 removed。`config/io.rs` に統合済み

### CLI additions
- **`aaai export`** — 承認済みエントリを CSV / TSV に出力。13 カラム: path, diff_type, status, reason, strategy, ticket, approved_by, approved_at, expires_at, enabled, note, created_at, updated_at
- **`aaai init`** — 対話型プロジェクト初期 configウィザード。Before/After パス・定義 file・承認者名を対話入力し `.aaai.yaml` を generation。`--non-interactive` フラグ support
- **`aaai history --stats`** — 全実行履歴のトレンド分析: 合格率・平均 OK/Pending/Failed 件数・直近 5 回 vs 前 5 回の傾向 (↑ improvement / ↓低下 / →安定)

## [0.6.0] — Phase 6: Production Readiness

### Tests (85  passing)
- core unit tests: 73 件（SARIF  test 2  added）
- CLI integration tests: 12 件

### Core additions
- **Entry versioning** — `created_at` / `updated_at` fields added to `AuditEntry`; `stamp_now()` auto-stamps on approval
- **`report/sarif.rs`** — SARIF v2.1.0 レポート generation。Failed → error、Pending → warning にマッピング
- **`ReportGenerator::build_markdown_string(include_diff: bool)`** — 差分テキスト埋め込みオプション付き Markdown  generation

### CLI additions
- **`aaai diff`** — 定義 file不要の純粋差分 display。`--content` で実差分テキスト、`--json-output` で JSON 出力
- **`aaai merge <BASE> <OVERLAY>`** — 2つの定義 fileをマージ。`--detect-conflicts` で競合チェックのみ実行
- **`aaai report --format sarif`** — SARIF v2.1.0 出力（GitHub Actions `upload-sarif` で PR アノテーション support）
- **`aaai report --include-diff`** — Markdown/HTML レポートに実差分テキストを埋め込み

### GitHub Actions CI/CD
- **`.github/workflows/ci.yaml`** —  test (Ubuntu/macOS/Windows)・フォーマットチェック・Clippy・MSRV  verified・セキュリティ監査
- **`.github/workflows/release.yaml`** — タグプッシュ時にクロスコンパイルビルド + GitHub Release 自動作成

### GUI additions
- **Undo  feature** — `Message::UndoApproval` で最後の承認を取り消し (最大 20 件スタック)
- **Keyboard shortcuts** — Ctrl+S (save), Ctrl+R (re-run), Ctrl+Z (undo), ↑↓ (navigate)

### Documentation完成
- `docs/src/strategies.md` — 全 5 戦略の詳細解説・使い分けガイド
- `docs/src/ci-integration.md` — GitHub Actions 例・SARIF アップロード・Watch モード・シェル補完インストール
- `docs/src/faq.md` — 13 件の FAQ（理由必須の理由・glob ルール・マージ・SARIF 活用など）
- `docs/src/SUMMARY.md` — mdBook 目次 updated (8 章)

## [0.5.0] — Phase 5: Polish

### Tests (83  passing)
- **core unit tests**: 71 件 (diff/audit/config/masking/project/templates/profile/history)
- **CLI integration tests**: 12 件 (実バイナリを使った end-to-end  test)
  - exit code 検証 (0/1/2)、JSON 出力の妥当性、glob ルール、HTML レポート generation、dry-run 動作など

### CLI additions
- **`aaai completions <shell>`** — bash / zsh / fish / powershell 向けシェル補完スクリプト generation (clap_complete)
- **`aaai watch`** — before・after・定義 fileの changedを監視し、 changed検出時に自動で監査を再実行 (notify crate、500ms デバウンス)
- **`aaai dashboard`** — colour-coded stat cards + attention list; `--detail` flag shows all changed entries
- **`aaai audit --progress`** — indicatif プログレスバーで大規模 folderの比較進捗を display
- **`aaai snap --dry-run`** —  fileを書き込まずに generation内容をプレビュー
- **`aaai report --format html`** — スタイル付き HTML レポートを出力（summary カード・ステータス色分け・チケット display・差分統計）

### Core additions
- **`diff/progress.rs`** — `DiffProgress` イベント + `ProgressSink` トレイト + `ChannelProgress` / `NullProgress`  implementation
- **`DiffEngine::compare_with_progress()`** — 進捗シンクを受け取るオーバーロード
- **`report/html.rs`** — セルフコンテイン HTML レポート generation (BootstrapなしのCSSインライン)

### GUI additions
- **Dashboard view** — shows summary cards (OK/Pending/Failed/Error/Ignored counts) + result banner + attention list when no file is selected
- **File tree search bar** — incremental path filter (search input placed below filter bar)

## [0.4.0] — Phase 4: Advanced

### Core modules (69  passing)
- **`diff/entry.rs` 強化** — `is_binary` フラグ・`before_size`/`after_size`・`before_sha256`・`DiffStats`（lines_added/removed/unchanged）フィールドを added
- **Parallel diff engine** — `rayon` `par_iter` for large folder comparison; sorted output guaranteed
- **Binary detection** — null-byte heuristic; binary files tracked by hash/size only, text strategies not applied
- **`diff/entry::fmt_size()`** — formats byte counts as human-readable strings (B/KB/MB/GB)
- **`masking/`** — `MaskingEngine` + 9 種のビルトインパターン（API キー、パスワード、AWS キー、GitHub トークン、Slack トークン、Bearer トークン、接続文字列パスワード、秘密鍵ヘッダー）。カスタムパターン added可能
- **`project/config.rs`** — `.aaai.yaml` の loading・ saved・上位ディレクトリへのオートディスカバリー

### CLI additions
- **`aaai config`** — `.aaai.yaml` を現在ディレクトリ付近から検索・ display。`--init` でスターターテンプレート generation
- **`aaai audit --mask-secrets`** — masks reason field in verbose output; also activated by `mask_secrets: true` in project config
- **`aaai audit --verbose`** — バイナリ fileの (binary file)  display、差分統計 (+N -N lines)、サイズ変化を added
- **レポートへのマスキング support** — `write_markdown` / `write_json` に `Option<&MaskingEngine>` 引数を added

### GUI additions
- **バイナリ fileパネル** — バイナリ差分選択時に専用パネルを display。 file種別・サイズ変化・before/after SHA-256 ハッシュ・一致/不一致の視覚的 display
- **差分統計バー** — テキスト差分ビューアの上部に `+N lines` / `−N lines` と サイズ変化を display

## [0.3.0] — Phase 3: Integrations

### Core additions
- **Approver tracking** — `approved_by` / `approved_at` fields added to all `AuditEntry`; auto-stamped on approval
- **Expiry dates** — `expires_at` (NaiveDate) field; expired entries shown as warnings in CLI and GUI
- **Ticket linkage** — `ticket` field (JIRA-123, INF-42, etc.) shown in reports and inspector
- **Empty reason → Pending** — `AuditEngine` now treats snap-generated entries with no reason as Pending
- **`.aaaiignore`** — `diff/ignore.rs`; gitignore-style pattern exclusion from diff; negation rules (`!pattern`) supported
- **Audit history** — `history/store.rs`; run log appended to `~/.aaai/history.jsonl` in JSONL format
- **Rule templates** — `templates/library.rs`; 8 built-in templates (version bump, port change, config value change, etc.)
- **Audit profiles** — `profile/store.rs`; before/after/definition combos saved to `~/.aaai/profiles.yaml`

### CLI additions
- **`aaai check`** — 定義 fileの妥当性を差分実行なしで検証。期限切れエントリも報告。Config  errorで exit 4
- **`aaai history`** — `~/.aaai/history.jsonl` から最近の監査実行を一覧 display。`--json-output`  support
- **`aaai snap --template <id>`** —  generation時にルールテンプレートを適用
- **`aaai snap --list-templates`** — テンプレート一覧 display
- **`aaai audit --ignore <FILE>`** — .aaaiignore  fileを明示指定
- **詳細終了コード** — 0=PASSED, 1=FAILED, 2=PENDING, 3=ERROR, 4=CONFIG_ERROR

### GUI additions
- **インスペクター Phase 3 フィールド** — ticket, approved_by, expires_at の display・編集
- **有効期限バッジ** — `EXPIRED` / `Expiring soon` のカラーバッジをインスペクターヘッダーに display
- **テンプレートピッカー** — インスペクターに "Apply template" ドロップダウンを added（8 テンプレート support）
- **プロ fileマネージャー** — Opening 画面にプロ file saved・ loading・ removed UI を added
- **Opening: ignore path フィールド** — .aaaiignore  fileのパスを Opening 画面で指定可能

### Tests
- Phase 3 動作カバー (51  test)：空理由 → Pending、Unchanged 自動 OK など

## [0.2.0] — Phase 2: Quality & Completeness

### 必須要件 support (別紙)
- **tests.rs 分離**: `diff/tests.rs`, `audit/tests.rs`, `config/tests.rs` に unit/integration  test 37 件を added
- **GUI 多言語 support**: rust-i18n v3 で日英 locale  file (`en.yaml` / `ja.yaml`) を implementation。フッターのロケールピッカーで切り替え可能

### Core  feature added
- **Glob パターンマッチング**: `path` フィールドに `logs/*.log` や `build/**` 形式の glob ルールを使用可能。完全一致ルールが優先
- **Unchanged エントリの自動 OK**: 差分のないエントリは監査ルールなしで自動 OK 判定
- **tests.rs**: config の glob マッチ testを含む 37 件の testが passing

### GUI  feature added (iced + snora)
- **フィルターバー**: Changed Only / All / Pending / Failed・Error の 4 モードで差分一覧を絞り込み
- **バッチ承認**: 複数エントリを選択（チェックボックス）し、共通理由で一括承認。snora `Sheet` (端パネル) として display
- **Toast subscription  fixed**: `App::subscription` を iced アプリケーションに正しく接続し、TTL 自動 removedが featureするよう fixed
- **差分ビューアの improvement**: 行番号 display、`iter_all_changes` ベースの安定したレンダリング
- **ロケールピッカー**: フッターに配置。`LANG` 環境変数でシステムロケールを自動検出

### CLI  feature added
- `--verbose`: OK / Ignored エントリも displayし、reason を併記
- `--quiet`: サマリー行のみ出力
- `--json-output`: 監査結果を JSON で stdout に出力（CI/CD での機械 processing向け）

## [Unreleased] — Phase 1

### Added
- Folder diff engine with seven diff types
- Audit definition YAML format (version 1)
- Content audit strategies: None, Checksum, LineMatch, Regex, Exact
- CLI subcommands: audit, snap, report
- GUI with snora/iced: opening screen and 3-pane main screen
- Approval flow requiring mandatory reason
- Markdown and JSON report generation
