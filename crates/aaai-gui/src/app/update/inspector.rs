use super::super::*;

impl App {
    pub(in crate::app) fn reason_changed(&mut self, s: String) {
        self.inspector.reason = s;
        // Keep reason_content in sync when set programmatically
        self.inspector.reason_content =
            iced::widget::text_editor::Content::with_text(&self.inspector.reason);
        self.validate_inspector();
    }

    pub(in crate::app) fn reason_action(&mut self, action: iced::widget::text_editor::Action) {
        // RFC 009: multi-line text editor for reason field
        self.inspector.reason_content.perform(action);
        self.inspector.reason = self
            .inspector
            .reason_content
            .text()
            .trim_end_matches('\n')
            .to_string();
        self.validate_inspector();
    }

    pub(in crate::app) fn note_changed(&mut self, s: String) {
        self.inspector.note = s;
    }

    pub(in crate::app) fn strategy_selected(&mut self, kind: StrategyKind) {
        // RFC 035 — payload is already `StrategyKind`; construct the
        // default `AuditStrategy` for that variant.
        self.inspector.strategy_kind = kind;
        self.inspector.strategy = kind.to_default_strategy();
        self.validate_inspector();
    }

    pub(in crate::app) fn checksum_changed(&mut self, s: String) {
        if let AuditStrategy::Checksum { expected_sha256 } = &mut self.inspector.strategy {
            *expected_sha256 = s;
        }
        self.validate_inspector();
    }

    pub(in crate::app) fn regex_pattern_changed(&mut self, s: String) {
        if let AuditStrategy::Regex { pattern, .. } = &mut self.inspector.strategy {
            *pattern = s;
        }
        self.validate_inspector();
    }

    pub(in crate::app) fn regex_target_changed(&mut self, new_target: RegexTarget) {
        // RFC 033 — payload is already `RegexTarget`; no string parsing needed.
        if let AuditStrategy::Regex { target, .. } = &mut self.inspector.strategy {
            *target = new_target;
        }
    }

    pub(in crate::app) fn add_line_rule(&mut self) {
        if let AuditStrategy::LineMatch { rules } = &mut self.inspector.strategy {
            rules.push(LineRule {
                action: LineAction::Added,
                line: String::new(),
            });
        }
    }

    pub(in crate::app) fn edit_rule(&mut self, idx: usize) {
        // RFC 012: toggle rule edit mode
        self.inspector.editing_rule = if self.inspector.editing_rule == Some(idx) {
            None
        } else {
            Some(idx)
        };
    }

    #[allow(clippy::collapsible_if)]
    pub(in crate::app) fn remove_line_rule(&mut self, i: usize) {
        if let AuditStrategy::LineMatch { rules } = &mut self.inspector.strategy {
            if i < rules.len() {
                rules.remove(i);
            }
        }
        self.validate_inspector();
    }

    #[allow(clippy::collapsible_if)]
    pub(in crate::app) fn line_rule_action_changed(&mut self, i: usize, new_action: LineAction) {
        // RFC 033 — payload is already `LineAction`; no string parsing
        // and no silent-drop on unknown variant.
        if let AuditStrategy::LineMatch { rules } = &mut self.inspector.strategy {
            if let Some(r) = rules.get_mut(i) {
                r.action = new_action;
            }
        }
    }

    #[allow(clippy::collapsible_if)]
    pub(in crate::app) fn line_rule_line_changed(&mut self, i: usize, s: String) {
        if let AuditStrategy::LineMatch { rules } = &mut self.inspector.strategy {
            if let Some(r) = rules.get_mut(i) {
                r.line = s;
            }
        }
        self.validate_inspector();
    }

    pub(in crate::app) fn exact_content_changed(&mut self, s: String) {
        if let AuditStrategy::Exact { expected_content } = &mut self.inspector.strategy {
            *expected_content = s;
        }
        self.validate_inspector();
    }

    pub(in crate::app) fn ticket_changed(&mut self, s: String) {
        self.inspector.ticket = s;
    }

    pub(in crate::app) fn approved_by_changed(&mut self, s: String) {
        self.inspector.approved_by = s;
    }

    pub(in crate::app) fn expires_at_changed(&mut self, s: String) {
        self.inspector.expires_at_str = s;
    }

    pub(in crate::app) fn apply_template(&mut self, id: String) {
        use aaai::templates::library as tmpl;
        if let Some(t) = tmpl::find(&id) {
            self.inspector.strategy = (t.strategy)();
            self.inspector.strategy_kind = StrategyKind::from_strategy(&self.inspector.strategy);
            self.validate_inspector();
        }
    }

    pub(in crate::app) fn toggle_advanced_inspector(&mut self) {
        self.advanced_inspector_expanded = !self.advanced_inspector_expanded;
    }

    pub(in crate::app) fn toggle_use_pattern(&mut self) {
        self.inspector.use_pattern = !self.inspector.use_pattern;
        if self.inspector.use_pattern {
            // RFC 055 — populate suggestions from the current path
            self.inspector.pattern_suggestions =
                App::suggest_patterns(&self.inspector.pattern_path);
            self.validate_pattern();
        } else {
            self.inspector.pattern_suggestions.clear();
            self.inspector.validation.pattern_error = None;
        }
    }

    pub(in crate::app) fn pattern_changed(&mut self, s: String) {
        self.inspector.pattern_path = s;
        self.validate_pattern();
    }

    pub(in crate::app) fn apply_pattern_suggestion(&mut self, s: String) {
        self.inspector.pattern_path = s;
        self.validate_pattern();
    }

    /// RFC 002: per-field real-time validation for the inspector.
    pub(in crate::app) fn validate_inspector(&mut self) {
        use aaai::config::definition::AuditStrategy;
        let ins = &self.inspector;
        let mut v = InspectorValidation::default();

        // Reason (required)
        if ins.reason.trim().is_empty() {
            // RFC 031 — i18n'd.
            v.reason_error = Some(t!("error.inspector.reason_required.message").to_string());
        }

        // ExpiresAt format
        if !ins.expires_at_str.trim().is_empty() {
            if chrono::NaiveDate::parse_from_str(&ins.expires_at_str, "%Y-%m-%d").is_err() {
                // RFC 031 — i18n-migrated; this was the last
                // hardcoded user-facing string in app.rs.
                v.expires_at_error =
                    Some(t!("error.inspector.expires_at_format.message").to_string());
            }
        }

        // Strategy-specific validation
        match &ins.strategy {
            AuditStrategy::Checksum { expected_sha256 } => {
                let s = expected_sha256.trim();
                if s.len() != 64 || !s.chars().all(|c| c.is_ascii_hexdigit()) {
                    // RFC 030 — message + actionable hint. The
                    // raw "64 hex chars" message assumes SHA-256
                    // familiarity; the hint says where the value
                    // comes from for users new to the tool.
                    let err = crate::error::UserError::from_i18n("error.inspector.invalid_sha256");
                    v.strategy_errors.push(FieldError {
                        field: "expected_sha256".into(),
                        message: err.message,
                        hint: Some(err.hint),
                    });
                }
            }
            AuditStrategy::LineMatch { rules } => {
                if rules.is_empty() {
                    // RFC 030 — message + actionable hint. New
                    // users may not realise the `+ Add rule`
                    // button below the empty rules list is where
                    // they go next.
                    let err = crate::error::UserError::from_i18n("error.inspector.empty_rules");
                    v.strategy_errors.push(FieldError {
                        field: "rules".into(),
                        message: err.message,
                        hint: Some(err.hint),
                    });
                }
                for (i, rule) in rules.iter().enumerate() {
                    if rule.line.trim().is_empty() {
                        // RFC 029 — hint stays None: the message ("rule line
                        // cannot be empty") already points at the action.
                        v.strategy_errors.push(FieldError {
                            field: format!("rule[{}].line", i),
                            message: t!("error.inspector.empty_rule_line.message").to_string(),
                            hint: None,
                        });
                    }
                }
            }
            AuditStrategy::Regex { pattern, .. } => {
                if let Err(e) = RegexCheck::new(pattern) {
                    // RFC 028 — hint is now a structural field, not
                    // composed into the message. The message line
                    // carries the localized description + the concrete
                    // syntax error from the regex compiler; the hint
                    // line carries the actionable next step
                    // (i.e. "test at regex101.com").
                    let err = crate::error::UserError::from_i18n("error.inspector.invalid_regex");
                    v.strategy_errors.push(FieldError {
                        field: "pattern".into(),
                        message: format!("{} ({})", err.message, e),
                        hint: Some(err.hint),
                    });
                }
            }
            AuditStrategy::Exact { expected_content } => {
                if expected_content.trim().is_empty() {
                    // RFC 029.
                    v.strategy_errors.push(FieldError {
                        field: "expected_content".into(),
                        message: t!("error.inspector.empty_expected.message").to_string(),
                        hint: None,
                    });
                }
            }
            AuditStrategy::None => {}
        }

        self.inspector.validation = v;
    }

    /// RFC 075 — pre-select the most helpful strategy for a given diff type.
    /// Applied only to new (unapproved) entries so it doesn't override
    /// existing user choices. `pub` so the inspector view can read it.
    pub(crate) fn recommended_strategy(diff_type: DiffType) -> StrategyKind {
        match diff_type {
            DiffType::Modified => StrategyKind::LineMatch,
            _ => StrategyKind::None,
        }
    }

    /// RFC 055 — derive up to 3 glob suggestions from a concrete path.
    pub(in crate::app) fn suggest_patterns(path: &str) -> Vec<String> {
        let parts: Vec<&str> = path.split('/').collect();
        let mut seen = std::collections::HashSet::new();
        let mut out: Vec<String> = Vec::new();

        let mut push = |s: String| {
            if seen.insert(s.clone()) && out.len() < 3 {
                out.push(s);
            }
        };

        // Extension (last component after last '.', not in a dir name)
        let ext: Option<&str> = path
            .rsplit('.')
            .next()
            .filter(|e| !e.contains('/') && !e.is_empty() && e.len() <= 6);

        if parts.len() >= 2 {
            push(format!("{}/**", parts[0]));
            if let Some(e) = ext {
                push(format!("{}/**/*.{}", parts[0], e));
            }
        }
        if let Some(e) = ext {
            push(format!("**/*.{}", e));
        }
        out
    }

    /// RFC 054 — validate the glob pattern in the Inspector.
    pub(in crate::app) fn validate_pattern(&mut self) {
        if !self.inspector.use_pattern {
            self.inspector.validation.pattern_error = None;
            return;
        }
        let pat = self.inspector.pattern_path.trim();
        if pat.is_empty() {
            self.inspector.validation.pattern_error =
                Some(t!("inspector.pattern_empty").to_string());
        } else if glob::Pattern::new(pat).is_err() {
            self.inspector.validation.pattern_error =
                Some(t!("inspector.pattern_invalid").to_string());
        } else {
            self.inspector.validation.pattern_error = None;
        }
    }
}
