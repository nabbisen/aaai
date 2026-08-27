use super::super::*;

impl App {
    pub(in crate::app) fn select_entry(&mut self, idx: usize) {
        self.selected_index = Some(idx);
        if let Some(far) = self.audit_result.as_ref().and_then(|r| r.results.get(idx)) {
            self.inspector = if let Some(entry) = &far.entry {
                InspectorState {
                    reason: entry.reason.clone(),
                    editing_rule: None,
                    reason_content: iced::widget::text_editor::Content::with_text(&entry.reason),
                    strategy_kind: StrategyKind::from_strategy(&entry.strategy),
                    strategy: entry.strategy.clone(),
                    note: entry.note.clone().unwrap_or_default(),
                    validation: InspectorValidation::default(),
                    ticket: entry.ticket.clone().unwrap_or_default(),
                    approved_by: entry.approved_by.clone().unwrap_or_default(),
                    expires_at_str: entry.expires_at.map(|d| d.to_string()).unwrap_or_default(),
                    // RFC 054 — reset pattern toggle; seed path from diff
                    use_pattern: false,
                    pattern_path: far.diff.path.clone(),
                    pattern_suggestions: Vec::new(),
                }
            } else {
                // RFC 075 — pre-select the recommended strategy for this
                // diff type so newcomers don't face a blank "None" with
                // no guidance. The user can always change it.
                let rec = App::recommended_strategy(far.diff.diff_type);
                InspectorState {
                    strategy_kind: rec,
                    strategy: rec.to_default_strategy(),
                    use_pattern: false,
                    pattern_path: far.diff.path.clone(),
                    pattern_suggestions: Vec::new(),
                    ..InspectorState::default()
                }
            };
        }
    }

    pub(in crate::app) fn set_filter(&mut self, f: FilterMode) {
        self.filter_mode = f;
        self.selected_index = None;
    }

    pub(in crate::app) fn search_query_changed(&mut self, s: String) {
        self.search_query = s;
    }

    pub(in crate::app) fn toggle_dir(&mut self, dir: String) {
        if self.collapsed_dirs.contains(&dir) {
            self.collapsed_dirs.remove(&dir);
        } else {
            self.collapsed_dirs.insert(dir);
        }
    }

    pub(in crate::app) fn pane_resized(&mut self, e: pane_grid::ResizeEvent) {
        self.panes.resize(e.split, e.ratio);
    }

    pub(in crate::app) fn pane_focused(&mut self, p: pane_grid::Pane) {
        self.focus = Some(p);
    }

    pub(in crate::app) fn noop(&mut self) {}

    pub(in crate::app) fn deselect_entry(&mut self) {
        self.selected_index = None;
    }

    pub(in crate::app) fn set_diff_view_mode(&mut self, mode: DiffViewMode) {
        self.diff_view_mode = mode;
    }

    pub(in crate::app) fn back_to_opening(&mut self) {
        if self.dirty {
            // RFC 041 — open confirmation dialog instead of passive toast.
            self.nav_guard_open = true;
            self.nav_guard_show_discard = false; // RFC 086 — start hidden
        } else {
            self.do_leave_to_opening();
        }
    }

    pub(in crate::app) fn focus_next(&mut self) {
        self.focus_target = match self.focus_target {
            FocusTarget::FileTree => FocusTarget::Inspector,
            FocusTarget::Inspector => FocusTarget::FileTree,
            FocusTarget::Search => FocusTarget::FileTree,
        };
    }

    pub(in crate::app) fn focus_prev(&mut self) {
        self.focus_target = match self.focus_target {
            FocusTarget::FileTree => FocusTarget::Inspector,
            FocusTarget::Inspector => FocusTarget::FileTree,
            FocusTarget::Search => FocusTarget::Inspector,
        };
    }

    pub(in crate::app) fn focus_search(&mut self) {
        // RFC 005: update logical focus; visual focus ring shown by search input
        self.focus_target = FocusTarget::Search;
    }

    pub(in crate::app) fn focus_inspector_reason(&mut self) {
        // RFC 005: update logical focus; inspector reason input highlighted
        self.focus_target = FocusTarget::Inspector;
    }

    #[allow(clippy::needless_return)]
    pub(in crate::app) fn diff_before_scrolled(
        &mut self,
        vp: iced::widget::scrollable::Viewport,
    ) -> Task<Message> {
        if self.diff_scroll_syncing {
            self.diff_scroll_syncing = false;
            return Task::none();
        }
        self.diff_scroll_syncing = true;
        let abs = vp.absolute_offset();
        return iced::widget::operation::scroll_to(
            crate::views::diff_view::DIFF_AFTER_ID.clone(),
            iced::widget::operation::AbsoluteOffset {
                x: Some(abs.x),
                y: Some(abs.y),
            },
        );
    }

    #[allow(clippy::needless_return)]
    pub(in crate::app) fn diff_after_scrolled(
        &mut self,
        vp: iced::widget::scrollable::Viewport,
    ) -> Task<Message> {
        if self.diff_scroll_syncing {
            self.diff_scroll_syncing = false;
            return Task::none();
        }
        self.diff_scroll_syncing = true;
        let abs = vp.absolute_offset();
        return iced::widget::operation::scroll_to(
            crate::views::diff_view::DIFF_BEFORE_ID.clone(),
            iced::widget::operation::AbsoluteOffset {
                x: Some(abs.x),
                y: Some(abs.y),
            },
        );
    }
}
