use cssparser::{ParseError, ParseErrorKind, Parser, ParserInput};

use crate::{CustomParseError, Parse};

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub enum AspectRatio {
    #[default]
    Auto,
    Ratio(f32),
    AutoRatio(f32),
}

impl AspectRatio {
    pub fn ratio(self) -> Option<f32> {
        match self {
            AspectRatio::Auto => None,
            AspectRatio::Ratio(ratio) | AspectRatio::AutoRatio(ratio) => Some(ratio),
        }
    }

    pub fn is_auto(self) -> bool {
        matches!(self, AspectRatio::Auto | AspectRatio::AutoRatio(_))
    }
}

impl<'i> Parse<'i> for AspectRatio {
    fn parse<'t>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i, CustomParseError<'i>>> {
        let location = input.current_source_location();

        let has_auto = input.try_parse(|parser| parser.expect_ident_matching("auto")).is_ok();

        // CSS allows either `auto`, `<number>`, `<number>/<number>`, or `auto <ratio>`.
        let ratio = input.try_parse(|parser| {
            let error_at = |location| ParseError {
                kind: ParseErrorKind::Custom(CustomParseError::InvalidValue),
                location,
            };

            let numerator = parser.expect_number()?;
            if numerator <= 0.0 || !numerator.is_finite() {
                return Err(error_at(parser.current_source_location()));
            }

            let denominator = parser
                .try_parse(|inner| {
                    inner.expect_delim('/')?;
                    let value = inner.expect_number()?;
                    if value <= 0.0 || !value.is_finite() {
                        return Err(error_at(inner.current_source_location()));
                    }

                    Ok(value)
                })
                .unwrap_or(1.0);

            let ratio = numerator / denominator;
            if ratio <= 0.0 || !ratio.is_finite() {
                return Err(error_at(parser.current_source_location()));
            }

            Ok(ratio)
        });

        let parsed = match (has_auto, ratio) {
            (true, Ok(ratio)) => AspectRatio::AutoRatio(ratio),
            (true, Err(_)) if input.is_exhausted() => AspectRatio::Auto,
            (false, Ok(ratio)) => AspectRatio::Ratio(ratio),
            _ => {
                return Err(ParseError {
                    kind: ParseErrorKind::Custom(CustomParseError::InvalidDeclaration),
                    location,
                });
            }
        };

        if !input.is_exhausted() {
            return Err(ParseError {
                kind: ParseErrorKind::Custom(CustomParseError::InvalidDeclaration),
                location,
            });
        }

        Ok(parsed)
    }
}

impl From<f32> for AspectRatio {
    fn from(value: f32) -> Self {
        if value > 0.0 && value.is_finite() { AspectRatio::Ratio(value) } else { AspectRatio::Auto }
    }
}

impl From<f64> for AspectRatio {
    fn from(value: f64) -> Self {
        AspectRatio::from(value as f32)
    }
}

impl From<&str> for AspectRatio {
    fn from(s: &str) -> Self {
        let mut input = ParserInput::new(s);
        let mut parser = Parser::new(&mut input);
        AspectRatio::parse(&mut parser).unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::assert_parse;

    assert_parse! {
        AspectRatio, parse_aspect_ratio,

        custom {
            success {
                "auto" => AspectRatio::Auto,
                "1" => AspectRatio::Ratio(1.0),
                "16/9" => AspectRatio::Ratio(16.0 / 9.0),
                "1.5" => AspectRatio::Ratio(1.5),
                "auto 4/3" => AspectRatio::AutoRatio(4.0 / 3.0),
            }

            failure {
                "",
                "none",
                "0",
                "-1",
                "1/0",
                "auto auto",
                "auto / 2",
                "16/9/2",
            }
        }
    }
}
