use crate::{CustomParseError, EasingFunction, Ident, Parse, duration::Duration};
use cssparser::{ParseError, Parser};

/// Defines a transition that allows to change property values smoothly, over a given duration.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Transition {
    /// A list of properties affected by transition.
    pub property: String,
    /// The duration of the transition.
    pub duration: Duration,
    /// The delay of the transition.
    pub delay: Option<Duration>,

    pub timing_function: Option<EasingFunction>,
}

impl Transition {
    /// Creates a new transition.
    pub fn new(
        property: String,
        duration: Duration,
        delay: Option<Duration>,
        timing_function: Option<EasingFunction>,
    ) -> Self {
        Self { property, duration, delay, timing_function }
    }

    fn parse_ordered<'i>(input: &mut Parser<'i>) -> Result<Self, ParseError<CustomParseError<'i>>> {
        let mut property = None;
        let mut duration = None;
        let mut delay = None;
        let mut timing_function = None;

        while !input.is_exhausted() {
            if let Ok(time) = input.try_parse(Duration::parse) {
                if duration.is_none() {
                    duration = Some(time);
                } else if delay.is_none() {
                    delay = Some(time);
                } else {
                    return Err(ParseError::custom(CustomParseError::InvalidDeclaration));
                }
            } else if let Ok(easing) = input.try_parse(EasingFunction::parse) {
                if timing_function.replace(easing).is_some() {
                    return Err(ParseError::custom(CustomParseError::InvalidDeclaration));
                }
            } else if let Ok(name) = input.try_parse(Ident::parse) {
                if property.replace(name.into()).is_some() {
                    return Err(ParseError::custom(CustomParseError::InvalidDeclaration));
                }
            } else {
                return Err(ParseError::custom(CustomParseError::InvalidDeclaration));
            }
        }

        match (property, duration) {
            (Some(property), Some(duration)) => {
                Ok(Self { property, duration, delay, timing_function })
            }
            _ => Err(ParseError::custom(CustomParseError::InvalidDeclaration)),
        }
    }

    fn parse_legacy<'i>(input: &mut Parser<'i>) -> Result<Self, ParseError<CustomParseError<'i>>> {
        let property = Ident::parse(input)?.into();
        let duration = Duration::parse(input)?;
        let mut delay = input.try_parse(Duration::parse).ok();
        let timing_function = input.try_parse(EasingFunction::parse).ok();
        if delay.is_none() && timing_function.is_some() {
            delay = input.try_parse(Duration::parse).ok();
        }

        if input.is_exhausted() {
            Ok(Self { property, duration, delay, timing_function })
        } else {
            Err(ParseError::custom(CustomParseError::InvalidDeclaration))
        }
    }
}

impl<'i> Parse<'i> for Transition {
    fn parse<'t>(input: &mut Parser<'i>) -> Result<Self, ParseError<CustomParseError<'i>>> {
        // Preserve legacy property names that also happen to be easing keywords.
        input.try_parse(Self::parse_ordered).or_else(|_| Self::parse_legacy(input))
    }
}

impl<'i> Parse<'i> for Vec<Transition> {
    fn parse<'t>(input: &mut Parser<'i>) -> Result<Self, ParseError<CustomParseError<'i>>> {
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
                "width 2s" => Transition::new(String::from("width"), Duration::from_secs(2), None, None),
                "height 2s 1s" => Transition::new(String::from("height"), Duration::from_secs(2), Some(Duration::from_secs(1)), None),
                "color 200ms linear" => Transition::new(String::from("color"), Duration::from_millis(200), None, Some(EasingFunction::Linear)),
                "opacity 200ms ease 50ms" => Transition::new(String::from("opacity"), Duration::from_millis(200), Some(Duration::from_millis(50)), Some(EasingFunction::Ease)),
                "opacity 200ms 50ms ease" => Transition::new(String::from("opacity"), Duration::from_millis(200), Some(Duration::from_millis(50)), Some(EasingFunction::Ease)),
                "width 1s cubic-bezier(0.1, 0.2, 0.3, 0.4) 2s" => Transition::new(String::from("width"), Duration::from_secs(1), Some(Duration::from_secs(2)), Some(EasingFunction::CubicBezier(0.1, 0.2, 0.3, 0.4))),
                "200ms ease opacity" => Transition::new(String::from("opacity"), Duration::from_millis(200), None, Some(EasingFunction::Ease)),
                "cubic-bezier(0.1, 0.2, 0.3, 0.4) 1s width 2s" => Transition::new(String::from("width"), Duration::from_secs(1), Some(Duration::from_secs(2)), Some(EasingFunction::CubicBezier(0.1, 0.2, 0.3, 0.4))),
                "ease 1s linear" => Transition::new(String::from("ease"), Duration::from_secs(1), None, Some(EasingFunction::Linear)),
            }

            failure {
                "height 2s 1s 1s",
                "width 1s ease linear 2s",
                "width 1s 2s ease 3s",
                "width 1s ease 2s 3s",
                "width 1s ease 2s unknown",
                "1s 2s",
                "width ease",
                "1s width height",
                "1s ease linear width",
                "1s 2s 3s width",
            }
        }
    }

    assert_parse! {
        Vec<Transition>, assert_transitions,

        custom {
            success {
                "height 1s 2s, width 3s 4s, rotation 5s 6s" => vec![
                    Transition::new(String::from("height"), Duration::from_secs(1), Some(Duration::from_secs(2)), None),
                    Transition::new(String::from("width"), Duration::from_secs(3), Some(Duration::from_secs(4)), None),
                    Transition::new(String::from("rotation"), Duration::from_secs(5), Some(Duration::from_secs(6)), None),
                ],
                "opacity 200ms ease 50ms, width 1s 2s linear" => vec![
                    Transition::new(String::from("opacity"), Duration::from_millis(200), Some(Duration::from_millis(50)), Some(EasingFunction::Ease)),
                    Transition::new(String::from("width"), Duration::from_secs(1), Some(Duration::from_secs(2)), Some(EasingFunction::Linear)),
                ],
                "200ms ease opacity, 1s 2s width" => vec![
                    Transition::new(String::from("opacity"), Duration::from_millis(200), None, Some(EasingFunction::Ease)),
                    Transition::new(String::from("width"), Duration::from_secs(1), Some(Duration::from_secs(2)), None),
                ],
            }

            failure {
                "height, width, rotation",
            }
        }
    }

    #[test]
    fn all_transition_component_orders_preserve_time_order() {
        let parts = ["width", "200ms", "ease", "50ms"];
        for a in 0..4 {
            for b in 0..4 {
                for c in 0..4 {
                    for d in 0..4 {
                        if a == b || a == c || a == d || b == c || b == d || c == d {
                            continue;
                        }
                        let order = [a, b, c, d];
                        let value = order.map(|index| parts[index]).join(" ");
                        let mut parser = Parser::new(&value);
                        let transition = Transition::parse(&mut parser).unwrap();
                        let first_time_is_200ms = order.iter().position(|&i| i == 1).unwrap()
                            < order.iter().position(|&i| i == 3).unwrap();
                        let (duration, delay) =
                            if first_time_is_200ms { (200, 50) } else { (50, 200) };
                        assert_eq!(
                            transition,
                            Transition::new(
                                "width".into(),
                                Duration::from_millis(duration),
                                Some(Duration::from_millis(delay)),
                                Some(EasingFunction::Ease),
                            ),
                            "{value}"
                        );
                    }
                }
            }
        }
    }
}
