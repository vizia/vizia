use crate::{CustomParseError, Parse};
use cssparser::{ParseError, ParseErrorKind, Parser, Token, match_ignore_ascii_case};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum StepPosition {
    JumpStart,
    #[default]
    JumpEnd,
    JumpNone,
    JumpBoth,
    Start,
    End,
}

impl<'i> Parse<'i> for StepPosition {
    fn parse<'t>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i, CustomParseError<'i>>> {
        let location = input.current_source_location();
        let ident = input.expect_ident_cloned()?;
        match_ignore_ascii_case! { &ident,
            "jump-start" => Ok(Self::JumpStart),
            "jump-end" => Ok(Self::JumpEnd),
            "jump-none" => Ok(Self::JumpNone),
            "jump-both" => Ok(Self::JumpBoth),
            "start" => Ok(Self::Start),
            "end" => Ok(Self::End),
            _ => Err(location.new_unexpected_token_error(Token::Ident(ident))),
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub enum EasingFunction {
    Linear,
    #[default]
    Ease,
    EaseIn,
    EaseOut,
    EaseInOut,
    CubicBezier(f32, f32, f32, f32),
    Steps(u32, StepPosition),
}

impl<'i> Parse<'i> for EasingFunction {
    fn parse<'t>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i, CustomParseError<'i>>> {
        let location = input.current_source_location();
        if let Ok(ident) = input.try_parse(|i| i.expect_ident_cloned()) {
            let keyword = match_ignore_ascii_case! { &ident,
                "linear" => EasingFunction::Linear,
                "ease" => EasingFunction::Ease,
                "ease-in" => EasingFunction::EaseIn,
                "ease-out" => EasingFunction::EaseOut,
                "ease-in-out" => EasingFunction::EaseInOut,
                "step-start" => EasingFunction::Steps(1, StepPosition::Start),
                "step-end" => EasingFunction::Steps(1, StepPosition::End),
                _ => return Err(location.new_unexpected_token_error(Token::Ident(ident.clone()))),
            };
            return Ok(keyword);
        }

        let function = input.expect_function()?.clone();
        input.parse_nested_block(|input| {
            match_ignore_ascii_case! { &function,
                "cubic-bezier" => {
                    let x1 = input.expect_number()?;
                    input.expect_comma()?;
                    let y1 = input.expect_number()?;
                    input.expect_comma()?;
                    let x2 = input.expect_number()?;
                    input.expect_comma()?;
                    let y2 = input.expect_number()?;
                    if !(0.0..=1.0).contains(&x1) || !(0.0..=1.0).contains(&x2) {
                        return Err(ParseError {
                            kind: ParseErrorKind::Custom(CustomParseError::InvalidValue),
                            location,
                        });
                    }
                    Ok(EasingFunction::CubicBezier(x1, y1, x2, y2))
                },
                "steps" => {
                    let count = input.expect_integer()?;
                    if count <= 0 {
                        return Err(ParseError {
                            kind: ParseErrorKind::Custom(CustomParseError::InvalidValue),
                            location,
                        });
                    }
                    let position = input.try_parse(|input| {
                        input.expect_comma()?;
                        StepPosition::parse(input)
                    }).unwrap_or_default();
                    if position == StepPosition::JumpNone && count == 1 {
                        return Err(ParseError {
                            kind: ParseErrorKind::Custom(CustomParseError::InvalidValue),
                            location,
                        });
                    }
                    Ok(EasingFunction::Steps(count as u32, position))
                },
                _ => Err(location.new_unexpected_token_error(Token::Ident(function.clone())))
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cssparser::ParserInput;

    fn parse(text: &str) -> Result<EasingFunction, ()> {
        let mut input = ParserInput::new(text);
        let mut parser = Parser::new(&mut input);
        EasingFunction::parse(&mut parser).map_err(|_| ())
    }

    #[test]
    fn parses_level_one_easing_functions() {
        assert_eq!(parse("linear"), Ok(EasingFunction::Linear));
        assert_eq!(parse("step-start"), Ok(EasingFunction::Steps(1, StepPosition::Start)));
        assert_eq!(parse("step-end"), Ok(EasingFunction::Steps(1, StepPosition::End)));
        assert_eq!(
            parse("steps(4, jump-start)"),
            Ok(EasingFunction::Steps(4, StepPosition::JumpStart))
        );
        assert_eq!(parse("steps(3)"), Ok(EasingFunction::Steps(3, StepPosition::JumpEnd)));
    }

    #[test]
    fn validates_cubic_bezier_x_coordinates_and_step_count() {
        assert!(parse("cubic-bezier(-0.1, 0, 1, 1)").is_err());
        assert!(parse("cubic-bezier(0, 0, 1.1, 1)").is_err());
        assert!(parse("steps(0)").is_err());
        assert!(parse("steps(1, jump-none)").is_err());
    }
}
