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
            Self::CurrentColor => RGBA::rgba(0, 0, 0, 0),
            Self::RGBA(rgba) => *rgba,
        }
    }

    pub fn r(&self) -> u8 {
        match self {
            Self::CurrentColor => 0,
            Self::RGBA(col) => col.r(),
        }
    }

    pub fn g(&self) -> u8 {
        match self {
            Self::CurrentColor => 0,
            Self::RGBA(col) => col.g(),
        }
    }

    pub fn b(&self) -> u8 {
        match self {
            Self::CurrentColor => 0,
            Self::RGBA(col) => col.b(),
        }
    }

    pub fn a(&self) -> u8 {
        match self {
            Self::CurrentColor => 0,
            Self::RGBA(col) => col.a(),
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
        Self::RGBA(rgba)
    }
}

impl From<cssparser::Color> for Color {
    fn from(color: cssparser::Color) -> Self {
        match color {
            cssparser::Color::CurrentColor => Self::CurrentColor,
            cssparser::Color::RGBA(rgba) => Self::RGBA(rgba.into()),
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

impl From<cssparser::RGBA> for RGBA {
    fn from(rgba: cssparser::RGBA) -> Self {
        Self::rgba(rgba.red, rgba.green, rgba.blue, rgba.alpha)
    }
}

impl RGBA {
    pub const BLACK: Self = Self::rgba(0, 0, 0, 255);
    pub const SILVER: Self = Self::rgba(192, 192, 192, 255);
    pub const GRAY: Self = Self::rgba(128, 128, 128, 255);
    pub const WHITE: Self = Self::rgba(255, 255, 255, 255);
    pub const MAROON: Self = Self::rgba(128, 0, 0, 255);
    pub const RED: Self = Self::rgba(255, 0, 0, 255);
    pub const PURPLE: Self = Self::rgba(128, 0, 128, 255);
    pub const FUCHSIA: Self = Self::rgba(255, 0, 255, 255);
    pub const GREEN: Self = Self::rgba(0, 128, 0, 255);
    pub const LIME: Self = Self::rgba(0, 255, 0, 255);
    pub const OLIVE: Self = Self::rgba(128, 128, 0, 255);
    pub const YELLOW: Self = Self::rgba(255, 255, 0, 255);
    pub const NAVY: Self = Self::rgba(0, 0, 128, 255);
    pub const BLUE: Self = Self::rgba(0, 0, 255, 255);
    pub const TEAL: Self = Self::rgba(0, 128, 128, 255);
    pub const AQUA: Self = Self::rgba(0, 255, 255, 255);
    pub const ALICEBLUE: Self = Self::rgba(240, 248, 255, 255);
    pub const ANTIQUEWHITE: Self = Self::rgba(250, 235, 215, 255);
    pub const AQUAMARINE: Self = Self::rgba(127, 255, 212, 255);
    pub const AZURE: Self = Self::rgba(240, 255, 255, 255);
    pub const BEIGE: Self = Self::rgba(245, 245, 220, 255);
    pub const BISQUE: Self = Self::rgba(255, 228, 196, 255);
    pub const BLANCHEDALMOND: Self = Self::rgba(255, 235, 205, 255);
    pub const BLUEVIOLET: Self = Self::rgba(138, 43, 226, 255);
    pub const BROWN: Self = Self::rgba(165, 42, 42, 255);
    pub const BURLYWOOD: Self = Self::rgba(222, 184, 135, 255);
    pub const CADETBLUE: Self = Self::rgba(95, 158, 160, 255);
    pub const CHARTREUSE: Self = Self::rgba(127, 255, 0, 255);
    pub const CHOCOLATE: Self = Self::rgba(210, 105, 30, 255);
    pub const CORAL: Self = Self::rgba(255, 127, 80, 255);
    pub const CORNFLOWERBLUE: Self = Self::rgba(100, 149, 237, 255);
    pub const CORNSILK: Self = Self::rgba(255, 248, 220, 255);
    pub const CRIMSON: Self = Self::rgba(220, 20, 60, 255);
    pub const CYAN: Self = Self::rgba(0, 255, 255, 255);
    pub const DARKBLUE: Self = Self::rgba(0, 0, 139, 255);
    pub const DARKCYAN: Self = Self::rgba(0, 139, 139, 255);
    pub const DARKGOLDENROD: Self = Self::rgba(184, 134, 11, 255);
    pub const DARKGRAY: Self = Self::rgba(169, 169, 169, 255);
    pub const DARKGREEN: Self = Self::rgba(0, 100, 0, 255);
    pub const DARKGREY: Self = Self::rgba(169, 169, 169, 255);
    pub const DARKKHAKI: Self = Self::rgba(189, 183, 107, 255);
    pub const DARKMAGENTA: Self = Self::rgba(139, 0, 139, 255);
    pub const DARKOLIVEGREEN: Self = Self::rgba(85, 107, 47, 255);
    pub const DARKORANGE: Self = Self::rgba(255, 140, 0, 255);
    pub const DARKORCHID: Self = Self::rgba(153, 50, 204, 255);
    pub const DARKRED: Self = Self::rgba(139, 0, 0, 255);
    pub const DARKSALMON: Self = Self::rgba(233, 150, 122, 255);
    pub const DARKSEAGREEN: Self = Self::rgba(143, 188, 143, 255);
    pub const DARKSLATEBLUE: Self = Self::rgba(72, 61, 139, 255);
    pub const DARKSLATEGRAY: Self = Self::rgba(47, 79, 79, 255);
    pub const DARKSLATEGREY: Self = Self::rgba(47, 79, 79, 255);
    pub const DARKTURQUOISE: Self = Self::rgba(0, 206, 209, 255);
    pub const DARKVIOLET: Self = Self::rgba(148, 0, 211, 255);
    pub const DEEPPINK: Self = Self::rgba(255, 20, 147, 255);
    pub const DEEPSKYBLUE: Self = Self::rgba(0, 191, 255, 255);
    pub const DIMGRAY: Self = Self::rgba(105, 105, 105, 255);
    pub const DIMGREY: Self = Self::rgba(105, 105, 105, 255);
    pub const DODGERBLUE: Self = Self::rgba(30, 144, 255, 255);
    pub const FIREBRICK: Self = Self::rgba(178, 34, 34, 255);
    pub const FLORALWHITE: Self = Self::rgba(255, 250, 240, 255);
    pub const FORESTGREEN: Self = Self::rgba(34, 139, 34, 255);
    pub const GAINSBORO: Self = Self::rgba(220, 220, 220, 255);
    pub const GHOSTWHITE: Self = Self::rgba(248, 248, 255, 255);
    pub const GOLD: Self = Self::rgba(255, 215, 0, 255);
    pub const GOLDENROD: Self = Self::rgba(218, 165, 32, 255);
    pub const GREENYELLOW: Self = Self::rgba(173, 255, 47, 255);
    pub const GREY: Self = Self::rgba(128, 128, 128, 255);
    pub const HONEYDEW: Self = Self::rgba(240, 255, 240, 255);
    pub const HOTPINK: Self = Self::rgba(255, 105, 180, 255);
    pub const INDIANRED: Self = Self::rgba(205, 92, 92, 255);
    pub const INDIGO: Self = Self::rgba(75, 0, 130, 255);
    pub const IVORY: Self = Self::rgba(255, 255, 240, 255);
    pub const KHAKI: Self = Self::rgba(240, 230, 140, 255);
    pub const LAVENDER: Self = Self::rgba(230, 230, 250, 255);
    pub const LAVENDERBLUSH: Self = Self::rgba(255, 240, 245, 255);
    pub const LAWNGREEN: Self = Self::rgba(124, 252, 0, 255);
    pub const LEMONCHIFFON: Self = Self::rgba(255, 250, 205, 255);
    pub const LIGHTBLUE: Self = Self::rgba(173, 216, 230, 255);
    pub const LIGHTCORAL: Self = Self::rgba(240, 128, 128, 255);
    pub const LIGHTCYAN: Self = Self::rgba(224, 255, 255, 255);
    pub const LIGHTGOLDENRODYELLOW: Self = Self::rgba(250, 250, 210, 255);
    pub const LIGHTGRAY: Self = Self::rgba(211, 211, 211, 255);
    pub const LIGHTGREEN: Self = Self::rgba(144, 238, 144, 255);
    pub const LIGHTGREY: Self = Self::rgba(211, 211, 211, 255);
    pub const LIGHTPINK: Self = Self::rgba(255, 182, 193, 255);
    pub const LIGHTSALMON: Self = Self::rgba(255, 160, 122, 255);
    pub const LIGHTSEAGREEN: Self = Self::rgba(32, 178, 170, 255);
    pub const LIGHTSKYBLUE: Self = Self::rgba(135, 206, 250, 255);
    pub const LIGHTSLATEGRAY: Self = Self::rgba(119, 136, 153, 255);
    pub const LIGHTSLATEGREY: Self = Self::rgba(119, 136, 153, 255);
    pub const LIGHTSTEELBLUE: Self = Self::rgba(176, 196, 222, 255);
    pub const LIGHTYELLOW: Self = Self::rgba(255, 255, 224, 255);
    pub const LIMEGREEN: Self = Self::rgba(50, 205, 50, 255);
    pub const LINEN: Self = Self::rgba(250, 240, 230, 255);
    pub const MAGENTA: Self = Self::rgba(255, 0, 255, 255);
    pub const MEDIUMAQUAMARINE: Self = Self::rgba(102, 205, 170, 255);
    pub const MEDIUMBLUE: Self = Self::rgba(0, 0, 205, 255);
    pub const MEDIUMORCHID: Self = Self::rgba(186, 85, 211, 255);
    pub const MEDIUMPURPLE: Self = Self::rgba(147, 112, 219, 255);
    pub const MEDIUMSEAGREEN: Self = Self::rgba(60, 179, 113, 255);
    pub const MEDIUMSLATEBLUE: Self = Self::rgba(123, 104, 238, 255);
    pub const MEDIUMSPRINGGREEN: Self = Self::rgba(0, 250, 154, 255);
    pub const MEDIUMTURQUOISE: Self = Self::rgba(72, 209, 204, 255);
    pub const MEDIUMVIOLETRED: Self = Self::rgba(199, 21, 133, 255);
    pub const MIDNIGHTBLUE: Self = Self::rgba(25, 25, 112, 255);
    pub const MINTCREAM: Self = Self::rgba(245, 255, 250, 255);
    pub const MISTYROSE: Self = Self::rgba(255, 228, 225, 255);
    pub const MOCCASIN: Self = Self::rgba(255, 228, 181, 255);
    pub const NAVAJOWHITE: Self = Self::rgba(255, 222, 173, 255);
    pub const OLDLACE: Self = Self::rgba(253, 245, 230, 255);
    pub const OLIVEDRAB: Self = Self::rgba(107, 142, 35, 255);
    pub const ORANGE: Self = Self::rgba(255, 165, 0, 255);
    pub const ORANGERED: Self = Self::rgba(255, 69, 0, 255);
    pub const ORCHID: Self = Self::rgba(218, 112, 214, 255);
    pub const PALEGOLDENROD: Self = Self::rgba(238, 232, 170, 255);
    pub const PALEGREEN: Self = Self::rgba(152, 251, 152, 255);
    pub const PALETURQUOISE: Self = Self::rgba(175, 238, 238, 255);
    pub const PALEVIOLETRED: Self = Self::rgba(219, 112, 147, 255);
    pub const PAPAYAWHIP: Self = Self::rgba(255, 239, 213, 255);
    pub const PEACHPUFF: Self = Self::rgba(255, 218, 185, 255);
    pub const PERU: Self = Self::rgba(205, 133, 63, 255);
    pub const PINK: Self = Self::rgba(255, 192, 203, 255);
    pub const PLUM: Self = Self::rgba(221, 160, 221, 255);
    pub const POWDERBLUE: Self = Self::rgba(176, 224, 230, 255);
    pub const REBECCAPURPLE: Self = Self::rgba(102, 51, 153, 255);
    pub const ROSYBROWN: Self = Self::rgba(188, 143, 143, 255);
    pub const ROYALBLUE: Self = Self::rgba(65, 105, 225, 255);
    pub const SADDLEBROWN: Self = Self::rgba(139, 69, 19, 255);
    pub const SALMON: Self = Self::rgba(250, 128, 114, 255);
    pub const SANDYBROWN: Self = Self::rgba(244, 164, 96, 255);
    pub const SEAGREEN: Self = Self::rgba(46, 139, 87, 255);
    pub const SEASHELL: Self = Self::rgba(255, 245, 238, 255);
    pub const SIENNA: Self = Self::rgba(160, 82, 45, 255);
    pub const SKYBLUE: Self = Self::rgba(135, 206, 235, 255);
    pub const SLATEBLUE: Self = Self::rgba(106, 90, 205, 255);
    pub const SLATEGRAY: Self = Self::rgba(112, 128, 144, 255);
    pub const SLATEGREY: Self = Self::rgba(112, 128, 144, 255);
    pub const SNOW: Self = Self::rgba(255, 250, 250, 255);
    pub const SPRINGGREEN: Self = Self::rgba(0, 255, 127, 255);
    pub const STEELBLUE: Self = Self::rgba(70, 130, 180, 255);
    pub const TAN: Self = Self::rgba(210, 180, 140, 255);
    pub const THISTLE: Self = Self::rgba(216, 191, 216, 255);
    pub const TOMATO: Self = Self::rgba(255, 99, 71, 255);
    pub const TURQUOISE: Self = Self::rgba(64, 224, 208, 255);
    pub const VIOLET: Self = Self::rgba(238, 130, 238, 255);
    pub const WHEAT: Self = Self::rgba(245, 222, 179, 255);
    pub const WHITESMOKE: Self = Self::rgba(245, 245, 245, 255);
    pub const YELLOWGREEN: Self = Self::rgba(154, 205, 50, 255);
    pub const TRANSPARENT: Self = Self::rgba(0, 0, 0, 0);

    /// Creates a new RGBA from RGB values
    #[must_use]
    pub const fn rgb(red: u8, green: u8, blue: u8) -> Self {
        Self { red, green, blue, alpha: 255 }
    }

    /// Creates a new RGBA from RGBA values
    #[must_use]
    #[allow(clippy::self_named_constructors)]
    pub const fn rgba(red: u8, green: u8, blue: u8, alpha: u8) -> Self {
        Self { red, green, blue, alpha }
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
    fn from(src: Color) -> Self {
        Self::from_argb(src.a(), src.r(), src.g(), src.b())
    }
}

impl From<Color> for skia_safe::Color4f {
    fn from(src: Color) -> Self {
        Self {
            r: src.r() as f32 / 255.0,
            g: src.g() as f32 / 255.0,
            b: src.b() as f32 / 255.0,
            a: src.a() as f32 / 255.0,
        }
    }
}

impl From<RGBA> for skia_safe::Color {
    fn from(src: RGBA) -> Self {
        Self::from_argb(src.a(), src.r(), src.g(), src.b())
    }
}

impl From<&str> for Color {
    fn from(s: &str) -> Self {
        let mut input = ParserInput::new(s);
        let mut parser = Parser::new(&mut input);
        Self::parse(&mut parser).unwrap_or_default()
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
