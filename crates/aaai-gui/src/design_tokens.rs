//! Resolves an [`aaai::profile::prefs::Theme`] preference to a
//! [`snora::design::Tokens`] bundle (RFC 092, extended by RFC 094).
//!
//! The bundle is stored in [`crate::app::App::design_tokens`] and updated
//! whenever the theme choice or the OS light/dark mode changes. Every view
//! function that needs token-driven styling reads it from `app.design_tokens`.

use aaai::profile::prefs::Theme as AppTheme;
use snora::design::Tokens;

/// The minimum pointer-target size every snora control takes from us, in
/// logical pixels (RFC 116 §S1.3; WCAG 2.5.5 and the ABDD common rule,
/// `docs/src/abdd-audit.md` §6). snora ships `Tokens::targets.min` at 24 and
/// documents raising it to 44; it is our token, not snora's guarantee.
pub const TARGET_MIN_PX: f32 = 44.0;

/// A concrete theme: what is actually drawn. `Theme::System` is a *choice*,
/// never a preset — [`resolve`] turns it into one of these.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Preset {
    Light,
    Dark,
    HighContrastLight,
    HighContrastDark,
}

/// RFC 116 §S2 — the one place that matches on `Theme::System`.
///
/// `choice` is what the user picked; `os` is what the desktop reports.
/// `System` follows the OS: dark when the OS says dark, light otherwise (light
/// or "no preference"). Every other choice is used as it is. **High contrast
/// is never inferred**: the desktop portal reports light or dark only.
///
/// Both consumers go through here — [`tokens_for`] (aaai's own widgets) and
/// [`iced_theme_for`] (iced's window chrome) — so the two cannot disagree.
pub fn resolve(choice: &AppTheme, os: iced::theme::Mode) -> Preset {
    match choice {
        AppTheme::Light => Preset::Light,
        AppTheme::Dark => Preset::Dark,
        AppTheme::System => match os {
            iced::theme::Mode::Dark => Preset::Dark,
            iced::theme::Mode::Light | iced::theme::Mode::None => Preset::Light,
        },
        AppTheme::HighContrastLight => Preset::HighContrastLight,
        AppTheme::HighContrastDark => Preset::HighContrastDark,
    }
}

/// Return the resolved [`Tokens`] for a choice under the given OS mode.
pub fn tokens_for(choice: &AppTheme, os: iced::theme::Mode) -> Tokens {
    let mut tokens = match resolve(choice, os) {
        Preset::Light => Tokens::light(),
        Preset::Dark => Tokens::dark(),
        Preset::HighContrastLight => Tokens::high_contrast_light(),
        Preset::HighContrastDark => Tokens::high_contrast_dark(),
    };
    tokens.targets.min = TARGET_MIN_PX;
    tokens
}

/// The iced base theme for window chrome and built-in widgets. High-contrast
/// presets use the matching iced base; aaai's own widgets get the
/// high-contrast snora tokens (RFC 092).
pub fn iced_theme_for(choice: &AppTheme, os: iced::theme::Mode) -> iced::Theme {
    match resolve(choice, os) {
        Preset::Light | Preset::HighContrastLight => iced::Theme::Light,
        Preset::Dark | Preset::HighContrastDark => iced::Theme::Dark,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use iced::theme::Mode;

    const CHOICES: [AppTheme; 5] = [
        AppTheme::Light,
        AppTheme::Dark,
        AppTheme::System,
        AppTheme::HighContrastLight,
        AppTheme::HighContrastDark,
    ];
    const MODES: [Mode; 3] = [Mode::None, Mode::Light, Mode::Dark];

    /// The guarantee that snora's controls (the toast close mark among them)
    /// reach 44 px is snora's; the token that asks for it is ours, so the token
    /// is what is asserted — for every preset a user can choose.
    #[test]
    fn every_preset_asks_snora_for_44px_targets() {
        for choice in CHOICES {
            for os in MODES {
                let min = tokens_for(&choice, os).targets.min;
                assert!(
                    min >= 44.0,
                    "{choice:?}/{os:?}: targets.min is {min}, want >= 44.0"
                );
            }
        }
    }

    #[test]
    fn a_concrete_choice_ignores_the_os() {
        for os in MODES {
            assert_eq!(resolve(&AppTheme::Light, os), Preset::Light);
            assert_eq!(resolve(&AppTheme::Dark, os), Preset::Dark);
            assert_eq!(
                resolve(&AppTheme::HighContrastLight, os),
                Preset::HighContrastLight
            );
            assert_eq!(
                resolve(&AppTheme::HighContrastDark, os),
                Preset::HighContrastDark
            );
        }
    }

    #[test]
    fn system_follows_the_os_and_defaults_to_light() {
        assert_eq!(resolve(&AppTheme::System, Mode::Dark), Preset::Dark);
        assert_eq!(resolve(&AppTheme::System, Mode::Light), Preset::Light);
        assert_eq!(resolve(&AppTheme::System, Mode::None), Preset::Light);
    }

    #[test]
    fn system_never_resolves_to_high_contrast() {
        for os in MODES {
            assert!(!matches!(
                resolve(&AppTheme::System, os),
                Preset::HighContrastLight | Preset::HighContrastDark
            ));
        }
    }

    /// iced's chrome and aaai's widgets must be drawn from the same preset for
    /// the same inputs, or a dark window frame surrounds Light widgets.
    #[test]
    fn iced_theme_and_tokens_resolve_to_the_same_preset() {
        for choice in CHOICES {
            for os in MODES {
                let dark = matches!(
                    resolve(&choice, os),
                    Preset::Dark | Preset::HighContrastDark
                );
                assert_eq!(
                    iced_theme_for(&choice, os) == iced::Theme::Dark,
                    dark,
                    "{choice:?}/{os:?}: iced theme disagrees with the preset"
                );
                let mut expected = match resolve(&choice, os) {
                    Preset::Light => Tokens::light(),
                    Preset::Dark => Tokens::dark(),
                    Preset::HighContrastLight => Tokens::high_contrast_light(),
                    Preset::HighContrastDark => Tokens::high_contrast_dark(),
                };
                expected.targets.min = TARGET_MIN_PX;
                assert_eq!(
                    tokens_for(&choice, os),
                    expected,
                    "{choice:?}/{os:?}: tokens are not the resolved preset's"
                );
            }
        }
    }
}
