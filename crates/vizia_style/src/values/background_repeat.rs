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
    Axes {
        x: BackgroundRepeatMode,
        y: BackgroundRepeatMode,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BackgroundRepeatMode {
    Repeat,
    NoRepeat,
    Space,
    Round,
}

impl BackgroundRepeat {
    pub fn axes(self) -> (BackgroundRepeatMode, BackgroundRepeatMode) {
        use BackgroundRepeatMode::*;
        match self {
            Self::Repeat => (Repeat, Repeat),
            Self::RepeatX => (Repeat, NoRepeat),
            Self::RepeatY => (NoRepeat, Repeat),
            Self::NoRepeat => (NoRepeat, NoRepeat),
            Self::Axes { x, y } => (x, y),
        }
    }

    fn from_axes(x: BackgroundRepeatMode, y: BackgroundRepeatMode) -> Self {
        use BackgroundRepeatMode::*;
        match (x, y) {
            (Repeat, Repeat) => Self::Repeat,
            (Repeat, NoRepeat) => Self::RepeatX,
            (NoRepeat, Repeat) => Self::RepeatY,
            (NoRepeat, NoRepeat) => Self::NoRepeat,
            _ => Self::Axes { x, y },
        }
    }
}

impl<'i> Parse<'i> for BackgroundRepeat {
    fn parse<'t>(input: &mut Parser<'i>) -> Result<Self, ParseError<CustomParseError<'i>>> {
        fn mode<'i>(
            input: &mut Parser<'i>,
        ) -> Result<BackgroundRepeatMode, ParseError<CustomParseError<'i>>> {
            let location = input.current_source_location();
            let ident = input.expect_ident()?;
            Ok(match_ignore_ascii_case! { ident,
                "repeat" => BackgroundRepeatMode::Repeat,
                "no-repeat" => BackgroundRepeatMode::NoRepeat,
                "space" => BackgroundRepeatMode::Space,
                "round" => BackgroundRepeatMode::Round,
                _ => return Err(location.new_unexpected_token_error(Token::Ident(ident.clone())))
            })
        }

        if input.try_parse(|input| input.expect_ident_matching("repeat-x")).is_ok() {
            return Ok(Self::RepeatX);
        }
        if input.try_parse(|input| input.expect_ident_matching("repeat-y")).is_ok() {
            return Ok(Self::RepeatY);
        }
        let x = mode(input)?;
        let y = input.try_parse(mode).unwrap_or(x);
        Ok(Self::from_axes(x, y))
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
            "round repeat-x",
            "space unknown",
            "repeat no-repeat,",
        ] {
            assert!(Vec::<BackgroundRepeat>::parse(&mut Parser::new(value)).is_err(), "{value}");
        }
    }

    #[test]
    fn background_repeat_parses_space_and_round_in_each_axis() {
        use BackgroundRepeatMode::*;
        let modes =
            [("repeat", Repeat), ("no-repeat", NoRepeat), ("space", Space), ("round", Round)];
        for (x_name, x) in modes {
            for (y_name, y) in modes {
                let value = format!("{x_name} {y_name}");
                let mut parser = Parser::new(&value);
                let values = Vec::<BackgroundRepeat>::parse(&mut parser).unwrap();
                assert_eq!(values[0].axes(), (x, y), "{value}");
                assert!(parser.is_exhausted(), "{value}");
            }
        }
        for (value, mode) in [("SPACE", Space), ("round", Round)] {
            let mut parser = Parser::new(value);
            let values = Vec::<BackgroundRepeat>::parse(&mut parser).unwrap();
            assert_eq!(values[0].axes(), (mode, mode), "{value}");
            assert!(parser.is_exhausted(), "{value}");
        }
    }
}
