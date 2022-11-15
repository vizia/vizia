use crate::error::CustomParseError;
use crate::horizontal_position_keyword::HorizontalPositionKeyword;
use crate::traits::Parse;
use crate::vertical_position_keyword::VerticalPositionKeyword;
use crate::{LengthPercentage, Percentage};
use cssparser::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Position {
    pub x: HorizontalPosition,
    pub y: VerticalPosition,
}

impl Position {
    pub fn center() -> Position {
        Position {
            x: HorizontalPosition::Center,
            y: VerticalPosition::Center,
        }
    }

    pub fn is_center(&self) -> bool {
        self.x.is_center() && self.y.is_center()
    }

    pub fn is_zero(&self) -> bool {
        self.x.is_zero() && self.y.is_zero()
    }
}

impl Default for Position {
    fn default() -> Position {
        Position {
            x: HorizontalPosition::Length(LengthPercentage::Percentage(Percentage(0.0))),
            y: VerticalPosition::Length(LengthPercentage::Percentage(Percentage(0.0))),
        }
    }
}

impl<'i> Parse<'i> for Position {
    fn parse<'t>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i, CustomParseError<'i>>> {
        match input.try_parse(HorizontalPosition::parse) {
            Ok(HorizontalPosition::Center) => {
                // Try parsing a vertical position next.
                if let Ok(y) = input.try_parse(VerticalPosition::parse) {
                    return Ok(Position {
                        x: HorizontalPosition::Center,
                        y,
                    });
                }

                // If it didn't work, assume the first actually represents a y position,
                // and the next is an x position. e.g. `center left` rather than `left center`.
                let x = input
                    .try_parse(HorizontalPosition::parse)
                    .unwrap_or(HorizontalPosition::Center);
                let y = VerticalPosition::Center;
                return Ok(Position { x, y });
            }
            Ok(x @ HorizontalPosition::Length(_)) => {
                // If we got a length as the first component, then the second must
                // be a keyword or length (not a side offset).
                if let Ok(y_keyword) = input.try_parse(VerticalPositionKeyword::parse) {
                    let y = VerticalPosition::Side(y_keyword, None);
                    return Ok(Position { x, y });
                }
                if let Ok(y_lp) = input.try_parse(LengthPercentage::parse) {
                    let y = VerticalPosition::Length(y_lp);
                    return Ok(Position { x, y });
                }
                let y = VerticalPosition::Center;
                let _ = input.try_parse(|i| i.expect_ident_matching("center"));
                return Ok(Position { x, y });
            }
            Ok(HorizontalPosition::Side(x_keyword, lp)) => {
                // If we got a horizontal side keyword (and optional offset), expect another for the vertical side.
                // e.g. `left center` or `left 20px center`
                if input
                    .try_parse(|i| i.expect_ident_matching("center"))
                    .is_ok()
                {
                    let x = HorizontalPosition::Side(x_keyword, lp);
                    let y = VerticalPosition::Center;
                    return Ok(Position { x, y });
                }

                // e.g. `left top`, `left top 20px`, `left 20px top`, or `left 20px top 20px`
                if let Ok(y_keyword) = input.try_parse(VerticalPositionKeyword::parse) {
                    let y_lp = input.try_parse(LengthPercentage::parse).ok();
                    let x = HorizontalPosition::Side(x_keyword, lp);
                    let y = VerticalPosition::Side(y_keyword, y_lp);
                    return Ok(Position { x, y });
                }

                // If we didn't get a vertical side keyword (e.g. `left 20px`), then apply the offset to the vertical side.
                let x = HorizontalPosition::Side(x_keyword, None);
                let y = lp.map_or(VerticalPosition::Center, VerticalPosition::Length);
                return Ok(Position { x, y });
            }
            _ => {}
        }

        // If the horizontal position didn't parse, then it must be out of order. Try vertical position keyword.
        let y_keyword = VerticalPositionKeyword::parse(input)?;
        let lp_and_x_pos: Result<_, ParseError<()>> = input.try_parse(|i| {
            let y_lp = i.try_parse(LengthPercentage::parse).ok();
            if let Ok(x_keyword) = i.try_parse(HorizontalPositionKeyword::parse) {
                let x_lp = i.try_parse(LengthPercentage::parse).ok();
                let x_pos = HorizontalPosition::Side(x_keyword, x_lp);
                return Ok((y_lp, x_pos));
            }
            i.expect_ident_matching("center")?;
            let x_pos = HorizontalPosition::Center;
            Ok((y_lp, x_pos))
        });

        if let Ok((y_lp, x)) = lp_and_x_pos {
            let y = VerticalPosition::Side(y_keyword, y_lp);
            return Ok(Position { x, y });
        }

        let x = HorizontalPosition::Center;
        let y = VerticalPosition::Side(y_keyword, None);
        Ok(Position { x, y })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum PositionComponent<S> {
    /// `center`
    Center,
    /// `<length-percentage>`
    Length(LengthPercentage),
    /// `<side> <length-percentage>?`
    Side(S, Option<LengthPercentage>),
}

impl<S> PositionComponent<S> {
    fn is_center(&self) -> bool {
        match self {
            PositionComponent::Center => true,
            PositionComponent::Length(LengthPercentage::Percentage(Percentage(p))) => *p == 0.5,
            _ => false,
        }
    }

    fn is_zero(&self) -> bool {
        matches!(self, PositionComponent::Length(len) if *len == 0.0)
    }
}

impl<'i, S: Parse<'i>> Parse<'i> for PositionComponent<S> {
    fn parse<'t>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i, CustomParseError<'i>>> {
        if input
            .try_parse(|i| i.expect_ident_matching("center"))
            .is_ok()
        {
            return Ok(PositionComponent::Center);
        }

        if let Ok(lp) = input.try_parse(|input| LengthPercentage::parse(input)) {
            return Ok(PositionComponent::Length(lp));
        }

        let keyword = S::parse(input)?;
        let lp = input.try_parse(|input| LengthPercentage::parse(input)).ok();
        Ok(PositionComponent::Side(keyword, lp))
    }
}

impl Into<LengthPercentage> for HorizontalPositionKeyword {
    fn into(self) -> LengthPercentage {
        match self {
            HorizontalPositionKeyword::Left => LengthPercentage::zero(),
            HorizontalPositionKeyword::Right => LengthPercentage::Percentage(Percentage(1.0)),
        }
    }
}

impl Into<LengthPercentage> for VerticalPositionKeyword {
    fn into(self) -> LengthPercentage {
        match self {
            VerticalPositionKeyword::Top => LengthPercentage::zero(),
            VerticalPositionKeyword::Bottom => LengthPercentage::Percentage(Percentage(1.0)),
        }
    }
}

pub type HorizontalPosition = PositionComponent<HorizontalPositionKeyword>;
pub type VerticalPosition = PositionComponent<VerticalPositionKeyword>;
