# Visual Verification Card Template

This file is the canonical template for the **Visual Verification** section
that RFC 017 (Visual Verification Harness) requires at the end of each
shipped RFC under `rfcs/done/`.

## How to use

1. Pick the RFC you want to verify (start with the unverified list from
   `scripts/list-unverified-rfcs.sh`).
2. Build and run the relevant aaai surface (GUI or CLI) at the version
   listed in the RFC's release row.
3. Compare the observed behaviour against the design document
   `aaai_uiux_design.pdf` (specifically the pages cited in the RFC).
4. Copy the **Card** block below to the very end of the RFC file.
5. Fill in the metadata, the `Checks` rows, and any notes.
6. If you took screenshots, save them under `rfcs/verification/<NNN>/`
   (this directory is `.gitignore`d on purpose — only the card itself
   is committed).
7. Commit the change. The card's presence is what makes the script
   stop listing the RFC as UNVERIFIED.

## Conventions

- **Verdict column** uses `✅` (matches design), `❌` (does not match —
  follow-up RFC needed), or `例外` followed by a one-line reason
  (acceptable deviation, e.g. platform-specific rendering difference).
- **Date format** is `YYYY-MM-DD`.
- **Build identifier** is the version tag plus the short git SHA, e.g.
  `v0.20.0 (git: a1b2c3d)`. If verifying a pre-release build, use the
  rc tag or the dev SHA.
- **One card per RFC.** If a later release changes the surface, append
  a new card below the older one rather than overwriting; the file then
  documents how the surface evolved.

## Card

Copy everything from the line starting with `## Visual Verification`
to the end of this file, paste it at the end of the target RFC, and
fill in the placeholders.

---

## Visual Verification

**Verified.** YYYY-MM-DD by <verifier name or handle>
**Build.** vX.Y.Z (git: <short-sha>)
**Platform.** <Ubuntu 24.04 / macOS 14 / Windows 11>
**Locale.** <en / ja>

### Checks

| 設計書参照 | 期待 | 実観測 | 判定 |
|---|---|---|---|
| p.<N> <領域名> | <期待される挙動・配置・文言> | <実際に見えたもの> | ✅ / ❌ / 例外: <理由> |
| ... | ... | ... | ... |

### Notes

- (任意) スクリーンショットの保管先: `rfcs/verification/<NNN>/<filename>.png`
- (任意) 例外が含まれる場合は、解消用の follow-up RFC 番号 (なければ "TBD") を併記する
- (任意) 他プラットフォームでの追加検証予定があればここに記録する
