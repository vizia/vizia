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
    pub fn center() -> Position {
        Position { x: HorizontalPosition::Center, y: VerticalPosition::Center }
    }

    pub fn is_center(&self) -> bool {
        self.x.is_center() && self.y.is_center()
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

impl<'i> Parse<'i> for Position {
    fn parse<'t>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i, CustomParseError<'i>>> {
        if let Some(x) = input.try_parse(HorizontalPosition::parse).ok() {
            // Try parsing a vertical position next.
            if let Some(y) = input.try_parse(VerticalPosition::parse).ok() {
                return Ok(Position { x, y });
            }

            // If it didn't work, assume the first actually represents a y position,
            // and the next is an x position. e.g. `center left` rather than `left center`.
            let x =
                input.try_parse(HorizontalPosition::parse).unwrap_or(HorizontalPosition::Center);
            let y = VerticalPosition::Center;
            return Ok(Position { x, y });
        } else if let Some(y) = input.try_parse(VerticalPosition::parse).ok() {
            // Try parsing a horizontal position next.
            if let Some(x) = input.try_parse(HorizontalPosition::parse).ok() {
                return Ok(Position { x, y });
            } else {
                return Ok(Position { x: HorizontalPosition::Center, y: VerticalPosition::Center });
            }
        } else {
            // Return default or return an error?
            return Ok(Position { x: HorizontalPosition::Center, y: VerticalPosition::Center });
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
            PositionComponent::Center => true,
            PositionComponent::Length(LengthOrPercentage::Percentage(p)) => *p == 0.5,
            _ => false,
        }
    }

    pub fn to_length_or_percentage(&self) -> LengthOrPercentage {
        match self {
            PositionComponent::Center => LengthOrPercentage::Percentage(0.5),
            PositionComponent::Length(len) => len.clone(),
            PositionComponent::Side(side) => (*side).into(),
        }
    }
}

impl<'i, S: Parse<'i> + Copy + Into<LengthOrPercentage>> Parse<'i> for PositionComponent<S> {
    fn parse<'t>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i, CustomParseError<'i>>> {
        if input.try_parse(|i| i.expect_ident_matching("center")).is_ok() {
            return Ok(PositionComponent::Center);
        }

        if let Ok(lp) = input.try_parse(|input| LengthOrPercentage::parse(input)) {
            return Ok(PositionComponent::Length(lp));
        }

        let keyword = S::parse(input)?;
        Ok(PositionComponent::Side(keyword))
    }
}

impl Into<LengthOrPercentage> for HorizontalPositionKeyword {
    fn into(self) -> LengthOrPercentage {
        match self {
            HorizontalPositionKeyword::Left => LengthOrPercentage::Length(Length::px(0.0)),
            HorizontalPositionKeyword::Right => LengthOrPercentage::Percentage(1.0),
        }
    }
}

impl Into<LengthOrPercentage> for VerticalPositionKeyword {
    fn into(self) -> LengthOrPercentage {
        match self {
            VerticalPositionKeyword::Top => LengthOrPercentage::Length(Length::px(0.0)),
            VerticalPositionKeyword::Bottom => LengthOrPercentage::Percentage(1.0),
        }
    }
}

pub type HorizontalPosition = PositionComponent<HorizontalPositionKeyword>;
pub type VerticalPosition = PositionComponent<VerticalPositionKeyword>;
