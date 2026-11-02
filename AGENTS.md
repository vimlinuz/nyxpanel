# AGENTS.md

## Project

`nyx_panel` is a Wayland panel/bar for the [niri](https://github.com/YaLTeR/niri)
compositor, written in Rust with [`iced`](https://docs.rs/iced) and
[`iced_layershell`](https://docs.rs/iced_layershell). It is a port of a
quickshell (QML) configuration located at
`~/nixos/homes/santosh/config/nyx_panel` (vertical left bar with workspaces,
clock/calendar, battery and notifications).

## Build / run

The build requires native Wayland/GPU libraries and `pkg-config`; use the Nix
flake dev shell. Run everything through `nix develop`:

```sh
nix develop --command cargo build        # build
nix develop --command cargo run          # run (needs a live niri/Wayland session)
nix develop --command cargo check        # fast type check
nix develop --command cargo-clippy       # lints (invoke cargo-clippy directly)
nix develop --command rustfmt --edition 2024 $(find src -name '*.rs')  # format
```

Notes:

- `cargo fmt` / `cargo clippy` may resolve rustup shims from `~/.cargo/bin` and
  fail with `libz.so.1` errors. Call the Nix binaries (`rustfmt`,
  `cargo-clippy`) directly instead, as shown above.
- There are no tests wired up yet; `cargo check` / `cargo-clippy` are the
  verification steps.
- The flake builds the crate with `naersk` (`nix build`) and the dev shell
  carries the runtime deps on the RPATH.

## Architecture

- `src/main.rs` — entry point. Builds the `iced_layershell` daemon
  (`build_pattern::daemon`), defines the initial bar layer surface, and holds
  `boot` / `update` / `view` / `subscription`, window-kind dispatch and popup
  open/close helpers.
- `src/message.rs` — the shared `Message` enum, annotated with
  `#[to_layer_message(multi)]`. This macro injects layer-shell control variants
  (`NewLayerShell`, `RemoveWindow`, ...); those are intercepted by the runtime
  and never reach `update`.
- `src/state.rs` — `State`, `WindowKind`. Popup windows are tracked in
  `State::ids` (map `window::Id -> WindowKind`); the initial bar surface has no
  registered id, so `kind_of(id) == None` means "bar". niri state lives in
  `State::niri` (`niri_ipc::state::EventStreamState`); UI code reads it through
  `State::workspaces()` / `State::active_windows()`.
- `src/system.rs` — blocking system integration run inside `Task::perform`:
  sysfs battery.
- `src/niri.rs` — niri integration via the `niri-ipc` crate. A long-lived
  background thread connects to `$NIRI_SOCKET`, requests `Request::EventStream`
  and forwards every `Event` as `Message::NiriEvent` (no polling). Workspace /
  window focus actions and `quit` are sent as `Request::Action`.
- `src/theme.rs`, `src/fonts.rs` — the "vague" colorscheme palette and the
  `JetBrainsMono Nerd Font` family. The Regular/Bold TTFs are vendored under
  `assets/fonts/` and embedded with `include_bytes!`, then registered in
  `main.rs` via `Settings::fonts`, so icons render even where the font isn't
  installed system-wide. `assets/fonts/OFL.txt` is the SIL OFL 1.1 license.
- `src/ui/` — views: `bar.rs` (bar + widgets), `calendar.rs`,
  `center.rs` (minimal notification center), `widgets.rs` (shared helpers).

### Adding a popup window

1. Open it with `Task::done(Message::NewLayerShell { settings, id })` using a
   fresh `iced::window::Id::unique()`, register the id via `State::register`,
   and add a `WindowKind` match arm in `main.rs::view`.
2. Close it with `Task::done(Message::RemoveWindow(id))` and `State::unregister`.
   `Message::WindowClosed(id)` also unregisters.
3. Escape closes any open popup (see `handle_event`).

## Known gaps / caveats

- `winit-core` is pinned to `=0.31.0-beta.2` in `Cargo.toml`.
  `iced_exdevtools 0.19.1` (a hard dependency of `iced_layershell`) matches
  exhaustively on `NativeKeyCode`, which became `#[non_exhaustive]` in
  `0.31.0-beta.3`, breaking the build. Remove the pin once upstream fixes it.
- Notification center is only a UI shell; there is no
  `org.freedesktop.Notifications` D-Bus daemon wired up yet. DND is a local
  boolean toggle only.
- No StatusNotifierItem system tray and no workspace thumbnails yet.
