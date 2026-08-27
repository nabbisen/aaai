use super::super::*;

impl App {
    #[allow(clippy::needless_return)]
    pub(in crate::app) fn rerun_audit(&mut self) -> Task<Message> {
        // RFC 037 — converted to async; toast fires from RerunDiffReady.
        return self.start_async_rerun();
    }

    pub(in crate::app) fn diff_loading(&mut self, msg: String) {
        self.load_progress = Some(msg);
    }

    pub(in crate::app) fn diff_ready(
        &mut self,
        diffs: Vec<aaai::DiffEntry>,
        definition: aaai::AuditDefinition,
        ignore: IgnoreRules,
    ) -> Task<Message> {
        self.is_loading = false;
        self.load_progress = None;
        // (Re)initialize 3-pane layout: FileTree | Diff | Inspector
        let (pane_state, pane_file_tree) = pane_grid::State::new(PaneKind::FileTree);
        self.panes = pane_state;
        // Split FileTree | right-column (Diff + Inspector)
        if let Some((right_pane, _)) =
            self.panes
                .split(pane_grid::Axis::Vertical, pane_file_tree, PaneKind::Diff)
        {
            // Split Diff | Inspector
            let _ = self
                .panes
                .split(pane_grid::Axis::Vertical, right_pane, PaneKind::Inspector);
        }
        let result = aaai::AuditEngine::evaluate(&diffs, &definition);
        self.diffs = diffs;
        self.audit_result = Some(result);
        self.definition = Some(definition);
        self.active_ignore = ignore;
        self.screen = Screen::Main;
        self.selected_index = None;
        self.dirty = false;

        // RFC 052 — auto-select the first Pending entry so the user
        // can start approving immediately without clicking in the tree.
        let first_pending: Option<usize> = self.audit_result.as_ref().and_then(|r| {
            r.results
                .iter()
                .enumerate()
                .find(|(_, far)| far.status == AuditStatus::Pending)
                .map(|(idx, _)| idx)
        });
        if let Some(idx) = first_pending {
            return Task::perform(async move { idx }, Message::SelectEntry);
        }
        Task::none()
    }

    pub(in crate::app) fn diff_failed(&mut self, err: String) {
        self.is_loading = false;
        self.load_progress = None;
        self.open_error = Some(crate::error::UserError::new(
            t!("error.diff.failed.message", reason = err),
            t!("error.diff.failed.hint"),
        ));
    }

    pub(in crate::app) fn rerun_diff_ready(
        &mut self,
        result: Result<Vec<aaai::DiffEntry>, String>,
    ) {
        self.is_loading = false;
        self.load_progress = None;
        match result {
            Ok(fresh_diffs) => {
                self.diffs = fresh_diffs;
                if let Some(def) = self.definition.clone() {
                    let audit_result = AuditEngine::evaluate(&self.diffs, &def);
                    // Record this run in history, matching CLI behaviour.
                    let before = PathBuf::from(&self.before_path);
                    let after = PathBuf::from(&self.after_path);
                    let defn = if self.definition_path.is_empty() {
                        None
                    } else {
                        Some(PathBuf::from(&self.definition_path))
                    };
                    let record =
                        HistoryRecord::new(&before, &after, defn.as_deref(), &audit_result.summary);
                    if let Err(e) = history_store::append(&record) {
                        log::warn!("Could not write history: {e}");
                    }
                    self.audit_result = Some(audit_result);
                }
                self.audit_dirty = false;
                self.push_toast(
                    ToastIntent::Info,
                    t!("toast.rerun").as_ref(),
                    t!("toast.rerun_complete").as_ref(),
                );
            }
            Err(e) => {
                self.push_toast(ToastIntent::Error, t!("toast.rerun").as_ref(), &e);
            }
        }
    }
}
