use crate::{duration::Duration, CustomParseError, Ident, Parse};
use cssparser::{ParseError, ParseErrorKind, Parser};

/// Defines a transition that allows to change property values smoothly, over a given duration.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Transition {
    /// A list of properties affected by transition.
    pub property: String,
    /// The duration of the transition.
    pub duration: Duration,
    /// The delay of the transition.
    pub delay: Option<Duration>,
}

impl Transition {
    /// Creates a new transition.
    pub fn new(property: String, duration: Duration, delay: Option<Duration>) -> Self {
        Self {
            property,
            duration,
            delay,
        }
    }
}

impl<'i> Parse<'i> for Transition {
    fn parse<'t>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i, CustomParseError<'i>>> {
        let location = input.current_source_location();

        let property = Ident::parse(input)?.into();
        let duration = Duration::parse(input)?;
        let delay = input.try_parse(Duration::parse).ok();

        if input.is_exhausted() {
            Ok(Self {
                property,
                duration,
                delay,
            })
        } else {
            Err(ParseError {
                kind: ParseErrorKind::Custom(CustomParseError::InvalidDeclaration),
                location,
            })
        }
    }
}

impl<'i> Parse<'i> for Vec<Transition> {
    fn parse<'t>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i, CustomParseError<'i>>> {
        input.parse_comma_separated(Transition::parse)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::assert_parse;

    assert_parse! {
        Transition, assert_transition,

        custom {
            success {
                "width 2s" => Transition::new(String::from("width"), Duration::from_secs(2), None),
                "height 2s 1s" => Transition::new(String::from("height"), Duration::from_secs(2), Some(Duration::from_secs(1))),
            }

            failure {
                "height 2s 1s 1s",
                "1s 2s height",
            }
        }
    }

    assert_parse! {
        Vec<Transition>, assert_transitions,

        custom {
            success {
                "height 1s 2s, width 3s 4s, rotation 5s 6s" => vec![
                    Transition::new(String::from("height"), Duration::from_secs(1), Some(Duration::from_secs(2))),
                    Transition::new(String::from("width"), Duration::from_secs(3), Some(Duration::from_secs(4))),
                    Transition::new(String::from("rotation"), Duration::from_secs(5), Some(Duration::from_secs(6))),
                ],
            }

            failure {
                "height, width, rotation",
            }
        }
    }
}
