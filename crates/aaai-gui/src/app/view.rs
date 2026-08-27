use super::*;

impl App {
    pub(crate) fn view(&self) -> Element<'_, Message> {
        let body = match self.screen {
            Screen::Opening => opening::view(self),
            Screen::Main => main_view::view(self),
        };

        let footer = self.view_footer();

        let layout = AppLayout::new(body)
            .footer(footer)
            .toasts(self.toasts.clone())
            .toast_position(ToastPosition::BottomEnd)
            .on_close_modals(Message::CloseModals)
            .on_close_menus(Message::CloseMenus);

        let base: Element<'_, Message> = render(layout);

        // RFC 036 — Settings dialog modal overlay
        if self.settings_open {
            use iced::widget::{container, mouse_area, stack};
            use iced::{Color, Length};

            let backdrop = mouse_area(
                container(
                    iced::widget::space()
                        .width(Length::Fill)
                        .height(Length::Fill),
                )
                .width(Length::Fill)
                .height(Length::Fill)
                .style(|_| container::Style {
                    background: Some(iced::Background::Color(Color {
                        r: 0.0,
                        g: 0.0,
                        b: 0.0,
                        a: 0.35,
                    })),
                    ..Default::default()
                }),
            )
            .on_press(Message::CloseSettings);

            let dialog = iced::widget::center(crate::views::settings_dialog::view(
                &self.settings_draft,
                &self.locale,
                &self.design_tokens,
            ));

            stack![base, backdrop, dialog].into()

        // RFC 038 — Keyboard help overlay (only on Main screen)
        } else if self.help_open && matches!(self.screen, Screen::Main) {
            use iced::widget::{container, mouse_area, stack};
            use iced::{Color, Length};

            let backdrop = mouse_area(
                container(
                    iced::widget::space()
                        .width(Length::Fill)
                        .height(Length::Fill),
                )
                .width(Length::Fill)
                .height(Length::Fill)
                .style(|_| container::Style {
                    background: Some(iced::Background::Color(Color {
                        r: 0.0,
                        g: 0.0,
                        b: 0.0,
                        a: 0.35,
                    })),
                    ..Default::default()
                }),
            )
            .on_press(Message::CloseHelp);

            let dialog =
                iced::widget::center(crate::views::help_overlay::view(&self.design_tokens));

            stack![base, backdrop, dialog].into()

        // RFC 041 — Navigation guard (only on Main screen)
        } else if self.nav_guard_open && matches!(self.screen, Screen::Main) {
            use iced::widget::{container, mouse_area, stack};
            use iced::{Color, Length};

            let backdrop = mouse_area(
                container(
                    iced::widget::space()
                        .width(Length::Fill)
                        .height(Length::Fill),
                )
                .width(Length::Fill)
                .height(Length::Fill)
                .style(|_| container::Style {
                    background: Some(iced::Background::Color(Color {
                        r: 0.0,
                        g: 0.0,
                        b: 0.0,
                        a: 0.50,
                    })),
                    ..Default::default()
                }),
            )
            .on_press(Message::NavGuardCancel);

            let dialog = iced::widget::center(crate::views::nav_guard::view(
                self.nav_guard_show_discard,
                &self.design_tokens,
            ));

            stack![base, backdrop, dialog].into()
        } else {
            base
        }
    }

    pub(in crate::app) fn view_footer(&self) -> Element<'_, Message> {
        use iced::widget::tooltip::Position;
        use iced::{
            Alignment::Center,
            Length,
            widget::{button, container, row, space, text, tooltip},
        };

        // RFC 036 — language picker moved to Settings dialog.
        // RFC 038 — ? button (help overlay) + ⚙ settings button.
        let help_btn = tooltip(
            button(
                text("?")
                    .size(self.design_tokens.typography.label.size)
                    .line_height(self.design_tokens.typography.label.line_height),
            )
            .on_press(Message::ToggleHelp)
            .padding(iced::Padding::from([
                self.design_tokens.spacing.xs,
                self.design_tokens.spacing.sm,
            ]))
            .style({
                let t = self.design_tokens.clone();
                move |_th, s| crate::style::btn_ghost(&t, s)
            }),
            text(t!("help.title").to_string())
                .size(self.design_tokens.typography.body_small.size)
                .line_height(self.design_tokens.typography.body_small.line_height),
            Position::Top,
        );

        let settings_btn = tooltip(
            button(
                text("⚙")
                    .size(self.design_tokens.typography.label.size)
                    .line_height(self.design_tokens.typography.label.line_height),
            )
            .on_press(Message::OpenSettings)
            .padding(iced::Padding::from([
                self.design_tokens.spacing.xs,
                self.design_tokens.spacing.sm,
            ]))
            .style({
                let t = self.design_tokens.clone();
                move |_th, s| crate::style::btn_ghost(&t, s)
            }),
            text(t!("settings.button_tooltip").to_string())
                .size(self.design_tokens.typography.body_small.size)
                .line_height(self.design_tokens.typography.body_small.line_height),
            Position::Top,
        );

        let left: Element<'_, Message> = if self.dirty {
            text(t!("footer.unsaved"))
                .size(self.design_tokens.typography.body_small.size)
                .line_height(self.design_tokens.typography.body_small.line_height)
                .color(crate::style::to_iced(self.design_tokens.palette.warning))
                .into()
        } else {
            text("")
                .size(self.design_tokens.typography.body_small.size)
                .line_height(self.design_tokens.typography.body_small.line_height)
                .into()
        };

        container(
            row![
                left,
                space().width(Length::Fill),
                help_btn,
                settings_btn,
                text(format!("v{}", env!("CARGO_PKG_VERSION")))
                    .size(self.design_tokens.typography.body_small.size)
                    .line_height(self.design_tokens.typography.body_small.line_height),
            ]
            .align_y(Center)
            .spacing(self.design_tokens.spacing.md),
        )
        .width(Length::Fill)
        .padding(iced::Padding::from([
            self.design_tokens.spacing.xs,
            self.design_tokens.spacing.lg,
        ]))
        .style(panel_style(self.design_tokens.clone()))
        .into()
    }
}
