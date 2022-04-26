use crate::prelude::*;

/// A basic element with no interactivity.
///
/// An element has no children and no special rendering logic. It can be used to render a rectangle,
/// a line or a circle using view styling rules.
///
/// # Examples
///
/// ## Element without styling
///
/// An element can be used without styling which displays nothing at all.
///
/// ```
/// # use vizia_core::*;
/// #
/// # let cx = &mut Context::new();
/// #
/// Element::new(cx);
/// ```
///
/// ## Element as a rectangle
///
/// An element can be used to display a rectangle like this black one with a size of 100 by 100 pixels.
///
/// ```
/// # use vizia_core::*;
/// #
/// # let cx = &mut Context::new();
/// #
/// Element::new(cx)
///     .width(Pixels(100.0))
///     .height(Pixels(100.0))
///     .background_color(Color::black());
/// ```
///
/// ## Element as a line
///
/// An element can be used to display a line like this black one with a size of 100 by 1 pixels.
///
/// ```
/// # use vizia_core::*;
/// #
/// # let cx = &mut Context::new();
/// #
/// Element::new(cx)
///      .width(Pixels(100.0))
///      .height(Pixels(1.0))
///      .background_color(Color::black());
/// ```
///
/// ## Element as a circle
///
/// An element can be used to display a circle like this black one with a diameter of 100 pixels.
/// To create a perfect circle the width and height of the element have to be equal and the
/// border radius has to be set to half of the width or height.
///
/// ```
/// # use vizia_core::*;
/// #
/// # let cx = &mut Context::new();
/// #
/// Element::new(cx)
///     .width(Pixels(100.0))
///     .height(Pixels(100.0))
///     .border_radius(Pixels(50.0))
///     .background_color(Color::black());
/// ```
pub struct Element;

impl Element {
    /// Creates a new element.
    ///
    /// # Examples
    ///
    /// ```
    /// # use vizia_core::*;
    /// #
    /// # let cx = &mut Context::new();
    /// #
    /// Element::new(cx);
    /// ```
    pub fn new(cx: &mut Context) -> Handle<Self> {
        Self {}.build(cx, |_| {})
    }
}

impl View for Element {
    fn element(&self) -> Option<&'static str> {
        Some("element")
    }
}
