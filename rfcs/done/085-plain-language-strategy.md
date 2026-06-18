# RFC 085 — Plain-language strategy labels

**Status.** Implemented (v0.32.0 — Phase 24)
**Tracks.** Plain-Language GUI (review §3.1, §6)
**Touches.** `locales/{en,ja}.yaml` — inspector + batch strategy keys.

Content Audit Strategy → "How should aaai check this?"; None → "Only
that it changed"; Checksum → File fingerprint; LineMatch → Specific
line changes; Regex → Text pattern; Exact → Exact text. StrategyKind
enum and validation unchanged (LocalizedOption display/value separation).
