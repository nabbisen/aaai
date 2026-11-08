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
        // RFC 038 — Help overlay + settings dialog buttons.
        //
        // RFC 115 §S1 — these are the only route to the theme picker (and so
        // to the high-contrast themes), so each carries a visible text label
        // beside its glyph, a resting border (`btn_secondary`) and 44 px of
        // padding-derived height. The tooltips stay as supplementary help.
        let footer_btn = |glyph: &'static str,
                          label: String,
                          tooltip_text: String,
                          msg: Message|
         -> Element<'_, Message> {
            let tokens = &self.design_tokens;
            tooltip(
                button(
                    row![
                        text(glyph)
                            .size(tokens.typography.label.size)
                            .line_height(tokens.typography.label.line_height),
                        text(label)
                            .size(tokens.typography.label.size)
                            .line_height(tokens.typography.label.line_height),
                    ]
                    .spacing(tokens.spacing.xs)
                    .align_y(Center),
                )
                .on_press(msg)
                .padding(iced::Padding::from([tokens.spacing.lg, tokens.spacing.lg]))
                .style({
                    let t = tokens.clone();
                    move |_th, s| crate::style::btn_secondary(&t, s)
                }),
                text(tooltip_text)
                    .size(tokens.typography.body_small.size)
                    .line_height(tokens.typography.body_small.line_height),
                Position::Top,
            )
            .into()
        };

        let help_btn = footer_btn(
            "?",
            t!("help.button_label").to_string(),
            t!("help.title").to_string(),
            Message::ToggleHelp,
        );

        let settings_btn = footer_btn(
            "⚙",
            t!("settings.button_tooltip").to_string(),
            t!("settings.button_tooltip").to_string(),
            Message::OpenSettings,
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
