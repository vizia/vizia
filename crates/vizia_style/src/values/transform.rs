use crate::{
    Angle, CustomParseError, Length, LengthOrPercentage, Matrix, Parse, PercentageOrNumber, Scale,
    Translate,
};
use cssparser::{
    match_ignore_ascii_case, ParseError, Parser, Token, _cssparser_internal_to_lowercase,
};

/// An individual transform function.
#[derive(Debug, PartialEq, Clone)]
pub enum Transform {
    /// A 2D translation.
    Translate(Translate),
    /// A translation in the X direction.
    TranslateX(LengthOrPercentage),
    /// A translation in the Y direction.
    TranslateY(LengthOrPercentage),
    /// A 2D scale.
    Scale(Scale),
    /// A scale in the X direction.
    ScaleX(PercentageOrNumber),
    /// A scale in the Y direction.
    ScaleY(PercentageOrNumber),
    /// A 2D rotation.
    Rotate(Angle),
    /// A rotation around the X axis.
    RotateX(Angle),
    /// A rotation around the Y axis.
    RotateY(Angle),
    /// A 2D skew.
    Skew(Angle, Angle),
    /// A skew along the X axis.
    SkewX(Angle),
    /// A skew along the Y axis.
    SkewY(Angle),
    /// A perspective transform.
    Perspective(Length),
    /// A 2D matrix transform.
    Matrix(Matrix<f32>),
}

impl<'i> Parse<'i> for Transform {
    fn parse<'t>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i, CustomParseError<'i>>> {
        let function = input.expect_function()?.clone();

        input.parse_nested_block(|input| {
            let location = input.current_source_location();

            match_ignore_ascii_case! { &function,
                "translate" => {
                    let translate = Translate::parse(input)?;
                    Ok(Transform::Translate(translate))
                },
                "translatex" => {
                    let x = LengthOrPercentage::parse(input)?;
                    Ok(Transform::TranslateX(x))
                },
                "translatey" => {
                    let y = LengthOrPercentage::parse(input)?;
                    Ok(Transform::TranslateY(y))
                },
                "scale" => {
                    let scale = Scale::parse(input)?;
                    Ok(Transform::Scale(scale))
                },
                "scalex" => {
                    let x = PercentageOrNumber::parse(input)?;
                    Ok(Transform::ScaleX(x))
                },
                "scaley" => {
                    let y = PercentageOrNumber::parse(input)?;
                    Ok(Transform::ScaleY(y))
                },
                "rotate" => {
                    let angle = Angle::parse(input)?;
                    Ok(Transform::Rotate(angle))
                },
                "rotatex" => {
                    let x = Angle::parse(input)?;
                    Ok(Transform::RotateX(x))
                },
                "rotatey" => {
                    let y = Angle::parse(input)?;
                    Ok(Transform::RotateY(y))
                },
                "skew" => {
                    let x = Angle::parse(input)?;
                    input.expect_comma()?;
                    let y = Angle::parse(input)?;
                    Ok(Transform::Skew(x, y))
                },
                "skewx" => {
                    let x = Angle::parse(input)?;
                    Ok(Transform::SkewX(x))
                },
                "skewy" => {
                    let y = Angle::parse(input)?;
                    Ok(Transform::SkewY(y))
                },
                "perspective" => {
                    let length = Length::parse(input)?;
                    Ok(Transform::Perspective(length))
                },
                "matrix" => {
                    let matrix = Matrix::parse(input)?;
                    Ok(Transform::Matrix(matrix))
                },
                _ => Err(location.new_unexpected_token_error(Token::Ident(function)))
            }
        })
    }
}

impl<'i> Parse<'i> for Vec<Transform> {
    fn parse<'t>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i, CustomParseError<'i>>> {
        let mut results = vec![Transform::parse(input)?];
        loop {
            if input.is_exhausted() {
                return Ok(results);
            }

            input.skip_whitespace();
            let location = input.current_source_location();

            if let Ok(transform) = input.try_parse(Transform::parse) {
                results.push(transform);
            } else {
                return Err(cssparser::ParseError {
                    kind: cssparser::ParseErrorKind::Custom(CustomParseError::InvalidDeclaration),
                    location,
                });
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::assert_parse;
    use crate::LengthValue;

    assert_parse! {
        Transform, assert_transform,

        custom {
            success {
                "translate(10px, 50%)" => Transform::Translate(Translate::new(LengthOrPercentage::Length(Length::px(10.0)), LengthOrPercentage::Percentage(0.5))),
                "translatex(20px)" => Transform::TranslateX(LengthOrPercentage::Length(Length::px(20.0))),
                "translatey(10%)" => Transform::TranslateY(LengthOrPercentage::Percentage(0.1)),

                "scale(20, 30%)" => Transform::Scale(Scale::new(PercentageOrNumber::Number(20.0), PercentageOrNumber::Percentage(0.3))),
                "scalex(40)" => Transform::ScaleX(PercentageOrNumber::Number(40.0)),
                "scaley(50%)" => Transform::ScaleY(PercentageOrNumber::Percentage(0.5)),

                "rotate(50deg)" => Transform::Rotate(Angle::Deg(50.0)),
                "rotatex(30grad)" => Transform::RotateX(Angle::Grad(30.0)),
                "rotatey(20turn)" => Transform::RotateY(Angle::Turn(20.0)),

                "skew(60rad, 70turn)" => Transform::Skew(Angle::Rad(60.0), Angle::Turn(70.0)),
                "skewx(90grad)" => Transform::SkewX(Angle::Grad(90.0)),
                "skewy(120deg)" => Transform::SkewY(Angle::Deg(120.0)),

                "perspective(20px)" => Transform::Perspective(Length::px(20.0)),
                "matrix(1, 2, 3, 4, 5, 6)" => Transform::Matrix(Matrix::new(1.0, 2.0, 3.0, 4.0, 5.0, 6.0)),
            }

            failure {
                "somefunction(10px)",
                "scalematrix(1, 2, 3, 4, 5, 6)",
                "rotate(20)",
                "scale(30%)",
                "skewx(20px)",
                "translate(5in)",
                "abc",
            }
        }
    }

    assert_parse! {
        Vec<Transform>, assert_vec_transform,

        custom {
            success {
                "translate(10px, 20%) scale(30%, 40) rotate(50grad) skew(60turn, 70rad) perspective(80cm) matrix(10, 20, 30, 40, 50, 60)" =>
                    vec![
                        Transform::Translate(Translate::new(LengthOrPercentage::Length(Length::px(10.0)), LengthOrPercentage::Percentage(0.2))),
                        Transform::Scale(Scale::new(PercentageOrNumber::Percentage(0.3), PercentageOrNumber::Number(40.0))),
                        Transform::Rotate(Angle::Grad(50.0)),
                        Transform::Skew(Angle::Turn(60.0), Angle::Rad(70.0)),
                        Transform::Perspective(Length::Value(LengthValue::Cm(80.0))),
                        Transform::Matrix(Matrix::new(10.0, 20.0, 30.0, 40.0, 50.0, 60.0)),
                    ],
            }

            failure {
                "somefunction(10px) thing(20px) test(50%)",
                "scalematrix(1, 2, 3, 4, 5, 6)",
                "rotate(20)",
                "scale(30%)",
                "skewx(20px)",
                "translate(5in)",
                "abc",
            }
        }
    }
}
