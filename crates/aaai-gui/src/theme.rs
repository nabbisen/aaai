//! Status colors and the token-aware status-color resolver for aaai GUI.
//!
//! # WCAG AA / AAA compliance
//!
//! Every color in this module is verified to meet WCAG 2.1 AA contrast (≥ 4.5:1)
//! against white in both directions it is used:
//!   - as a **badge background** with white text (inspector / dashboard badges)
//!   - as **text foreground** on a white surface (status pill, diff lines)
//!
//! High-contrast variants reach ≥ 7:1 (approaching WCAG AAA) as required by
//! RFC 094.
//!
//! # Approach 1 (RFC 094 §3.7)
//!
//! OK / Pending / Failed colors are read live from `tokens.palette` so they
//! follow the active snora-design preset (including the HC presets). Error and
//! Ignored have no snora palette role; they are hand-picked below.
//!
//! The pixel-identical invariant for Light/Dark (RFC 094 §5.1) holds because
//! `Tokens::light().palette.success` is exactly the same float triple as the
//! v0.35.0 `OK_COLOR` constant: the snora light preset is the source of both.
//!
//! | Color | Standard | HC variant | Provenance |
//! |---|---|---|---|
//! | OK      | token `success`  | token `success` HC  | snora-design |
//! | Pending | token `warning`  | token `warning` HC  | snora-design |
//! | Failed  | token `danger`   | token `danger` HC   | snora-design |
//! | Error   | `#B22EB2` 5.33:1 | `#7B1F7B` 9.04:1   | hand-picked  |
//! | Ignored | `#6B6B6B` 5.32:1 | `#525252` 7.81:1   | hand-picked  |
//! | Added   | token `success`  | token `success` HC  | snora-design |
//! | Removed | token `danger`   | token `danger` HC   | snora-design |

use aaai::AuditStatus;
use iced::Color;
use snora::design::Tokens;

// ── Hand-picked constants for roles not covered by snora-design ───────────

/// Standard error color — purple #B22EB2, 5.33:1 on white.
/// Kept distinct from FAILED so "couldn't read" reads differently from
/// "rule no longer matches" (design doc p.9 status-vocabulary distinction).
pub const ERROR_COLOR: Color = Color {
    r: 0.70,
    g: 0.18,
    b: 0.70,
    a: 1.0,
};

/// High-contrast error color — purple #7B1F7B, 9.04:1 on white.
pub const ERROR_HC: Color = Color {
    r: 0.482353,
    g: 0.121569,
    b: 0.482353,
    a: 1.0,
};

/// Standard ignored color — grey #6B6B6B, 5.32:1 on white.
pub const IGNORED_COLOR: Color = Color {
    r: 0.420000,
    g: 0.420000,
    b: 0.420000,
    a: 1.0,
};

/// High-contrast ignored color — grey #525252, 7.81:1 on white.
pub const IGNORED_HC: Color = Color {
    r: 0.321569,
    g: 0.321569,
    b: 0.321569,
    a: 1.0,
};

// ── Token-aware status color resolver ────────────────────────────────────

/// Convert a `snora_design::Color` to an `iced::Color`.
#[inline]
fn to_iced(c: snora::design::Color) -> Color {
    Color {
        r: c.r,
        g: c.g,
        b: c.b,
        a: c.a,
    }
}

/// Return the display color for an [`AuditStatus`], respecting the active
/// design-token preset.
///
/// For standard themes (Light / Dark), OK / Pending / Failed resolve to the
/// same values as the v0.35.0 hand-picked constants — pixel-identical, just
/// read from the token palette rather than a hard-coded literal. Under
/// high-contrast presets they escalate automatically to ≥ 8:1 values.
/// Error and Ignored fall back to hand-picked HC constants (no snora role).
/// `is_hc` is `app.theme.is_high_contrast()`.
pub fn status_color(status: AuditStatus, tokens: &Tokens, is_hc: bool) -> Color {
    match status {
        AuditStatus::Ok => to_iced(tokens.palette.success),
        AuditStatus::Pending => to_iced(tokens.palette.warning),
        AuditStatus::Failed => to_iced(tokens.palette.danger),
        AuditStatus::Error => {
            if is_hc {
                ERROR_HC
            } else {
                ERROR_COLOR
            }
        }
        AuditStatus::Ignored => {
            if is_hc {
                IGNORED_HC
            } else {
                IGNORED_COLOR
            }
        }
    }
}

/// Shorthand for diff-view added lines — same as OK / success.
pub fn added_color(tokens: &Tokens) -> Color {
    to_iced(tokens.palette.success)
}

/// Shorthand for diff-view removed lines — same as Failed / danger.
pub fn removed_color(tokens: &Tokens) -> Color {
    to_iced(tokens.palette.danger)
}

#[cfg(test)]
mod tests;
