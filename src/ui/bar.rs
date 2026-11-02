//! The vertical left bar and all of its widgets.

use iced::alignment;
use iced::widget::{column, container, text};
use iced::{Element, Length, Pixels};

use crate::fonts::NERD_BOLD;
use crate::message::Message;
use crate::state::State;
use crate::theme;
use crate::ui::widgets::{divider, glyph, glyph_centered, pressable};

// Icon glyphs (Nerd Font codepoints, matching the quickshell panel exactly).
const I_BELL: &str = "\u{f0f3}";
const I_BELL_OFF: &str = "\u{f1f6}";
const I_WS_ACTIVE: &str = "\u{f066}";
const I_WS_INACTIVE: &str = "\u{f0ae3}";

pub fn bar(state: &State) -> Element<'_, Message> {
    let workspaces = workspaces(state);
    let clock = clock(state);
    let bottom = column![battery(state), notifications(state)]
        .spacing(4)
        .width(Length::Fill)
        .align_x(alignment::Horizontal::Center);

    // The bar is only as tall as its content (workspaces + clock + controls).
    let content = column![workspaces, clock, bottom]
        .spacing(4)
        .padding(2)
        .width(Length::Fill)
        .align_x(alignment::Horizontal::Center);

    let panel = container(content)
        .width(Length::Fill)
        .height(Length::Shrink)
        .style(|_: &iced::Theme| container::Style {
            background: Some(iced::Background::Color(theme::BG_PANEL)),
            border: iced::Border {
                width: 1.0,
                color: theme::BORDER,
                ..Default::default()
            },
            ..Default::default()
        });

    // Center the (content-sized) bar vertically on the full-height surface.
    container(panel)
        .width(Length::Fill)
        .height(Length::Fill)
        .center_y(Length::Fill)
        .into()
}

// ---------------------------------------------------------------------------
// Workspaces
// ---------------------------------------------------------------------------

fn workspaces(state: &State) -> Element<'_, Message> {
    let mut col = column![]
        .spacing(4)
        .width(Length::Fill)
        .align_x(alignment::Horizontal::Center);

    for ws in state.workspaces() {
        let has_window = state.workspace_has_window(ws.id);
        if !(ws.is_active || has_window) {
            continue;
        }
        let id = ws.id;
        let active = ws.is_active;
        let (glyph_str, offset) = if active {
            (I_WS_ACTIVE, 104.0)
        } else {
            (I_WS_INACTIVE, 159.0)
        };
        let inner = glyph_centered(glyph_str, 16.0, theme::FG, active, offset);
        let btn_id = ws_button_id(id);
        col = col.push(pressable(
            btn_id,
            inner,
            Length::Fill,
            Pixels(24.0),
            active,
            None,
            Message::FocusWorkspace(id),
            None,
            None,
        ));
    }

    // Divider between workspace dots and window indicators.
    col = col.push(divider(Length::Fill, 2.0, theme::FG_BRIGHT, 0.25));

    // Window indicators for the active workspace.
    for win in state.active_windows() {
        let label = win
            .app_id
            .as_deref()
            .and_then(|s| s.chars().next())
            .map(|c| c.to_uppercase().to_string())
            .unwrap_or_else(|| "?".into());
        let focused = win.is_focused;
        let wid = win.id;
        let inner = text(label)
            .size(Pixels(16.0))
            .font(NERD_BOLD)
            .color(theme::FG);
        let box_ = container(inner)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .style(move |_: &iced::Theme| container::Style {
                background: if focused {
                    Some(iced::Background::Color(theme::mix(
                        theme::VISUAL,
                        theme::BG_PANEL,
                        0.55,
                    )))
                } else {
                    None
                },
                ..Default::default()
            });
        let id = win_button_id(wid);
        col = col.push(pressable(
            id,
            box_,
            Length::Fill,
            Pixels(24.0),
            false,
            None,
            Message::FocusWindow(wid),
            None,
            None,
        ));
    }

    col.into()
}

// Unique-ish static ids for hover tracking. Workspaces/windows are dynamic so
// we can't use `&'static str` per item; instead we skip hover styling for them.
fn ws_button_id(_id: u64) -> &'static str {
    "ws-item"
}
fn win_button_id(_id: u64) -> &'static str {
    "win-item"
}

// ---------------------------------------------------------------------------
// Clock (vertical, click opens calendar)
// ---------------------------------------------------------------------------

fn clock(state: &State) -> Element<'_, Message> {
    let now = state.now;
    let hour = now.format("%I").to_string();
    let minute = now.format("%M").to_string();
    let dow = now.format("%a").to_string();
    let day = now.format("%-d").to_string();
    let month = now.format("%b").to_string();

    let big = |s: String| {
        text(s)
            .size(Pixels(16.0))
            .color(theme::FG_BRIGHT)
            .font(NERD_BOLD)
            .width(Length::Fill)
            .align_x(alignment::Horizontal::Center)
    };
    let small = |s: String| {
        text(s)
            .size(Pixels(11.0))
            .color(theme::FG_BRIGHT)
            .font(NERD_BOLD)
            .width(Length::Fill)
            .align_x(alignment::Horizontal::Center)
    };

    let body = column![
        big(hour),
        big(minute),
        divider(Length::Fill, 2.0, theme::FG_BRIGHT, 0.25),
        small(dow),
        small(day),
        small(month),
    ]
    .spacing(1)
    .width(Length::Fill)
    .padding(Pixels(6.0))
    .align_x(alignment::Horizontal::Center);

    pressable(
        "clock",
        body,
        Length::Fill,
        Pixels(110.0),
        false,
        state.hover,
        Message::ToggleCalendar,
        None,
        None,
    )
}

// ---------------------------------------------------------------------------
// Battery
// ---------------------------------------------------------------------------

fn battery(state: &State) -> Element<'_, Message> {
    let b = &state.battery;
    if !b.available {
        let body = column![
            glyph("\u{f0079}", 13.0, theme::FG, true),
            text("N/A")
                .size(Pixels(8.0))
                .color(theme::FG)
                .font(NERD_BOLD)
                .width(Length::Fill)
                .align_x(alignment::Horizontal::Center),
        ]
        .spacing(1)
        .width(Length::Fill)
        .align_x(alignment::Horizontal::Center);
        return container(body).width(Length::Fill).into();
    }

    let pct = (b.pct * 100.0).round() as u32;
    let color = if b.is_charging() {
        theme::SUCCESS
    } else if b.pct <= 0.20 {
        theme::ERROR
    } else if b.pct <= 0.30 {
        theme::WARNING
    } else {
        theme::FG
    };
    let icon = battery_glyph(pct, b.is_charging(), b.plugged);

    let body = column![
        glyph(icon, 13.0, color, true),
        text(format!("{pct}%"))
            .size(Pixels(8.0))
            .color(color)
            .font(NERD_BOLD)
            .width(Length::Fill)
            .align_x(alignment::Horizontal::Center),
    ]
    .spacing(1)
    .width(Length::Fill)
    .align_x(alignment::Horizontal::Center);

    container(body).width(Length::Fill).into()
}

// 10-level Nerd Font battery set, matching the quickshell panel.
fn battery_glyph(pct: u32, charging: bool, plugged: bool) -> &'static str {
    if charging {
        return "\u{f140b}";
    }
    if plugged {
        return "\u{f17e2}";
    }
    match pct {
        p if p <= 10 => "\u{f007a}",
        p if p <= 20 => "\u{f007b}",
        p if p <= 30 => "\u{f007c}",
        p if p <= 40 => "\u{f007d}",
        p if p <= 50 => "\u{f007e}",
        p if p <= 60 => "\u{f007f}",
        p if p <= 70 => "\u{f0080}",
        p if p <= 80 => "\u{f0081}",
        p if p <= 90 => "\u{f0082}",
        _ => "\u{f0079}",
    }
}

// ---------------------------------------------------------------------------
// Notifications (bell) — toggles a minimal center + DND
// ---------------------------------------------------------------------------

fn notifications(state: &State) -> Element<'_, Message> {
    let (icon, color) = if state.dnd {
        (I_BELL_OFF, theme::MUTED)
    } else {
        (I_BELL, theme::FG)
    };
    pressable(
        "notifications",
        glyph(icon, 13.0, color, false),
        Length::Fill,
        Pixels(24.0),
        state.center_open,
        state.hover,
        Message::ToggleCenter,
        Some(Message::ToggleDnd),
        None,
    )
}
