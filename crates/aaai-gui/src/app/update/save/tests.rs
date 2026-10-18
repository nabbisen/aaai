//! RFC 104 §6 items 3–4 — the report written by the GUI export path is
//! masked, and is masked the same way `aaai report` masks it.
//!
//! These drive `App::write_report` directly. They deliberately build no
//! `App`: `App::default()` calls `UserPrefs::load()`, which reads the
//! operator's real `prefs.yaml` (RFC 104 §4.3, §7a).

use super::*;
use aaai::{
    AuditEntry, AuditStatus, AuditStrategy, DiffEntry, DiffType, ReportGenerator,
    audit::result::FileAuditResult,
};
use std::path::Path;

/// Matches the built-in "AWS access key" pattern
/// (`crates/aaai/src/masking/patterns.rs`). Taken from the engine's own
/// report guard, `crates/aaai/src/report/generator/tests.rs` (`CANARY`).
const CANARY: &str = "AKIAAAAAAAAAAAAAAAAA";

/// Matches only the project-level custom pattern written by
/// [`project_root_with_custom_pattern`]; no built-in pattern recognises it.
const CUSTOM_CANARY: &str = "INTERNAL-4711";

const REASON_PREFIX: &str = "legitimate-looking review note";

fn result_with_reason(reason: String) -> AuditResult {
    let diff = DiffEntry {
        path: "config/server.toml".into(),
        diff_type: DiffType::Modified,
        is_dir: false,
        before_text: None,
        after_text: None,
        is_binary: false,
        before_size: None,
        after_size: None,
        before_sha256: None,
        after_sha256: None,
        stats: None,
        error_detail: None,
    };
    let entry = AuditEntry {
        path: diff.path.clone(),
        diff_type: DiffType::Modified,
        reason,
        strategy: AuditStrategy::None,
        enabled: true,
        ticket: None,
        approved_by: None,
        approved_at: None,
        expires_at: None,
        note: None,
        created_at: None,
        updated_at: None,
    };
    AuditResult::new(vec![FileAuditResult {
        diff,
        entry: Some(entry),
        status: AuditStatus::Ok,
        detail: None,
        warnings: Vec::new(),
    }])
}

/// A before-root whose `.aaai.yaml` declares one custom mask pattern.
fn project_root_with_custom_pattern() -> tempfile::TempDir {
    let root = tempfile::tempdir().unwrap();
    std::fs::write(
        root.path().join(".aaai.yaml"),
        "version: \"1\"\ncustom_mask_patterns:\n  - \"INTERNAL-[0-9]{4}\"\n",
    )
    .unwrap();
    root
}

/// Replaces the report's generation timestamp, the only run-dependent bytes.
fn without_timestamps(text: &str) -> String {
    regex::Regex::new(r"\d{4}-\d{2}-\d{2}[T ]\d{2}:\d{2}:\d{2}")
        .unwrap()
        .replace_all(text, "<timestamp>")
        .into_owned()
}

fn export(result: &AuditResult, before: &Path, out: &Path) -> String {
    let masker = App::report_masker(before);
    App::write_report(
        result,
        before,
        before,
        None,
        out,
        aaai::Masking::Enabled(&masker),
    )
    .unwrap();
    std::fs::read_to_string(out).unwrap()
}

#[test]
fn markdown_export_does_not_contain_the_secret() {
    let dir = tempfile::tempdir().unwrap();
    let out = dir.path().join("report.md");
    let result = result_with_reason(format!("{REASON_PREFIX} {CANARY}"));

    let written = export(&result, dir.path(), &out);

    assert!(!written.contains(CANARY), "secret reached the written file");
    // Surrounding content is present, and the secret's position holds the
    // mask — a file that passes only because nothing was written would not.
    assert!(
        written.contains(&format!("{REASON_PREFIX} ***MASKED***")),
        "reason not rendered with its secret masked:\n{written}"
    );
}

#[test]
fn json_export_does_not_contain_the_secret() {
    let dir = tempfile::tempdir().unwrap();
    let out = dir.path().join("report.json");
    let result = result_with_reason(format!("{REASON_PREFIX} {CANARY}"));

    let written = export(&result, dir.path(), &out);

    assert!(!written.contains(CANARY), "secret reached the written file");
    assert!(
        written.contains(&format!("{REASON_PREFIX} ***MASKED***")),
        "reason not rendered with its secret masked:\n{written}"
    );
}

/// RFC 104 §6 item 4: the same input through the GUI export path and through
/// the sequence `aaai report` runs (`aaai-cli/src/cmd/report.rs:57-62`)
/// writes the same bytes — including a custom pattern that only the project
/// config, discovered relative to the before-root, supplies.
#[test]
fn gui_export_matches_cli_report_for_the_same_input() {
    let root = project_root_with_custom_pattern();
    let result = result_with_reason(format!("{REASON_PREFIX} {CANARY} {CUSTOM_CANARY}"));

    for (name, json) in [("report.md", false), ("report.json", true)] {
        let gui_out = root.path().join(format!("gui-{name}"));
        let gui = export(&result, root.path(), &gui_out);

        // The CLI sequence, spelled out rather than shared with the GUI.
        let custom_patterns = aaai::project::config::ProjectConfig::discover(root.path())
            .unwrap_or(None)
            .map(|(config, _)| config.custom_mask_patterns)
            .unwrap_or_default();
        let masker = aaai::MaskingEngine::with_custom(&custom_patterns);
        let masking = aaai::Masking::Enabled(&masker);
        let cli_out = root.path().join(format!("cli-{name}"));
        if json {
            ReportGenerator::write_json(&result, root.path(), root.path(), None, &cli_out, masking)
        } else {
            ReportGenerator::write_markdown(
                &result,
                root.path(),
                root.path(),
                None,
                &cli_out,
                masking,
            )
        }
        .unwrap();
        let cli = std::fs::read_to_string(&cli_out).unwrap();

        assert!(
            !gui.contains(CUSTOM_CANARY),
            "{name}: custom pattern missed"
        );
        assert!(!gui.contains(CANARY), "{name}: built-in pattern missed");
        assert_eq!(
            without_timestamps(&gui),
            without_timestamps(&cli),
            "{name}: GUI and CLI exports differ"
        );
    }
}
