use unicode_bidi::BidiInfo;

use crate::{entity::Entity, style::Style};

/// Resolves the effective base direction for text on this entity.
///
/// `Direction::Auto` is resolved from the first strong character in the
/// element's text content using the Unicode Bidirectional Algorithm.
pub(crate) fn resolved_text_direction(style: &Style, entity: Entity) -> crate::style::Direction {
    match style.direction.get(entity).copied() {
        Some(crate::style::Direction::LeftToRight) => crate::style::Direction::LeftToRight,
        Some(crate::style::Direction::RightToLeft) => crate::style::Direction::RightToLeft,
        Some(crate::style::Direction::Auto) => {
            let text = style.text.get(entity).map(String::as_str).unwrap_or_default();
            first_strong_text_direction(text).unwrap_or(crate::style::Direction::LeftToRight)
        }
        None => crate::style::Direction::LeftToRight,
    }
}

fn first_strong_text_direction(text: &str) -> Option<crate::style::Direction> {
    if text.is_empty() {
        return None;
    }

    let bidi = BidiInfo::new(text, None);
    let paragraph = bidi.paragraphs.first()?;

    if paragraph.level.is_rtl() {
        Some(crate::style::Direction::RightToLeft)
    } else {
        Some(crate::style::Direction::LeftToRight)
    }
}