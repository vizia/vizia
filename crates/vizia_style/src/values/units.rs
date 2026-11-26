use crate::{AutoKeyword, CustomParseError, LengthPixels, Parse, Percentage, Stretch};
use cssparser::*;
pub use morphorm::Units;

impl<'i> Parse<'i> for Units {
    fn parse<'t>(input: &mut Parser<'i>) -> Result<Self, ParseError<CustomParseError<'i>>> {
        if input
            .try_parse(|input| -> Result<(), ParseError<CustomParseError<'i>>> {
                if input.expect_number()? == 0.0 {
                    Ok(())
                } else {
                    Err(ParseError::custom(CustomParseError::InvalidDeclaration))
                }
            })
            .is_ok()
        {
            return Ok(Units::Pixels(0.0));
        }
        if let Ok(value) = input.try_parse(AutoKeyword::parse) {
            return Ok(value.into());
        }
        if let Ok(value) = input.try_parse(Stretch::parse) {
            return Ok(value.into());
        }
        if let Ok(value) = input.try_parse(Percentage::parse) {
            return Ok(value.into());
        }
        if let Ok(value) = input.try_parse(LengthPixels::parse) {
            return Ok(value.into());
        }
        Err(ParseError::custom(CustomParseError::InvalidDeclaration))
    }
}

impl<'i> Parse<'i> for Vec<Units> {
    fn parse<'t>(input: &mut Parser<'i>) -> Result<Self, ParseError<CustomParseError<'i>>> {
        let mut values = Vec::default();
        while let Ok(val) = input.try_parse(Units::parse) {
            values.push(val);
        }

        Ok(values)
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
    use crate::{LengthValue, tests::assert_parse};

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

        custom {
            success {
                "0" => Units::Pixels(0.0),
                "-0" => Units::Pixels(0.0),
                "0.0" => Units::Pixels(0.0),
                "0s" => Units::Stretch(0.0),
            }

            failure {
                "1",
                "-1",
                "0.1",
                "0foo",
            }
        }
    }
}
