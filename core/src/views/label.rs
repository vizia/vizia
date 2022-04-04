use crate::{Context, Handle, Res, View};

/// A label used to display text to the screen.
///
/// # Examples
///
/// ## Basic label
///
/// A label can be used to simply display some text on the screen.
///
/// ```
/// # use vizia_core::*;
/// #
/// # let cx = &mut Context::new();
/// #
/// Label::new(cx, "Text");
/// ```
///
/// ## Label for a button
///
/// A label can also be used inside of a button to be able to add text to it.
///
/// ```
/// # use vizia_core::*;
/// #
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
    /// # use vizia_core::*;
    /// #
    /// # let cx = &mut Context::new();
    /// #
    /// let label = Label::new(cx, "Text");
    /// ```
    pub fn new<'a, T>(cx: &'a mut Context, text: impl Res<T>) -> Handle<'a, Self>
    where
        T: ToString,
    {
        Self {}.build2(cx, |_| {}).text(text)
    }
}

impl View for Label {
    fn element(&self) -> Option<String> {
        Some(String::from("label"))
    }
}
