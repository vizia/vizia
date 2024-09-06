use crate::{CustomParseError, Parse};
use cssparser::*;

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub enum EasingFunction {
    #[default]
    Linear,
    Ease,
    EaseIn,
    EaseOut,
    EaseInOut,
    CubicBezier(f32, f32, f32, f32),
    // TODO: Steps
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
            //   "step-start" => EasingFunction::Steps { count: 1, position: StepPosition::Start },
            //   "step-end" => EasingFunction::Steps { count: 1, position: StepPosition::End },
              _ => return Err(location.new_unexpected_token_error(Token::Ident(ident.clone())))
            };
            return Ok(keyword);
        }

        let function = input.expect_function()?.clone();
        input.parse_nested_block(|input| {
            match_ignore_ascii_case! { &function,
              "cubic-bezier" => {
                // let x1 = CSSNumber::parse(input)?;
                let x1 = input.try_parse(|input| input.expect_number())?;
                input.expect_comma()?;
                let y1 = input.try_parse(|input| input.expect_number())?;
                input.expect_comma()?;
                let x2 = input.try_parse(|input| input.expect_number())?;
                input.expect_comma()?;
                let y2 = input.try_parse(|input| input.expect_number())?;
                Ok(EasingFunction::CubicBezier(x1, y1, x2, y2))
              },
            //   "steps" => {
            //     let count = CSSInteger::parse(input)?;
            //     let position = input.try_parse(|input| {
            //       input.expect_comma()?;
            //       StepPosition::parse(input)
            //     }).unwrap_or_default();
            //     Ok(EasingFunction::Steps { count, position })
            //   },
              _ => return Err(location.new_unexpected_token_error(Token::Ident(function.clone())))
            }
        })
    }
}
