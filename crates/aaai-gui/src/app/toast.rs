use super::*;

impl App {
    pub(in crate::app) fn push_toast(&mut self, intent: ToastIntent, title: &str, body: &str) {
        let id = self.toast_id;
        self.toast_id += 1;
        self.toasts.push(Toast::new(
            id,
            intent,
            title.to_string(),
            body.to_string(),
            Message::DismissToast(id),
        ));
    }

    /// Push a toast with a two-line body following RFC 020's
    /// message + hint pattern (RFC 026 §3). The first line names what
    /// happened in user-facing terms; the second line — prefixed with
    /// a 💡 marker for visual distinction — names what to do next.
    ///
    /// Use this for *actionable* errors. For purely informational
    /// success / info toasts, use [`Self::push_toast`] directly.
    pub(in crate::app) fn push_toast_with_hint(
        &mut self,
        intent: ToastIntent,
        title: &str,
        message: &str,
        hint: &str,
    ) {
        let body = format!("{message}\n\n💡 {hint}");
        self.push_toast(intent, title, &body);
    }

    /// Push a toast carrying an already-built [`crate::error::UserError`].
    /// Convenience over [`Self::push_toast_with_hint`] for call-sites
    /// that constructed the error elsewhere (e.g. via
    /// `UserError::from_i18n("error.save.failed")`).
    ///
    /// Currently no internal site uses this — the two existing save_failed
    /// sites and the inspector regex site each build the toast inline. This
    /// method is part of RFC 026's public surface for future error sites
    /// (e.g. when DiffFailed, profile delete failure, or export failure
    /// gain proper UserError plumbing).
    #[allow(dead_code)]
    pub(in crate::app) fn push_user_error_toast(
        &mut self,
        intent: ToastIntent,
        title: &str,
        err: &crate::error::UserError,
    ) {
        self.push_toast_with_hint(intent, title, &err.message, &err.hint);
    }
}
