//! Small shared widgets used across the panel.

use iced::widget::{container, mouse_area};
use iced::{Color, Element, Length, Pixels};

use crate::fonts::{NERD, NERD_BOLD};
use crate::message::Message;
use crate::theme;

/// Opacity for a widget button background given active / hover state.
/// Mirrors the quickshell `WidgetButton` opacities.
pub fn bg_opacity(active: bool, hovered: bool) -> f32 {
    if active {
        0.70
    } else if hovered {
        0.45
    } else {
        0.40
    }
}

/// A centered glyph button may look off-centre when a glyph's ink extends
/// beyond its advance width. For example the `oct-sun` brightness icon is
/// drawn from 0..1000 units while its advance is only 600, so the advance
/// centre is 200 units left of the ink centre and the icon drifts right.
/// This wraps the glyph and pads the trailing edge to pull the ink back to
/// the centre. `offset` is the ink-centre offset in font units (1000/em).
pub fn glyph_centered(
    s: impl Into<String>,
    size: f32,
    color: Color,
    bold: bool,
    offset: f32,
) -> Element<'static, Message> {
    container(glyph(s, size, color, bold))
        .padding(iced::Padding {
            top: 0.0,
            right: 2.0 * offset / 1000.0 * size,
            bottom: 0.0,
            left: 0.0,
        })
        .into()
}

/// A single nerd-font glyph as text.
pub fn glyph(
    s: impl Into<String>,
    size: f32,
    color: Color,
    bold: bool,
) -> iced::widget::Text<'static> {
    iced::widget::text(s.into())
        .size(Pixels(size))
        .color(color)
        .font(if bold { NERD_BOLD } else { NERD })
}

/// A centered glyph/button background used by the bar widgets.
///
/// Wraps `content` in a hover/click aware `MouseArea` and gives it a subtle
/// rounded background that brightens on hover and when `active`.
#[allow(clippy::too_many_arguments)]
pub fn pressable<'a>(
    id: &'static str,
    content: impl Into<Element<'a, Message>>,
    width: impl Into<Length>,
    height: impl Into<Length>,
    active: bool,
    hovered: Option<&'static str>,
    on_left: Message,
    on_right: Option<Message>,
    on_scroll: Option<Box<dyn Fn(f32) -> Message + 'a>>,
) -> Element<'a, Message> {
    let is_hover = hovered == Some(id);
    let opacity = bg_opacity(active, is_hover);

    let body: Element<'a, Message> = container(content.into())
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x(Length::Fill)
        .center_y(Length::Fill)
        .style(move |_: &iced::Theme| container::Style {
            background: Some(iced::Background::Color(theme::mix(
                theme::VISUAL,
                theme::BG_PANEL,
                1.0 - opacity,
            ))),
            ..Default::default()
        })
        .into();

    let mut area = mouse_area(body)
        .on_press(on_left)
        .on_enter(Message::Hover(id))
        .on_exit(Message::Unhover(id))
        .interaction(iced::mouse::Interaction::Pointer);

    if let Some(right) = on_right {
        area = area.on_right_press(right);
    }
    if let Some(scroll) = on_scroll {
        area = area.on_scroll(move |delta| {
            let y = match delta {
                iced::mouse::ScrollDelta::Lines { y, .. } => y,
                iced::mouse::ScrollDelta::Pixels { y, .. } => y,
            };
            scroll(y)
        });
    }

    container(area).width(width).height(height).into()
}

/// A bare glyph button (no background) — used for tray items etc.
#[allow(dead_code)]
pub fn glyph_button<'a>(
    id: &'static str,
    glyph_str: &'static str,
    size: f32,
    color: Color,
    hovered: Option<&'static str>,
    on_left: Message,
    on_right: Option<Message>,
) -> Element<'a, Message> {
    let content = glyph(glyph_str, size, color, false);
    let is_hover = hovered == Some(id);
    let opacity = if is_hover { 0.45 } else { 0.40 };

    let mut area = mouse_area(
        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .style(move |_: &iced::Theme| container::Style {
                background: Some(iced::Background::Color(theme::mix(
                    theme::VISUAL,
                    theme::BG_PANEL,
                    1.0 - opacity,
                ))),
                ..Default::default()
            }),
    )
    .on_press(on_left)
    .on_enter(Message::Hover(id))
    .on_exit(Message::Unhover(id))
    .interaction(iced::mouse::Interaction::Pointer);

    if let Some(right) = on_right {
        area = area.on_right_press(right);
    }

    container(area)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}

/// A thin divider bar (like the quickshell 3px separator).
pub fn divider<'a>(width: Length, height: f32, color: Color, opacity: f32) -> Element<'a, Message> {
    container("")
        .width(width)
        .height(Pixels(height))
        .style(move |_: &iced::Theme| container::Style {
            background: Some(iced::Background::Color(Color {
                a: opacity,
                ..color
            })),
            ..Default::default()
        })
        .into()
}
