//! niri integration via the `niri-ipc` crate.
//!
//! Instead of shelling out to `niri msg --json`, the panel keeps a live
//! connection to niri's event stream ([`subscription`]) and sends actions over
//! the same socket. The event stream delivers the full state up-front and then
//! incremental updates, so no polling is required.

use std::time::Duration;

use iced::Subscription;
use iced::futures::channel::mpsc;
use niri_ipc::socket::Socket;
use niri_ipc::{Action, Request, Response, WorkspaceReferenceArg};

use crate::message::Message;

/// Live workspaces/windows updates from niri's event stream.
pub fn subscription() -> Subscription<Message> {
    Subscription::run(event_stream)
}

fn event_stream() -> mpsc::UnboundedReceiver<Message> {
    let (tx, rx) = mpsc::unbounded();

    std::thread::spawn(move || {
        loop {
            // Connect and request the event stream. On any failure, back off
            // briefly and reconnect (niri may not be up yet).
            if let Ok(mut socket) = Socket::connect()
                && matches!(socket.send(Request::EventStream), Ok(Ok(Response::Handled)))
            {
                let mut read_event = socket.read_events();
                while let Ok(event) = read_event() {
                    if tx.unbounded_send(Message::NiriEvent(event)).is_err() {
                        return; // app shut down
                    }
                }
            }
            std::thread::sleep(Duration::from_secs(2));
        }
    });

    rx
}

fn send(request: Request) {
    if let Ok(mut socket) = Socket::connect() {
        let _ = socket.send(request);
    }
}

/// Focus the workspace with the given stable id.
pub fn focus_workspace(id: u64) {
    send(Request::Action(Action::FocusWorkspace {
        reference: WorkspaceReferenceArg::Id(id),
    }));
}

/// Focus the window with the given id.
pub fn focus_window(id: u64) {
    send(Request::Action(Action::FocusWindow { id }));
}
