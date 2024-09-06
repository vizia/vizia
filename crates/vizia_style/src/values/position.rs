use crate::error::CustomParseError;
use crate::horizontal_position_keyword::HorizontalPositionKeyword;
use crate::traits::Parse;
use crate::vertical_position_keyword::VerticalPositionKeyword;
use crate::{Length, LengthOrPercentage};
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

    pub fn center() -> Self {
        Self { x: HorizontalPosition::Center, y: VerticalPosition::Center }
    }

    pub fn is_center(&self) -> bool {
        self.x.is_center() && self.y.is_center()
    }
}

impl Default for Position {
    fn default() -> Self {
        Self {
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
        Self {
            x: HorizontalPosition::Length(value.0.into()),
            y: VerticalPosition::Length(value.1.into()),
        }
    }
}

impl<'i> Parse<'i> for Position {
    fn parse<'t>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i, CustomParseError<'i>>> {
        if let Ok(x) = input.try_parse(HorizontalPosition::parse) {
            // Try parsing a vertical position next.
            if let Ok(y) = input.try_parse(VerticalPosition::parse) {
                return Ok(Self { x, y });
            }

            // If it didn't work, assume the first actually represents a y position,
            // and the next is an x position. e.g. `center left` rather than `left center`.
            let x =
                input.try_parse(HorizontalPosition::parse).unwrap_or(HorizontalPosition::Center);
            let y: PositionComponent<VerticalPositionKeyword> = VerticalPosition::Center;
            Ok(Self { x, y })
        } else if let Ok(y) = input.try_parse(VerticalPosition::parse) {
            // Try parsing a horizontal position next.
            if let Ok(x) = input.try_parse(HorizontalPosition::parse) {
                return Ok(Self { x, y });
            } else {
                return Ok(Self { x: HorizontalPosition::Center, y: VerticalPosition::Center });
            }
        } else {
            // Return default or return an error?
            return Ok(Self { x: HorizontalPosition::Center, y: VerticalPosition::Center });
        }
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
            Self::Center => true,
            Self::Length(LengthOrPercentage::Percentage(p)) => *p == 50.0,
            _ => false,
        }
    }

    pub fn to_length_or_percentage(&self) -> LengthOrPercentage {
        match self {
            Self::Center => LengthOrPercentage::Percentage(50.0),
            Self::Length(len) => len.clone(),
            Self::Side(side) => (*side).into(),
        }
    }
}

impl<'i, S: Parse<'i> + Copy + Into<LengthOrPercentage>> Parse<'i> for PositionComponent<S> {
    fn parse<'t>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i, CustomParseError<'i>>> {
        if input.try_parse(|i| i.expect_ident_matching("center")).is_ok() {
            return Ok(Self::Center);
        }

        if let Ok(lp) = input.try_parse(|input| LengthOrPercentage::parse(input)) {
            return Ok(Self::Length(lp));
        }

        let keyword = S::parse(input)?;
        Ok(Self::Side(keyword))
    }
}

impl From<HorizontalPositionKeyword> for LengthOrPercentage {
    fn from(value: HorizontalPositionKeyword) -> Self {
        match value {
            HorizontalPositionKeyword::Left => Self::Length(Length::px(0.0)),
            HorizontalPositionKeyword::Right => Self::Percentage(100.0),
        }
    }
}

impl From<VerticalPositionKeyword> for LengthOrPercentage {
    fn from(value: VerticalPositionKeyword) -> Self {
        match value {
            VerticalPositionKeyword::Top => Self::Length(Length::px(0.0)),
            VerticalPositionKeyword::Bottom => Self::Percentage(100.0),
        }
    }
}

pub type HorizontalPosition = PositionComponent<HorizontalPositionKeyword>;
pub type VerticalPosition = PositionComponent<VerticalPositionKeyword>;

impl From<HorizontalPosition> for LengthOrPercentage {
    fn from(value: HorizontalPosition) -> Self {
        match value {
            HorizontalPosition::Center => Self::Percentage(50.0),
            HorizontalPosition::Length(val) => val,
            HorizontalPosition::Side(side) => side.into(),
        }
    }
}

impl From<VerticalPosition> for LengthOrPercentage {
    fn from(value: VerticalPosition) -> Self {
        match value {
            VerticalPosition::Center => Self::Percentage(50.0),
            VerticalPosition::Length(val) => val,
            VerticalPosition::Side(side) => side.into(),
        }
    }
}

impl From<HorizontalPositionKeyword> for HorizontalPosition {
    fn from(value: HorizontalPositionKeyword) -> Self {
        Self::Side(value)
    }
}

impl From<VerticalPositionKeyword> for VerticalPosition {
    fn from(value: VerticalPositionKeyword) -> Self {
        Self::Side(value)
    }
}
