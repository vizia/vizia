use crate::{CustomParseError, Length, Parse, impl_parse};
use cssparser::ParseError;

/// A length value in pixels.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct LengthPixels(pub f32);

impl_parse! {
    LengthPixels,

    custom {
        |input| {

            if let Some(pixels) = input.try_parse(Length::parse)?.to_px() {
                return Ok(Self(pixels));
            }

            Err(ParseError::custom(CustomParseError::InvalidDeclaration))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{LengthValue, tests::assert_parse};

    assert_parse! {
        LengthPixels, assert_length_pixels,

        dimension {
            "px" => LengthPixels,
            "in" => LengthPixels(LengthValue::PX_PER_IN),
            "cm" => LengthPixels(LengthValue::PX_PER_CM),
            "mm" => LengthPixels(LengthValue::PX_PER_MM),
            "q" => LengthPixels(LengthValue::PX_PER_Q),
            "pt" => LengthPixels(LengthValue::PX_PER_PT),
            "pc" => LengthPixels(LengthValue::PX_PER_PC),
        }
    }
}
