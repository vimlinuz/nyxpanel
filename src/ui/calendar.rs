//! Calendar popup, opened by clicking the clock.

use chrono::{Datelike, Local, NaiveDate};
use iced::alignment;
use iced::widget::{column, container, mouse_area, row, text};
use iced::{Element, Length, Pixels};

use crate::fonts::NERD_BOLD;
use crate::message::Message;
use crate::state::State;
use crate::theme;

const CELL_W: f32 = 30.0;
const CELL_H: f32 = 28.0;
const GAP: f32 = 2.0;

pub fn calendar(state: &State) -> Element<'_, Message> {
    let month = state.cal_month;
    let year = state.cal_year;
    let today = Local::now().date_naive();

    let title = NaiveDate::from_ymd_opt(year, month + 1, 1)
        .map(|d| d.format("%B %Y").to_string())
        .unwrap_or_default();

    // Header: prev, title (go to today), next.
    let nav = row![
        nav_button("\u{f104}", Message::CalendarPrev, "cal-prev", state),
        mouse_area(
            container(
                text(title)
                    .size(Pixels(13.0))
                    .color(theme::FG_BRIGHT)
                    .font(NERD_BOLD)
                    .width(Length::Fill)
                    .align_x(alignment::Horizontal::Center)
            )
            .width(Length::Fill)
            .height(Pixels(24.0))
            .center_y(Length::Fill),
        )
        .on_press(Message::CalendarToday)
        .interaction(iced::mouse::Interaction::Pointer),
        nav_button("\u{f105}", Message::CalendarNext, "cal-next", state),
    ]
    .spacing(4)
    .width(Length::Fill)
    .align_y(alignment::Vertical::Center);

    // Weekday header (Su..Sa).
    let mut week = row![].spacing(GAP);
    for (i, name) in ["S", "M", "T", "W", "T", "F", "S"].iter().enumerate() {
        let color = if i == 0 || i == 6 {
            theme::MUTED
        } else {
            theme::COMMENT
        };
        week = week.push(
            container(
                text(*name)
                    .size(Pixels(10.0))
                    .color(color)
                    .font(NERD_BOLD)
                    .width(Length::Fixed(CELL_W))
                    .align_x(alignment::Horizontal::Center),
            )
            .height(Pixels(18.0)),
        );
    }

    // 42-cell (6x7) grid so it never jumps between months.
    let first_day = NaiveDate::from_ymd_opt(year, month + 1, 1)
        .map(|d| d.weekday().num_days_from_sunday() as i32)
        .unwrap_or(0);
    let days_in_month = days_in_month(year, month);

    let mut grid = column![].spacing(GAP);
    for week_row in 0..6 {
        let mut cells = row![].spacing(GAP);
        for col in 0..7 {
            let day_num = (week_row * 7 + col) - first_day + 1;
            let in_month = day_num >= 1 && day_num <= days_in_month;
            let date =
                NaiveDate::from_ymd_opt(year, month + 1, day_num.clamp(1, days_in_month) as u32);
            let is_today = in_month && date == Some(today);
            let is_weekend = in_month && (col == 0 || col == 6);

            let text_color = if !in_month {
                theme::BG_PANEL
            } else if is_today {
                iced::Color::from_rgb8(0, 0, 0)
            } else if is_weekend {
                theme::MUTED
            } else {
                theme::FG
            };

            let cell = container(
                text(if in_month {
                    day_num.to_string()
                } else {
                    String::new()
                })
                .size(Pixels(11.0))
                .color(text_color)
                .font(NERD_BOLD),
            )
            .width(Length::Fixed(CELL_W))
            .height(Length::Fixed(CELL_H))
            .center_x(Length::Fixed(CELL_W))
            .center_y(Length::Fixed(CELL_H))
            .style(move |_: &iced::Theme| container::Style {
                background: if is_today {
                    Some(iced::Background::Color(theme::BORDER))
                } else {
                    None
                },
                border: iced::Border {
                    radius: 6.0.into(),
                    ..Default::default()
                },
                ..Default::default()
            });

            cells = cells.push(cell);
        }
        grid = grid.push(cells);
    }

    let content = column![nav, week, grid]
        .spacing(4)
        .padding(10)
        .width(Length::Fixed(248.0))
        .align_x(alignment::Horizontal::Center);

    container(content)
        .width(Length::Fixed(248.0))
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

fn nav_button<'a>(
    glyph: &'static str,
    msg: Message,
    id: &'static str,
    state: &State,
) -> Element<'a, Message> {
    let hovered = state.hover == Some(id);
    let color = if hovered { theme::FG_BRIGHT } else { theme::FG };
    mouse_area(
        container(text(glyph).size(Pixels(14.0)).color(color).font(NERD_BOLD))
            .width(Pixels(24.0))
            .height(Pixels(24.0))
            .center_x(Pixels(24.0))
            .center_y(Pixels(24.0)),
    )
    .on_press(msg)
    .on_enter(Message::Hover(id))
    .on_exit(Message::Unhover(id))
    .interaction(iced::mouse::Interaction::Pointer)
    .into()
}

fn days_in_month(year: i32, month: u32) -> i32 {
    let (y, m) = if month == 11 {
        (year + 1, 1)
    } else {
        (year, month + 2)
    };
    let next_first = NaiveDate::from_ymd_opt(y, m, 1).expect("valid date");
    let this_first = NaiveDate::from_ymd_opt(year, month + 1, 1).expect("valid date");
    (next_first - this_first).num_days() as i32
}
