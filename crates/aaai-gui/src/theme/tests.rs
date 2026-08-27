use super::*;
use iced::Color;

fn luminance(c: Color) -> f32 {
    fn lin(ch: f32) -> f32 {
        if ch <= 0.03928 {
            ch / 12.92
        } else {
            ((ch + 0.055) / 1.055).powf(2.4)
        }
    }
    0.2126 * lin(c.r) + 0.7152 * lin(c.g) + 0.0722 * lin(c.b)
}
fn contrast(a: Color, b: Color) -> f32 {
    let (l1, l2) = (luminance(a), luminance(b));
    let (hi, lo) = if l1 > l2 { (l1, l2) } else { (l2, l1) };
    (hi + 0.05) / (lo + 0.05)
}
const WHITE: Color = Color {
    r: 1.0,
    g: 1.0,
    b: 1.0,
    a: 1.0,
};

/// All standard-theme status colors must meet WCAG AA (≥ 4.5:1).
#[test]
fn standard_status_colors_meet_aa() {
    let tokens = snora::design::Tokens::light();
    let statuses = [
        AuditStatus::Ok,
        AuditStatus::Pending,
        AuditStatus::Failed,
        AuditStatus::Error,
        AuditStatus::Ignored,
    ];
    for s in statuses {
        let c = status_color(s, &tokens, false);
        let r = contrast(c, WHITE);
        assert!(r >= 4.5, "{s:?} standard contrast {r:.2}:1 < 4.5:1");
    }
}

/// All HC-theme status colors must meet ≥ 7:1.
#[test]
fn hc_status_colors_meet_enhanced_contrast() {
    let tokens = snora::design::Tokens::high_contrast_light();
    let statuses = [
        AuditStatus::Ok,
        AuditStatus::Pending,
        AuditStatus::Failed,
        AuditStatus::Error,
        AuditStatus::Ignored,
    ];
    for s in statuses {
        let c = status_color(s, &tokens, true);
        let r = contrast(c, WHITE);
        assert!(r >= 7.0, "{s:?} HC contrast {r:.2}:1 < 7:1");
    }
}

/// Light-theme status colors are pixel-identical to the v0.35.0 constants.
#[test]
fn light_theme_pixels_identical_to_v035_constants() {
    let tokens = snora::design::Tokens::light();
    // v0.35.0 constants (from snora-design light palette)
    let v035 = [
        (
            AuditStatus::Ok,
            Color {
                r: 0.082353,
                g: 0.501961,
                b: 0.239216,
                a: 1.0,
            },
        ),
        (
            AuditStatus::Pending,
            Color {
                r: 0.603922,
                g: 0.356863,
                b: 0.000000,
                a: 1.0,
            },
        ),
        (
            AuditStatus::Failed,
            Color {
                r: 0.701961,
                g: 0.149020,
                b: 0.117647,
                a: 1.0,
            },
        ),
        (AuditStatus::Error, ERROR_COLOR),
        (AuditStatus::Ignored, IGNORED_COLOR),
    ];
    for (s, expected) in v035 {
        let got = status_color(s, &tokens, false);
        let diff = (got.r - expected.r)
            .abs()
            .max((got.g - expected.g).abs())
            .max((got.b - expected.b).abs());
        assert!(
            diff < 1e-5,
            "{s:?}: got ({:.6},{:.6},{:.6}) expected ({:.6},{:.6},{:.6})",
            got.r,
            got.g,
            got.b,
            expected.r,
            expected.g,
            expected.b
        );
    }
}

/// Sanity check for the contrast helper.
#[test]
fn contrast_helper_black_on_white_is_21() {
    let black = Color::BLACK;
    let r = contrast(black, WHITE);
    assert!((r - 21.0).abs() < 0.1, "expected ~21:1 got {r:.2}");
}
