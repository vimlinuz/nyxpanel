//! Application state.

use std::collections::HashMap;

use chrono::{DateTime, Datelike, Local};
use niri_ipc::state::{EventStreamState, EventStreamStatePart};
use niri_ipc::{Window, Workspace};

use crate::system::BatteryInfo;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindowKind {
    #[allow(dead_code)]
    Bar,
    Calendar,
    Center,
}

pub struct State {
    pub hover: Option<&'static str>,
    pub now: DateTime<Local>,
    pub tick: u64,

    pub battery: BatteryInfo,

    pub niri: EventStreamState,

    pub dnd: bool,

    // calendar popup
    pub calendar_open: bool,
    pub cal_month: u32, // 0-11
    pub cal_year: i32,

    // notification center (minimal)
    pub center_open: bool,

    pub ids: HashMap<iced::window::Id, WindowKind>,
}

impl Default for State {
    fn default() -> Self {
        let now = Local::now();
        Self {
            hover: None,
            now,
            tick: 0,
            battery: BatteryInfo::default(),
            niri: EventStreamState::default(),
            dnd: false,
            calendar_open: false,
            cal_month: now.month0(),
            cal_year: now.year(),
            center_open: false,
            ids: HashMap::new(),
        }
    }
}

impl State {
    pub fn kind_of(&self, id: iced::window::Id) -> Option<WindowKind> {
        self.ids.get(&id).copied()
    }

    pub fn id_of(&self, kind: WindowKind) -> Option<iced::window::Id> {
        self.ids
            .iter()
            .find(|(_, k)| **k == kind)
            .map(|(id, _)| *id)
    }

    pub fn register(&mut self, id: iced::window::Id, kind: WindowKind) {
        self.ids.insert(id, kind);
    }

    pub fn unregister(&mut self, id: iced::window::Id) {
        self.ids.remove(&id);
    }

    /// Applies a niri event-stream update (workspaces, windows, ...).
    pub fn apply_niri_event(&mut self, event: niri_ipc::Event) {
        self.niri.apply(event);
    }

    /// Workspaces sorted by their position on the output.
    pub fn workspaces(&self) -> Vec<&Workspace> {
        let mut list: Vec<&Workspace> = self.niri.workspaces.workspaces.values().collect();
        list.sort_by_key(|w| (w.idx, w.id));
        list
    }

    /// Whether any window is currently on the given workspace.
    pub fn workspace_has_window(&self, workspace_id: u64) -> bool {
        self.niri
            .windows
            .windows
            .values()
            .any(|w| w.workspace_id == Some(workspace_id))
    }

    pub fn active_workspace_id(&self) -> Option<u64> {
        self.niri
            .workspaces
            .workspaces
            .values()
            .find(|w| w.is_active)
            .map(|w| w.id)
    }

    /// Windows on the active workspace, in reading order.
    pub fn active_windows(&self) -> Vec<&Window> {
        let Some(ws) = self.active_workspace_id() else {
            return Vec::new();
        };
        let mut list: Vec<&Window> = self
            .niri
            .windows
            .windows
            .values()
            .filter(|w| w.workspace_id == Some(ws))
            .collect();
        list.sort_by_key(|w| {
            w.layout
                .pos_in_scrolling_layout
                .unwrap_or((usize::MAX, usize::MAX))
        });
        list
    }
}
