use crate::{duration::Duration, CustomParseError, EasingFunction, Ident, Parse};
use cssparser::{ParseError, Parser};

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Animation<'i> {
    pub name: AnimationName<'i>,
    pub duration: Duration,
    pub delay: Duration,
    pub timing_function: EasingFunction,
    pub iteration_count: AnimationIterationCount,
    pub direction: AnimationDirection,
    pub fill_mode: AnimationFillMode,
}

impl<'i> Parse<'i> for Animation<'i> {
    fn parse<'t>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i, CustomParseError<'i>>> {
        let mut name = None;
        let mut duration = None;
        let mut timing_function = None;
        let mut iteration_count = None;
        let mut direction = None;
        let mut delay = None;
        let mut fill_mode = None;

        loop {
            if name.is_none() {
                if let Ok(value) = input.try_parse(AnimationName::parse) {
                    name = Some(value);
                    continue;
                }
            }

            if duration.is_none() {
                if let Ok(value) = input.try_parse(Duration::parse) {
                    duration = Some(value);
                    continue;
                }
            }

            if timing_function.is_none() {
                if let Ok(value) = input.try_parse(EasingFunction::parse) {
                    timing_function = Some(value);
                    continue;
                }
            }

            if iteration_count.is_none() {
                if let Ok(value) = input.try_parse(AnimationIterationCount::parse) {
                    iteration_count = Some(value);
                    continue;
                }
            }

            if direction.is_none() {
                if let Ok(value) = input.try_parse(AnimationDirection::parse) {
                    direction = Some(value);
                    continue;
                }
            }

            if delay.is_none() {
                if let Ok(value) = input.try_parse(Duration::parse) {
                    delay = Some(value);
                    continue;
                }
            }

            if fill_mode.is_none() {
                if let Ok(value) = input.try_parse(AnimationFillMode::parse) {
                    fill_mode = Some(value);
                    continue;
                }
            }

            break;
        }

        Ok(Self {
            name: name.unwrap_or_default(),
            duration: duration.unwrap_or_default(),
            delay: delay.unwrap_or_default(),
            timing_function: timing_function.unwrap_or_default(),
            iteration_count: iteration_count.unwrap_or_default(),
            direction: direction.unwrap_or_default(),
            fill_mode: fill_mode.unwrap_or_default(),
        })
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub enum AnimationName<'i> {
    #[default]
    None,
    Ident(Ident<'i>),
    String(String),
}

// implement display for AnimationName
impl std::fmt::Display for AnimationName<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnimationName::None => write!(f, "none"),
            AnimationName::Ident(ident) => write!(f, "{}", ident.0),
            AnimationName::String(string) => write!(f, "\"{}\"", string),
        }
    }
}

impl<'i> Parse<'i> for AnimationName<'i> {
    fn parse<'t>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i, CustomParseError<'i>>> {
        let location = input.current_source_location();
        if let Ok(ident) = input.try_parse(Ident::parse) {
            return Ok(AnimationName::Ident(ident));
        }

        if let Ok(string) = input.try_parse(String::parse) {
            return Ok(AnimationName::String(string));
        }

        Err(location.new_custom_error(CustomParseError::InvalidValue))
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AnimationIterationCount {
    Infinite,
    Count(u32),
}

impl AnimationIterationCount {
    pub fn is_infinite(&self) -> bool {
        matches!(self, AnimationIterationCount::Infinite)
    }

    pub fn to_count(&self) -> u32 {
        match self {
            AnimationIterationCount::Infinite => u32::MAX,
            AnimationIterationCount::Count(count) => *count,
        }
    }
}

impl Default for AnimationIterationCount {
    fn default() -> Self {
        AnimationIterationCount::Count(1)
    }
}

impl<'i> Parse<'i> for AnimationIterationCount {
    fn parse<'t>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i, CustomParseError<'i>>> {
        let location = input.current_source_location();
        if let Ok(count) = input.try_parse(u32::parse) {
            return Ok(AnimationIterationCount::Count(count));
        }

        if input.try_parse(|input| input.expect_ident_matching("infinite")).is_ok() {
            return Ok(AnimationIterationCount::Infinite);
        }

        Err(location.new_custom_error(CustomParseError::InvalidValue))
    }
}

#[derive(Default, Clone, Copy, Debug, PartialEq)]
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
        if input.try_parse(|input| input.expect_ident_matching("normal")).is_ok() {
            return Ok(AnimationDirection::Normal);
        }

        if input.try_parse(|input| input.expect_ident_matching("reverse")).is_ok() {
            return Ok(AnimationDirection::Reverse);
        }

        if input.try_parse(|input| input.expect_ident_matching("alternate")).is_ok() {
            return Ok(AnimationDirection::Alternate);
        }

        if input.try_parse(|input| input.expect_ident_matching("alternate-reverse")).is_ok() {
            return Ok(AnimationDirection::AlternateReverse);
        }

        Err(location.new_custom_error(CustomParseError::InvalidValue))
    }
}

#[derive(Default, Clone, Copy, Debug, PartialEq)]
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
        if input.try_parse(|input| input.expect_ident_matching("none")).is_ok() {
            return Ok(AnimationFillMode::None);
        }

        if input.try_parse(|input| input.expect_ident_matching("forwards")).is_ok() {
            return Ok(AnimationFillMode::Forwards);
        }

        if input.try_parse(|input| input.expect_ident_matching("backwards")).is_ok() {
            return Ok(AnimationFillMode::Backwards);
        }

        if input.try_parse(|input| input.expect_ident_matching("both")).is_ok() {
            return Ok(AnimationFillMode::Both);
        }

        Err(location.new_custom_error(CustomParseError::InvalidValue))
    }
}
