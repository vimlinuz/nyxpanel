//! The `Message` type shared by the whole panel.
//!
//! The `#[to_layer_message(multi)]` attribute augments this enum with the
//! layer-shell control variants (opening new surfaces, changing anchors,
//! margins, ...). See the iced_layershell docs.

use iced::Event;
use iced::keyboard;
use iced::keyboard::key::Named;
use iced::window;
use iced_layershell::to_layer_message;

use crate::system::BatteryInfo;

#[to_layer_message(multi)]
#[derive(Debug, Clone)]
pub enum Message {
    // ---- window lifecycle ----
    WindowClosed(window::Id),

    // ---- raw input ----
    IcedEvent(Event),

    // ---- hover bookkeeping (drives widget button styling) ----
    Hover(&'static str),
    Unhover(&'static str),

    // ---- clock ----
    Tick,

    // ---- system data ----
    BatteryLoaded(BatteryInfo),
    NiriEvent(niri_ipc::Event),

    // ---- calendar popup ----
    ToggleCalendar,
    CloseCalendar,
    CalendarPrev,
    CalendarNext,
    CalendarToday,

    // ---- workspaces ----
    FocusWorkspace(u64),
    FocusWindow(u64),

    // ---- notification center ----
    ToggleCenter,
    CloseCenter,
    ToggleDnd,

    // ---- misc ----
    Noop,
}

/// Escape key pressed — used to close popups.
pub fn is_escape(event: &Event) -> bool {
    matches!(
        event,
        Event::Keyboard(keyboard::Event::KeyPressed {
            key: keyboard::Key::Named(Named::Escape),
            ..
        })
    )
}
