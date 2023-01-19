use crate::{macros::impl_parse, AutoKeyword, LengthPixels, Parse, Percentage, Stretch};
use cssparser::*;
pub use morphorm::Units;

impl_parse! {
    Units,

    try_parse {
        AutoKeyword,
        Stretch,
        Percentage,
        LengthPixels,
    }
}

impl From<AutoKeyword> for Units {
    fn from(_: AutoKeyword) -> Self {
        Units::Auto
    }
}

impl From<Stretch> for Units {
    fn from(stretch: Stretch) -> Self {
        Self::Stretch(stretch.0)
    }
}

impl From<Percentage> for Units {
    fn from(percentage: Percentage) -> Self {
        Self::Percentage(percentage.0)
    }
}

impl From<LengthPixels> for Units {
    fn from(length_pixels: LengthPixels) -> Self {
        Self::Pixels(length_pixels.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{tests::assert_parse, LengthValue};

    assert_parse! {
        Units, parse_units,

        ident {
            "auto" => Units::Auto,
        }

        percentage {
            Units::Percentage,
        }

        dimension {
            "px" => Units::Pixels,
            "in" => Units::Pixels(LengthValue::PX_PER_IN),
            "cm" => Units::Pixels(LengthValue::PX_PER_CM),
            "mm" => Units::Pixels(LengthValue::PX_PER_MM),
            "q" => Units::Pixels(LengthValue::PX_PER_Q),
            "pt" => Units::Pixels(LengthValue::PX_PER_PT),
            "pc" => Units::Pixels(LengthValue::PX_PER_PC),
            "s" => Units::Stretch,
        }
    }
}

// impl From<&str> for Units {
//     fn from(s: &str) -> Self {
//         let mut input = ParserInput::new(&s);
//         let mut parser = Parser::new(&mut input);
//         Units::parse(&mut parser).unwrap_or_default()
//     }
// }