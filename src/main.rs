//! nyx_panel — a Wayland bar for the niri compositor, built with iced.
//!
//! This ports the quickshell `nyx_panel` configuration (a vertical left bar
//! with workspaces, a clock/calendar, and the usual system widgets) to Rust
//! using [`iced`] and [`iced_layershell`].
//!
//! [`iced`]: https://docs.rs/iced
//! [`iced_layershell`]: https://docs.rs/iced_layershell

mod fonts;
mod message;
mod niri;
mod state;
mod system;
mod theme;
mod ui;

use std::borrow::Cow;
use std::time::Duration;

use chrono::{Datelike, Local};
use iced::window;
use iced::{Element, Event, Subscription, Task};
use iced_layershell::build_pattern::daemon;
use iced_layershell::reexport::{
    Anchor, KeyboardInteractivity, Layer, NewLayerShellSettings, OutputOption,
};
use iced_layershell::settings::{LayerShellSettings, Settings, StartMode};

use message::Message;
use state::{State, WindowKind};

/// Width of the vertical bar, in logical pixels.
const BAR_W: u32 = 30;

pub fn main() -> iced_layershell::Result {
    daemon(boot, "nyx_panel", update, view)
        .subscription(subscription)
        .style(style)
        .settings(Settings {
            layer_settings: LayerShellSettings {
                anchor: Anchor::Left | Anchor::Top | Anchor::Bottom,
                size: Some((BAR_W, 0)),
                exclusive_zone: BAR_W as i32,
                margin: (0, 0, 0, 0),
                keyboard_interactivity: KeyboardInteractivity::None,
                start_mode: StartMode::Active,
                layer: Layer::Top,
                events_transparent: false,
            },
            default_font: fonts::NERD,
            fonts: vec![
                Cow::Borrowed(fonts::NERD_REGULAR_BYTES),
                Cow::Borrowed(fonts::NERD_BOLD_BYTES),
            ],
            default_text_size: iced::Pixels(13.0),
            antialiasing: true,
            ..Default::default()
        })
        .run()
}

// ---------------------------------------------------------------------------
// Boot
// ---------------------------------------------------------------------------

fn boot() -> (State, Task<Message>) {
    let state = State::default();
    let task = Task::batch(vec![Task::perform(
        async { system::read_battery() },
        Message::BatteryLoaded,
    )]);
    (state, task)
}

// ---------------------------------------------------------------------------
// Subscriptions
// ---------------------------------------------------------------------------

fn subscription(_state: &State) -> Subscription<Message> {
    Subscription::batch([
        iced::time::every(Duration::from_secs(1)).map(|_| Message::Tick),
        iced::event::listen().map(Message::IcedEvent),
        iced::window::close_events().map(Message::WindowClosed),
        niri::subscription(),
    ])
}

// ---------------------------------------------------------------------------
// Update
// ---------------------------------------------------------------------------

fn update(state: &mut State, message: Message) -> Task<Message> {
    match message {
        Message::WindowClosed(id) => {
            state.unregister(id);
            Task::none()
        }

        Message::IcedEvent(event) => handle_event(state, event),

        Message::Hover(id) => {
            state.hover = Some(id);
            Task::none()
        }
        Message::Unhover(id) => {
            if state.hover == Some(id) {
                state.hover = None;
            }
            Task::none()
        }

        Message::Tick => {
            state.now = Local::now();
            state.tick = state.tick.wrapping_add(1);
            tick_polls(state)
        }

        Message::BatteryLoaded(info) => {
            state.battery = info;
            Task::none()
        }
        Message::NiriEvent(event) => {
            state.apply_niri_event(event);
            Task::none()
        }

        Message::ToggleCalendar => toggle_kind(state, WindowKind::Calendar),
        Message::CloseCalendar => close_kind(state, WindowKind::Calendar),
        Message::CalendarPrev => {
            if state.cal_month == 0 {
                state.cal_month = 11;
                state.cal_year -= 1;
            } else {
                state.cal_month -= 1;
            }
            Task::none()
        }
        Message::CalendarNext => {
            if state.cal_month == 11 {
                state.cal_month = 0;
                state.cal_year += 1;
            } else {
                state.cal_month += 1;
            }
            Task::none()
        }
        Message::CalendarToday => {
            let now = Local::now();
            state.cal_month = now.month0();
            state.cal_year = now.year();
            Task::none()
        }

        Message::FocusWorkspace(id) => {
            Task::perform(async move { niri::focus_workspace(id) }, |_| Message::Noop)
        }
        Message::FocusWindow(id) => {
            Task::perform(async move { niri::focus_window(id) }, |_| Message::Noop)
        }

        Message::ToggleCenter => toggle_kind(state, WindowKind::Center),
        Message::CloseCenter => close_kind(state, WindowKind::Center),
        Message::ToggleDnd => {
            state.dnd = !state.dnd;
            Task::none()
        }

        Message::Noop => Task::none(),

        // Layer-shell control variants (NewLayerShell, RemoveWindow, ...) are
        // intercepted by the runtime before ever reaching `update`.
        _ => Task::none(),
    }
}

fn tick_polls(state: &State) -> Task<Message> {
    let mut tasks: Vec<Task<Message>> = Vec::new();

    if state.tick.is_multiple_of(10) {
        tasks.push(Task::perform(
            async { system::read_battery() },
            Message::BatteryLoaded,
        ));
    }

    Task::batch(tasks)
}

fn handle_event(state: &mut State, event: Event) -> Task<Message> {
    if message::is_escape(&event) {
        let mut tasks: Vec<Task<Message>> = Vec::new();
        if state.calendar_open {
            tasks.push(close_kind(state, WindowKind::Calendar));
        }
        if state.center_open {
            tasks.push(close_kind(state, WindowKind::Center));
        }
        return Task::batch(tasks);
    }

    Task::none()
}

// ---------------------------------------------------------------------------
// Window management helpers
// ---------------------------------------------------------------------------

fn toggle_kind(state: &mut State, kind: WindowKind) -> Task<Message> {
    let open = match kind {
        WindowKind::Calendar => state.calendar_open,
        WindowKind::Center => state.center_open,
        WindowKind::Bar => true,
    };
    if open {
        return close_kind(state, kind);
    }

    let id = window::Id::unique();
    state.register(id, kind);
    match kind {
        WindowKind::Calendar => state.calendar_open = true,
        WindowKind::Center => state.center_open = true,
        WindowKind::Bar => {}
    }

    Task::done(Message::NewLayerShell {
        settings: settings_for(kind),
        id,
    })
}

fn close_kind(state: &mut State, kind: WindowKind) -> Task<Message> {
    let Some(id) = state.id_of(kind) else {
        return Task::none();
    };
    state.unregister(id);
    match kind {
        WindowKind::Calendar => state.calendar_open = false,
        WindowKind::Center => state.center_open = false,
        WindowKind::Bar => {}
    }
    Task::done(Message::RemoveWindow(id))
}

fn settings_for(kind: WindowKind) -> NewLayerShellSettings {
    match kind {
        WindowKind::Calendar => NewLayerShellSettings {
            size: Some((248, 248)),
            anchor: Anchor::Left,
            exclusive_zone: Some(0),
            margin: Some((0, 44, 0, 0)),
            keyboard_interactivity: KeyboardInteractivity::OnDemand,
            layer: Layer::Top,
            output_option: OutputOption::Active,
            ..Default::default()
        },
        WindowKind::Center => NewLayerShellSettings {
            size: Some((400, 120)),
            anchor: Anchor::Right | Anchor::Top,
            exclusive_zone: Some(0),
            margin: Some((4, 0, 0, 12)),
            keyboard_interactivity: KeyboardInteractivity::None,
            layer: Layer::Top,
            output_option: OutputOption::Active,
            ..Default::default()
        },
        WindowKind::Bar => NewLayerShellSettings::default(),
    }
}

// ---------------------------------------------------------------------------
// View
// ---------------------------------------------------------------------------

fn view(state: &State, id: window::Id) -> Element<'_, Message> {
    match state.kind_of(id) {
        Some(WindowKind::Calendar) => ui::calendar::calendar(state),
        Some(WindowKind::Center) => ui::center::center(state),
        Some(WindowKind::Bar) | None => ui::bar::bar(state),
    }
}

fn style(_state: &State, _theme: &iced::Theme) -> iced::theme::Style {
    iced::theme::Style {
        background_color: iced::Color::TRANSPARENT,
        text_color: theme::FG,
    }
}
