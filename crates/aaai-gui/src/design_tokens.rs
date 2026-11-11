//! Resolves an [`aaai::profile::prefs::Theme`] preference to a
//! [`snora::design::Tokens`] bundle (RFC 092, extended by RFC 094).
//!
//! The bundle is stored in [`crate::app::App::design_tokens`] and updated
//! whenever the theme preference changes via `Message::SetTheme`. Every view
//! function that needs token-driven styling reads it from `app.design_tokens`.

use aaai::profile::prefs::Theme as AppTheme;
use snora::design::Tokens;

/// The minimum pointer-target size every snora control takes from us, in
/// logical pixels (RFC 116 §S1.3; WCAG 2.5.5 and the ABDD common rule,
/// `docs/src/abdd-audit.md` §6). snora ships `Tokens::targets.min` at 24 and
/// documents raising it to 44; it is our token, not snora's guarantee.
pub const TARGET_MIN_PX: f32 = 44.0;

/// Return the resolved [`Tokens`] for the given [`AppTheme`].
///
/// `System` falls back to the light preset until iced 0.14 exposes a reliable
/// OS dark-mode query (RFC 093 §5.1).
pub fn tokens_for(theme: &AppTheme) -> Tokens {
    let mut tokens = match theme {
        AppTheme::Light => Tokens::light(),
        AppTheme::Dark => Tokens::dark(),
        AppTheme::System => Tokens::light(),
        AppTheme::HighContrastLight => Tokens::high_contrast_light(),
        AppTheme::HighContrastDark => Tokens::high_contrast_dark(),
    };
    tokens.targets.min = TARGET_MIN_PX;
    tokens
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The guarantee that snora's controls (the toast close mark among them)
    /// reach 44 px is snora's; the token that asks for it is ours, so the token
    /// is what is asserted — for every preset a user can choose.
    #[test]
    fn every_preset_asks_snora_for_44px_targets() {
        for theme in [
            AppTheme::Light,
            AppTheme::Dark,
            AppTheme::System,
            AppTheme::HighContrastLight,
            AppTheme::HighContrastDark,
        ] {
            let min = tokens_for(&theme).targets.min;
            assert!(min >= 44.0, "{theme:?}: targets.min is {min}, want >= 44.0");
        }
    }
}
