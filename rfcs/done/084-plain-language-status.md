# RFC 084 — Plain-language status labels and hints

**Status.** Implemented (v0.32.0 — Phase 24)
**Tracks.** Plain-Language GUI (review §3.1, §3.2)
**Touches.** `locales/{en,ja}.yaml` — status, toolbar, filter, main sections.

OK → All set; Pending → Needs review; Failed → Doesn't match;
Error → Couldn't check; Ignored → Skipped. Overall verdict badge:
Passed → All set, Failed → Needs attention. Status legend reworded to
match. The AuditStatus enum and CLI/report output are unchanged — only
GUI display strings differ (design doc p.9: shared judgment vocabulary
preserved at the model level).
