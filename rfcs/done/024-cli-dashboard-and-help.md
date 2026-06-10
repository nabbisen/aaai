# RFC 024 — CLI Dashboard & Help Discoverability Polish

**Status.** Implemented (v0.20.0)
**Priority.** v1.0 nice-to-have
**Tracks.** 設計書 p.4 (CLI 表示の約束: 結論先出し / 色だけに依存しない / 迷わないヘルプ / CI に優しい) / RFC 001 の発展
**Touches.** `crates/aaai-cli/src/cmd/dashboard.rs` · `crates/aaai-cli/src/main.rs` (after-help hint) · `crates/aaai-cli/src/cmd/{audit,snap,report,check,lint,diff,merge,history,init,watch,config,export,version_cmd}.rs`

---

## 1. 要件定義

### 1.1 目的

設計書 p.4 CLI 表示の約束「迷わないヘルプ」と「結論先出し」を、`aaai dashboard` および
全サブコマンドの `--help` 末尾の **「次の操作」ヒント** に展開する。RFC 001 は audit
出力に Zone 4 hint を実装したが、他のサブコマンドや dashboard には未適用。

### 1.2 解決すべき問題

| 問題 | 設計書原則 |
|---|---|
| `aaai dashboard` の結果はカードを表示するが、「次にどうする」のヒントがない | p.4「結論を先に出す」「迷わないヘルプ」 |
| `aaai audit --help` で何ができるか分かるが、「次に snap を作ろう」のような誘導がない | p.4「迷わないヘルプ」 |
| `aaai snap --help` の例示は十分だが、出力後に何をすべきか書かれていない | 同上 |
| エラー終了時の終了コードは確定しているが、ユーザーが終了コードの意味を CLI から引けない | (新規) |

### 1.3 機能要件

| ID | 要件 | 必須/任意 |
|---|---|---|
| FR-1 | `aaai dashboard` 出力末尾に Next-action hint を表示 (audit と同じパターン) | 必須 |
| FR-2 | 全サブコマンドの `--help` 末尾 (clap の `after_help`) に「次の操作」ヒントを追加 | 必須 |
| FR-3 | `aaai --help` (no subcommand) 末尾に「初めての方は `aaai init` から」と誘導 | 必須 |
| FR-4 | (任意) `aaai exit-codes` (新規サブコマンド) で終了コード一覧を表示 | 任意 |
| FR-5 | `aaai dashboard` の出力を設計書 p.4 のスタットカード構造に近づける | 必須 (微調整) |

### 1.4 非機能要件

| ID | 要件 |
|---|---|
| NFR-1 | `--quiet` 指定時はヒントを出さない |
| NFR-2 | JSON 出力モード (`--json-output`) ではヒントを stderr に出さない (機械処理を妨げない) |
| NFR-3 | ヒントは英語のみで初期実装 (CLI の i18n は本 RFC 範囲外) |

---

## 2. 外部設計（基本設計）

### 2.1 `aaai dashboard` の Next-action hint

```
$ aaai dashboard --left ./before --right ./after --config audit.yaml

  ╭───────────────────────────────────────────────╮
  │   Audit Dashboard                              │
  ╰───────────────────────────────────────────────╯

  ┌─────────┐ ┌─────────┐ ┌─────────┐ ┌─────────┐ ┌─────────┐
  │   110   │ │    8    │ │    4    │ │    1    │ │    0    │
  │   OK    │ │ Pending │ │ Failed  │ │  Error  │ │ Ignored │
  └─────────┘ └─────────┘ └─────────┘ └─────────┘ └─────────┘

  Result: FAILED — 4 entries do not match their expected rules.

  Next: review ✗ Failed entries in the diff viewer,
        update rules or reason, then re-run `aaai audit`.
```

### 2.2 サブコマンド `--help` 末尾ヒント (例)

```
$ aaai audit --help
...
Examples:
  aaai audit -l ./before -r ./after -c audit.yaml
  aaai audit ... --json-output
  aaai audit ... --mask-secrets

Next steps:
  - If status is PENDING: open audit.yaml and fill in 'reason' for each entry.
  - If status is FAILED: review the diff and update rules.
  - When all PASSED: run `aaai report -o report.md` to generate a report.

Exit codes:
  0  PASSED
  1  FAILED
  2  PENDING
  3  ERROR
  4  CONFIG_ERROR
```

サブコマンドごとに以下のような Next-steps:

| サブコマンド | Next steps |
|---|---|
| `audit` | (上記) |
| `snap` | "Edit the generated audit.yaml and fill in 'reason' for each entry, then `aaai audit`." |
| `report` | "Review the report file. Re-run audit to refresh after rule changes." |
| `check` | "If issues: edit audit.yaml. If clean: run `aaai audit` to evaluate against actual folders." |
| `lint` | (similar) |
| `diff` | "Run `aaai snap` to create an audit definition from these diffs." |
| `merge` | "After merging, validate with `aaai check` before running audit." |
| `init` | "Now edit .aaai.yaml or run `aaai snap` to generate an initial audit definition." |
| `watch` | "Press Ctrl+C to stop. While watching, edit files or the audit definition to see re-runs." |
| `history` | "Use --stats to see trends. Use --max-entries N to prune old runs." |

### 2.3 `aaai --help` (no subcommand) 末尾

```
$ aaai --help
...

Getting started:
  aaai init                        # interactive setup wizard
  aaai snap -l ./a -r ./b -o audit.yaml   # create audit definition
  aaai audit -l ./a -r ./b -c audit.yaml  # run audit

See `aaai <subcommand> --help` for details on each command.
For the desktop UI, run `aaai-gui`.
```

### 2.4 (任意) `aaai exit-codes`

```
$ aaai exit-codes
Exit code reference:
  0  PASSED        All entries match expected rules
  1  FAILED        One or more entries do not match
  2  PENDING       Unresolved (no reason filled in) entries exist
  3  ERROR         File-level read or compare errors
  4  CONFIG_ERROR  Invalid audit.yaml or other config error
```

---

## 3. 内部設計（詳細設計）

### 3.1 clap の after_help 構文

```rust
#[derive(Args)]
#[command(after_help = AUDIT_AFTER_HELP)]
pub struct AuditArgs { ... }

const AUDIT_AFTER_HELP: &str = "\
Next steps:
  - If status is PENDING: ...
  ...
";
```

`AUDIT_AFTER_HELP` 等の定数はサブコマンドファイル内に置く。

### 3.2 `aaai dashboard` の Next-action 共通化

`audit.rs` 内の Zone 4 logic を `crates/aaai-cli/src/cmd/mod.rs` または新規 `next_hint.rs` に
ヘルパー関数として切り出す:

```rust
pub fn next_action_hint(s: &AuditSummary) -> Option<String> {
    if s.pending > 0 && s.failed == 0 {
        Some(format!("Next: open audit.yaml and fill in 'reason' for {} Pending entr{}, then re-run `aaai audit`.",
            s.pending, if s.pending == 1 { "y" } else { "ies" }))
    } else if s.failed > 0 {
        Some(format!("Next: review {} Failed entr{} in the diff viewer, update rules or reason, then re-run.",
            s.failed, if s.failed == 1 { "y" } else { "ies" }))
    } else if s.error > 0 {
        Some(format!("Next: check {} Error{} for file access issues.", s.error, if s.error == 1 { "" } else { "s" }))
    } else {
        Some("Next: run `aaai report` to generate a report.".to_string())
    }
}
```

`audit.rs` と `dashboard.rs` の両方からこれを呼ぶ。

### 3.3 設計書 p.4 のスタットカード再現

現状の `aaai dashboard` 出力は概ね一致しているが、以下の点を再点検:

- カードが横並び 5 個 (OK / Pending / Failed / Error / Ignored)
- 各カードに数字とラベル
- 結果バナー "Result: PASSED" or "Result: FAILED"
- カードの色は OK = 緑 / Pending = 黄 / Failed = 赤 / Error = 赤 / Ignored = 灰

dashboard.rs を確認して、必要に応じてサイズ・配色を微調整。

### 3.4 `aaai exit-codes` (任意 FR-4)

新規 `crates/aaai-cli/src/cmd/exit_codes.rs`:

```rust
pub fn run() {
    println!("Exit code reference:");
    println!("  0  PASSED        All entries match expected rules");
    // ... 上記表
}
```

clap での登録:
```rust
.subcommand(Command::new("exit-codes").about("Show exit code reference"))
```

---

## 4. プログラム設計

### 4.1 実装手順

| Step | 作業 | 検証 |
|---|---|---|
| 1 | `cmd/next_hint.rs` を新規作成、`next_action_hint` を実装 | 単体テスト |
| 2 | `audit.rs` の Zone 4 logic をリファクタして `next_action_hint` を呼ぶ | 既存挙動と一致 |
| 3 | `dashboard.rs` の出力末尾に Next-action hint を追加 | 視覚確認 |
| 4 | 全サブコマンドに `after_help` を追加 | `--help` 出力確認 |
| 5 | `main.rs` のトップレベル Command に `after_help` を追加 | 同上 |
| 6 | (任意) `cmd/exit_codes.rs` を実装 | `aaai exit-codes` 動作確認 |
| 7 | integration test に new help / hint の存在確認を追加 | green |

### 4.2 影響範囲

| ファイル | 変更行 |
|---|---|
| `crates/aaai-cli/src/cmd/next_hint.rs` (新規) | 約 30 行 |
| `crates/aaai-cli/src/cmd/audit.rs` | 約 10 行 (Zone 4 を関数呼び出しに) |
| `crates/aaai-cli/src/cmd/dashboard.rs` | 約 10 行 |
| 全サブコマンド (`*.rs`) | 各 5〜10 行 (after_help 定数 + 属性) |
| `crates/aaai-cli/src/main.rs` | 約 8 行 |
| `crates/aaai-cli/src/cmd/exit_codes.rs` (新規・任意) | 約 25 行 |
| `crates/aaai-cli/src/tests.rs` | 約 30 行 (テスト追加) |

### 4.3 リスク

| リスク | 対策 |
|---|---|
| `after_help` で `--help` 出力が長くなりすぎる | 1 サブコマンドあたり 5〜8 行以内に収める |
| Next-action hint の i18n 化を本 RFC で行わない判断が将来の負債になる | CLI は英語固定で長く運用されており影響小。GUI と分けて判断 |
| 既存の integration test が `--help` 出力をマッチしている場合に壊れる | テスト側のマッチは構造化 (`contains("Next steps")`) を使い、行ごとの完全一致を避ける |

---

## 5. 完了条件

- [ ] `aaai dashboard` の最後に Next-action hint が表示される
- [ ] 全サブコマンドの `--help` 末尾に "Next steps:" 節がある
- [ ] `aaai --help` (no subcommand) に "Getting started:" 節がある
- [ ] `audit.rs` の既存 Zone 4 hint がリファクタ後も同等の文言を出す
- [ ] `--quiet` 時はヒントを抑制
- [ ] integration test green
- [ ] (任意) `aaai exit-codes` が動作する

## 6. 依存

- なし (CLI 単独の改善)

## 7. v1.0 範囲外

- CLI の多言語化 (英語固定で v1.0 出荷)
- `aaai exit-codes` の出力フォーマット (簡素なテキストのみで開始)
