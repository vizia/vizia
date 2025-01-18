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

    /// Return a new [RGBA] from the Color
    pub fn get_rgba(&self) -> RGBA {
        match self {
            Color::CurrentColor => RGBA::rgba(0, 0, 0, 0),
            Color::RGBA(rgba) => *rgba,
        }
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
        cssparser_color::Color,
    }
}

impl From<RGBA> for Color {
    fn from(rgba: RGBA) -> Self {
        Color::RGBA(rgba)
    }
}

impl From<cssparser_color::Color> for Color {
    fn from(color: cssparser_color::Color) -> Self {
        match color {
            cssparser_color::Color::CurrentColor => Color::CurrentColor,
            cssparser_color::Color::Rgba(rgba) => Color::RGBA(rgba.into()),
            _ => Color::CurrentColor,
        }
    }
}

impl Default for Color {
    fn default() -> Self {
        Self::CurrentColor
    }
}

impl Color {
    pub const fn black() -> Self {
        Self::RGBA(RGBA::BLACK)
    }
    pub const fn silver() -> Self {
        Self::RGBA(RGBA::SILVER)
    }
    pub const fn gray() -> Self {
        Self::RGBA(RGBA::GRAY)
    }
    pub const fn white() -> Self {
        Self::RGBA(RGBA::WHITE)
    }
    pub const fn maroon() -> Self {
        Self::RGBA(RGBA::MAROON)
    }
    pub const fn red() -> Self {
        Self::RGBA(RGBA::RED)
    }
    pub const fn purple() -> Self {
        Self::RGBA(RGBA::PURPLE)
    }
    pub const fn fuchsia() -> Self {
        Self::RGBA(RGBA::FUCHSIA)
    }
    pub const fn green() -> Self {
        Self::RGBA(RGBA::GREEN)
    }
    pub const fn lime() -> Self {
        Self::RGBA(RGBA::LIME)
    }
    pub const fn olive() -> Self {
        Self::RGBA(RGBA::OLIVE)
    }
    pub const fn yellow() -> Self {
        Self::RGBA(RGBA::YELLOW)
    }
    pub const fn navy() -> Self {
        Self::RGBA(RGBA::NAVY)
    }
    pub const fn blue() -> Self {
        Self::RGBA(RGBA::BLUE)
    }
    pub const fn teal() -> Self {
        Self::RGBA(RGBA::TEAL)
    }
    pub const fn aqua() -> Self {
        Self::RGBA(RGBA::AQUA)
    }
    pub const fn aliceblue() -> Self {
        Self::RGBA(RGBA::ALICEBLUE)
    }
    pub const fn antiquewhite() -> Self {
        Self::RGBA(RGBA::ANTIQUEWHITE)
    }
    pub const fn aquamarine() -> Self {
        Self::RGBA(RGBA::AQUAMARINE)
    }
    pub const fn azure() -> Self {
        Self::RGBA(RGBA::AZURE)
    }
    pub const fn beige() -> Self {
        Self::RGBA(RGBA::BEIGE)
    }
    pub const fn bisque() -> Self {
        Self::RGBA(RGBA::BISQUE)
    }
    pub const fn blanchedalmond() -> Self {
        Self::RGBA(RGBA::BLANCHEDALMOND)
    }
    pub const fn blueviolet() -> Self {
        Self::RGBA(RGBA::BLUEVIOLET)
    }
    pub const fn brown() -> Self {
        Self::RGBA(RGBA::BROWN)
    }
    pub const fn burlywood() -> Self {
        Self::RGBA(RGBA::BURLYWOOD)
    }
    pub const fn cadetblue() -> Self {
        Self::RGBA(RGBA::CADETBLUE)
    }
    pub const fn chartreuse() -> Self {
        Self::RGBA(RGBA::CHARTREUSE)
    }
    pub const fn chocolate() -> Self {
        Self::RGBA(RGBA::CHOCOLATE)
    }
    pub const fn coral() -> Self {
        Self::RGBA(RGBA::CORAL)
    }
    pub const fn cornflowerblue() -> Self {
        Self::RGBA(RGBA::CORNFLOWERBLUE)
    }
    pub const fn cornsilk() -> Self {
        Self::RGBA(RGBA::CORNSILK)
    }
    pub const fn crimson() -> Self {
        Self::RGBA(RGBA::CRIMSON)
    }
    pub const fn cyan() -> Self {
        Self::RGBA(RGBA::CYAN)
    }
    pub const fn darkblue() -> Self {
        Self::RGBA(RGBA::DARKBLUE)
    }
    pub const fn darkcyan() -> Self {
        Self::RGBA(RGBA::DARKCYAN)
    }
    pub const fn darkgoldenrod() -> Self {
        Self::RGBA(RGBA::DARKGOLDENROD)
    }
    pub const fn darkgray() -> Self {
        Self::RGBA(RGBA::DARKGRAY)
    }
    pub const fn darkgreen() -> Self {
        Self::RGBA(RGBA::DARKGREEN)
    }
    pub const fn darkgrey() -> Self {
        Self::RGBA(RGBA::DARKGREY)
    }
    pub const fn darkkhaki() -> Self {
        Self::RGBA(RGBA::DARKKHAKI)
    }
    pub const fn darkmagenta() -> Self {
        Self::RGBA(RGBA::DARKMAGENTA)
    }
    pub const fn darkolivegreen() -> Self {
        Self::RGBA(RGBA::DARKOLIVEGREEN)
    }
    pub const fn darkorange() -> Self {
        Self::RGBA(RGBA::DARKORANGE)
    }
    pub const fn darkorchid() -> Self {
        Self::RGBA(RGBA::DARKORCHID)
    }
    pub const fn darkred() -> Self {
        Self::RGBA(RGBA::DARKRED)
    }
    pub const fn darksalmon() -> Self {
        Self::RGBA(RGBA::DARKSALMON)
    }
    pub const fn darkseagreen() -> Self {
        Self::RGBA(RGBA::DARKSEAGREEN)
    }
    pub const fn darkslateblue() -> Self {
        Self::RGBA(RGBA::DARKSLATEBLUE)
    }
    pub const fn darkslategray() -> Self {
        Self::RGBA(RGBA::DARKSLATEGRAY)
    }
    pub const fn darkslategrey() -> Self {
        Self::RGBA(RGBA::DARKSLATEGREY)
    }
    pub const fn darkturquoise() -> Self {
        Self::RGBA(RGBA::DARKTURQUOISE)
    }
    pub const fn darkviolet() -> Self {
        Self::RGBA(RGBA::DARKVIOLET)
    }
    pub const fn deeppink() -> Self {
        Self::RGBA(RGBA::DEEPPINK)
    }
    pub const fn deepskyblue() -> Self {
        Self::RGBA(RGBA::DEEPSKYBLUE)
    }
    pub const fn dimgray() -> Self {
        Self::RGBA(RGBA::DIMGRAY)
    }
    pub const fn dimgrey() -> Self {
        Self::RGBA(RGBA::DIMGREY)
    }
    pub const fn dodgerblue() -> Self {
        Self::RGBA(RGBA::DODGERBLUE)
    }
    pub const fn firebrick() -> Self {
        Self::RGBA(RGBA::FIREBRICK)
    }
    pub const fn floralwhite() -> Self {
        Self::RGBA(RGBA::FLORALWHITE)
    }
    pub const fn forestgreen() -> Self {
        Self::RGBA(RGBA::FORESTGREEN)
    }
    pub const fn gainsboro() -> Self {
        Self::RGBA(RGBA::GAINSBORO)
    }
    pub const fn ghostwhite() -> Self {
        Self::RGBA(RGBA::GHOSTWHITE)
    }
    pub const fn gold() -> Self {
        Self::RGBA(RGBA::GOLD)
    }
    pub const fn goldenrod() -> Self {
        Self::RGBA(RGBA::GOLDENROD)
    }
    pub const fn greenyellow() -> Self {
        Self::RGBA(RGBA::GREENYELLOW)
    }
    pub const fn grey() -> Self {
        Self::RGBA(RGBA::GREY)
    }
    pub const fn honeydew() -> Self {
        Self::RGBA(RGBA::HONEYDEW)
    }
    pub const fn hotpink() -> Self {
        Self::RGBA(RGBA::HOTPINK)
    }
    pub const fn indianred() -> Self {
        Self::RGBA(RGBA::INDIANRED)
    }
    pub const fn indigo() -> Self {
        Self::RGBA(RGBA::INDIGO)
    }
    pub const fn ivory() -> Self {
        Self::RGBA(RGBA::IVORY)
    }
    pub const fn khaki() -> Self {
        Self::RGBA(RGBA::KHAKI)
    }
    pub const fn lavender() -> Self {
        Self::RGBA(RGBA::LAVENDER)
    }
    pub const fn lavenderblush() -> Self {
        Self::RGBA(RGBA::LAVENDERBLUSH)
    }
    pub const fn lawngreen() -> Self {
        Self::RGBA(RGBA::LAWNGREEN)
    }
    pub const fn lemonchiffon() -> Self {
        Self::RGBA(RGBA::LEMONCHIFFON)
    }
    pub const fn lightblue() -> Self {
        Self::RGBA(RGBA::LIGHTBLUE)
    }
    pub const fn lightcoral() -> Self {
        Self::RGBA(RGBA::LIGHTCORAL)
    }
    pub const fn lightcyan() -> Self {
        Self::RGBA(RGBA::LIGHTCYAN)
    }
    pub const fn lightgoldenrodyellow() -> Self {
        Self::RGBA(RGBA::LIGHTGOLDENRODYELLOW)
    }
    pub const fn lightgray() -> Self {
        Self::RGBA(RGBA::LIGHTGRAY)
    }
    pub const fn lightgreen() -> Self {
        Self::RGBA(RGBA::LIGHTGREEN)
    }
    pub const fn lightgrey() -> Self {
        Self::RGBA(RGBA::LIGHTGREY)
    }
    pub const fn lightpink() -> Self {
        Self::RGBA(RGBA::LIGHTPINK)
    }
    pub const fn lightsalmon() -> Self {
        Self::RGBA(RGBA::LIGHTSALMON)
    }
    pub const fn lightseagreen() -> Self {
        Self::RGBA(RGBA::LIGHTSEAGREEN)
    }
    pub const fn lightskyblue() -> Self {
        Self::RGBA(RGBA::LIGHTSKYBLUE)
    }
    pub const fn lightslategray() -> Self {
        Self::RGBA(RGBA::LIGHTSLATEGRAY)
    }
    pub const fn lightslategrey() -> Self {
        Self::RGBA(RGBA::LIGHTSLATEGREY)
    }
    pub const fn lightsteelblue() -> Self {
        Self::RGBA(RGBA::LIGHTSTEELBLUE)
    }
    pub const fn lightyellow() -> Self {
        Self::RGBA(RGBA::LIGHTYELLOW)
    }
    pub const fn limegreen() -> Self {
        Self::RGBA(RGBA::LIMEGREEN)
    }
    pub const fn linen() -> Self {
        Self::RGBA(RGBA::LINEN)
    }
    pub const fn magenta() -> Self {
        Self::RGBA(RGBA::MAGENTA)
    }
    pub const fn mediumaquamarine() -> Self {
        Self::RGBA(RGBA::MEDIUMAQUAMARINE)
    }
    pub const fn mediumblue() -> Self {
        Self::RGBA(RGBA::MEDIUMBLUE)
    }
    pub const fn mediumorchid() -> Self {
        Self::RGBA(RGBA::MEDIUMORCHID)
    }
    pub const fn mediumpurple() -> Self {
        Self::RGBA(RGBA::MEDIUMPURPLE)
    }
    pub const fn mediumseagreen() -> Self {
        Self::RGBA(RGBA::MEDIUMSEAGREEN)
    }
    pub const fn mediumslateblue() -> Self {
        Self::RGBA(RGBA::MEDIUMSLATEBLUE)
    }
    pub const fn mediumspringgreen() -> Self {
        Self::RGBA(RGBA::MEDIUMSPRINGGREEN)
    }
    pub const fn mediumturquoise() -> Self {
        Self::RGBA(RGBA::MEDIUMTURQUOISE)
    }
    pub const fn mediumvioletred() -> Self {
        Self::RGBA(RGBA::MEDIUMVIOLETRED)
    }
    pub const fn midnightblue() -> Self {
        Self::RGBA(RGBA::MIDNIGHTBLUE)
    }
    pub const fn mintcream() -> Self {
        Self::RGBA(RGBA::MINTCREAM)
    }
    pub const fn mistyrose() -> Self {
        Self::RGBA(RGBA::MISTYROSE)
    }
    pub const fn moccasin() -> Self {
        Self::RGBA(RGBA::MOCCASIN)
    }
    pub const fn navajowhite() -> Self {
        Self::RGBA(RGBA::NAVAJOWHITE)
    }
    pub const fn oldlace() -> Self {
        Self::RGBA(RGBA::OLDLACE)
    }
    pub const fn olivedrab() -> Self {
        Self::RGBA(RGBA::OLIVEDRAB)
    }
    pub const fn orange() -> Self {
        Self::RGBA(RGBA::ORANGE)
    }
    pub const fn orangered() -> Self {
        Self::RGBA(RGBA::ORANGERED)
    }
    pub const fn orchid() -> Self {
        Self::RGBA(RGBA::ORCHID)
    }
    pub const fn palegoldenrod() -> Self {
        Self::RGBA(RGBA::PALEGOLDENROD)
    }
    pub const fn palegreen() -> Self {
        Self::RGBA(RGBA::PALEGREEN)
    }
    pub const fn paleturquoise() -> Self {
        Self::RGBA(RGBA::PALETURQUOISE)
    }
    pub const fn palevioletred() -> Self {
        Self::RGBA(RGBA::PALEVIOLETRED)
    }
    pub const fn papayawhip() -> Self {
        Self::RGBA(RGBA::PAPAYAWHIP)
    }
    pub const fn peachpuff() -> Self {
        Self::RGBA(RGBA::PEACHPUFF)
    }
    pub const fn peru() -> Self {
        Self::RGBA(RGBA::PERU)
    }
    pub const fn pink() -> Self {
        Self::RGBA(RGBA::PINK)
    }
    pub const fn plum() -> Self {
        Self::RGBA(RGBA::PLUM)
    }
    pub const fn powderblue() -> Self {
        Self::RGBA(RGBA::POWDERBLUE)
    }
    pub const fn rebeccapurple() -> Self {
        Self::RGBA(RGBA::REBECCAPURPLE)
    }
    pub const fn rosybrown() -> Self {
        Self::RGBA(RGBA::ROSYBROWN)
    }
    pub const fn royalblue() -> Self {
        Self::RGBA(RGBA::ROYALBLUE)
    }
    pub const fn saddlebrown() -> Self {
        Self::RGBA(RGBA::SADDLEBROWN)
    }
    pub const fn salmon() -> Self {
        Self::RGBA(RGBA::SALMON)
    }
    pub const fn sandybrown() -> Self {
        Self::RGBA(RGBA::SANDYBROWN)
    }
    pub const fn seagreen() -> Self {
        Self::RGBA(RGBA::SEAGREEN)
    }
    pub const fn seashell() -> Self {
        Self::RGBA(RGBA::SEASHELL)
    }
    pub const fn sienna() -> Self {
        Self::RGBA(RGBA::SIENNA)
    }
    pub const fn skyblue() -> Self {
        Self::RGBA(RGBA::SKYBLUE)
    }
    pub const fn slateblue() -> Self {
        Self::RGBA(RGBA::SLATEBLUE)
    }
    pub const fn slategray() -> Self {
        Self::RGBA(RGBA::SLATEGRAY)
    }
    pub const fn slategrey() -> Self {
        Self::RGBA(RGBA::SLATEGREY)
    }
    pub const fn snow() -> Self {
        Self::RGBA(RGBA::SNOW)
    }
    pub const fn springgreen() -> Self {
        Self::RGBA(RGBA::SPRINGGREEN)
    }
    pub const fn steelblue() -> Self {
        Self::RGBA(RGBA::STEELBLUE)
    }
    pub const fn tan() -> Self {
        Self::RGBA(RGBA::TAN)
    }
    pub const fn thistle() -> Self {
        Self::RGBA(RGBA::THISTLE)
    }
    pub const fn tomato() -> Self {
        Self::RGBA(RGBA::TOMATO)
    }
    pub const fn turquoise() -> Self {
        Self::RGBA(RGBA::TURQUOISE)
    }
    pub const fn violet() -> Self {
        Self::RGBA(RGBA::VIOLET)
    }
    pub const fn wheat() -> Self {
        Self::RGBA(RGBA::WHEAT)
    }
    pub const fn whitesmoke() -> Self {
        Self::RGBA(RGBA::WHITESMOKE)
    }
    pub const fn yellowgreen() -> Self {
        Self::RGBA(RGBA::YELLOWGREEN)
    }
    pub const fn transparent() -> Self {
        Self::RGBA(RGBA::TRANSPARENT)
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

impl From<Color> for RGBA {
    fn from(color: Color) -> Self {
        color.get_rgba()
    }
}

impl From<cssparser_color::RgbaLegacy> for RGBA {
    fn from(rgba: cssparser_color::RgbaLegacy) -> Self {
        Self::rgba(rgba.red, rgba.green, rgba.blue, (rgba.alpha * 255.0) as u8)
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
    #[allow(clippy::self_named_constructors)]
    pub const fn rgba(red: u8, green: u8, blue: u8, alpha: u8) -> Self {
        RGBA { red, green, blue, alpha }
    }

    /// Creates a new RGBA from HSL values.
    #[must_use]
    pub fn hsl(h: f32, s: f32, l: f32) -> Self {
        Self::hsla(h, s, l, 1.0)
    }

    /// Creates a new RGBA from HSLA values.
    #[must_use]
    pub fn hsla(h: f32, s: f32, l: f32, a: f32) -> Self {
        let a = (a * 255.0) as u8;
        let mut h = h % 1.0;

        if h < 0.0 {
            h += 1.0;
        }

        let s = s.clamp(0.0, 1.0);
        let l = l.clamp(0.0, 1.0);

        let m2 = if l <= 0.5 { l * (1.0 + s) } else { l + s - l * s };
        let m1 = 2.0 * l - m2;

        let r = (hue(h + 1.0 / 3.0, m1, m2).clamp(0.0, 1.0) * 255.255) as u8;
        let g = (hue(h, m1, m2).clamp(0.0, 1.0) * 255.0) as u8;
        let b = (hue(h - 1.0 / 3.0, m1, m2).clamp(0.0, 1.0) * 255.0) as u8;

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

impl From<Color> for skia_safe::Color {
    fn from(src: Color) -> skia_safe::Color {
        skia_safe::Color::from_argb(src.a(), src.r(), src.g(), src.b())
    }
}

impl From<Color> for skia_safe::Color4f {
    fn from(src: Color) -> Self {
        skia_safe::Color4f {
            r: src.r() as f32 / 255.0,
            g: src.g() as f32 / 255.0,
            b: src.b() as f32 / 255.0,
            a: src.a() as f32 / 255.0,
        }
    }
}

impl From<RGBA> for skia_safe::Color {
    fn from(src: RGBA) -> skia_safe::Color {
        skia_safe::Color::from_argb(src.a(), src.r(), src.g(), src.b())
    }
}

impl From<&str> for Color {
    fn from(s: &str) -> Self {
        let mut input = ParserInput::new(s);
        let mut parser = Parser::new(&mut input);
        Color::parse(&mut parser).unwrap_or_default()
    }
}

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Color::CurrentColor => write!(f, "current-color"),
            Color::RGBA(rgba) if rgba.a() == 255 => {
                write!(f, "rgb({}, {}, {})", rgba.r(), rgba.g(), rgba.b())
            }
            Color::RGBA(rgba) if rgba.a() != 255 => {
                write!(f, "rgba({}, {}, {}, {})", rgba.r(), rgba.g(), rgba.b(), rgba.a())
            }
            _ => unreachable!(),
        }
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
