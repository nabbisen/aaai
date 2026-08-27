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
}
