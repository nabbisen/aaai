//! RFC 115 §S2 — every interactive control the views build reaches the ABDD
//! 44 px target size (`docs/src/abdd-audit.md` §6) on both axes.
//!
//! # What is asserted
//!
//! For every `button`, `text_input`, `text_editor` and `pick_list` the views
//! construct, in every one of the four theme presets:
//!
//! * **height** `>= 44 px`. Exact: `text size x line height + padding`, with
//!   the padding and text size **read from the view source** by
//!   [`scan`] and evaluated against the live design tokens. Nothing about a
//!   control's height is copied into this file, so a padding change in a view,
//!   or a spacing/typography change in snora, changes this result.
//! * **width** `>= 44 px`, except where the control is `Length::Fill` (its
//!   width is the layout's, not its own). A content-sized control's width is
//!   `horizontal padding + an estimate of its label`. The estimate is a
//!   **floor**, not a measurement: no font is loaded in a test, so each
//!   character is counted at 0.5 em (1.0 em for wide CJK characters), over
//!   the *shortest* of the English and Japanese labels. Height is the axis the
//!   rule turns on; treat a width failure that sits on the estimate as "not
//!   proven", and the height failures as measured.
//!
//! # Why the sizes are read from source
//!
//! A table of numbers kept beside the views goes stale the moment a view's
//! padding changes — which is exactly how `docs/src/abdd-audit.md` §6 went
//! blank. Reading the source closes that. The cost is a scanner that knows
//! this code base's conventions; it errors loudly on anything it cannot
//! evaluate, and a second test fails when a control is added or removed
//! without being registered below.
//!
//! # Limits, stated rather than hidden
//!
//! * A control's *content* height is read from the text sized inside its own
//!   constructor. Where a row is built elsewhere and passed in (the file-tree
//!   row), the registry names the text role instead; it is a lower bound.
//! * Padding written by something other than a literal chain call (a helper
//!   returning a `Padding`, say) is an error, not a silent skip.
//! * Widgets that are interactive but not built through the four
//!   constructors above (for example a snora prefab) are not seen by the
//!   scan. snora's own controls take their minimum from the `targets.min`
//!   token, which `design_tokens` sets and asserts; anything else a
//!   dependency builds is listed as an external exemption by hand.
//!
//! # Adding or moving a control — ids are ordinal
//!
//! A control's id is `file::enclosing_fn#n`, where `n` counts the interactive
//! widgets in that function from the top. **Inserting a control means
//! re-checking every later entry in that function:** each of their ids shifts
//! by one, and with it the width metadata and the `KNOWN_DEBT` record attached
//! to the old id. `registry_matches_source` catches most shifts because each
//! entry also records the widget `Kind` the scan must find under its id; a
//! shift between two controls of the same kind it cannot see, so re-read the
//! entries, do not just append one.
//!
//! # Known debt — a ratchet, not an exemption
//!
//! [`KNOWN_DEBT`] lists the controls that were below 44 px when this check
//! landed, each with its worst measured height. It exists so the rule is
//! enforced *now* for every new control while the old ones are fixed by RFC
//! 116, the rescoped RFC 106 and RFC 115 §S2b. It **may only shrink**: a listed
//! control that gets worse, or that reaches 44 px and is not removed, fails.
//!
//! # Exemptions
//!
//! Each carries its reason. "It currently fails" is never one — that is what
//! known debt is for, and debt is recorded, measured and scheduled.

mod scan;

use std::collections::BTreeMap;
use std::path::Path;

use aaai::profile::prefs::Theme;
use scan::{Kind, Site, measure};
use snora::design::Tokens;

/// The ABDD common rule, `docs/src/abdd-audit.md` §6 (WCAG 2.5.5, AAA).
const TARGET_PX: f32 = 44.0;

/// Locales whose labels bound a control's width from below.
const LOCALES: [&str; 2] = ["en", "ja"];

/// How a control's width is established.
enum Width {
    /// `Length::Fill`: set by the layout. Recorded so the axis is visibly
    /// accounted for rather than silently skipped.
    Fill,
    /// Content-sized: an optional leading glyph plus the shortest localized
    /// label among `keys`.
    Label {
        glyphs: &'static str,
        keys: &'static [&'static str],
    },
    /// Content is data the registry cannot name (a path suggestion, a chosen
    /// option); counted at `min_glyphs` characters.
    Dynamic { min_glyphs: usize },
}

struct Entry {
    id: &'static str,
    /// The kind of widget the scan must find under `id`. Ids are ordinal
    /// within a function, so a control inserted above this one shifts every
    /// later id; a changed kind is how `registry_matches_source` notices.
    kind: Kind,
    width: Width,
    /// Text role to use when the constructor's arguments hold no sized text.
    content_role: Option<&'static str>,
    /// `Some(reason)` removes the control from the assertion.
    exempt: Option<&'static str>,
}

const fn e(id: &'static str, kind: Kind, width: Width) -> Entry {
    Entry {
        id,
        kind,
        width,
        content_role: None,
        exempt: None,
    }
}

const fn label(
    id: &'static str,
    kind: Kind,
    glyphs: &'static str,
    keys: &'static [&'static str],
) -> Entry {
    e(id, kind, Width::Label { glyphs, keys })
}

const BACKDROP: &str = "Full-window click-to-dismiss backdrop (`Length::Fill` on both axes behind a modal). \
It is not a pointer target of a size anyone aims at; the dialog's own Close / Cancel buttons are the \
targets, and they are registered and asserted individually.";

/// Every interactive widget the scan finds, with how its width is established.
/// The scan must find exactly these (see `registry_matches_source`).
const REGISTRY: &[Entry] = &[
    // ── app/view.rs ─────────────────────────────────────────────────────
    Entry {
        exempt: Some(BACKDROP),
        ..e("app/view.rs::view#1", Kind::MouseArea, Width::Fill)
    },
    Entry {
        exempt: Some(BACKDROP),
        ..e("app/view.rs::view#2", Kind::MouseArea, Width::Fill)
    },
    Entry {
        exempt: Some(BACKDROP),
        ..e("app/view.rs::view#3", Kind::MouseArea, Width::Fill)
    },
    label(
        "app/view.rs::view_footer#1",
        Kind::Button,
        "?",
        &["help.button_label", "settings.button_tooltip"],
    ),
    // ── views/dashboard.rs ──────────────────────────────────────────────
    label(
        "views/dashboard.rs::view#1",
        Kind::Button,
        "",
        &["toolbar.report_output"],
    ),
    label(
        "views/dashboard.rs::view#2",
        Kind::Button,
        "",
        &["dashboard.new_audit"],
    ),
    // ── views/diff_view.rs ──────────────────────────────────────────────
    label(
        "views/diff_view.rs::build_tab_bar#1",
        Kind::Button,
        "",
        &[
            "diff.tab_side_by_side",
            "diff.tab_unified",
            "diff.tab_changed_only",
        ],
    ),
    // ── views/help_overlay.rs ───────────────────────────────────────────
    label(
        "views/help_overlay.rs::view#1",
        Kind::Button,
        "",
        &["help.close"],
    ),
    // ── views/inspector.rs ──────────────────────────────────────────────
    e("views/inspector.rs::view#1", Kind::TextEditor, Width::Fill),
    label(
        "views/inspector.rs::view#2",
        Kind::Button,
        "▸ ",
        &["inspector.use_pattern"],
    ),
    e("views/inspector.rs::view#3", Kind::TextInput, Width::Fill),
    e(
        "views/inspector.rs::view#4",
        Kind::Button,
        Width::Dynamic { min_glyphs: 1 },
    ),
    e("views/inspector.rs::view#5", Kind::TextInput, Width::Fill),
    e("views/inspector.rs::view#6", Kind::TextInput, Width::Fill),
    e("views/inspector.rs::view#7", Kind::TextInput, Width::Fill),
    label(
        "views/inspector.rs::view#8",
        Kind::PickList,
        "",
        &[
            "inspector.strategy_none",
            "inspector.strategy_checksum",
            "inspector.strategy_linematch",
            "inspector.strategy_regex",
            "inspector.strategy_exact",
        ],
    ),
    label(
        "views/inspector.rs::view#9",
        Kind::PickList,
        "",
        &["inspector.template_none"],
    ),
    e("views/inspector.rs::view#10", Kind::TextInput, Width::Fill),
    label(
        "views/inspector.rs::view#11",
        Kind::Button,
        "▸ ",
        &[
            "inspector.advanced_toggle_show",
            "inspector.advanced_toggle_hide",
        ],
    ),
    label(
        "views/inspector.rs::view#12",
        Kind::Button,
        "",
        &["inspector.revert_to_pending"],
    ),
    e(
        "views/inspector.rs::build_strategy_form#1",
        Kind::TextInput,
        Width::Fill,
    ),
    label(
        "views/inspector.rs::build_strategy_form#2",
        Kind::PickList,
        "",
        &["inspector.action_added", "inspector.action_removed"],
    ),
    e(
        "views/inspector.rs::build_strategy_form#3",
        Kind::TextInput,
        Width::Fill,
    ),
    label(
        "views/inspector.rs::build_strategy_form#4",
        Kind::Button,
        "✕",
        &[],
    ),
    label(
        "views/inspector.rs::build_strategy_form#5",
        Kind::Button,
        "✓",
        &[],
    ),
    e(
        "views/inspector.rs::build_strategy_form#6",
        Kind::Button,
        Width::Fill,
    ),
    label(
        "views/inspector.rs::build_strategy_form#7",
        Kind::Button,
        "",
        &["inspector.add_rule"],
    ),
    e(
        "views/inspector.rs::build_strategy_form#8",
        Kind::TextInput,
        Width::Fill,
    ),
    label(
        "views/inspector.rs::build_strategy_form#9",
        Kind::PickList,
        "",
        &[
            "inspector.target_added_lines",
            "inspector.target_removed_lines",
            "inspector.target_all_changed_lines",
        ],
    ),
    e(
        "views/inspector.rs::build_strategy_form#10",
        Kind::TextInput,
        Width::Fill,
    ),
    // ── views/main_view.rs ──────────────────────────────────────────────
    label(
        "views/main_view.rs::build_toolbar#1",
        Kind::Button,
        "←",
        &[
            "toolbar.open",
            "toolbar.save",
            "toolbar.run_audit",
            "toolbar.report_output",
            "toolbar.undo",
        ],
    ),
    label(
        "views/main_view.rs::build_filter_bar#1",
        Kind::Button,
        "",
        &[
            "filter.all",
            "filter.changed",
            "filter.pending",
            "filter.errors",
        ],
    ),
    label(
        "views/main_view.rs::build_filter_bar#2",
        Kind::Button,
        "?",
        &[],
    ),
    e(
        "views/main_view.rs::build_search_bar#1",
        Kind::TextInput,
        Width::Fill,
    ),
    label(
        "views/main_view.rs::build_file_tree#1",
        Kind::Button,
        "",
        &["main.coach_dismiss"],
    ),
    e(
        "views/main_view.rs::build_file_tree#2",
        Kind::Button,
        Width::Fill,
    ),
    Entry {
        content_role: Some("body_small"),
        ..e(
            "views/main_view.rs::build_file_row#1",
            Kind::Button,
            Width::Fill,
        )
    },
    label(
        "views/main_view.rs::build_bottom_bar#1",
        Kind::Button,
        "",
        &["bottombar.approve_and_save"],
    ),
    // ── views/nav_guard.rs ──────────────────────────────────────────────
    label(
        "views/nav_guard.rs::view#1",
        Kind::Button,
        "",
        &["nav_guard.cancel"],
    ),
    label(
        "views/nav_guard.rs::view#2",
        Kind::Button,
        "",
        &["nav_guard.save_and_leave"],
    ),
    label(
        "views/nav_guard.rs::view#3",
        Kind::Button,
        "",
        &["nav_guard.discard_and_leave"],
    ),
    label(
        "views/nav_guard.rs::view#4",
        Kind::Button,
        "",
        &["nav_guard.more_choices"],
    ),
    // ── views/opening.rs ────────────────────────────────────────────────
    label(
        "views/opening.rs::view#1",
        Kind::Button,
        "",
        &["opening.start_button"],
    ),
    label(
        "views/opening.rs::folder_picker_card#1",
        Kind::Button,
        "",
        &["opening.change_folder", "opening.pick_folder"],
    ),
    label(
        "views/opening.rs::optional_settings_section#1",
        Kind::Button,
        "▸ ",
        &["opening.optional_section"],
    ),
    e(
        "views/opening.rs::file_picker_row#1",
        Kind::TextInput,
        Width::Fill,
    ),
    label(
        "views/opening.rs::file_picker_row#2",
        Kind::Button,
        "",
        &["opening.pick_file"],
    ),
    label(
        "views/opening.rs::recent_projects_section#1",
        Kind::Button,
        "",
        &["opening.open_recent"],
    ),
    label(
        "views/opening.rs::recent_projects_section#2",
        Kind::Button,
        "×",
        &[],
    ),
    // ── views/settings_dialog.rs ────────────────────────────────────────
    e(
        "views/settings_dialog.rs::view#1",
        Kind::PickList,
        Width::Dynamic { min_glyphs: 7 },
    ),
    e(
        "views/settings_dialog.rs::view#2",
        Kind::TextInput,
        Width::Fill,
    ),
    label("views/settings_dialog.rs::view#3", Kind::Button, "×", &[]),
    label(
        "views/settings_dialog.rs::view#4",
        Kind::Button,
        "",
        &["settings.add_dir"],
    ),
    label(
        "views/settings_dialog.rs::view#5",
        Kind::Button,
        "",
        &["settings.cancel"],
    ),
    label(
        "views/settings_dialog.rs::view#6",
        Kind::Button,
        "",
        &["settings.save"],
    ),
    e(
        "views/settings_dialog.rs::view#7",
        Kind::PickList,
        Width::Fill,
    ),
];

/// **Known debt, not exemption.** Controls that were below 44 px when this
/// check landed (RFC 115 §S2), each with its **worst computed height across
/// the four presets**, in pixels. A listed control is excused on both axes (all
/// eight width shortfalls are also height shortfalls).
///
/// The list is a ratchet and **may only shrink**. `every_control_reaches_44px`
/// fails when
///
/// * a control that is not listed and not exempt is below 44 px — new controls
///   must meet the rule from the day they are written;
/// * a listed control is now *lower* than recorded — it got worse;
/// * a listed control now reaches 44 px on both axes — a stale entry, delete
///   it, so the list cannot outlive the fix;
/// * a listed id is not in [`REGISTRY`].
///
/// An exemption says "this is not a target"; debt says "this is a target and
/// it is too small". Remediation is scheduled: RFC 116 (snora 0.57
/// `Tokens::targets.min`), the rescoped RFC 106, and RFC 115 §S2b. Do not add
/// to this list to make a test pass; fix the control.
const KNOWN_DEBT: &[(&str, f32)] = &[
    ("views/dashboard.rs::view#1", 32.8),
    ("views/dashboard.rs::view#2", 32.8),
    ("views/diff_view.rs::build_tab_bar#1", 42.8),
    ("views/help_overlay.rs::view#1", 32.8),
    ("views/inspector.rs::build_strategy_form#1", 36.8),
    ("views/inspector.rs::build_strategy_form#10", 36.8),
    ("views/inspector.rs::build_strategy_form#2", 28.8),
    ("views/inspector.rs::build_strategy_form#3", 36.8),
    ("views/inspector.rs::build_strategy_form#4", 24.8),
    ("views/inspector.rs::build_strategy_form#5", 24.8),
    ("views/inspector.rs::build_strategy_form#7", 24.8),
    ("views/inspector.rs::build_strategy_form#8", 36.8),
    ("views/inspector.rs::build_strategy_form#9", 28.8),
    ("views/inspector.rs::view#10", 36.8),
    ("views/inspector.rs::view#11", 24.8),
    ("views/inspector.rs::view#12", 32.8),
    ("views/inspector.rs::view#2", 24.8),
    ("views/inspector.rs::view#3", 34.9),
    ("views/inspector.rs::view#4", 24.8),
    ("views/inspector.rs::view#5", 36.8),
    ("views/inspector.rs::view#6", 36.8),
    ("views/inspector.rs::view#7", 36.8),
    ("views/inspector.rs::view#8", 36.8),
    ("views/inspector.rs::view#9", 22.3),
    ("views/main_view.rs::build_bottom_bar#1", 40.8),
    ("views/main_view.rs::build_file_row#1", 26.9),
    ("views/main_view.rs::build_file_tree#1", 24.8),
    ("views/main_view.rs::build_file_tree#2", 24.8),
    ("views/main_view.rs::build_filter_bar#1", 40.8),
    ("views/main_view.rs::build_filter_bar#2", 32.8),
    ("views/main_view.rs::build_search_bar#1", 26.9),
    ("views/main_view.rs::build_toolbar#1", 40.8),
    ("views/nav_guard.rs::view#1", 32.8),
    ("views/nav_guard.rs::view#2", 32.8),
    ("views/nav_guard.rs::view#3", 32.8),
    ("views/nav_guard.rs::view#4", 32.8),
    ("views/opening.rs::file_picker_row#1", 34.9),
    ("views/opening.rs::file_picker_row#2", 32.8),
    ("views/opening.rs::folder_picker_card#1", 40.8),
    ("views/opening.rs::optional_settings_section#1", 32.8),
    ("views/opening.rs::recent_projects_section#1", 32.8),
    ("views/opening.rs::recent_projects_section#2", 32.8),
    ("views/opening.rs::view#1", 40.8),
    ("views/settings_dialog.rs::view#1", 24.9),
    ("views/settings_dialog.rs::view#2", 26.9),
    ("views/settings_dialog.rs::view#3", 24.8),
    ("views/settings_dialog.rs::view#4", 24.8),
    ("views/settings_dialog.rs::view#5", 32.8),
    ("views/settings_dialog.rs::view#6", 32.8),
    ("views/settings_dialog.rs::view#7", 30.8),
];

/// Controls that are interactive and ours to the user, but built by a
/// dependency, so the scan cannot see them and we cannot change them. Each is
/// `(name, width, height, reason)`.
///
/// **Empty since RFC 116.** The one entry — snora's toast dismiss `×`, 24 x 24
/// under snora 0.38 — went when we moved to snora 0.57, which sizes it (and
/// every other snora control) from `Tokens::targets.min`. That token is ours,
/// and `design_tokens::tests::every_preset_asks_snora_for_44px_targets` asserts
/// it. Keep the list for the next control a dependency builds that no token
/// reaches; do not delete the check that every entry states a reason.
const EXTERNAL_EXEMPTIONS: &[(&str, f32, f32, &str)] = &[];

fn presets() -> [(&'static str, Tokens); 4] {
    [
        (
            "light",
            crate::design_tokens::tokens_for(&Theme::Light, iced::theme::Mode::None),
        ),
        (
            "dark",
            crate::design_tokens::tokens_for(&Theme::Dark, iced::theme::Mode::None),
        ),
        (
            "high_contrast_light",
            crate::design_tokens::tokens_for(&Theme::HighContrastLight, iced::theme::Mode::None),
        ),
        (
            "high_contrast_dark",
            crate::design_tokens::tokens_for(&Theme::HighContrastDark, iced::theme::Mode::None),
        ),
    ]
}

fn src_dir() -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("src")
}

fn registry() -> BTreeMap<&'static str, &'static Entry> {
    REGISTRY.iter().map(|x| (x.id, x)).collect()
}

/// Wide (East Asian / emoji) characters take a full em; the rest half.
fn advance_em(c: char) -> f32 {
    match c as u32 {
        0x1100..=0x115F
        | 0x2E80..=0xA4CF
        | 0xAC00..=0xD7A3
        | 0xF900..=0xFAFF
        | 0xFE30..=0xFE6F
        | 0xFF00..=0xFF60
        | 0xFFE0..=0xFFE6
        | 0x1F300..=0x1FAFF => 1.0,
        _ => 0.5,
    }
}

fn floor_width(text: &str, font: f32) -> f32 {
    text.chars().map(|c| advance_em(c) * font).sum()
}

/// Estimated content width, or `None` when the control's width is the layout's.
fn width_estimate(entry: &Entry, m: &scan::Measure) -> Option<f32> {
    if m.fill_width {
        return None;
    }
    let content = match &entry.width {
        Width::Fill => return None,
        Width::Dynamic { min_glyphs } => *min_glyphs as f32 * 0.5 * m.font,
        Width::Label { glyphs, keys } => {
            let glyph_w = floor_width(glyphs, m.font);
            if keys.is_empty() {
                glyph_w
            } else {
                let narrowest = keys
                    .iter()
                    .flat_map(|k| {
                        LOCALES
                            .iter()
                            .map(move |loc| floor_width(&rust_i18n::t!(*k, locale = *loc), m.font))
                    })
                    .fold(f32::INFINITY, f32::min);
                glyph_w + narrowest
            }
        }
    };
    Some(m.pad_h + content)
}

struct Row {
    id: String,
    line: usize,
    kind: Kind,
    height: f32,
    width: Option<f32>,
}

/// Measure every non-exempt site under one preset. Errors name the site.
fn measure_all(name: &str, t: &Tokens, sites: &[Site]) -> Result<Vec<Row>, Vec<String>> {
    let reg = registry();
    let (mut rows, mut errors) = (Vec::new(), Vec::new());
    for s in sites {
        let Some(entry) = reg.get(s.id.as_str()) else {
            errors.push(format!("[{name}] {}: not in the registry", s.id));
            continue;
        };
        if entry.exempt.is_some() {
            continue;
        }
        match measure(s, t, entry.content_role) {
            Ok(m) => rows.push(Row {
                id: s.id.clone(),
                line: s.line,
                kind: s.kind,
                height: m.height,
                width: width_estimate(entry, &m),
            }),
            Err(e) => errors.push(format!("[{name}] {e}")),
        }
    }
    if errors.is_empty() {
        Ok(rows)
    } else {
        Err(errors)
    }
}

/// The scan finds exactly the controls the registry lists — no more, no fewer —
/// each of the kind the registry says it is, and every exemption states a
/// reason.
#[test]
fn registry_matches_source() {
    let sites = scan::scan_tree(&src_dir());
    let reg = registry();

    let mut problems = Vec::new();
    for s in &sites {
        match reg.get(s.id.as_str()) {
            None => problems.push(format!("control in source but not registered: {}", s.id)),
            // Ids are ordinal within a function: a control inserted above this
            // one moves every later id by one, which usually changes the kind
            // found under an id. Re-check every later entry in that function.
            Some(x) if x.kind != s.kind => problems.push(format!(
                "{} is a {:?} in source (line {}) but registered as a {:?}; ids are ordinal \
                 within a function, so a control was probably inserted or removed above it \
                 — re-check every later entry in that function, including its KNOWN_DEBT record",
                s.id, s.kind, s.line, x.kind
            )),
            Some(_) => {}
        }
    }
    for x in REGISTRY {
        if !sites.iter().any(|s| s.id == x.id) {
            problems.push(format!("registered but not found in source: {}", x.id));
        }
    }
    let mut seen = std::collections::BTreeSet::new();
    for x in REGISTRY {
        if !seen.insert(x.id) {
            problems.push(format!("registered twice: {}", x.id));
        }
    }
    for x in REGISTRY {
        if let Some(reason) = x.exempt
            && reason.trim().len() < 40
        {
            problems.push(format!("exemption without a real reason: {}", x.id));
        }
    }
    for (name, _, _, reason) in EXTERNAL_EXEMPTIONS {
        if reason.trim().len() < 40 {
            problems.push(format!("external exemption without a real reason: {name}"));
        }
    }
    assert!(problems.is_empty(), "{}", problems.join("\n"));
}

/// Worst size of one control across the four presets.
struct Worst {
    line: usize,
    kind: Kind,
    height: f32,
    width: Option<f32>,
}

impl Worst {
    fn short(&self) -> bool {
        self.height < TARGET_PX - 0.01
    }
    fn narrow(&self) -> bool {
        self.width.is_some_and(|w| w < TARGET_PX - 0.01)
    }
}

/// Apply the known-debt ratchet to measured sizes. Returns
/// `(problems, new_failures)`; both empty means the check passes.
fn ratchet(
    worst: &BTreeMap<String, Worst>,
    known_debt: &[(&str, f32)],
    registered: &[&str],
) -> (Vec<String>, Vec<String>) {
    let debt: BTreeMap<&str, f32> = known_debt.iter().copied().collect();
    let mut problems = Vec::new();

    for (id, _) in known_debt {
        if !registered.contains(id) {
            problems.push(format!("KNOWN_DEBT lists {id}, which is not in REGISTRY"));
        }
    }
    if debt.len() != known_debt.len() {
        problems.push("KNOWN_DEBT lists a control twice".to_string());
    }

    let describe = |id: &str, w: &Worst| {
        let w_txt = w.width.map_or("fill".to_string(), |w| format!("{w:.1}"));
        let flag_h = if w.short() { "HEIGHT" } else { "      " };
        let flag_w = if w.narrow() { "WIDTH(est)" } else { "" };
        format!(
            "  {id} (line {}, {:?}): {:.1} high x {w_txt} wide  {flag_h} {flag_w}",
            w.line, w.kind, w.height
        )
    };

    let mut new_failures = Vec::new();
    for (id, w) in worst {
        match debt.get(id.as_str()) {
            None if w.short() || w.narrow() => new_failures.push(describe(id, w)),
            None => {}
            Some(&recorded) => {
                // Recorded heights are rounded to 0.1 px.
                if w.height < recorded - 0.06 {
                    problems.push(format!(
                        "{id} got worse: {:.1} px high, recorded as {recorded:.1} in KNOWN_DEBT",
                        w.height
                    ));
                } else if !w.short() && !w.narrow() {
                    problems.push(format!(
                        "{id} now reaches {TARGET_PX} px on both axes: delete it from KNOWN_DEBT \
                         (the list may only shrink)"
                    ));
                }
            }
        }
    }
    (problems, new_failures)
}

/// Every interactive control reaches 44 px on both axes in every preset —
/// except the controls recorded in [`KNOWN_DEBT`], which may only improve.
///
/// A failure lists each control that does not, with its computed size. Do not
/// lower `TARGET_PX`, add an exemption, or add to `KNOWN_DEBT` to make this
/// pass: fix the control, or record a reasoned exemption in the registry.
#[test]
fn every_control_reaches_44px() {
    let sites = scan::scan_tree(&src_dir());
    let mut worst: BTreeMap<String, Worst> = BTreeMap::new();
    let mut errors = Vec::new();

    for (name, tokens) in presets() {
        match measure_all(name, &tokens, &sites) {
            Err(es) => errors.extend(es),
            Ok(rows) => {
                for r in rows {
                    // Keep the worst size seen across presets, per axis.
                    let slot = worst.entry(r.id.clone()).or_insert(Worst {
                        line: r.line,
                        kind: r.kind,
                        height: r.height,
                        width: r.width,
                    });
                    slot.height = slot.height.min(r.height);
                    slot.width = match (slot.width, r.width) {
                        (Some(a), Some(b)) => Some(a.min(b)),
                        (a, b) => a.or(b),
                    };
                }
            }
        }
    }
    assert!(
        errors.is_empty(),
        "unreadable controls:\n{}",
        errors.join("\n")
    );

    let registered: Vec<&str> = REGISTRY.iter().map(|x| x.id).collect();
    let (problems, new_failures) = ratchet(&worst, KNOWN_DEBT, &registered);

    let mut report = problems.join("\n");
    if !new_failures.is_empty() {
        report.push_str(&format!(
            "\n{} control(s) are below {TARGET_PX} px on at least one axis and are not in \
             KNOWN_DEBT (height is exact; width is a floor estimate):\n{}\n",
            new_failures.len(),
            new_failures.join("\n"),
        ));
    }
    assert!(
        report.trim().is_empty(),
        "{report}\n\
         known debt: {} control(s); exempt, by reason: {} dependency control(s), {} backdrop(s)",
        KNOWN_DEBT.len(),
        EXTERNAL_EXEMPTIONS.len(),
        REGISTRY.iter().filter(|x| x.exempt.is_some()).count(),
    );
}

#[cfg(test)]
mod ratchet_tests {
    use super::*;

    fn w(height: f32, width: Option<f32>) -> Worst {
        Worst {
            line: 1,
            kind: Kind::Button,
            height,
            width,
        }
    }

    fn run(
        measured: &[(&str, f32, Option<f32>)],
        debt: &[(&str, f32)],
    ) -> (Vec<String>, Vec<String>) {
        let worst = measured
            .iter()
            .map(|(id, h, wd)| (id.to_string(), w(*h, *wd)))
            .collect();
        let registered: Vec<&str> = measured.iter().map(|m| m.0).collect();
        ratchet(&worst, debt, &registered)
    }

    #[test]
    fn a_new_small_control_fails() {
        let (problems, fresh) = run(&[("a", 30.0, None)], &[]);
        assert!(problems.is_empty());
        assert_eq!(fresh.len(), 1, "an unlisted 30 px control must be reported");
    }

    #[test]
    fn a_new_narrow_control_fails() {
        let (_, fresh) = run(&[("a", 48.0, Some(20.0))], &[]);
        assert_eq!(fresh.len(), 1);
    }

    #[test]
    fn a_listed_control_that_got_worse_fails() {
        let (problems, fresh) = run(&[("a", 24.0, None)], &[("a", 30.0)]);
        assert!(fresh.is_empty());
        assert!(problems.iter().any(|p| p.contains("got worse")));
    }

    #[test]
    fn a_listed_control_that_improved_but_is_still_short_passes() {
        let (problems, fresh) = run(&[("a", 40.0, None)], &[("a", 30.0)]);
        assert!(problems.is_empty() && fresh.is_empty());
    }

    #[test]
    fn a_listed_control_at_its_recorded_height_passes_despite_rounding() {
        // Recorded to 0.1; the true value may differ by up to 0.05.
        let (problems, _) = run(&[("a", 24.76, None)], &[("a", 24.8)]);
        assert!(problems.is_empty());
    }

    #[test]
    fn a_stale_entry_fails() {
        let (problems, _) = run(&[("a", 44.0, Some(60.0))], &[("a", 30.0)]);
        assert!(problems.iter().any(|p| p.contains("delete it")));
    }

    #[test]
    fn a_listed_id_missing_from_the_registry_fails() {
        let (problems, _) = run(&[], &[("gone", 30.0)]);
        assert!(problems.iter().any(|p| p.contains("not in REGISTRY")));
    }

    #[test]
    fn a_met_control_not_on_the_list_passes() {
        let (problems, fresh) = run(&[("a", 48.8, Some(70.0))], &[]);
        assert!(problems.is_empty() && fresh.is_empty());
    }
}
