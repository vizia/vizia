use crate::animation::Interpolator;
use std::fmt;
use std::fmt::Formatter;

/// Describes a color.
///
/// This type is part of the prelude.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Copy, Clone)]
pub struct Color {
    pub data: u32,
}

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if self.a() == 0 {
            write!(f, "transparent")
        } else if self.a() == 255 {
            write!(f, "#{:02x}{:02x}{:02x}", self.r(), self.g(), self.b())
        } else {
            write!(f, "#{:02x}{:02x}{:02x}{:02x}", self.r(), self.g(), self.b(), self.a())
        }
    }
}

impl Color {
    // Create a new color from RGB
    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Color { data: ((r as u32) << 24) | ((g as u32) << 16) | ((b as u32) << 8) | 0x0000_00FF }
    }

    // Create a new color from RGBA
    pub const fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Color { data: ((r as u32) << 24) | ((g as u32) << 16) | ((b as u32) << 8) | (a as u32) }
    }

    // Create a new color from raw RGBA data represented by `u32`
    pub const fn from_raw(data: u32) -> Self {
        Color { data }
    }

    /// Returns color value specified by hue, saturation and lightness.
    /// HSL values are all in range [0..1], alpha will be set to 1.0.
    pub fn hsl(h: f32, s: f32, l: f32) -> Self {
        Self::hsla(h, s, l, 1.0)
    }

    /// Returns color value specified by hue, saturation, lightness and alpha.
    /// All values are in range [0..1]
    pub fn hsla(h: f32, s: f32, l: f32, a: f32) -> Self {
        let a = (a * 255.0) as u8;
        let mut h = h % 1.0;

        if h < 0.0 {
            h += 1.0;
        }

        let s = s.max(0.0).min(1.0);
        let l = l.max(0.0).min(1.0);

        let m2 = if l <= 0.5 { l * (1.0 + s) } else { l + s - l * s };
        let m1 = 2.0 * l - m2;

        let r = (hue(h + 1.0 / 3.0, m1, m2).max(0.0).min(1.0) * 255.0) as u8;
        let g = (hue(h, m1, m2).max(0.0).min(1.0) * 255.0) as u8;
        let b = (hue(h - 1.0 / 3.0, m1, m2).max(0.0).min(1.0) * 255.0) as u8;

        Color { data: ((r as u32) << 24) | ((g as u32) << 16) | ((b as u32) << 8) | (a as u32) }
    }

    // Get the red value
    pub fn r(self) -> u8 {
        ((self.data & 0xFF00_0000) >> 24) as u8
    }

    // Get the green value
    pub fn g(self) -> u8 {
        ((self.data & 0x00FF_0000) >> 16) as u8
    }

    // Get the blue value
    pub fn b(self) -> u8 {
        ((self.data & 0x0000_FF00) >> 8) as u8
    }

    // Get the alpha value
    pub fn a(self) -> u8 {
        (self.data & 0x0000_00FF) as u8
    }

    // Interpolate between two colors
    pub fn interpolate(start_color: Color, end_color: Color, scale: f64) -> Color {
        let r = Color::interp(start_color.r(), end_color.r(), scale);
        let g = Color::interp(start_color.g(), end_color.g(), scale);
        let b = Color::interp(start_color.b(), end_color.b(), scale);
        let a = Color::interp(start_color.a(), end_color.a(), scale);
        Color::rgba(r, g, b, a)
    }

    fn interp(start_color: u8, end_color: u8, scale: f64) -> u8 {
        (end_color as f64 - start_color as f64).mul_add(scale, start_color as f64) as u8
    }
}

impl From<&str> for Color {
    fn from(s: &str) -> Color {
        let clean_hex = s.trim_start_matches('#');
        match clean_hex.len() {
            3 | 4 => {
                let hex = clean_hex.as_bytes();
                let r = (hex[0] as char).to_digit(16).unwrap() as u8 * 17;
                let g = (hex[1] as char).to_digit(16).unwrap() as u8 * 17;
                let b = (hex[2] as char).to_digit(16).unwrap() as u8 * 17;

                let mut data = ((r as u32) << 24) | ((g as u32) << 16) | ((b as u32) << 8);

                if clean_hex.len() == 3 {
                    data |= 0x00_000_0FF;
                } else {
                    let a = (hex[0] as char).to_digit(16).unwrap() as u8 * 17;
                    data |= a as u32;
                }

                Color { data }
            }

            6 | 8 => {
                let mut x = match u32::from_str_radix(&clean_hex, 16) {
                    Ok(x) => x,
                    Err(_) => 0,
                };

                if clean_hex.len() == 6 {
                    x = (x << 8) | 0x00_000_0FF;
                }

                Color { data: x }
            }
            _ => Color { data: 0 },
        }
    }
}

impl From<String> for Color {
    fn from(s: String) -> Color {
        Color::from(s.as_str())
    }
}

impl From<Color> for femtovg::Color {
    fn from(src: Color) -> femtovg::Color {
        femtovg::Color::rgba(src.r(), src.g(), src.b(), src.a())
    }
}

impl PartialEq for Color {
    fn eq(&self, other: &Color) -> bool {
        self.data == other.data
    }
}

impl std::fmt::Debug for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "rgba({}, {}, {}, {})", self.r(), self.g(), self.b(), self.a())
    }
}

impl Default for Color {
    fn default() -> Self {
        Color::rgba(0, 0, 0, 0)
    }
}

impl Interpolator for Color {
    fn interpolate(start: &Self, end: &Self, t: f32) -> Self {
        Color::interpolate(start.clone(), end.clone(), t as f64)
    }
}

fn hue(mut h: f32, m1: f32, m2: f32) -> f32 {
    if h < 0.0 {
        h += 1.0;
    }
    if h > 1.0 {
        h -= 1.0;
    }

    if h < 1.0 / 6.0 {
        return m1 + (m2 - m1) * h * 6.0;
    }
    if h < 3.0 / 6.0 {
        return m2;
    }
    if h < 4.0 / 6.0 {
        return m1 + (m2 - m1) * (2.0 / 3.0 - h) * 6.0;
    }

    m1
}

macro_rules! implement_color {
    ($name:ident, $c0:tt, $c1:tt, $c2:tt, $c3:tt, $c4:tt, $c5:tt, $c6:tt, $c7:tt, $c8:tt, $c9:tt) => {
        paste::paste! {
            pub const [<$name _50>]: Color = Color::from_raw([<$c0 FF>]);
            pub const [<$name _100>]: Color = Color::from_raw([<$c1 FF>]);
            pub const [<$name _200>]: Color = Color::from_raw([<$c2 FF>]);
            pub const [<$name _300>]: Color = Color::from_raw([<$c3 FF>]);
            pub const [<$name _400>]: Color = Color::from_raw([<$c4 FF>]);
            pub const [<$name _500>]: Color = Color::from_raw([<$c5 FF>]);
            pub const [<$name _600>]: Color = Color::from_raw([<$c6 FF>]);
            pub const [<$name _700>]: Color = Color::from_raw([<$c7 FF>]);
            pub const [<$name _800>]: Color = Color::from_raw([<$c8 FF>]);
            pub const [<$name _900>]: Color = Color::from_raw([<$c9 FF>]);
        }
    };
}

impl Color {
    #[deprecated]
    pub const fn transparent() -> Self {
        Self { data: 0x00000000 }
    }

    #[deprecated]
    pub const fn black() -> Self {
        Self { data: 0x000000FF }
    }

    #[deprecated]
    pub const fn white() -> Self {
        Self { data: 0xFFFFFFFF }
    }

    #[deprecated]
    pub const fn red() -> Self {
        Self { data: 0xFF0000FF }
    }

    #[deprecated]
    pub const fn green() -> Self {
        Self { data: 0x008000FF }
    }

    #[deprecated]
    pub const fn blue() -> Self {
        Self { data: 0x0000FFFF }
    }

    #[deprecated]
    pub const fn yellow() -> Self {
        Self { data: 0xFFFF00FF }
    }

    #[deprecated]
    pub const fn cyan() -> Self {
        Self { data: 0x00FFFFFF }
    }

    #[deprecated]
    pub const fn magenta() -> Self {
        Self { data: 0xFF00FFFF }
    }

    pub const TRANSPARENT: Color = Color::from_raw(0x00000000);
    pub const WHITE: Color = Color::from_raw(0xFFFFFFFF);
    pub const BLACK: Color = Color::from_raw(0x000000FF);
    implement_color!(
        RED, 0xFFEBEE, 0xFFCDD2, 0xEF9A9A, 0xE57373, 0xEF5350, 0xF44336, 0xE53935, 0xD32F2F,
        0xC62828, 0xB71C1C
    );
    implement_color!(
        PINK, 0xFCE4EC, 0xF8BBD0, 0xF48FB1, 0xF06292, 0xEC407A, 0xE91E63, 0xD81B60, 0xC2185B,
        0xAD1457, 0x880E4F
    );
    implement_color!(
        PURPLE, 0xF3E5F5, 0xE1BEE7, 0xCE93D8, 0xBA68C8, 0xAB47BC, 0x9C27B0, 0x8E24AA, 0x7B1FA2,
        0x6A1B9A, 0x4A148C
    );
    implement_color!(
        DEEP_PURPLE,
        0xEDE7F6,
        0xD1C4E9,
        0xB39DDB,
        0x9575CD,
        0x7E57C2,
        0x673AB7,
        0x5E35B1,
        0x512DA8,
        0x4527A0,
        0x311B92
    );
    implement_color!(
        INDIGO, 0xE8EAF6, 0xC5CAE9, 0x9FA8DA, 0x7986CB, 0x5C6BC0, 0x3F51B5, 0x3949AB, 0x303F9F,
        0x283593, 0x1A237E
    );
    implement_color!(
        BLUE, 0xE3F2FD, 0xBBDEFB, 0x90CAF9, 0x64B5F6, 0x42A5F5, 0x2196F3, 0x1E88E5, 0x1976D2,
        0x1565C0, 0x0D47A1
    );
    implement_color!(
        LIGHT_BLUE, 0xE1F5FE, 0xB3E5FC, 0x81D4FA, 0x4FC3F7, 0x29B6F6, 0x03A9F4, 0x039BE5, 0x0288D1,
        0x0277BD, 0x01579B
    );
    implement_color!(
        CYAN, 0xE0F7FA, 0xB2EBF2, 0x80DEEA, 0x4DD0E1, 0x26C6DA, 0x00BCD4, 0x00ACC1, 0x0097A7,
        0x00838F, 0x006064
    );
    implement_color!(
        TEAL, 0xE0F2F1, 0xE0F2F1, 0x80CBC4, 0x4DB6AC, 0x26A69A, 0x009688, 0x00897B, 0x00796B,
        0x00695C, 0x004D40
    );
    implement_color!(
        GREEN, 0xE8F5E9, 0xC8E6C9, 0xA5D6A7, 0x81C784, 0x66BB6A, 0x4CAF50, 0x43A047, 0x388E3C,
        0x2E7D32, 0x1B5E20
    );
    implement_color!(
        LIGHT_GREEN,
        0xF1F8E9,
        0xDCEDC8,
        0xC5E1A5,
        0xAED581,
        0x9CCC65,
        0x8BC34A,
        0x7CB342,
        0x7CB342,
        0x558B2F,
        0x33691E
    );
    implement_color!(
        LIME, 0xF9FBE7, 0xF0F4C3, 0xE6EE9C, 0xDCE775, 0xD4E157, 0xCDDC39, 0xC0CA33, 0xAFB42B,
        0x9E9D24, 0x9E9D24
    );
    implement_color!(
        YELLOW, 0xFFFDE7, 0xFFF9C4, 0xFFF59D, 0xFFF176, 0xFFEE58, 0xFFEB3B, 0xFDD835, 0xFBC02D,
        0xF9A825, 0xF57F17
    );
    implement_color!(
        AMBER, 0xFFF8E1, 0xFFECB3, 0xFFE082, 0xFFD54F, 0xFFCA28, 0xFFC107, 0xFFB300, 0xFFA000,
        0xFF8F00, 0xFF6F00
    );
    implement_color!(
        ORANGE, 0xFFF3E0, 0xFFE0B2, 0xFFCC80, 0xFFB74D, 0xFFA726, 0xFF9800, 0xFB8C00, 0xF57C00,
        0xEF6C00, 0xE65100
    );
    implement_color!(
        DEEP_ORANGE,
        0xFBE9E7,
        0xFFCCBC,
        0xFFAB91,
        0xFF8A65,
        0xFF7043,
        0xFF5722,
        0xF4511E,
        0xE64A19,
        0xD84315,
        0xD84315
    );
    implement_color!(
        BROWN, 0xEFEBE9, 0xD7CCC8, 0xBCAAA4, 0xA1887F, 0x8D6E63, 0x795548, 0x6D4C41, 0x5D4037,
        0x4E342E, 0x3E2723
    );
    implement_color!(
        GREY, 0xFAFAFA, 0xF5F5F5, 0xEEEEEE, 0xE0E0E0, 0xBDBDBD, 0xBDBDBD, 0x757575, 0x616161,
        0x424242, 0x212121
    );
    implement_color!(
        BLUE_GREY, 0xECEFF1, 0xCFD8DC, 0xB0BEC5, 0x90A4AE, 0x78909C, 0x607D8B, 0x546E7A, 0x455A64,
        0x37474F, 0x263238
    );
}

#[cfg(test)]
mod tests {
    use super::Color;

    #[test]
    fn test_hex() {
        let hex_color = "#FF00FF88";
        let color = Color::from(hex_color);

        assert_eq!(color, Color::rgba(255, 0, 255, 136));
    }

    #[test]
    fn test_short_hex() {
        let hex_color = "#FFF";
        let color = Color::from(hex_color);

        assert_eq!(color, Color::rgba(255, 255, 255, 255));
    }
}
