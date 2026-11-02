//! Minimal notification center.
//!
//! A full freedesktop notification daemon (D-Bus `org.freedesktop.Notifications`)
//! is not wired up yet; this window provides the panel UI shell that the real
//! notification list will populate later.

use iced::alignment;
use iced::widget::{column, container, mouse_area, row, text};
use iced::{Element, Length, Pixels};

use crate::fonts::NERD_BOLD;
use crate::message::Message;
use crate::state::State;
use crate::theme;

pub fn center(state: &State) -> Element<'_, Message> {
    let header = row![
        text("Notifications")
            .size(Pixels(13.0))
            .color(theme::FG_BRIGHT)
            .font(NERD_BOLD)
            .width(Length::Fill)
            .align_x(alignment::Horizontal::Left),
        icon_button(
            if state.dnd { "\u{f1f6}" } else { "\u{f0f3}" },
            Message::ToggleDnd,
            "center-dnd",
            state,
        ),
        icon_button("\u{f1f8}", Message::Noop, "center-clear", state),
        icon_button("\u{f00d}", Message::CloseCenter, "center-close", state),
    ]
    .spacing(4)
    .width(Length::Fill)
    .align_y(alignment::Vertical::Center);

    let body = column![
        text("No notifications")
            .size(Pixels(12.0))
            .color(theme::COMMENT)
            .font(NERD_BOLD)
            .width(Length::Fill)
            .align_x(alignment::Horizontal::Center),
    ]
    .padding([24, 0])
    .width(Length::Fill);

    let content = column![
        container(header).width(Length::Fill),
        container("")
            .width(Length::Fill)
            .height(Pixels(1.0))
            .style(|_: &iced::Theme| container::Style {
                background: Some(iced::Background::Color(theme::mix(
                    theme::BORDER,
                    theme::BG_PANEL,
                    0.4
                ))),
                ..Default::default()
            }),
        body,
    ]
    .spacing(6)
    .padding(8)
    .width(Length::Fixed(400.0));

    container(content)
        .width(Length::Fixed(400.0))
        .style(|_: &iced::Theme| container::Style {
            background: Some(iced::Background::Color(theme::BG_PANEL)),
            border: iced::Border {
                width: 1.0,
                radius: 12.0.into(),
                color: theme::BORDER,
            },
            ..Default::default()
        })
        .into()
}

fn icon_button<'a>(
    glyph: &'static str,
    msg: Message,
    id: &'static str,
    state: &State,
) -> Element<'a, Message> {
    let hovered = state.hover == Some(id);
    let opacity = if hovered { 0.45 } else { 0.0 };
    mouse_area(
        container(
            text(glyph)
                .size(Pixels(14.0))
                .color(theme::FG)
                .font(NERD_BOLD),
        )
        .width(Pixels(26.0))
        .height(Pixels(26.0))
        .center_x(Pixels(26.0))
        .center_y(Pixels(26.0))
        .style(move |_: &iced::Theme| container::Style {
            background: Some(iced::Background::Color(theme::mix(
                theme::VISUAL,
                theme::BG_PANEL,
                1.0 - opacity,
            ))),
            border: iced::Border {
                radius: 6.0.into(),
                ..Default::default()
            },
            ..Default::default()
        }),
    )
    .on_press(msg)
    .on_enter(Message::Hover(id))
    .on_exit(Message::Unhover(id))
    .interaction(iced::mouse::Interaction::Pointer)
    .into()
}
