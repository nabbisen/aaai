# RFC 003 — ABDD Status Display

**Status.** Implemented (v0.12.0)  
**Tracks.** GUI ファイルツリーおよびステータス表示のアクセシビリティ。  
設計書 p.8「Accessible by Default Design チェック観点」  
**Touches.** `crates/aaai-gui/src/views/main_view.rs` · `theme.rs` · `locales/*.yaml`

## Summary

ファイルツリーのステータス表示を「色のみ」から「色＋アイコン＋テキスト」に変更し、
色覚多様性・モノクロ環境・スクリーンリーダーへの配慮を加える。
設計書が定義する ABDD (Accessible by Default Design) の最小要件を実装する。

## 問題

### 1. ステータスが色のみ

現状のファイルツリー行:
```
[+ 緑バッジ] new_file.rs
[✗ 赤バッジ] config.toml
```

バッジの色が変わるが、同じ `+` アイコンが使われている。
色覚多様性のあるユーザー・CI ログ・スクリーンキャプチャで判別が困難。

設計書 p.8: 「状態を色だけで表さない。文字・アイコン・件数を併用」

### 2. ステータステキストが行内にない

どのファイルが OK で、どれが Pending かをファイルツリー内で
テキストとして読める場所がない（インスペクターを開かないと分からない）。

### 3. diff アイコンとステータスアイコンが混在

現状: diff type (Added/Modified/Removed) と audit status が
同じバッジに混在しており、情報の意味が曖昧。

## 設計

### 3-1. ファイルツリー行の新レイアウト

```
[diff badge] [path]                         [status badge]
```

| 列 | 内容 | 変更 |
|---|---|---|
| diff badge | `+` `-` `~` `T` `!` `?` （diff type） | 現状維持 |
| path | ファイル名 | 現状維持 |
| status badge | `✓ OK` / `⚠ Pending` / `✗ Failed` / `! Error` / `— Ignored` | **新規** |
| warn badge | `⚠N` | 現状維持 |

```
[+] new_file.rs                          [⚠ Pending]
[~] config/server.toml                   [✗ Failed ]
[~] docs/README.md              [⚠1]    [⚠ Pending]
[~] config/app.toml                      [✓ OK     ]
```

### 3-2. ステータスバッジのデザイン

```rust
fn status_badge<'a>(status: AuditStatus) -> Element<'a, Message> {
    let (icon, label, color) = match status {
        AuditStatus::Ok      => ("✓", t!("status.ok"),      theme::OK_COLOR),
        AuditStatus::Pending => ("⚠", t!("status.pending"), theme::PENDING_COLOR),
        AuditStatus::Failed  => ("✗", t!("status.failed"),  theme::FAILED_COLOR),
        AuditStatus::Error   => ("!", t!("status.error"),   theme::ERROR_COLOR),
        AuditStatus::Ignored => ("—", t!("status.ignored"), theme::IGNORED_COLOR),
    };
    container(
        row![
            text(icon).size(10),
            text(label.to_string()).size(10),
        ]
        .spacing(3)
        .align_y(iced::Alignment::Center)
    )
    .padding(Padding::from([2.0, 5.0]))
    .style(move |_| iced::widget::container::Style {
        background: Some(iced::Background::Color(color.with_alpha(0.15))),
        border: iced::Border { color, width: 1.0, radius: 3.0.into() },
        ..Default::default()
    })
    .into()
}
```

- 幅は 72px 固定（英語・日本語で同じ幅になるよう最長テキストに合わせる）
- アイコンと短縮テキストの組み合わせで色なしでも判別可能
- `--no-unicode` 対応（フラグではなく locale で自動切替）

### 3-3. ツールバー verdict バッジの変更

現状:
```
[PASSED] OK: 3  Pending: 1  Failed: 1  Error: 0
```

変更後:
```
[✓ PASSED]  ✓ 3  ⚠ 1  ✗ 1  ! 0
```

記号をカウント数の前置きに使い、色なしでも解読可能にする。

### 3-4. diff type バッジの記号整理

現状の diff_icon と ABDD 対応版:

| diff_type | 現状 | ABDD |
|---|---|---|
| Added | `+` | `+` |
| Removed | `−` | `−` |
| Modified | `~` | `~` |
| TypeChanged | `T` | `T` |
| Unreadable | `!` | `!` |
| Incomparable | `?` | `?` |
| Unchanged | ` ` | ` ` |

diff type バッジ自体の記号は変更しない（意味が明確なため）。
ただし色は status badge に委ね、diff badge は中立色（グレー系）に統一する。

### 3-5. i18n キー追加

```yaml
status:
  ok:      "OK"
  pending: "Pending"
  failed:  "Failed"
  error:   "Error"
  ignored: "Ignored"
```

## データモデル

データモデルの変更なし。`AuditStatus` は既存のものをそのまま使用。

## 実装方針

1. `main_view.rs` の `build_file_row()` に `status_badge()` を追加
2. 行の右端に右揃えで配置（`space().width(Length::Fill)` を挟む）
3. `colored_badge()` は diff type バッジ用として残すが、status 系は `status_badge()` に分離
4. ツールバーの verdict バッジ文字列生成を更新

## 代替案

**A. ツールチップのみでテキスト表示**: ホバー必要でキーボード・スクリーンリーダー非対応。却下。  
**B. 専用の status 列を追加**: 情報密度が上がりすぎる。行内バッジが適切。

## Open Questions

- ステータスバッジの幅（72px）はペインが狭い場合に省略表示が必要かもしれない。
  `Length::Shrink` + 最小幅保証で対応する方向で検討。
- Unchanged エントリは現状デフォルト非表示。ABDD 観点では「表示できるが判別できる」ことが重要。
  フィルター再設計は RFC 003 の範囲外とする。
