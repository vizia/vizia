use crate::animation::Interpolator;
use crate::entity::Entity;

/// Display determines whether an entity will be rendered and acted on by the layout system.
/// To make an entity invisible to rendering but still visible to layout, see [Visibility].
///
/// This type is part of the prelude.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Display {
    None,
    Flex,
}

impl Default for Display {
    fn default() -> Self {
        Display::Flex
    }
}

impl From<bool> for Display {
    fn from(val: bool) -> Self {
        if val {
            Display::Flex
        } else {
            Display::None
        }
    }
}

impl Interpolator for Display {
    fn interpolate(_start: &Self, end: &Self, _t: f32) -> Self {
        return *end;
    }
}

/// Visibility determines whether an entity will be rendered.
/// An invisible entity will still be acted upon by the layout system.
/// Use [Display] to hide an entity from both rendering and layout.
///
/// This type is part of the prelude.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Visibility {
    Visible,
    Invisible,
}

impl From<bool> for Visibility {
    fn from(val: bool) -> Self {
        if val {
            Visibility::Visible
        } else {
            Visibility::Invisible
        }
    }
}

impl Default for Visibility {
    fn default() -> Self {
        Visibility::Visible
    }
}

impl Interpolator for Visibility {
    fn interpolate(_start: &Self, end: &Self, _t: f32) -> Self {
        return *end;
    }
}

/// The opacity of a view, between 0.0 and 1.0.
///
/// This type is part of the prelude.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Opacity(pub f32);

impl Default for Opacity {
    fn default() -> Self {
        Opacity(1.0)
    }
}

impl Interpolator for Opacity {
    fn interpolate(start: &Self, end: &Self, t: f32) -> Self {
        return Opacity(start.0 + (end.0 - start.0) * t);
    }
}

/// Determines whether content should be rendered outside of the bounding box of an element.
///
/// This type is part of the prelude.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Overflow {
    Visible,
    Hidden,
}

impl Default for Overflow {
    fn default() -> Self {
        Overflow::Visible
    }
}

/// Next and previous widgets which receive focus.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FocusOrder {
    pub next: Entity,
    pub prev: Entity,
}

impl Default for FocusOrder {
    fn default() -> Self {
        FocusOrder { next: Entity::null(), prev: Entity::null() }
    }
}

/// The shape the default view drawing algorithm should use for handling borders with a given
/// radius.
///
/// This type is part of the prelude.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BorderCornerShape {
    Round,
    Bevel,
}

impl Default for BorderCornerShape {
    fn default() -> Self {
        BorderCornerShape::Round
    }
}
