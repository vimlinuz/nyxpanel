//! Font helpers for the panel.
//!
//! The panel ships its own copy of the JetBrains Mono Nerd Font (Regular and
//! Bold) under `assets/fonts/`, so icons and glyphs render identically on any
//! system, whether or not the font is installed globally. The bytes are loaded
//! into the renderer in `main.rs` via `Settings::fonts`.

use iced::Font;

/// The font family embedded in the vendored TTF files.
pub const FAMILY: &str = "JetBrainsMono Nerd Font";

/// Embedded Regular face, loaded into the renderer at boot.
pub const NERD_REGULAR_BYTES: &[u8] =
    include_bytes!("../assets/fonts/JetBrainsMonoNerdFont-Regular.ttf");

/// Embedded Bold face, loaded into the renderer at boot.
pub const NERD_BOLD_BYTES: &[u8] = include_bytes!("../assets/fonts/JetBrainsMonoNerdFont-Bold.ttf");

/// JetBrains Mono Nerd Font, resolved from the embedded faces.
pub const NERD: Font = Font::with_name(FAMILY);

pub const NERD_BOLD: Font = Font {
    weight: iced::font::Weight::Bold,
    ..NERD
};
