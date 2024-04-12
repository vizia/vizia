use cssparser::*;
use skia_safe::{font_arguments::variation_position::Coordinate, FourByteTag};

use crate::{CustomParseError, Parse};

#[derive(Copy, Clone, PartialEq, Default, Debug)]
pub struct FontVariation(pub Coordinate);

impl<'i> Parse<'i> for FontVariation {
    fn parse<'t>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i, CustomParseError<'i>>> {
        let location = input.current_source_location();

        let axis = FourByteTag::parse(input)?;
        let value = f32::parse(input)?;

        if input.is_exhausted() {
            Ok(Self(Coordinate { axis, value }))
        } else {
            Err(ParseError {
                kind: ParseErrorKind::Custom(CustomParseError::InvalidDeclaration),
                location,
            })
        }
    }
}

impl<'i> Parse<'i> for Vec<FontVariation> {
    fn parse<'t>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i, CustomParseError<'i>>> {
        input.parse_comma_separated(FontVariation::parse)
    }
}

impl<'i> Parse<'i> for FourByteTag {
    fn parse<'t>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i, CustomParseError<'i>>> {
        let location = input.current_source_location();

        let name = input.expect_string()?;
        let bytes = name.as_bytes();

        if bytes.len() != 4 {
            return Err(ParseError {
                kind: ParseErrorKind::Custom(CustomParseError::InvalidValue),
                location,
            });
        }

        let bytes = std::array::from_fn(|i| bytes[i]);

        Ok(u32::from_be_bytes(bytes).into())
    }
}

impl From<&str> for FontVariation {
    fn from(s: &str) -> Self {
        let mut input = ParserInput::new(s);
        let mut parser = Parser::new(&mut input);
        FontVariation::parse(&mut parser).unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::assert_parse;

    assert_parse! {
        FontVariation, assert_font_variation,

        custom {
            success {
                "\"slnt\" -5.0" => FontVariation ( Coordinate {axis: ('s', 'l', 'n', 't').into(), value: -5.0 }),
                "\"wdth\" 125.0" => FontVariation (Coordinate { axis: ('w', 'd', 't', 'h').into(), value: 125.0 }),
                "\"wght\" 400" => FontVariation (Coordinate { axis: ('w', 'g', 'h', 't').into(), value: 400.0 }),
            }
            failure {
                "1234 0",
                "\"123\" 0",
                "\"1234\" ?",
                "\"12345\" 0",
            }
        }
    }

    assert_parse! {
        Vec<FontVariation>, assert_font_variations,

        custom {
            success {
                r#"
                    "slnt" -5.0, "wdth" 125.0, "wght" 400
                "# => vec![
                    FontVariation (Coordinate { axis: ('s', 'l', 'n', 't').into(), value: -5.0 }),
                    FontVariation (Coordinate { axis: ('w', 'd', 't', 'h').into(), value: 125.0 }),
                    FontVariation (Coordinate { axis: ('w', 'g', 'h', 't').into(), value: 400.0 }),
                ],
            }

            failure {
                r#"
                    "slnt" -5.0 "wdth" 125.0 "wght" 400
                "#,
            }
        }
    }
}
