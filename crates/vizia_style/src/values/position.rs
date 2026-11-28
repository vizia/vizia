use crate::error::CustomParseError;
use crate::horizontal_position_keyword::HorizontalPositionKeyword;
use crate::traits::Parse;
use crate::values::length::length::parse_unitless_zero;
use crate::vertical_position_keyword::VerticalPositionKeyword;
use crate::{Length, LengthOrPercentage, ParserErrorExt};
use cssparser::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Position {
    pub x: HorizontalPosition,
    pub y: VerticalPosition,
}

impl Position {
    pub fn new<H: Into<HorizontalPosition>, V: Into<VerticalPosition>>(
        horizontal: H,
        vertical: V,
    ) -> Self {
        Self { x: horizontal.into(), y: vertical.into() }
    }

    pub fn center() -> Position {
        Position { x: HorizontalPosition::Center, y: VerticalPosition::Center }
    }

    pub fn is_center(&self) -> bool {
        self.x.is_center() && self.y.is_center()
    }

    pub(crate) fn parse_background<'i>(
        input: &mut Parser<'i>,
    ) -> Result<Self, ParseError<CustomParseError<'i>>> {
        enum Offset {
            Pixels(f32),
            Percentage(f32),
        }

        fn offset<'i>(input: &mut Parser<'i>) -> Result<Offset, ParseError<CustomParseError<'i>>> {
            if input.try_parse(parse_unitless_zero).is_ok() {
                return Ok(Offset::Pixels(0.0));
            }
            match LengthOrPercentage::parse(input)? {
                LengthOrPercentage::Percentage(value) => Ok(Offset::Percentage(value)),
                LengthOrPercentage::Length(length) => length
                    .to_px()
                    .map(Offset::Pixels)
                    .ok_or_else(|| input.new_error_for_next_token()),
                LengthOrPercentage::Calc { .. } => Err(input.new_error_for_next_token()),
            }
        }

        fn resolve_offset(value: Offset, from_end: bool) -> LengthOrPercentage {
            match (value, from_end) {
                (Offset::Percentage(value), true) => LengthOrPercentage::Percentage(100.0 - value),
                (Offset::Percentage(value), false) => LengthOrPercentage::Percentage(value),
                (Offset::Pixels(px), true) => {
                    LengthOrPercentage::Calc { percentage: 100.0, px: -px }
                }
                (Offset::Pixels(px), false) => LengthOrPercentage::px(px),
            }
        }

        fn horizontal<'i>(
            input: &mut Parser<'i>,
        ) -> Result<(HorizontalPosition, bool), ParseError<CustomParseError<'i>>> {
            if input.try_parse(|input| input.expect_ident_matching("center")).is_ok() {
                return Ok((HorizontalPosition::Center, false));
            }
            let side = HorizontalPositionKeyword::parse(input)?;
            if let Ok(value) = input.try_parse(offset) {
                Ok((
                    HorizontalPosition::Length(resolve_offset(
                        value,
                        matches!(side, HorizontalPositionKeyword::Right),
                    )),
                    true,
                ))
            } else {
                Ok((HorizontalPosition::Side(side), false))
            }
        }

        fn vertical<'i>(
            input: &mut Parser<'i>,
        ) -> Result<(VerticalPosition, bool), ParseError<CustomParseError<'i>>> {
            if input.try_parse(|input| input.expect_ident_matching("center")).is_ok() {
                return Ok((VerticalPosition::Center, false));
            }
            let side = VerticalPositionKeyword::parse(input)?;
            if let Ok(value) = input.try_parse(offset) {
                Ok((
                    VerticalPosition::Length(resolve_offset(
                        value,
                        matches!(side, VerticalPositionKeyword::Bottom),
                    )),
                    true,
                ))
            } else {
                Ok((VerticalPosition::Side(side), false))
            }
        }

        let edge_position = |input: &mut Parser<'i>| {
            let ((x, x_offset), (y, y_offset)) = if let Ok(values) = input.try_parse(|input| {
                Ok::<_, ParseError<CustomParseError<'i>>>((horizontal(input)?, vertical(input)?))
            }) {
                values
            } else {
                let y = vertical(input)?;
                let x = horizontal(input)?;
                (x, y)
            };
            if !x_offset && !y_offset {
                return Err(input.new_error_for_next_token());
            }
            Ok(Position { x, y })
        };
        let position = input.try_parse(edge_position).or_else(|_| Position::parse(input))?;
        if matches!(
            position.x,
            HorizontalPosition::Length(LengthOrPercentage::Length(Length::Calc(_)))
        ) || matches!(
            position.y,
            VerticalPosition::Length(LengthOrPercentage::Length(Length::Calc(_)))
        ) {
            return Err(input.new_custom_error(CustomParseError::InvalidValue));
        }
        Ok(position)
    }
}

impl Default for Position {
    fn default() -> Position {
        Position {
            x: HorizontalPosition::Length(LengthOrPercentage::Percentage(0.0)),
            y: VerticalPosition::Length(LengthOrPercentage::Percentage(0.0)),
        }
    }
}

impl<L1, L2> From<(L1, L2)> for Position
where
    L1: Into<LengthOrPercentage>,
    L2: Into<LengthOrPercentage>,
{
    fn from(value: (L1, L2)) -> Self {
        Position {
            x: HorizontalPosition::Length(value.0.into()),
            y: VerticalPosition::Length(value.1.into()),
        }
    }
}

impl<'i> Parse<'i> for Position {
    fn parse<'t>(input: &mut Parser<'i>) -> Result<Self, ParseError<CustomParseError<'i>>> {
        if let Ok(x) = input.try_parse(HorizontalPosition::parse) {
            // Try parsing a vertical position next.
            if let Ok(y) = input.try_parse(VerticalPosition::parse) {
                return Ok(Position { x, y });
            }

            // If it didn't work, assume the first actually represents a y position,
            // and the next is an x position. e.g. `center left` rather than `left center`.
            let x = input.try_parse(HorizontalPosition::parse).unwrap_or(x);
            let y: PositionComponent<VerticalPositionKeyword> = VerticalPosition::Center;
            Ok(Position { x, y })
        } else if let Ok(y) = input.try_parse(VerticalPosition::parse) {
            // Try parsing a horizontal position next.
            if let Ok(x) = input.try_parse(HorizontalPosition::parse) {
                Ok(Position { x, y })
            } else {
                Ok(Position { x: HorizontalPosition::Center, y })
            }
        } else {
            // Return default or return an error?
            Ok(Position { x: HorizontalPosition::Center, y: VerticalPosition::Center })
        }
    }
}

impl<'i> Parse<'i> for Vec<Position> {
    fn parse<'t>(input: &mut Parser<'i>) -> Result<Self, ParseError<CustomParseError<'i>>> {
        input.skip_whitespace();
        if input.is_exhausted() {
            return Err(input.new_error_for_next_token());
        }

        input.parse_comma_separated(|input| {
            if input.is_exhausted() {
                return Err(input.new_error_for_next_token());
            }
            Position::parse_background(input)
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum PositionComponent<S: Copy + Into<LengthOrPercentage>> {
    /// `center`
    Center,
    /// `<length-percentage>`
    Length(LengthOrPercentage),
    /// `<side> <length-percentage>?`
    Side(S),
}

impl<S: Copy + Into<LengthOrPercentage>> PositionComponent<S> {
    fn is_center(&self) -> bool {
        match self {
            PositionComponent::Center => true,
            PositionComponent::Length(LengthOrPercentage::Percentage(p)) => *p == 50.0,
            PositionComponent::Length(LengthOrPercentage::Calc { percentage, px }) => {
                *percentage == 50.0 && *px == 0.0
            }
            _ => false,
        }
    }

    pub fn to_length_or_percentage(&self) -> LengthOrPercentage {
        match self {
            PositionComponent::Center => LengthOrPercentage::Percentage(50.0),
            PositionComponent::Length(len) => len.clone(),
            PositionComponent::Side(side) => (*side).into(),
        }
    }
}

impl<'i, S: Parse<'i> + Copy + Into<LengthOrPercentage>> Parse<'i> for PositionComponent<S> {
    fn parse<'t>(input: &mut Parser<'i>) -> Result<Self, ParseError<CustomParseError<'i>>> {
        if input.try_parse(|i| i.expect_ident_matching("center")).is_ok() {
            return Ok(PositionComponent::Center);
        }

        if let Ok(zero) = input.try_parse(parse_unitless_zero) {
            return Ok(PositionComponent::Length(LengthOrPercentage::Length(zero)));
        }

        if let Ok(lp) = input.try_parse(LengthOrPercentage::parse) {
            return Ok(PositionComponent::Length(lp));
        }

        let keyword = S::parse(input)?;
        Ok(PositionComponent::Side(keyword))
    }
}

impl From<HorizontalPositionKeyword> for LengthOrPercentage {
    fn from(value: HorizontalPositionKeyword) -> Self {
        match value {
            HorizontalPositionKeyword::Left => LengthOrPercentage::Length(Length::px(0.0)),
            HorizontalPositionKeyword::Right => LengthOrPercentage::Percentage(100.0),
        }
    }
}

impl From<VerticalPositionKeyword> for LengthOrPercentage {
    fn from(value: VerticalPositionKeyword) -> Self {
        match value {
            VerticalPositionKeyword::Top => LengthOrPercentage::Length(Length::px(0.0)),
            VerticalPositionKeyword::Bottom => LengthOrPercentage::Percentage(100.0),
        }
    }
}

pub type HorizontalPosition = PositionComponent<HorizontalPositionKeyword>;
pub type VerticalPosition = PositionComponent<VerticalPositionKeyword>;

impl From<HorizontalPosition> for LengthOrPercentage {
    fn from(value: HorizontalPosition) -> Self {
        match value {
            HorizontalPosition::Center => LengthOrPercentage::Percentage(50.0),
            HorizontalPosition::Length(val) => val,
            HorizontalPosition::Side(side) => side.into(),
        }
    }
}

impl From<VerticalPosition> for LengthOrPercentage {
    fn from(value: VerticalPosition) -> Self {
        match value {
            VerticalPosition::Center => LengthOrPercentage::Percentage(50.0),
            VerticalPosition::Length(val) => val,
            VerticalPosition::Side(side) => side.into(),
        }
    }
}

impl From<HorizontalPositionKeyword> for HorizontalPosition {
    fn from(value: HorizontalPositionKeyword) -> Self {
        HorizontalPosition::Side(value)
    }
}

impl From<VerticalPositionKeyword> for VerticalPosition {
    fn from(value: VerticalPositionKeyword) -> Self {
        VerticalPosition::Side(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unitless_zero_positions_parse_per_axis() {
        let zero = LengthOrPercentage::px(0.0);
        let center = LengthOrPercentage::Percentage(50.0);
        for (value, x, y) in [
            ("0", zero.clone(), center),
            ("0 0", zero.clone(), zero.clone()),
            ("-0 25%", zero.clone(), LengthOrPercentage::Percentage(25.0)),
            ("right 0", LengthOrPercentage::Percentage(100.0), zero.clone()),
            ("0 bottom", zero.clone(), LengthOrPercentage::Percentage(100.0)),
        ] {
            let mut parser = Parser::new(value);
            let positions = Vec::<Position>::parse(&mut parser).unwrap();
            assert_eq!(positions.len(), 1);
            assert_eq!(positions[0].x.to_length_or_percentage(), x, "{value}");
            assert_eq!(positions[0].y.to_length_or_percentage(), y, "{value}");
            assert!(parser.is_exhausted(), "{value}");
        }
        let mut parser = Parser::new("0 0, 0 10px");
        let positions = Vec::<Position>::parse(&mut parser).unwrap();
        assert_eq!(positions.len(), 2);
        assert_eq!(positions[0].y.to_length_or_percentage(), zero);
        assert_eq!(positions[1].y.to_length_or_percentage(), LengthOrPercentage::px(10.0));

        for value in ["1", "0 1", "0 0 0", "0,"] {
            assert!(Vec::<Position>::parse(&mut Parser::new(value)).is_err(), "{value}");
        }
    }

    #[test]
    fn single_value_positions_keep_the_specified_axis() {
        let center = LengthOrPercentage::Percentage(50.0);
        for (value, x, y) in [
            ("left", LengthOrPercentage::px(0.0), center.clone()),
            ("right", LengthOrPercentage::Percentage(100.0), center.clone()),
            ("top", center.clone(), LengthOrPercentage::px(0.0)),
            ("bottom", center.clone(), LengthOrPercentage::Percentage(100.0)),
            ("center", center.clone(), center.clone()),
            ("10px", LengthOrPercentage::px(10.0), center.clone()),
            ("25%", LengthOrPercentage::Percentage(25.0), center.clone()),
        ] {
            let mut parser = Parser::new(value);
            let position = Position::parse(&mut parser).unwrap();
            assert_eq!(position.x.to_length_or_percentage(), x, "{value}");
            assert_eq!(position.y.to_length_or_percentage(), y, "{value}");
            assert!(parser.is_exhausted(), "{value}");
        }
    }

    #[test]
    fn two_value_positions_and_layers_remain_supported() {
        let mut parser = Parser::new("left top, top right, center left, 10px 20%");
        let positions = Vec::<Position>::parse(&mut parser).unwrap();
        let expected = [
            (LengthOrPercentage::px(0.0), LengthOrPercentage::px(0.0)),
            (LengthOrPercentage::Percentage(100.0), LengthOrPercentage::px(0.0)),
            (LengthOrPercentage::px(0.0), LengthOrPercentage::Percentage(50.0)),
            (LengthOrPercentage::px(10.0), LengthOrPercentage::Percentage(20.0)),
        ];
        assert_eq!(positions.len(), expected.len());
        for (position, (x, y)) in positions.iter().zip(expected) {
            assert_eq!(position.x.to_length_or_percentage(), x);
            assert_eq!(position.y.to_length_or_percentage(), y);
        }
        assert!(parser.is_exhausted());
    }

    #[test]
    fn background_positions_reject_unresolved_calculations() {
        for value in [
            "calc(1px + 1em) 0",
            "0 calc(1px + 1em)",
            "left top, calc(1px + 1em) 0",
            "right calc(1px + 1em) bottom",
        ] {
            assert!(Vec::<Position>::parse(&mut Parser::new(value)).is_err(), "{value}");
        }
        for value in ["calc(1px + 2px) 0", "right 5px bottom 2px"] {
            assert!(Vec::<Position>::parse(&mut Parser::new(value)).is_ok(), "{value}");
        }
    }
}
