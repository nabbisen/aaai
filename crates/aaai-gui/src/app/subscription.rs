use super::*;

/// RFC 023 §3.1 — subscription source for drag-and-drop window events.
/// We listen for the three iced window events that bracket a drag:
/// `FileHovered` / `FilesHoveredLeft` / `FileDropped`. The handler in
/// [`App`] decides what to do with the payload (it ignores events while
/// the user is on a screen other than Opening).
pub(super) fn dnd_sub() -> Subscription<Message> {
    iced::event::listen_with(|event, _status, _id| {
        use iced::event::Event;
        use iced::window::Event as WinEvent;
        match event {
            Event::Window(WinEvent::FileHovered(_)) => Some(Message::FileHoverEnter),
            Event::Window(WinEvent::FilesHoveredLeft) => Some(Message::FileHoverLeave),
            Event::Window(WinEvent::FileDropped(p)) => Some(Message::FileDropped(p)),
            _ => None,
        }
    })
}

impl App {
    pub(crate) fn subscription(&self) -> Subscription<Message> {
        let toast_sub = snora::toast::subscription(&self.toasts, || Message::ToastTick);
        let kb_sub = iced::keyboard::listen().map(|event| {
            use iced::keyboard::{Event as KbEvent, Key, Modifiers};
            match event {
                KbEvent::KeyPressed { key, modifiers, .. } => {
                    match (key.as_ref(), modifiers) {
                        (Key::Character("s"), m) if m.contains(Modifiers::CTRL) => {
                            Message::SaveDefinition
                        }
                        (Key::Character("r"), m) if m.contains(Modifiers::CTRL) => {
                            Message::RerunAudit
                        }
                        (Key::Character("z"), m)
                            if m.contains(Modifiers::CTRL) && m.contains(Modifiers::SHIFT) =>
                        {
                            Message::RevertSelectedEntry
                        }
                        (Key::Character("z"), m) if m.contains(Modifiers::CTRL) => {
                            Message::UndoApproval
                        }
                        // RFC 005: Ctrl+E → export report
                        (Key::Character("e"), m) if m.contains(Modifiers::CTRL) => {
                            Message::ExportReport
                        }
                        (Key::Named(iced::keyboard::key::Named::ArrowDown), _) => {
                            Message::SelectNext
                        }
                        (Key::Named(iced::keyboard::key::Named::ArrowUp), _) => Message::SelectPrev,
                        // RFC 005: Tab / Shift+Tab for pane focus cycling
                        (Key::Named(iced::keyboard::key::Named::Tab), m)
                            if m.contains(Modifiers::SHIFT) =>
                        {
                            Message::FocusPrev
                        }
                        (Key::Named(iced::keyboard::key::Named::Tab), _) => Message::FocusNext,
                        // RFC 005: / key → focus search
                        (Key::Character("/"), m)
                            if !m.contains(Modifiers::CTRL) && !m.contains(Modifiers::ALT) =>
                        {
                            Message::FocusSearch
                        }
                        // RFC 051 — Ctrl+Enter submits approval (the reason text is
                        // trimmed in the handler, so an accidental trailing newline
                        // from the text_editor is harmless).
                        (Key::Named(iced::keyboard::key::Named::Enter), m)
                            if m.contains(Modifiers::CTRL) =>
                        {
                            Message::ApproveAndSave
                        }
                        // RFC 005: Enter → focus inspector reason
                        (Key::Named(iced::keyboard::key::Named::Enter), _) => {
                            Message::FocusInspectorReason
                        }
                        // RFC 038: ? key → toggle keyboard help overlay
                        (Key::Character("?"), _) => Message::ToggleHelp,
                        // Escape — handled via EscapeKey to avoid capturing self in the closure
                        (Key::Named(iced::keyboard::key::Named::Escape), _) => Message::EscapeKey,
                        _ => Message::Noop,
                    }
                }
                _ => Message::Noop,
            }
        });
        // RFC 021 §3.5 — 30-second wall-clock tick. Only enabled when at
        // least one timestamp is present, so we don't burn CPU re-rendering
        // until the user has actually saved or exported once.
        let needs_tick = self.last_saved_at.is_some() || self.last_reported_at.is_some();
        let time_sub: Subscription<Message> = if needs_tick {
            iced::time::every(std::time::Duration::from_secs(30)).map(|_| Message::RelativeTimeTick)
        } else {
            Subscription::none()
        };

        Subscription::batch([toast_sub, kb_sub, dnd_sub(), time_sub])
    }
}
