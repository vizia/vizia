use crate::Entity;
use crate::Interpolator;

/// Display determines whether an entity will be rendered and acted on by the layout system.
/// To make an entity invisible to rendering but still layed out, see [Visibility].
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Display {
    None,
    Flex,
}

impl Default for Display {
    fn default() -> Self {
        Display::Flex
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
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Visibility {
    Visible,
    Invisible,
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


/// Describes the opacity of a entity.
#[derive(Copy, Clone, PartialEq, Debug)]
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

/// Next and previous widgets which receive focus.
#[derive(Debug, Clone)]
pub struct FocusOrder {
    pub next: Entity,
    pub prev: Entity,
}

impl Default for FocusOrder {
    fn default() -> Self {
        FocusOrder {
            next: Entity::null(),
            prev: Entity::null(),
        }
    }
}

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