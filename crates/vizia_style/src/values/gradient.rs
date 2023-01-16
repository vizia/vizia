use crate::{
    Angle, Color, CustomParseError, HorizontalPositionKeyword, LengthOrPercentage, Parse,
    VerticalPositionKeyword,
};
use cssparser::*;

#[derive(Debug, Clone, PartialEq)]
pub enum Gradient {
    None,
    Linear(LinearGradient),
    // Radial(RadialGradient),
}

impl Default for Gradient {
    fn default() -> Self {
        Gradient::None
    }
}

impl From<LinearGradient> for Gradient {
    fn from(linear_gradient: LinearGradient) -> Self {
        Gradient::Linear(linear_gradient)
    }
}

impl<'i> Parse<'i> for Gradient {
    fn parse<'t>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i, CustomParseError<'i>>> {
        let location = input.current_source_location();
        let func = input.expect_function()?.clone();
        input.parse_nested_block(|input| {
            match_ignore_ascii_case! { &func,
              "linear-gradient" => Ok(Gradient::Linear(LinearGradient::parse(input)?)),
              //"radial-gradient" => Ok(Gradient::Radial(RadialGradient::parse(input)?)),
              _ => Err(location.new_unexpected_token_error(cssparser::Token::Ident(func.clone())))
            }
        })
    }
}

impl From<&str> for Gradient {
    fn from(s: &str) -> Self {
        let mut input = ParserInput::new(&s);
        let mut parser = Parser::new(&mut input);
        Gradient::parse(&mut parser).unwrap_or_default()
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LineDirection {
    Angle(Angle),
    Horizontal(HorizontalPositionKeyword),
    Vertical(VerticalPositionKeyword),
    Corner { horizontal: HorizontalPositionKeyword, vertical: VerticalPositionKeyword },
}

impl<'i> Parse<'i> for LineDirection {
    fn parse<'t>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i, CustomParseError<'i>>> {
        if let Ok(angle) = input.try_parse(Angle::parse) {
            return Ok(LineDirection::Angle(angle));
        }

        input.expect_ident_matching("to")?;

        if let Ok(x) = input.try_parse(HorizontalPositionKeyword::parse) {
            if let Ok(y) = input.try_parse(VerticalPositionKeyword::parse) {
                return Ok(LineDirection::Corner { horizontal: x, vertical: y });
            }
            return Ok(LineDirection::Horizontal(x));
        }

        let y = VerticalPositionKeyword::parse(input)?;
        if let Ok(x) = input.try_parse(HorizontalPositionKeyword::parse) {
            return Ok(LineDirection::Corner { horizontal: x, vertical: y });
        }
        Ok(LineDirection::Vertical(y))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct LinearGradient {
    pub direction: LineDirection,
    pub stops: Vec<ColorStop<LengthOrPercentage>>,
}

impl<'i> Parse<'i> for LinearGradient {
    fn parse<'t>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i, CustomParseError<'i>>> {
        let direction = if let Ok(direction) = input.try_parse(|input| LineDirection::parse(input))
        {
            input.expect_comma()?;
            direction
        } else {
            LineDirection::Vertical(VerticalPositionKeyword::Bottom)
        };
        let stops = parse_items(input)?;
        Ok(LinearGradient { direction, stops })
    }
}

fn parse_items<'i, 't, D: Parse<'i>>(
    input: &mut Parser<'i, 't>,
) -> Result<Vec<ColorStop<D>>, ParseError<'i, CustomParseError<'i>>> {
    let mut items = Vec::new();
    let mut seen_stop = false;

    loop {
        input.parse_until_before(Delimiter::Comma, |input| {
            // if seen_stop {
            //     if let Ok(hint) = input.try_parse(D::parse) {
            //         seen_stop = false;
            //         items.push(GradientItem::Hint(hint));
            //         return Ok(());
            //     }
            // }

            let stop = ColorStop::parse(input)?;

            if let Ok(position) = input.try_parse(D::parse) {
                let color = stop.color.clone();
                items.push(stop);

                items.push(ColorStop { color, position: Some(position) })
            } else {
                items.push(stop);
            }

            seen_stop = true;
            Ok(())
        })?;

        match input.next() {
            Err(_) => break,
            Ok(Token::Comma) => continue,
            _ => unreachable!(),
        }
    }

    Ok(items)
}

// pub struct RadialGradient {
//     pub position: Position,
//     pub stops: Vec<ColorStop<LengthOrPercentage>>,
// }

// impl<'i> RadialGradient {
//     fn parse<'t>(
//         input: &mut Parser<'i, 't>,
//         vendor_prefix: VendorPrefix,
//     ) -> Result<RadialGradient, ParseError<'i, ParserError<'i>>> {
//         let shape = input.try_parse(EndingShape::parse).ok();
//         let position = input
//             .try_parse(|input| {
//                 input.expect_ident_matching("at")?;
//                 Position::parse(input)
//             })
//             .ok();

//         if shape.is_some() || position.is_some() {
//             input.expect_comma()?;
//         }

//         let items = parse_items(input)?;
//         Ok(RadialGradient {
//             shape: shape.unwrap_or_default(),
//             position: position.unwrap_or(Position::center()),
//             items,
//             vendor_prefix,
//         })
//     }
// }

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ColorStop<D> {
    pub color: Color,
    pub position: Option<D>,
}

impl<'i, D: Parse<'i>> Parse<'i> for ColorStop<D> {
    fn parse<'t>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i, CustomParseError<'i>>> {
        let color = Color::parse(input)?;
        let position = input.try_parse(D::parse).ok();
        Ok(ColorStop { color, position })
    }
}
