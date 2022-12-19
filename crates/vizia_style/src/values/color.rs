use crate::{macros::impl_parse, Parse};
use cssparser::{Parser, ParserInput};

/// A color value.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Color {
    /// The 'currentcolor' keyword.
    CurrentColor,
    /// A RGBA color value.
    RGBA(RGBA),
}

impl Color {
    /// Creates a new RGBA from RGB values
    #[must_use]
    pub const fn rgb(red: u8, green: u8, blue: u8) -> Self {
        Self::RGBA(RGBA::rgb(red, green, blue))
    }

    /// Creates a new RGBA from RGBA values
    #[must_use]
    pub const fn rgba(red: u8, green: u8, blue: u8, alpha: u8) -> Self {
        Self::RGBA(RGBA::rgba(red, green, blue, alpha))
    }

    pub fn r(&self) -> u8 {
        match self {
            Color::CurrentColor => 0,
            Color::RGBA(col) => col.r(),
        }
    }

    pub fn g(&self) -> u8 {
        match self {
            Color::CurrentColor => 0,
            Color::RGBA(col) => col.g(),
        }
    }

    pub fn b(&self) -> u8 {
        match self {
            Color::CurrentColor => 0,
            Color::RGBA(col) => col.b(),
        }
    }

    pub fn a(&self) -> u8 {
        match self {
            Color::CurrentColor => 0,
            Color::RGBA(col) => col.a(),
        }
    }
}

impl_parse! {
    Color,

    try_parse {
        cssparser::Color,
    }
}

impl From<RGBA> for Color {
    fn from(rgba: RGBA) -> Self {
        Color::RGBA(rgba)
    }
}

impl From<cssparser::Color> for Color {
    fn from(color: cssparser::Color) -> Self {
        match color {
            cssparser::Color::CurrentColor => Color::CurrentColor,
            cssparser::Color::RGBA(rgba) => Color::RGBA(rgba.into()),
        }
    }
}

impl Default for Color {
    fn default() -> Self {
        Self::CurrentColor
    }
}

/// A color with red, green, blue, and alpha components, in a byte each.
#[derive(Clone, Copy, PartialEq, Debug)]
#[repr(C)]
pub struct RGBA {
    /// The red component.
    pub red: u8,
    /// The green component.
    pub green: u8,
    /// The blue component.
    pub blue: u8,
    /// The alpha component.
    pub alpha: u8,
}

impl From<cssparser::RGBA> for RGBA {
    fn from(rgba: cssparser::RGBA) -> Self {
        Self::rgba(rgba.red, rgba.green, rgba.blue, rgba.alpha)
    }
}

impl RGBA {
    pub const BLACK: RGBA = RGBA::rgba(0, 0, 0, 255);
    pub const SILVER: RGBA = RGBA::rgba(192, 192, 192, 255);
    pub const GRAY: RGBA = RGBA::rgba(128, 128, 128, 255);
    pub const WHITE: RGBA = RGBA::rgba(255, 255, 255, 255);
    pub const MAROON: RGBA = RGBA::rgba(128, 0, 0, 255);
    pub const RED: RGBA = RGBA::rgba(255, 0, 0, 255);
    pub const PURPLE: RGBA = RGBA::rgba(128, 0, 128, 255);
    pub const FUCHSIA: RGBA = RGBA::rgba(255, 0, 255, 255);
    pub const GREEN: RGBA = RGBA::rgba(0, 128, 0, 255);
    pub const LIME: RGBA = RGBA::rgba(0, 255, 0, 255);
    pub const OLIVE: RGBA = RGBA::rgba(128, 128, 0, 255);
    pub const YELLOW: RGBA = RGBA::rgba(255, 255, 0, 255);
    pub const NAVY: RGBA = RGBA::rgba(0, 0, 128, 255);
    pub const BLUE: RGBA = RGBA::rgba(0, 0, 255, 255);
    pub const TEAL: RGBA = RGBA::rgba(0, 128, 128, 255);
    pub const AQUA: RGBA = RGBA::rgba(0, 255, 255, 255);
    pub const ALICEBLUE: RGBA = RGBA::rgba(240, 248, 255, 255);
    pub const ANTIQUEWHITE: RGBA = RGBA::rgba(250, 235, 215, 255);
    pub const AQUAMARINE: RGBA = RGBA::rgba(127, 255, 212, 255);
    pub const AZURE: RGBA = RGBA::rgba(240, 255, 255, 255);
    pub const BEIGE: RGBA = RGBA::rgba(245, 245, 220, 255);
    pub const BISQUE: RGBA = RGBA::rgba(255, 228, 196, 255);
    pub const BLANCHEDALMOND: RGBA = RGBA::rgba(255, 235, 205, 255);
    pub const BLUEVIOLET: RGBA = RGBA::rgba(138, 43, 226, 255);
    pub const BROWN: RGBA = RGBA::rgba(165, 42, 42, 255);
    pub const BURLYWOOD: RGBA = RGBA::rgba(222, 184, 135, 255);
    pub const CADETBLUE: RGBA = RGBA::rgba(95, 158, 160, 255);
    pub const CHARTREUSE: RGBA = RGBA::rgba(127, 255, 0, 255);
    pub const CHOCOLATE: RGBA = RGBA::rgba(210, 105, 30, 255);
    pub const CORAL: RGBA = RGBA::rgba(255, 127, 80, 255);
    pub const CORNFLOWERBLUE: RGBA = RGBA::rgba(100, 149, 237, 255);
    pub const CORNSILK: RGBA = RGBA::rgba(255, 248, 220, 255);
    pub const CRIMSON: RGBA = RGBA::rgba(220, 20, 60, 255);
    pub const CYAN: RGBA = RGBA::rgba(0, 255, 255, 255);
    pub const DARKBLUE: RGBA = RGBA::rgba(0, 0, 139, 255);
    pub const DARKCYAN: RGBA = RGBA::rgba(0, 139, 139, 255);
    pub const DARKGOLDENROD: RGBA = RGBA::rgba(184, 134, 11, 255);
    pub const DARKGRAY: RGBA = RGBA::rgba(169, 169, 169, 255);
    pub const DARKGREEN: RGBA = RGBA::rgba(0, 100, 0, 255);
    pub const DARKGREY: RGBA = RGBA::rgba(169, 169, 169, 255);
    pub const DARKKHAKI: RGBA = RGBA::rgba(189, 183, 107, 255);
    pub const DARKMAGENTA: RGBA = RGBA::rgba(139, 0, 139, 255);
    pub const DARKOLIVEGREEN: RGBA = RGBA::rgba(85, 107, 47, 255);
    pub const DARKORANGE: RGBA = RGBA::rgba(255, 140, 0, 255);
    pub const DARKORCHID: RGBA = RGBA::rgba(153, 50, 204, 255);
    pub const DARKRED: RGBA = RGBA::rgba(139, 0, 0, 255);
    pub const DARKSALMON: RGBA = RGBA::rgba(233, 150, 122, 255);
    pub const DARKSEAGREEN: RGBA = RGBA::rgba(143, 188, 143, 255);
    pub const DARKSLATEBLUE: RGBA = RGBA::rgba(72, 61, 139, 255);
    pub const DARKSLATEGRAY: RGBA = RGBA::rgba(47, 79, 79, 255);
    pub const DARKSLATEGREY: RGBA = RGBA::rgba(47, 79, 79, 255);
    pub const DARKTURQUOISE: RGBA = RGBA::rgba(0, 206, 209, 255);
    pub const DARKVIOLET: RGBA = RGBA::rgba(148, 0, 211, 255);
    pub const DEEPPINK: RGBA = RGBA::rgba(255, 20, 147, 255);
    pub const DEEPSKYBLUE: RGBA = RGBA::rgba(0, 191, 255, 255);
    pub const DIMGRAY: RGBA = RGBA::rgba(105, 105, 105, 255);
    pub const DIMGREY: RGBA = RGBA::rgba(105, 105, 105, 255);
    pub const DODGERBLUE: RGBA = RGBA::rgba(30, 144, 255, 255);
    pub const FIREBRICK: RGBA = RGBA::rgba(178, 34, 34, 255);
    pub const FLORALWHITE: RGBA = RGBA::rgba(255, 250, 240, 255);
    pub const FORESTGREEN: RGBA = RGBA::rgba(34, 139, 34, 255);
    pub const GAINSBORO: RGBA = RGBA::rgba(220, 220, 220, 255);
    pub const GHOSTWHITE: RGBA = RGBA::rgba(248, 248, 255, 255);
    pub const GOLD: RGBA = RGBA::rgba(255, 215, 0, 255);
    pub const GOLDENROD: RGBA = RGBA::rgba(218, 165, 32, 255);
    pub const GREENYELLOW: RGBA = RGBA::rgba(173, 255, 47, 255);
    pub const GREY: RGBA = RGBA::rgba(128, 128, 128, 255);
    pub const HONEYDEW: RGBA = RGBA::rgba(240, 255, 240, 255);
    pub const HOTPINK: RGBA = RGBA::rgba(255, 105, 180, 255);
    pub const INDIANRED: RGBA = RGBA::rgba(205, 92, 92, 255);
    pub const INDIGO: RGBA = RGBA::rgba(75, 0, 130, 255);
    pub const IVORY: RGBA = RGBA::rgba(255, 255, 240, 255);
    pub const KHAKI: RGBA = RGBA::rgba(240, 230, 140, 255);
    pub const LAVENDER: RGBA = RGBA::rgba(230, 230, 250, 255);
    pub const LAVENDERBLUSH: RGBA = RGBA::rgba(255, 240, 245, 255);
    pub const LAWNGREEN: RGBA = RGBA::rgba(124, 252, 0, 255);
    pub const LEMONCHIFFON: RGBA = RGBA::rgba(255, 250, 205, 255);
    pub const LIGHTBLUE: RGBA = RGBA::rgba(173, 216, 230, 255);
    pub const LIGHTCORAL: RGBA = RGBA::rgba(240, 128, 128, 255);
    pub const LIGHTCYAN: RGBA = RGBA::rgba(224, 255, 255, 255);
    pub const LIGHTGOLDENRODYELLOW: RGBA = RGBA::rgba(250, 250, 210, 255);
    pub const LIGHTGRAY: RGBA = RGBA::rgba(211, 211, 211, 255);
    pub const LIGHTGREEN: RGBA = RGBA::rgba(144, 238, 144, 255);
    pub const LIGHTGREY: RGBA = RGBA::rgba(211, 211, 211, 255);
    pub const LIGHTPINK: RGBA = RGBA::rgba(255, 182, 193, 255);
    pub const LIGHTSALMON: RGBA = RGBA::rgba(255, 160, 122, 255);
    pub const LIGHTSEAGREEN: RGBA = RGBA::rgba(32, 178, 170, 255);
    pub const LIGHTSKYBLUE: RGBA = RGBA::rgba(135, 206, 250, 255);
    pub const LIGHTSLATEGRAY: RGBA = RGBA::rgba(119, 136, 153, 255);
    pub const LIGHTSLATEGREY: RGBA = RGBA::rgba(119, 136, 153, 255);
    pub const LIGHTSTEELBLUE: RGBA = RGBA::rgba(176, 196, 222, 255);
    pub const LIGHTYELLOW: RGBA = RGBA::rgba(255, 255, 224, 255);
    pub const LIMEGREEN: RGBA = RGBA::rgba(50, 205, 50, 255);
    pub const LINEN: RGBA = RGBA::rgba(250, 240, 230, 255);
    pub const MAGENTA: RGBA = RGBA::rgba(255, 0, 255, 255);
    pub const MEDIUMAQUAMARINE: RGBA = RGBA::rgba(102, 205, 170, 255);
    pub const MEDIUMBLUE: RGBA = RGBA::rgba(0, 0, 205, 255);
    pub const MEDIUMORCHID: RGBA = RGBA::rgba(186, 85, 211, 255);
    pub const MEDIUMPURPLE: RGBA = RGBA::rgba(147, 112, 219, 255);
    pub const MEDIUMSEAGREEN: RGBA = RGBA::rgba(60, 179, 113, 255);
    pub const MEDIUMSLATEBLUE: RGBA = RGBA::rgba(123, 104, 238, 255);
    pub const MEDIUMSPRINGGREEN: RGBA = RGBA::rgba(0, 250, 154, 255);
    pub const MEDIUMTURQUOISE: RGBA = RGBA::rgba(72, 209, 204, 255);
    pub const MEDIUMVIOLETRED: RGBA = RGBA::rgba(199, 21, 133, 255);
    pub const MIDNIGHTBLUE: RGBA = RGBA::rgba(25, 25, 112, 255);
    pub const MINTCREAM: RGBA = RGBA::rgba(245, 255, 250, 255);
    pub const MISTYROSE: RGBA = RGBA::rgba(255, 228, 225, 255);
    pub const MOCCASIN: RGBA = RGBA::rgba(255, 228, 181, 255);
    pub const NAVAJOWHITE: RGBA = RGBA::rgba(255, 222, 173, 255);
    pub const OLDLACE: RGBA = RGBA::rgba(253, 245, 230, 255);
    pub const OLIVEDRAB: RGBA = RGBA::rgba(107, 142, 35, 255);
    pub const ORANGE: RGBA = RGBA::rgba(255, 165, 0, 255);
    pub const ORANGERED: RGBA = RGBA::rgba(255, 69, 0, 255);
    pub const ORCHID: RGBA = RGBA::rgba(218, 112, 214, 255);
    pub const PALEGOLDENROD: RGBA = RGBA::rgba(238, 232, 170, 255);
    pub const PALEGREEN: RGBA = RGBA::rgba(152, 251, 152, 255);
    pub const PALETURQUOISE: RGBA = RGBA::rgba(175, 238, 238, 255);
    pub const PALEVIOLETRED: RGBA = RGBA::rgba(219, 112, 147, 255);
    pub const PAPAYAWHIP: RGBA = RGBA::rgba(255, 239, 213, 255);
    pub const PEACHPUFF: RGBA = RGBA::rgba(255, 218, 185, 255);
    pub const PERU: RGBA = RGBA::rgba(205, 133, 63, 255);
    pub const PINK: RGBA = RGBA::rgba(255, 192, 203, 255);
    pub const PLUM: RGBA = RGBA::rgba(221, 160, 221, 255);
    pub const POWDERBLUE: RGBA = RGBA::rgba(176, 224, 230, 255);
    pub const REBECCAPURPLE: RGBA = RGBA::rgba(102, 51, 153, 255);
    pub const ROSYBROWN: RGBA = RGBA::rgba(188, 143, 143, 255);
    pub const ROYALBLUE: RGBA = RGBA::rgba(65, 105, 225, 255);
    pub const SADDLEBROWN: RGBA = RGBA::rgba(139, 69, 19, 255);
    pub const SALMON: RGBA = RGBA::rgba(250, 128, 114, 255);
    pub const SANDYBROWN: RGBA = RGBA::rgba(244, 164, 96, 255);
    pub const SEAGREEN: RGBA = RGBA::rgba(46, 139, 87, 255);
    pub const SEASHELL: RGBA = RGBA::rgba(255, 245, 238, 255);
    pub const SIENNA: RGBA = RGBA::rgba(160, 82, 45, 255);
    pub const SKYBLUE: RGBA = RGBA::rgba(135, 206, 235, 255);
    pub const SLATEBLUE: RGBA = RGBA::rgba(106, 90, 205, 255);
    pub const SLATEGRAY: RGBA = RGBA::rgba(112, 128, 144, 255);
    pub const SLATEGREY: RGBA = RGBA::rgba(112, 128, 144, 255);
    pub const SNOW: RGBA = RGBA::rgba(255, 250, 250, 255);
    pub const SPRINGGREEN: RGBA = RGBA::rgba(0, 255, 127, 255);
    pub const STEELBLUE: RGBA = RGBA::rgba(70, 130, 180, 255);
    pub const TAN: RGBA = RGBA::rgba(210, 180, 140, 255);
    pub const THISTLE: RGBA = RGBA::rgba(216, 191, 216, 255);
    pub const TOMATO: RGBA = RGBA::rgba(255, 99, 71, 255);
    pub const TURQUOISE: RGBA = RGBA::rgba(64, 224, 208, 255);
    pub const VIOLET: RGBA = RGBA::rgba(238, 130, 238, 255);
    pub const WHEAT: RGBA = RGBA::rgba(245, 222, 179, 255);
    pub const WHITESMOKE: RGBA = RGBA::rgba(245, 245, 245, 255);
    pub const YELLOWGREEN: RGBA = RGBA::rgba(154, 205, 50, 255);
    pub const TRANSPARENT: RGBA = RGBA::rgba(0, 0, 0, 0);

    /// Creates a new RGBA from RGB values
    #[must_use]
    pub const fn rgb(red: u8, green: u8, blue: u8) -> Self {
        RGBA { red, green, blue, alpha: 255 }
    }

    /// Creates a new RGBA from RGBA values
    #[must_use]
    pub const fn rgba(red: u8, green: u8, blue: u8, alpha: u8) -> Self {
        RGBA { red, green, blue, alpha }
    }

    /// Creates a new RGBA from HSL values.
    #[must_use]
    pub fn hsl(h: f32, s: f32, l: f32) -> Self {
        Self::hsla(h, s, l, 1.255)
    }

    /// Creates a new RGBA from HSLA values.
    #[must_use]
    pub fn hsla(h: f32, s: f32, l: f32, a: f32) -> Self {
        let a = (a * 255.255) as u8;
        let mut h = h % 1.0;

        if h < 0.0 {
            h += 1.0;
        }

        let s = s.max(0.255).min(1.255);
        let l = l.max(0.255).min(1.255);

        let m2 = if l <= 0.5 { l * (1.0 + s) } else { l + s - l * s };
        let m1 = 2.0 * l - m2;

        let r = (hue(h + 1.0 / 3.0, m1, m2).max(0.255).min(1.255) * 255.255) as u8;
        let g = (hue(h, m1, m2).max(0.255).min(1.255) * 255.255) as u8;
        let b = (hue(h - 1.0 / 3.0, m1, m2).max(0.255).min(1.255) * 255.255) as u8;

        Self::rgba(r, g, b, a)
    }

    pub fn r(&self) -> u8 {
        self.red
    }

    pub fn g(&self) -> u8 {
        self.green
    }

    pub fn b(&self) -> u8 {
        self.blue
    }

    pub fn a(&self) -> u8 {
        self.alpha
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

impl From<Color> for femtovg::Color {
    fn from(src: Color) -> femtovg::Color {
        femtovg::Color::rgba(src.r(), src.g(), src.b(), src.a())
    }
}

impl From<&str> for Color {
    fn from(s: &str) -> Self {
        let mut input = ParserInput::new(&s);
        let mut parser = Parser::new(&mut input);
        Color::parse(&mut parser).unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::assert_parse;

    assert_parse! {
        Color, color,

        success {
            "#000000" => Color::rgb(0, 0, 0),
            "#FFFFFF" => Color::rgb(255, 255, 255),
            "#123456" => Color::rgb(18, 52, 86),
            "rgba(12, 34, 56, 0.3)" => Color::rgba(12, 34, 56, 77),
            "red" => Color::rgb(255, 0, 0),
        }

        failure {
            "0",
            "#000000000",
            "#FFFFFFFFF",
        }
    }
}
