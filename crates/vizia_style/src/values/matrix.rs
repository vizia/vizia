use crate::{CustomParseError, Parse};
use cssparser::{ParseError, Parser};

/// A 2D matrix.
#[derive(Debug, PartialEq, Clone)]
pub struct Matrix<T> {
    pub a: T,
    pub b: T,
    pub c: T,
    pub d: T,
    pub e: T,
    pub f: T,
}

impl<T> Matrix<T> {
    /// Creates a new 2D matrix.
    pub fn new(a: T, b: T, c: T, d: T, e: T, f: T) -> Self {
        Self { a, b, c, d, e, f }
    }
}

impl<'i, T> Parse<'i> for Matrix<T>
where
    T: Parse<'i> + Clone,
{
    fn parse<'t>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i, CustomParseError<'i>>> {
        let location = input.current_source_location();
        let fields = input.parse_comma_separated(T::parse)?;

        if fields.len() == 6 {
            let a = fields[0].clone();
            let b = fields[1].clone();
            let c = fields[2].clone();
            let d = fields[3].clone();
            let e = fields[4].clone();
            let f = fields[5].clone();

            Ok(Self::new(a, b, c, d, e, f))
        } else {
            Err(cssparser::ParseError {
                kind: cssparser::ParseErrorKind::Custom(CustomParseError::InvalidDeclaration),
                location,
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::assert_parse;

    assert_parse! {
        Matrix<u8>, assert_matrix,

        custom {
            success {
                "1, 2, 3, 4, 5, 6" => Matrix::new(1, 2, 3, 4, 5, 6),
                "6, 5, 4, 3, 2, 1" => Matrix::new(6, 5, 4, 3, 2, 1),
            }

            failure {
                "1 2 3 4 5 6",
                "1 2 3 4 5",
                "test",
            }
        }
    }
}
