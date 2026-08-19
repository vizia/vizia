use crate::{CustomParseError, EasingFunction, Parse};
use cssparser::{ParseError, ParseErrorKind, Parser, Token, match_ignore_ascii_case};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum AnimationName {
    #[default]
    None,
    Custom(String),
}

impl<'i> Parse<'i> for AnimationName {
    fn parse<'t>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i, CustomParseError<'i>>> {
        let ident = input.expect_ident_cloned()?;
        if ident.as_ref().eq_ignore_ascii_case("none") {
            Ok(Self::None)
        } else {
            Ok(Self::Custom(ident.to_string()))
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct AnimationNames(pub Vec<AnimationName>);

impl<'i> Parse<'i> for AnimationNames {
    fn parse<'t>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i, CustomParseError<'i>>> {
        input.parse_comma_separated(AnimationName::parse).map(Self)
    }
}

/// A CSS animation time in seconds. Unlike `std::time::Duration`, this can represent
/// the negative values permitted by `animation-delay`.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct AnimationTime(pub f32);

impl AnimationTime {
    pub fn seconds(self) -> f32 {
        self.0
    }
}

impl<'i> Parse<'i> for AnimationTime {
    fn parse<'t>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i, CustomParseError<'i>>> {
        let location = input.current_source_location();
        match input.next()? {
            Token::Dimension { value, unit, .. } if unit.as_ref().eq_ignore_ascii_case("s") => {
                Ok(Self(*value))
            }
            Token::Dimension { value, unit, .. } if unit.as_ref().eq_ignore_ascii_case("ms") => {
                Ok(Self(*value / 1000.0))
            }
            token => Err(location.new_unexpected_token_error(token.clone())),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct AnimationDuration(pub AnimationTime);

impl<'i> Parse<'i> for AnimationDuration {
    fn parse<'t>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i, CustomParseError<'i>>> {
        let location = input.current_source_location();
        let time = AnimationTime::parse(input)?;
        if time.0 < 0.0 {
            return Err(ParseError {
                kind: ParseErrorKind::Custom(CustomParseError::InvalidValue),
                location,
            });
        }
        Ok(Self(time))
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct AnimationDurations(pub Vec<AnimationDuration>);

impl<'i> Parse<'i> for AnimationDurations {
    fn parse<'t>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i, CustomParseError<'i>>> {
        input.parse_comma_separated(AnimationDuration::parse).map(Self)
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct AnimationDelays(pub Vec<AnimationTime>);

impl<'i> Parse<'i> for AnimationDelays {
    fn parse<'t>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i, CustomParseError<'i>>> {
        input.parse_comma_separated(AnimationTime::parse).map(Self)
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct AnimationTimingFunctions(pub Vec<EasingFunction>);

impl<'i> Parse<'i> for AnimationTimingFunctions {
    fn parse<'t>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i, CustomParseError<'i>>> {
        input.parse_comma_separated(EasingFunction::parse).map(Self)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AnimationIterationCount {
    Number(f32),
    Infinite,
}

impl Default for AnimationIterationCount {
    fn default() -> Self {
        Self::Number(1.0)
    }
}

impl<'i> Parse<'i> for AnimationIterationCount {
    fn parse<'t>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i, CustomParseError<'i>>> {
        if input.try_parse(|i| i.expect_ident_matching("infinite")).is_ok() {
            return Ok(Self::Infinite);
        }
        let location = input.current_source_location();
        let count = input.expect_number()?;
        if count < 0.0 {
            return Err(ParseError {
                kind: ParseErrorKind::Custom(CustomParseError::InvalidValue),
                location,
            });
        }
        Ok(Self::Number(count))
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct AnimationIterationCounts(pub Vec<AnimationIterationCount>);

impl<'i> Parse<'i> for AnimationIterationCounts {
    fn parse<'t>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i, CustomParseError<'i>>> {
        input.parse_comma_separated(AnimationIterationCount::parse).map(Self)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AnimationDirection {
    #[default]
    Normal,
    Reverse,
    Alternate,
    AlternateReverse,
}

impl<'i> Parse<'i> for AnimationDirection {
    fn parse<'t>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i, CustomParseError<'i>>> {
        let location = input.current_source_location();
        let ident = input.expect_ident_cloned()?;
        match_ignore_ascii_case! { &ident,
            "normal" => Ok(Self::Normal),
            "reverse" => Ok(Self::Reverse),
            "alternate" => Ok(Self::Alternate),
            "alternate-reverse" => Ok(Self::AlternateReverse),
            _ => Err(location.new_unexpected_token_error(Token::Ident(ident))),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct AnimationDirections(pub Vec<AnimationDirection>);

impl<'i> Parse<'i> for AnimationDirections {
    fn parse<'t>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i, CustomParseError<'i>>> {
        input.parse_comma_separated(AnimationDirection::parse).map(Self)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AnimationFillMode {
    #[default]
    None,
    Forwards,
    Backwards,
    Both,
}

impl<'i> Parse<'i> for AnimationFillMode {
    fn parse<'t>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i, CustomParseError<'i>>> {
        let location = input.current_source_location();
        let ident = input.expect_ident_cloned()?;
        match_ignore_ascii_case! { &ident,
            "none" => Ok(Self::None),
            "forwards" => Ok(Self::Forwards),
            "backwards" => Ok(Self::Backwards),
            "both" => Ok(Self::Both),
            _ => Err(location.new_unexpected_token_error(Token::Ident(ident))),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct AnimationFillModes(pub Vec<AnimationFillMode>);

impl<'i> Parse<'i> for AnimationFillModes {
    fn parse<'t>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i, CustomParseError<'i>>> {
        input.parse_comma_separated(AnimationFillMode::parse).map(Self)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AnimationPlayState {
    #[default]
    Running,
    Paused,
}

impl<'i> Parse<'i> for AnimationPlayState {
    fn parse<'t>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i, CustomParseError<'i>>> {
        let location = input.current_source_location();
        let ident = input.expect_ident_cloned()?;
        match_ignore_ascii_case! { &ident,
            "running" => Ok(Self::Running),
            "paused" => Ok(Self::Paused),
            _ => Err(location.new_unexpected_token_error(Token::Ident(ident))),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct AnimationPlayStates(pub Vec<AnimationPlayState>);

impl<'i> Parse<'i> for AnimationPlayStates {
    fn parse<'t>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i, CustomParseError<'i>>> {
        input.parse_comma_separated(AnimationPlayState::parse).map(Self)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AnimationComposition {
    #[default]
    Replace,
    Add,
    Accumulate,
}

impl<'i> Parse<'i> for AnimationComposition {
    fn parse<'t>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i, CustomParseError<'i>>> {
        let location = input.current_source_location();
        let ident = input.expect_ident_cloned()?;
        match_ignore_ascii_case! { &ident,
            "replace" => Ok(Self::Replace),
            "add" => Ok(Self::Add),
            "accumulate" => Ok(Self::Accumulate),
            _ => Err(location.new_unexpected_token_error(Token::Ident(ident))),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct AnimationCompositions(pub Vec<AnimationComposition>);

impl<'i> Parse<'i> for AnimationCompositions {
    fn parse<'t>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i, CustomParseError<'i>>> {
        input.parse_comma_separated(AnimationComposition::parse).map(Self)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AnimationTimelineAxis {
    #[default]
    Block,
    Inline,
    X,
    Y,
}

impl AnimationTimelineAxis {
    fn from_ident(ident: &str) -> Option<Self> {
        if ident.eq_ignore_ascii_case("block") {
            Some(Self::Block)
        } else if ident.eq_ignore_ascii_case("inline") {
            Some(Self::Inline)
        } else if ident.eq_ignore_ascii_case("x") {
            Some(Self::X)
        } else if ident.eq_ignore_ascii_case("y") {
            Some(Self::Y)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AnimationScroller {
    Root,
    #[default]
    Nearest,
    Self_,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum AnimationTimeline {
    #[default]
    Auto,
    None,
    Named(String),
    Scroll {
        scroller: AnimationScroller,
        axis: AnimationTimelineAxis,
    },
    View {
        axis: AnimationTimelineAxis,
    },
}

impl<'i> Parse<'i> for AnimationTimeline {
    fn parse<'t>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i, CustomParseError<'i>>> {
        if let Ok(ident) = input.try_parse(|input| input.expect_ident_cloned()) {
            if ident.as_ref().eq_ignore_ascii_case("auto") {
                return Ok(Self::Auto);
            }
            if ident.as_ref().eq_ignore_ascii_case("none") {
                return Ok(Self::None);
            }
            if ident.as_ref().starts_with("--") {
                return Ok(Self::Named(ident.to_string()));
            }
            return Err(input.new_custom_error(CustomParseError::InvalidValue));
        }

        if input.try_parse(|input| input.expect_function_matching("scroll")).is_ok() {
            return input.parse_nested_block(|input| {
                let mut scroller = AnimationScroller::Nearest;
                let mut axis = AnimationTimelineAxis::Block;
                let mut seen_scroller = false;
                let mut seen_axis = false;
                while !input.is_exhausted() {
                    let location = input.current_source_location();
                    let ident = input.expect_ident_cloned()?;
                    if let Some(value) = AnimationTimelineAxis::from_ident(ident.as_ref()) {
                        if seen_axis {
                            return Err(location.new_unexpected_token_error(Token::Ident(ident)));
                        }
                        seen_axis = true;
                        axis = value;
                        continue;
                    }
                    let value = if ident.as_ref().eq_ignore_ascii_case("root") {
                        Some(AnimationScroller::Root)
                    } else if ident.as_ref().eq_ignore_ascii_case("nearest") {
                        Some(AnimationScroller::Nearest)
                    } else if ident.as_ref().eq_ignore_ascii_case("self") {
                        Some(AnimationScroller::Self_)
                    } else {
                        None
                    };
                    let Some(value) = value else {
                        return Err(location.new_unexpected_token_error(Token::Ident(ident)));
                    };
                    if seen_scroller {
                        return Err(location.new_unexpected_token_error(Token::Ident(ident)));
                    }
                    seen_scroller = true;
                    scroller = value;
                }
                Ok(Self::Scroll { scroller, axis })
            });
        }

        if input.try_parse(|input| input.expect_function_matching("view")).is_ok() {
            return input.parse_nested_block(|input| {
                let axis = if input.is_exhausted() {
                    AnimationTimelineAxis::Block
                } else {
                    let location = input.current_source_location();
                    let ident = input.expect_ident_cloned()?;
                    let Some(axis) = AnimationTimelineAxis::from_ident(ident.as_ref()) else {
                        return Err(location.new_unexpected_token_error(Token::Ident(ident)));
                    };
                    if !input.is_exhausted() {
                        let token = input.next()?.clone();
                        return Err(location.new_unexpected_token_error(token));
                    }
                    axis
                };
                Ok(Self::View { axis })
            });
        }

        let location = input.current_source_location();
        let token = input.next()?.clone();
        Err(location.new_unexpected_token_error(token))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct AnimationTimelines(pub Vec<AnimationTimeline>);

impl<'i> Parse<'i> for AnimationTimelines {
    fn parse<'t>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i, CustomParseError<'i>>> {
        input.parse_comma_separated(AnimationTimeline::parse).map(Self)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct AnimationShorthandItem {
    pub name: AnimationName,
    pub duration: AnimationDuration,
    pub timing_function: EasingFunction,
    pub delay: AnimationTime,
    pub iteration_count: AnimationIterationCount,
    pub direction: AnimationDirection,
    pub fill_mode: AnimationFillMode,
    pub play_state: AnimationPlayState,
}

impl Default for AnimationShorthandItem {
    fn default() -> Self {
        Self {
            name: AnimationName::None,
            duration: AnimationDuration::default(),
            timing_function: EasingFunction::Ease,
            delay: AnimationTime::default(),
            iteration_count: AnimationIterationCount::default(),
            direction: AnimationDirection::default(),
            fill_mode: AnimationFillMode::default(),
            play_state: AnimationPlayState::default(),
        }
    }
}

impl<'i> Parse<'i> for AnimationShorthandItem {
    fn parse<'t>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i, CustomParseError<'i>>> {
        let location = input.current_source_location();
        let mut result = Self::default();
        let mut duration = false;
        let mut delay = false;
        let mut timing = false;
        let mut iteration = false;
        let mut direction = false;
        let mut fill = false;
        let mut state = false;
        let mut name = false;

        while !input.is_exhausted() {
            if let Ok(time) = input.try_parse(AnimationTime::parse) {
                if !duration {
                    if time.0 < 0.0 {
                        return Err(ParseError {
                            kind: ParseErrorKind::Custom(CustomParseError::InvalidValue),
                            location,
                        });
                    }
                    result.duration = AnimationDuration(time);
                    duration = true;
                    continue;
                }
                if !delay {
                    result.delay = time;
                    delay = true;
                    continue;
                }
                return Err(ParseError {
                    kind: ParseErrorKind::Custom(CustomParseError::InvalidDeclaration),
                    location,
                });
            }

            if !timing {
                if let Ok(value) = input.try_parse(EasingFunction::parse) {
                    result.timing_function = value;
                    timing = true;
                    continue;
                }
            }
            if !iteration {
                if let Ok(value) = input.try_parse(AnimationIterationCount::parse) {
                    result.iteration_count = value;
                    iteration = true;
                    continue;
                }
            }
            if !direction {
                if let Ok(value) = input.try_parse(AnimationDirection::parse) {
                    result.direction = value;
                    direction = true;
                    continue;
                }
            }
            if !fill {
                if let Ok(value) = input.try_parse(AnimationFillMode::parse) {
                    result.fill_mode = value;
                    fill = true;
                    continue;
                }
            }
            if !state {
                if let Ok(value) = input.try_parse(AnimationPlayState::parse) {
                    result.play_state = value;
                    state = true;
                    continue;
                }
            }
            if !name {
                if let Ok(value) = input.try_parse(AnimationName::parse) {
                    result.name = value;
                    name = true;
                    continue;
                }
            }

            let token = input.next()?.clone();
            return Err(location.new_unexpected_token_error(token));
        }

        Ok(result)
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct AnimationShorthand(pub Vec<AnimationShorthandItem>);

impl<'i> Parse<'i> for AnimationShorthand {
    fn parse<'t>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i, CustomParseError<'i>>> {
        input.parse_comma_separated(AnimationShorthandItem::parse).map(Self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cssparser::ParserInput;

    fn parse_shorthand(text: &str) -> AnimationShorthand {
        let mut input = ParserInput::new(text);
        let mut parser = Parser::new(&mut input);
        AnimationShorthand::parse(&mut parser).expect("animation shorthand should parse")
    }

    #[test]
    fn parses_animation_timeline_values() {
        let mut input =
            ParserInput::new("auto, --gallery, scroll(nearest y), scroll(x self), view(block)");
        let mut parser = Parser::new(&mut input);
        let parsed = AnimationTimelines::parse(&mut parser).expect("timeline list should parse");
        assert_eq!(parsed.0.len(), 5);
        assert_eq!(parsed.0[0], AnimationTimeline::Auto);
        assert_eq!(parsed.0[1], AnimationTimeline::Named("--gallery".into()));
        assert_eq!(
            parsed.0[2],
            AnimationTimeline::Scroll {
                scroller: AnimationScroller::Nearest,
                axis: AnimationTimelineAxis::Y,
            }
        );
        assert_eq!(
            parsed.0[3],
            AnimationTimeline::Scroll {
                scroller: AnimationScroller::Self_,
                axis: AnimationTimelineAxis::X,
            }
        );
        assert_eq!(parsed.0[4], AnimationTimeline::View { axis: AnimationTimelineAxis::Block });
    }

    #[test]
    fn parses_animation_composition_list() {
        let mut input = ParserInput::new("replace, add, accumulate");
        let mut parser = Parser::new(&mut input);
        let parsed =
            AnimationCompositions::parse(&mut parser).expect("composition list should parse");
        assert_eq!(
            parsed.0,
            vec![
                AnimationComposition::Replace,
                AnimationComposition::Add,
                AnimationComposition::Accumulate,
            ]
        );
    }

    #[test]
    fn parses_shorthand_time_ambiguity_and_all_level_one_fields() {
        let parsed =
            parse_shorthand("slide 250ms ease-in -100ms 2.5 alternate-reverse both paused");
        let item = &parsed.0[0];
        assert_eq!(item.name, AnimationName::Custom("slide".into()));
        assert_eq!(item.duration.0, AnimationTime(0.25));
        assert_eq!(item.delay, AnimationTime(-0.1));
        assert_eq!(item.iteration_count, AnimationIterationCount::Number(2.5));
        assert_eq!(item.direction, AnimationDirection::AlternateReverse);
        assert_eq!(item.fill_mode, AnimationFillMode::Both);
        assert_eq!(item.play_state, AnimationPlayState::Paused);
        assert_eq!(item.timing_function, EasingFunction::EaseIn);
    }

    #[test]
    fn parses_comma_separated_animations() {
        let parsed = parse_shorthand("fade 1s, spin 2s linear infinite reverse forwards");
        assert_eq!(parsed.0.len(), 2);
        assert_eq!(parsed.0[1].name, AnimationName::Custom("spin".into()));
        assert_eq!(parsed.0[1].iteration_count, AnimationIterationCount::Infinite);
    }

    #[test]
    fn rejects_negative_duration_but_accepts_negative_delay() {
        let mut input = ParserInput::new("fade -1s");
        let mut parser = Parser::new(&mut input);
        assert!(AnimationShorthand::parse(&mut parser).is_err());

        let parsed = parse_shorthand("fade 1s -250ms");
        assert_eq!(parsed.0[0].delay, AnimationTime(-0.25));
    }
}
