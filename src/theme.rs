//! Vague colorscheme (https://github.com/vague-theme/vague.nvim).
//!
//! Central palette so the whole panel stays consistent with the quickshell theme.

use iced::Color;

pub const fn rgb(r: u8, g: u8, b: u8) -> Color {
    Color::from_rgb8(r, g, b)
}

pub const fn rgba(r: u8, g: u8, b: u8, a: u8) -> Color {
    Color::from_rgba8(r, g, b, a as f32 / 255.0)
}

// Neutrals.
#[allow(dead_code)]
pub const BG: Color = rgb(0x14, 0x14, 0x15);
#[allow(dead_code)]
pub const BG_INACTIVE: Color = rgb(0x1c, 0x1c, 0x24);
pub const BG_PANEL: Color = rgba(0x14, 0x14, 0x15, 0xbb);
#[allow(dead_code)]
pub const LINE: Color = rgb(0x25, 0x25, 0x30);
pub const FG: Color = rgb(0xcd, 0xcd, 0xcd);
pub const FG_BRIGHT: Color = rgb(0xd7, 0xd7, 0xd7);
pub const MUTED: Color = rgb(0x87, 0x87, 0x87);
pub const COMMENT: Color = rgb(0x60, 0x60, 0x79);
pub const BORDER: Color = rgb(0x87, 0x87, 0x87);
#[allow(dead_code)]
pub const BORDER_DIM: Color = rgb(0x60, 0x60, 0x79);

// Semantic.
#[allow(dead_code)]
pub const ACCENT: Color = rgb(0x7e, 0x98, 0xe8);
#[allow(dead_code)]
pub const ACCENT_ALT: Color = rgb(0xae, 0xae, 0xd1);
pub const ERROR: Color = rgb(0xd8, 0x64, 0x7e);
pub const WARNING: Color = rgb(0xf3, 0xbe, 0x7c);
pub const SUCCESS: Color = rgb(0x7f, 0xa5, 0x63);
pub const VISUAL: Color = rgb(0x33, 0x37, 0x38);

/// Lerp two colors.
pub fn mix(a: Color, b: Color, t: f32) -> Color {
    let t = t.clamp(0.0, 1.0);
    Color {
        r: a.r + (b.r - a.r) * t,
        g: a.g + (b.g - a.g) * t,
        b: a.b + (b.b - a.b) * t,
        a: a.a + (b.a - a.a) * t,
    }
}

/// Blend a color over an opaque background (approximates Qt alpha compositing).
#[allow(dead_code)]
pub fn over(fg: Color, bg: Color) -> Color {
    let a = fg.a + bg.a * (1.0 - fg.a);
    if a <= 0.0 {
        return Color::TRANSPARENT;
    }
    Color {
        r: (fg.r * fg.a + bg.r * bg.a * (1.0 - fg.a)) / a,
        g: (fg.g * fg.a + bg.g * bg.a * (1.0 - fg.a)) / a,
        b: (fg.b * fg.a + bg.b * bg.a * (1.0 - fg.a)) / a,
        a,
    }
}
