use crate::prelude::*;

/// A label used to display text to the screen.
///
/// # Examples
///
/// ## Basic label
///
/// A label can be used to simply display some text on the screen.
///
/// ```
/// # use vizia_core::prelude::*;
/// #
/// # let cx = &mut Context::new();
/// #
/// Label::new(cx, "Text");
/// ```
///
/// ## Label bound to data
///
/// A label can be bound to data using a lens which automatically updates the text whenever the underlying data changes.
///
/// ```
/// # use vizia_core::prelude::*;
/// # let cx = &mut Context::new();
/// #
/// #[derive(Lens)]
/// struct AppData {
///     text: String,
/// }
///
/// impl Model for AppData {}
///
/// AppData {
///     text: String::from("Text"),
/// }
/// .build(cx);
///
/// Label::new(cx, AppData::text);
/// ```
///
/// ## Label with text wrapping
///
/// A label automatically wraps the text if it doesn't fit inside of the width of the label.
///
/// ```
/// # use vizia_core::prelude::*;
/// # let mut cx = &mut Context::new();
/// #
/// Label::new(
///     cx,
///     "This is a really long text to showcase the text wrapping support of a label.",
/// )
/// .width(Pixels(100.0));
/// ```
///
/// ## Label without text wrapping
///
/// A label can also be configured to never wrap the text by using the [`text_wrap`](crate::prelude::Handle::text_wrap) method.
///
/// ```
/// # use vizia_core::prelude::*;
/// # let mut cx = &mut Context::new();
/// #
/// Label::new(
///     cx,
///     "This is a really long text to showcase disabled text wrapping of a label.",
/// )
/// .width(Pixels(100.0))
/// .text_wrap(false);
/// ```
///
/// ## Label for a button
///
/// A label can also be used inside of a button to be able to add text to it.
///
/// ```
/// # use vizia_core::prelude::*;
/// # let cx = &mut Context::new();
/// #
/// Button::new(cx, |_| {}, |cx| Label::new(cx, "Text"));
/// ```
pub struct Label;

impl Label {
    /// Creates a new label.
    ///
    /// # Examples
    ///
    /// ```
    /// # use vizia_core::prelude::*;
    /// #
    /// # let cx = &mut Context::new();
    /// #
    /// Label::new(cx, "Text");
    /// ```
    pub fn new<'a, T>(cx: &'a mut Context, text: impl Res<T>) -> Handle<'a, Self>
    where
        T: ToString,
    {
        Self {}.build(cx, |_| {}).text(text)
    }
}

impl View for Label {
    fn element(&self) -> Option<String> {
        Some(String::from("label"))
    }
}
