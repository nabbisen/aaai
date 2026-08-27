use iced::Subscription;

use super::Message;

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
