use crate::ParserErrorExt;
use crate::{CustomParseError, Parse};
use cssparser::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BackgroundRepeat {
    #[default]
    Repeat,
    RepeatX,
    RepeatY,
    NoRepeat,
}

impl<'i> Parse<'i> for BackgroundRepeat {
    fn parse<'t>(input: &mut Parser<'i>) -> Result<Self, ParseError<CustomParseError<'i>>> {
        let location = input.current_source_location();
        let ident = input.expect_ident()?;
        let first = match_ignore_ascii_case! { ident,
            "repeat" => BackgroundRepeat::Repeat,
            "repeat-x" => BackgroundRepeat::RepeatX,
            "repeat-y" => BackgroundRepeat::RepeatY,
            "no-repeat" => BackgroundRepeat::NoRepeat,
            _ => return Err(location.new_unexpected_token_error(Token::Ident(ident.clone())))
        };
        if matches!(first, BackgroundRepeat::Repeat | BackgroundRepeat::NoRepeat) {
            let second = input.try_parse(|input| -> Result<bool, BasicParseError> {
                if input.try_parse(|input| input.expect_ident_matching("repeat")).is_ok() {
                    Ok(true)
                } else {
                    input.expect_ident_matching("no-repeat")?;
                    Ok(false)
                }
            });
            if let Ok(repeat_y) = second {
                return Ok(match (first == BackgroundRepeat::Repeat, repeat_y) {
                    (true, true) => BackgroundRepeat::Repeat,
                    (true, false) => BackgroundRepeat::RepeatX,
                    (false, true) => BackgroundRepeat::RepeatY,
                    (false, false) => BackgroundRepeat::NoRepeat,
                });
            }
        }
        Ok(first)
    }
}

impl<'i> Parse<'i> for Vec<BackgroundRepeat> {
    fn parse<'t>(input: &mut Parser<'i>) -> Result<Self, ParseError<CustomParseError<'i>>> {
        input.parse_comma_separated(BackgroundRepeat::parse)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn background_repeat_accepts_axis_pairs_and_layers() {
        for (value, expected) in [
            ("repeat repeat", BackgroundRepeat::Repeat),
            ("repeat no-repeat", BackgroundRepeat::RepeatX),
            ("no-repeat repeat", BackgroundRepeat::RepeatY),
            ("no-repeat no-repeat", BackgroundRepeat::NoRepeat),
            ("REPEAT No-Repeat", BackgroundRepeat::RepeatX),
            ("repeat-x", BackgroundRepeat::RepeatX),
            ("repeat-y", BackgroundRepeat::RepeatY),
            ("repeat", BackgroundRepeat::Repeat),
            ("no-repeat", BackgroundRepeat::NoRepeat),
        ] {
            let mut parser = Parser::new(value);
            assert_eq!(
                Vec::<BackgroundRepeat>::parse(&mut parser).unwrap(),
                vec![expected],
                "{value}"
            );
            assert!(parser.is_exhausted(), "{value}");
        }
        let mut parser = Parser::new("repeat no-repeat, no-repeat repeat, repeat-x");
        assert_eq!(
            Vec::<BackgroundRepeat>::parse(&mut parser).unwrap(),
            vec![BackgroundRepeat::RepeatX, BackgroundRepeat::RepeatY, BackgroundRepeat::RepeatX]
        );
    }

    #[test]
    fn background_repeat_rejects_invalid_axis_pairs() {
        for value in [
            "repeat-x repeat",
            "repeat-y no-repeat",
            "repeat repeat repeat",
            "repeat space",
            "round no-repeat",
            "repeat no-repeat,",
        ] {
            assert!(Vec::<BackgroundRepeat>::parse(&mut Parser::new(value)).is_err(), "{value}");
        }
    }
}
