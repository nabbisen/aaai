use super::super::*;

impl App {
    #[allow(clippy::needless_return)]
    pub(in crate::app) fn approve_and_save(&mut self) -> Task<Message> {
        // RFC 002: approve + save in one action.
        // Both sub-handlers currently return Task::none(); combine them into
        // one Task so that if either is ever made async the chain stays correct.
        let t1 = self.update(Message::ApproveEntry);
        let t2 = self.update(Message::SaveDefinition);
        return Task::batch([t1, t2]);
    }

    #[allow(clippy::collapsible_if)]
    pub(in crate::app) fn approve_entry(&mut self) -> Task<Message> {
        if let Some(idx) = self.selected_index {
            if let Some(far) = self.audit_result.as_ref().and_then(|r| r.results.get(idx)) {
                let expires_at = if self.inspector.expires_at_str.trim().is_empty() {
                    None
                } else {
                    chrono::NaiveDate::parse_from_str(
                        self.inspector.expires_at_str.trim(),
                        "%Y-%m-%d",
                    )
                    .ok()
                };
                let entry = AuditEntry {
                    path: far.diff.path.clone(),
                    diff_type: far.diff.diff_type,
                    reason: self.inspector.reason.trim().to_string(),
                    strategy: self.inspector.strategy.clone(),
                    enabled: true,
                    ticket: {
                        let t = self.inspector.ticket.trim().to_string();
                        if t.is_empty() { None } else { Some(t) }
                    },
                    approved_by: {
                        let a = self.inspector.approved_by.trim().to_string();
                        if a.is_empty() { None } else { Some(a) }
                    },
                    approved_at: Some(chrono::Utc::now()),
                    expires_at,
                    note: {
                        let n = self.inspector.note.trim().to_string();
                        if n.is_empty() { None } else { Some(n) }
                    },
                    created_at: None,
                    updated_at: None,
                };
                match entry.is_approvable() {
                    Ok(()) => {
                        // RFC 054: use pattern path if override is active
                        let entry_path = if self.inspector.use_pattern
                            && !self.inspector.pattern_path.trim().is_empty()
                        {
                            self.inspector.pattern_path.trim().to_string()
                        } else {
                            far.diff.path.clone()
                        };
                        let path = entry_path.clone();
                        if let Some(def) = &mut self.definition {
                            let mut stamped = AuditEntry {
                                path: entry_path,
                                ..entry
                            };
                            stamped.stamp_now();
                            let path_for_undo = stamped.path.clone();
                            def.upsert_entry(stamped);
                            self.undo_stack.push(path_for_undo);
                            if self.undo_stack.len() > 20 {
                                self.undo_stack.remove(0);
                            }
                            self.dirty = true;
                            self.audit_dirty = true;
                            self.push_toast(
                                ToastIntent::Success,
                                t!("toast.approved").as_ref(),
                                &path,
                            );

                            // RFC 050 — auto-advance to next Pending entry so
                            // the user can keep approving without manual navigation.
                            let approved_path = path.clone();
                            let next_pending: Option<usize> =
                                self.audit_result.as_ref().and_then(|result| {
                                    let n = result.results.len();
                                    let start = (idx + 1) % n;
                                    (0..n).map(|i| (start + i) % n).find(|&i| {
                                        let r = &result.results[i];
                                        r.status == AuditStatus::Pending
                                            && r.diff.path != approved_path
                                    })
                                });

                            let rerun = self.start_async_rerun();
                            return if let Some(next_idx) = next_pending {
                                Task::batch([
                                    rerun,
                                    Task::perform(async move { next_idx }, Message::SelectEntry),
                                ])
                            } else {
                                rerun
                            };
                        }
                    }
                    Err(e) => {
                        self.inspector.validation.strategy_errors.push(FieldError {
                            field: "expires_at".into(),
                            message: e,
                            hint: None,
                        });
                    }
                }
            }
        }
        Task::none()
    }

    #[allow(clippy::collapsible_if)]
    pub(in crate::app) fn undo_approval(&mut self) -> Task<Message> {
        if let Some(path) = self.undo_stack.pop() {
            if let Some(def) = &mut self.definition {
                if let Some(idx) = def.entries.iter().position(|e| e.path == path) {
                    def.entries.remove(idx);
                    self.dirty = true;
                    self.audit_dirty = true;
                    self.push_toast(
                        ToastIntent::Info,
                        t!("toast.undo").as_ref(),
                        t!("toast.removed_approval", path = path.clone()).as_ref(),
                    );
                    // RFC 037 — async rerun.
                    return self.start_async_rerun();
                }
            }
        } else {
            self.push_toast(
                ToastIntent::Info,
                t!("toast.undo").as_ref(),
                t!("toast.nothing_to_undo").as_ref(),
            );
        }
        Task::none()
    }

    pub(in crate::app) fn select_next(&mut self) -> Task<Message> {
        if let Some(result) = &self.audit_result {
            let visible: Vec<usize> = result
                .results
                .iter()
                .enumerate()
                .filter(|(_, r)| {
                    self.filter_mode.passes(r)
                        && r.diff.diff_type != aaai::DiffType::Unchanged
                        && (self.search_query.is_empty()
                            || r.diff
                                .path
                                .to_lowercase()
                                .contains(&self.search_query.to_lowercase()))
                })
                .map(|(i, _)| i)
                .collect();
            if !visible.is_empty() {
                let next = match self.selected_index {
                    None => visible[0],
                    Some(cur) => {
                        let pos = visible.iter().position(|&i| i == cur).unwrap_or(0);
                        visible[(pos + 1) % visible.len()]
                    }
                };
                return self.update(Message::SelectEntry(next));
            }
        }
        Task::none()
    }

    pub(in crate::app) fn select_prev(&mut self) -> Task<Message> {
        if let Some(result) = &self.audit_result {
            let visible: Vec<usize> = result
                .results
                .iter()
                .enumerate()
                .filter(|(_, r)| {
                    self.filter_mode.passes(r)
                        && r.diff.diff_type != aaai::DiffType::Unchanged
                        && (self.search_query.is_empty()
                            || r.diff
                                .path
                                .to_lowercase()
                                .contains(&self.search_query.to_lowercase()))
                })
                .map(|(i, _)| i)
                .collect();
            if !visible.is_empty() {
                let prev = match self.selected_index {
                    None => *visible.last().unwrap(),
                    Some(cur) => {
                        let pos = visible.iter().position(|&i| i == cur).unwrap_or(0);
                        visible[(pos + visible.len() - 1) % visible.len()]
                    }
                };
                return self.update(Message::SelectEntry(prev));
            }
        }
        Task::none()
    }

    #[allow(clippy::collapsible_if)]
    pub(in crate::app) fn revert_selected_entry(&mut self) -> Task<Message> {
        if let (Some(idx), Some(def)) = (self.selected_index, &mut self.definition) {
            if let Some(diff) = self.diffs.get(idx) {
                let path = diff.path.clone();
                if let Some(pos) = def.entries.iter().position(|e| e.path == path) {
                    def.entries.remove(pos);
                    self.dirty = true;
                    self.push_toast(
                        ToastIntent::Info,
                        t!("toast.reverted").as_ref(),
                        t!("toast.reverted_path", path = path).as_ref(),
                    );
                    return self.start_async_rerun();
                }
            }
        }
        Task::none()
    }
}
