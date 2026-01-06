use crate::prelude::*;

/// A label used to display text.
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
/// # let cx = &mut Context::default();
/// #
/// Label::new(cx, "Hello World");
/// ```
///
/// ## Label bound to data
///
/// A label can be bound to data using a signal which automatically updates the text whenever the underlying data changes.
///
/// ```
/// # use vizia_core::prelude::*;
/// #
/// # let cx = &mut Context::default();
/// #
/// let count = cx.state(0i32);
/// let text = cx.derived({
///     let count = count;
///     move |s| format!("Count: {}", count.get(s))
/// });
/// Label::new(cx, text);
/// ```
///
/// ## Label with text wrapping
///
/// A label automatically wraps the text if it doesn't fit inside of the width of the label.
///
/// ```
/// # use vizia_core::prelude::*;
/// #
/// # let mut cx = &mut Context::default();
/// #
/// Label::new(cx, "This is a really long text to showcase the text wrapping support of a label.")
///     .width(Pixels(100.0));
/// ```
///
/// ## Label without text wrapping
///
/// A label can also be configured to never wrap the text by using the [`text_wrap`](crate::prelude::Handle::text_wrap) method.
///
/// ```
/// # use vizia_core::prelude::*;
/// #
/// # let mut cx = &mut Context::default();
/// #
/// Label::new(cx, "This is a really long text to showcase disabled text wrapping of a label.")
///     .width(Pixels(100.0))
///     .text_wrap(false);
/// ```
///
/// ## Label for a button
///
/// A label can also be used inside of a button to be able to add text to it.
///
/// ```
/// # use vizia_core::prelude::*;
/// # let cx = &mut Context::default();
/// #
/// Button::new(cx, |cx| Label::new(cx, "Click me"));
/// ```
pub struct Label {
    describing: Option<String>,
}

impl Label {
    /// Creates a new [Label] view.
    ///
    /// Accepts either a plain value or a `Signal<T>` for reactive text.
    ///
    /// # Examples
    ///
    /// ```
    /// # use vizia_core::prelude::*;
    /// #
    /// # let cx = &mut Context::default();
    /// #
    /// // Static text
    /// Label::new(cx, "Hello World");
    ///
    /// // Reactive text
    /// let text = cx.state("Text");
    /// Label::new(cx, text);
    /// ```
    pub fn new<T>(cx: &mut Context, text: impl Res<T> + Clone + 'static) -> Handle<Self>
    where
        T: ToStringLocalized + Clone + 'static,
    {
        Self { describing: None }.build(cx, |_| {}).text(text.clone()).role(Role::Label).name(text)
    }

    /// Creates a new rich [Label] view with inline child elements.
    ///
    /// Accepts either a plain value or a `Signal<T>` for reactive text.
    pub fn rich<T>(
        cx: &mut Context,
        text: impl Res<T> + Clone + 'static,
        children: impl Fn(&mut Context),
    ) -> Handle<Self>
    where
        T: ToStringLocalized + Clone + 'static,
    {
        Self { describing: None }
            .build(cx, |cx| {
                children(cx);
            })
            .text(text.clone())
            .role(Role::Label)
            .name(text)
    }
}

impl Handle<'_, Label> {
    /// Which form element does this label describe.
    ///
    /// # Examples
    ///
    /// ```
    /// # use vizia_core::prelude::*;
    /// #
    /// # let cx = &mut Context::default();
    /// # let text = cx.state("hello");
    /// # let value = cx.state(false);
    /// Checkbox::new(cx, value)
    ///     .on_toggle(move |cx| value.update(cx, |v| *v = !*v))
    ///     .id("checkbox_identifier");
    /// Label::new(cx, text).describing("checkbox_identifier");
    /// ```
    pub fn describing(mut self, entity_identifier: impl Into<String>) -> Self {
        let identifier = entity_identifier.into();
        if let Some(id) = self.cx.resolve_entity_identifier(&identifier) {
            self.cx.style.labelled_by.insert(id, self.entity);
        }
        let hidden = self.context().state(true);
        self.modify(|label| label.describing = Some(identifier)).class("describing").hidden(hidden)
    }
}

impl View for Label {
    fn element(&self) -> Option<&'static str> {
        Some("label")
    }

    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|window_event, meta| match window_event {
            WindowEvent::Press { .. } | WindowEvent::PressDown { .. } => {
                if cx.current() == cx.mouse.left.pressed && meta.target == cx.current() {
                    if let Some(describing) = self
                        .describing
                        .as_ref()
                        .and_then(|identity| cx.resolve_entity_identifier(identity))
                    {
                        let old = cx.current;
                        cx.current = describing;
                        cx.focus_with_visibility(false);
                        let message = if matches!(window_event, WindowEvent::Press { .. }) {
                            WindowEvent::Press { mouse: false }
                        } else {
                            WindowEvent::PressDown { mouse: false }
                        };
                        cx.emit_to(describing, message);
                        cx.current = old;
                    }
                }
            }
            _ => {}
        });
    }
}

/// A view which represents a span of text within a label.
pub struct TextSpan {}

impl TextSpan {
    /// Create a new [TextSpan] view.
    ///
    /// Accepts either a plain value or a `Signal<T>` for reactive text.
    pub fn new<'a, T>(
        cx: &'a mut Context,
        text: impl Res<T> + 'static,
        children: impl Fn(&mut Context),
    ) -> Handle<'a, Self>
    where
        T: ToStringLocalized + Clone + 'static,
    {
        Self {}
            .build(cx, |cx| {
                cx.style.text_span.insert(cx.current(), true);
                cx.style.display.insert(cx.current(), Display::None);
                cx.style.pointer_events.insert(cx.current(), PointerEvents::None);
                children(cx);
            })
            .text(text)
    }
}

impl View for TextSpan {
    fn element(&self) -> Option<&'static str> {
        Some("text-span")
    }
}
