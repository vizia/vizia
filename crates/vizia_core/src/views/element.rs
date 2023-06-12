use crate::prelude::*;

/// A basic element with no interactivity.
///
/// An element has no children and no special rendering logic. It can be used to render a rectangle,
/// a line or a circle using view styling rules.
///
/// # Examples
///
/// ## Element without visible styling
///
/// An element can be used without visible styling which displays nothing at all. This is useful to
/// create an invisible spacer between other views.
///
/// ```
/// # use vizia_core::prelude::*;
/// #
/// # let cx = &mut Context::default();
/// #
/// Element::new(cx)
///     .width(Pixels(100.0))
///     .height(Pixels(100.0));
/// ```
///
/// ## Element as a rectangle
///
/// An element can be used to display a rectangle like this black one with a size of 100 by 100 pixels.
///
/// ```
/// # use vizia_core::prelude::*;
/// #
/// # let cx = &mut Context::default();
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
/// # use vizia_core::prelude::*;
/// #
/// # let cx = &mut Context::default();
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
/// border radius has to be set to fifty percent.
///
/// ```
/// # use vizia_core::prelude::*;
/// #
/// # let cx = &mut Context::default();
/// #
/// Element::new(cx)
///     .width(Pixels(100.0))
///     .height(Pixels(100.0))
///     .border_radius(Percentage(50.0))
///     .background_color(Color::black());
/// ```
///
/// ## Element as an image
///
/// An element can be used to display an image like this 100 by 100 pixels one. The image can
/// be set by using a stylesheet or by using a lens. The image has to be loaded manually by
/// using the [`Context::load_image`](crate::prelude::Context::load_image) method.
///
/// ```
/// # use vizia_core::prelude::*;
/// #
/// # #[derive(Lens)]
/// # struct AppData {
/// #     picture: String,
/// # }
/// #
/// # impl Model for AppData {}
/// #
/// # let cx = &mut Context::default();
/// #
/// # AppData { picture: String::from("test.png") }.build(cx);
/// #
/// Element::new(cx)
///     .width(Pixels(100.0))
///     .height(Pixels(100.0));
/// ```
pub struct Element;

impl Element {
    /// Creates a new element.
    ///
    /// # Examples
    ///
    /// ```
    /// # use vizia_core::prelude::*;
    /// #
    /// # let cx = &mut Context::default();
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
