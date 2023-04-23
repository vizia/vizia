use crate::{CustomParseError, Parse};
use cssparser::*;

/// Describes four sides of a rectangle.
///
/// It is for example used for [`Overflow`](crate::Overflow) or [`BorderRadius`](crate::BorderRadius).
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Rect<T>(
    /// The first value.
    pub T,
    /// The second value.
    pub T,
    /// The third value.
    pub T,
    /// The fourth value.
    pub T,
);

impl<'i, T> Parse<'i> for Rect<T>
where
    T: Parse<'i> + Clone,
{
    fn parse<'t>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i, CustomParseError<'i>>> {
        let location = input.current_source_location();
        let first = T::parse(input)?;

        let second = if let Ok(second) = input.try_parse(T::parse) {
            second
        } else {
            return Ok(Self(first.clone(), first.clone(), first.clone(), first));
        };

        let third = if let Ok(third) = input.try_parse(T::parse) {
            third
        } else {
            return Ok(Self(first.clone(), second.clone(), first, second));
        };

        let fourth = if let Ok(fourth) = input.try_parse(T::parse) {
            fourth
        } else {
            return Ok(Self(first, second.clone(), third, second));
        };

        if input.is_exhausted() {
            Ok(Self(first, second, third, fourth))
        } else {
            Err(cssparser::ParseError {
                kind: cssparser::ParseErrorKind::Custom(CustomParseError::InvalidDeclaration),
                location,
            })
        }
    }
}

impl<T: Default> Default for Rect<T> {
    fn default() -> Self {
        Self(T::default(), T::default(), T::default(), T::default())
    }
}

impl<T: Clone + Default> From<&str> for Rect<T>
where
    for<'i> T: Parse<'i>,
{
    fn from(s: &str) -> Self {
        let mut input = ParserInput::new(s);
        let mut parser = Parser::new(&mut input);
        Rect::parse(&mut parser).unwrap_or_default()
    }
}

impl<T: Clone> From<(T, T)> for Rect<T> {
    fn from(value: (T, T)) -> Self {
        Rect(value.0.clone(), value.1.clone(), value.0.clone(), value.1)
    }
}

impl<T: Clone> From<(T, T, T)> for Rect<T> {
    fn from(value: (T, T, T)) -> Self {
        Rect(value.0.clone(), value.1.clone(), value.2.clone(), value.1)
    }
}

impl<T: Clone> From<(T, T, T, T)> for Rect<T> {
    fn from(value: (T, T, T, T)) -> Self {
        Rect(value.0.clone(), value.1.clone(), value.2.clone(), value.3)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::assert_parse;

    assert_parse! {
        Rect<u8>, assert_rect,

        custom {
            success {
                "1" => Rect(1, 1, 1, 1),
                "1 2" => Rect(1, 2, 1, 2),
                "1 2 3" => Rect(1, 2, 3, 2),
                "1 2 3 4" => Rect(1, 2, 3, 4),
            }

            failure {
                "1 2 3 4 5",
                "test",
            }
        }
    }
}
