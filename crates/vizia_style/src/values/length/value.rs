use crate::{impl_parse, Parse, TryAdd};

/// A length value.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum LengthValue {
    /// Pixels.
    ///
    /// A pixel is the smallest viewable unit.
    /// The length of a pixel depends on the used device.
    Px(f32),
    /// Inches.
    ///
    /// An inch is roughly 96 pixels or about 2.54cm.
    In(f32),
    /// Centimeters.
    ///
    /// A centimeter is roughly 37.8 pixels.
    Cm(f32),
    /// Millimeters.
    ///
    /// A millimeter is roughly 3.78 pixels or 1/10th of a centimeter.
    Mm(f32),
    /// Quarter.
    ///
    /// A quarter of a millimeter is roughly 1/40th of 1cm.
    Q(f32),
    /// Points.
    ///
    /// A point is roughly 1.3333 pixels or 1/72th of an inch.
    Pt(f32),
    /// Picas.
    ///
    /// A pica is roughly 16 pixels or 1/6th of an inch.
    Pc(f32),
    /// em.
    ///
    /// An em is equal to the computed `font-size` property of the element on which it is used.
    Em(f32),
    /// ex.
    ///
    /// An ex is the height of the letter `x` of the font, or 0.5em.
    Ex(f32),
    /// Character unit.
    ///
    /// A character unit is defined as the width of the character `0` of the font.
    Ch(f32),
    /// Root em.
    ///
    /// An rem is equal to the computed `font-size` property of the root of the document.
    /// For websites the root of the document is the `<html>` element.
    Rem(f32),
    /// View width.
    ///
    /// A length equal to 1% of the width of the viewport.
    Vw(f32),
    /// View height.
    ///
    /// A length equal to 1% of the height of the viewport.
    Vh(f32),
    /// Viewport minimum.
    ///
    /// A viewport minimum is equal to 1% of the viewport's smallest dimension.
    /// For example 1vmin is equal to 6px on a browser window that is 1200px wide and 600px high.
    Vmin(f32),
    /// Viewport maximum.
    ///
    /// A viewport maximum is equal to 1% of the viewport's largest dimension.
    /// For example 1vmax is equal to 12px on a browser window that is 1200px wide and 600px high.
    Vmax(f32),
}

impl Default for LengthValue {
    fn default() -> Self {
        LengthValue::Px(0.0)
    }
}

impl_parse! {
    LengthValue,

    tokens {
        dimension {
            "px" => LengthValue::Px,
            "in" => LengthValue::In,
            "cm" => LengthValue::Cm,
            "mm" => LengthValue::Mm,
            "q" => LengthValue::Q,
            "pt" => LengthValue::Pt,
            "pc" => LengthValue::Pc,
            "em" => LengthValue::Em,
            "ex" => LengthValue::Ex,
            "ch" => LengthValue::Ch,
            "rem" => LengthValue::Rem,
            "vw" => LengthValue::Vw,
            "vh" => LengthValue::Vh,
            "vmin" => LengthValue::Vmin,
            "vmax" => LengthValue::Vmax,
        }
    }
}

impl LengthValue {
    /// The amount of pixels per inch.
    pub const PX_PER_IN: f32 = 96.0;

    /// The amount of pixels per centimeter.
    pub const PX_PER_CM: f32 = Self::PX_PER_IN / 2.54;

    /// The amount of pixels per millimeter.
    pub const PX_PER_MM: f32 = Self::PX_PER_CM / 10.0;

    /// The amount of pixels per quarter-millimeter.
    pub const PX_PER_Q: f32 = Self::PX_PER_CM / 40.0;

    /// The amount of pixels per point.
    pub const PX_PER_PT: f32 = Self::PX_PER_IN / 72.0;

    /// The amount of pixels per pica.
    pub const PX_PER_PC: f32 = Self::PX_PER_IN / 6.0;

    /// Returns the amount of pixels of the length if calculateable.
    pub fn to_px(&self) -> Option<f32> {
        match self {
            LengthValue::Px(value) => Some(*value),
            LengthValue::In(value) => Some(value * Self::PX_PER_IN),
            LengthValue::Cm(value) => Some(value * Self::PX_PER_CM),
            LengthValue::Mm(value) => Some(value * Self::PX_PER_MM),
            LengthValue::Q(value) => Some(value * Self::PX_PER_Q),
            LengthValue::Pt(value) => Some(value * Self::PX_PER_PT),
            LengthValue::Pc(value) => Some(value * Self::PX_PER_PC),
            _ => None,
        }
    }

    pub fn to_unit_value(&self) -> (f32, &str) {
        use LengthValue::*;
        match self {
            Px(value) => (*value, "px"),
            In(value) => (*value, "in"),
            Cm(value) => (*value, "cm"),
            Mm(value) => (*value, "mm"),
            Q(value) => (*value, "q"),
            Pt(value) => (*value, "pt"),
            Pc(value) => (*value, "pc"),
            Em(value) => (*value, "em"),
            Ex(value) => (*value, "ex"),
            Ch(value) => (*value, "ch"),
            Rem(value) => (*value, "rem"),
            Vw(value) => (*value, "vw"),
            Vh(value) => (*value, "vh"),
            Vmin(value) => (*value, "vmin"),
            Vmax(value) => (*value, "vmax"),
        }
    }
}

impl TryAdd<LengthValue> for LengthValue {
    fn try_add(&self, other: &LengthValue) -> Option<LengthValue> {
        use LengthValue::*;
        match (self, other) {
            (Px(a), Px(b)) => Some(Px(a + b)),
            (In(a), In(b)) => Some(In(a + b)),
            (Cm(a), Cm(b)) => Some(Cm(a + b)),
            (Mm(a), Mm(b)) => Some(Mm(a + b)),
            (Q(a), Q(b)) => Some(Q(a + b)),
            (Pt(a), Pt(b)) => Some(Pt(a + b)),
            (Pc(a), Pc(b)) => Some(Pc(a + b)),
            (Em(a), Em(b)) => Some(Em(a + b)),
            (Ex(a), Ex(b)) => Some(Ex(a + b)),
            (Ch(a), Ch(b)) => Some(Ch(a + b)),
            (Rem(a), Rem(b)) => Some(Rem(a + b)),
            (Vw(a), Vw(b)) => Some(Vw(a + b)),
            (Vh(a), Vh(b)) => Some(Vh(a + b)),
            (Vmin(a), Vmin(b)) => Some(Vmin(a + b)),
            (Vmax(a), Vmax(b)) => Some(Vmax(a + b)),
            (a, b) => {
                if let (Some(a), Some(b)) = (a.to_px(), b.to_px()) {
                    Some(Px(a + b))
                } else {
                    None
                }
            }
        }
    }
}

impl std::ops::Mul<f32> for LengthValue {
    type Output = Self;

    fn mul(self, other: f32) -> LengthValue {
        use LengthValue::*;
        match self {
            Px(value) => Px(value * other),
            In(value) => In(value * other),
            Cm(value) => Cm(value * other),
            Mm(value) => Mm(value * other),
            Q(value) => Q(value * other),
            Pt(value) => Pt(value * other),
            Pc(value) => Pc(value * other),
            Em(value) => Em(value * other),
            Ex(value) => Ex(value * other),
            Ch(value) => Ch(value * other),
            Rem(value) => Rem(value * other),
            Vw(value) => Vw(value * other),
            Vh(value) => Vh(value * other),
            Vmin(value) => Vmin(value * other),
            Vmax(value) => Vmax(value * other),
        }
    }
}

impl std::cmp::PartialEq<f32> for LengthValue {
    fn eq(&self, other: &f32) -> bool {
        use LengthValue::*;
        match self {
            Px(value) => value == other,
            In(value) => value == other,
            Cm(value) => value == other,
            Mm(value) => value == other,
            Q(value) => value == other,
            Pt(value) => value == other,
            Pc(value) => value == other,
            Em(value) => value == other,
            Ex(value) => value == other,
            Ch(value) => value == other,
            Rem(value) => value == other,
            Vw(value) => value == other,
            Vh(value) => value == other,
            Vmin(value) => value == other,
            Vmax(value) => value == other,
        }
    }
}

impl std::cmp::PartialOrd<f32> for LengthValue {
    fn partial_cmp(&self, other: &f32) -> Option<std::cmp::Ordering> {
        use LengthValue::*;
        match self {
            Px(value) => value.partial_cmp(other),
            In(value) => value.partial_cmp(other),
            Cm(value) => value.partial_cmp(other),
            Mm(value) => value.partial_cmp(other),
            Q(value) => value.partial_cmp(other),
            Pt(value) => value.partial_cmp(other),
            Pc(value) => value.partial_cmp(other),
            Em(value) => value.partial_cmp(other),
            Ex(value) => value.partial_cmp(other),
            Ch(value) => value.partial_cmp(other),
            Rem(value) => value.partial_cmp(other),
            Vw(value) => value.partial_cmp(other),
            Vh(value) => value.partial_cmp(other),
            Vmin(value) => value.partial_cmp(other),
            Vmax(value) => value.partial_cmp(other),
        }
    }
}

impl std::cmp::PartialOrd<LengthValue> for LengthValue {
    fn partial_cmp(&self, other: &LengthValue) -> Option<std::cmp::Ordering> {
        use LengthValue::*;
        match (self, other) {
            (Em(a), Em(b))
            | (Ex(a), Ex(b))
            | (Ch(a), Ch(b))
            | (Rem(a), Rem(b))
            | (Vw(a), Vw(b))
            | (Vh(a), Vh(b))
            | (Vmin(a), Vmin(b))
            | (Vmax(a), Vmax(b)) => a.partial_cmp(b),
            (a, b) => {
                if let (Some(a), Some(b)) = (a.to_px(), b.to_px()) {
                    a.partial_cmp(&b)
                } else {
                    None
                }
            }
        }
    }
}

impl std::fmt::Display for LengthValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LengthValue::Px(v) => write!(f, "{v}px"),
            LengthValue::In(_) => todo!(),
            LengthValue::Cm(_) => todo!(),
            LengthValue::Mm(_) => todo!(),
            LengthValue::Q(_) => todo!(),
            LengthValue::Pt(_) => todo!(),
            LengthValue::Pc(_) => todo!(),
            LengthValue::Em(_) => todo!(),
            LengthValue::Ex(_) => todo!(),
            LengthValue::Ch(_) => todo!(),
            LengthValue::Rem(_) => todo!(),
            LengthValue::Vw(_) => todo!(),
            LengthValue::Vh(_) => todo!(),
            LengthValue::Vmin(_) => todo!(),
            LengthValue::Vmax(_) => todo!(),
        }
    }
}
