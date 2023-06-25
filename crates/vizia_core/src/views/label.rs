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
/// Label::new(cx, "Text");
/// ```
///
/// ## Label bound to data
///
/// A label can be bound to data using a lens which automatically updates the text whenever the underlying data changes.
///
/// ```
/// # use vizia_core::prelude::*;
/// #
/// # let cx = &mut Context::default();
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
/// #
/// # let mut cx = &mut Context::default();
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
/// #
/// # let mut cx = &mut Context::default();
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
/// # let cx = &mut Context::default();
/// #
/// Button::new(cx, |_| {}, |cx| Label::new(cx, "Text"));
/// ```
pub struct Label {
    describing: Option<String>,
}

impl Label {
    /// Creates a new label.
    ///
    /// # Examples
    ///
    /// ```
    /// # use vizia_core::prelude::*;
    /// #
    /// # let cx = &mut Context::default();
    /// #
    /// Label::new(cx, "Text");
    /// ```
    pub fn new<T>(cx: &mut Context, text: impl Res<T> + Clone) -> Handle<Self>
    where
        T: ToStringLocalized,
    {
        Self { describing: None }
            .build(cx, |_| {})
            .text(text.clone())
            .role(Role::StaticText)
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
    /// # #[derive(Lens)]
    /// # struct AppData {
    /// #     value: bool,
    /// # }
    /// #
    /// # impl Model for AppData {}
    /// #
    /// # enum AppEvent {
    /// #     ToggleValue,
    /// # }
    /// #
    /// # let cx = &mut Context::default();
    /// #
    /// # AppData { value: false }.build(cx);
    /// #
    /// Checkbox::new(cx, AppData::value).on_toggle(|cx| cx.emit(AppEvent::ToggleValue)).id("checkbox_identifier");
    /// Label::new(cx, "hello").describing("checkbox_identifier");
    /// ```
    pub fn describing(self, entity_identifier: impl Into<String>) -> Self {
        let identifier = entity_identifier.into();
        if let Some(id) = self.cx.resolve_entity_identifier(&identifier) {
            self.cx.style.labelled_by.insert(id, self.entity);
        }
        self.modify(|label| label.describing = Some(identifier)).class("describing")
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

pub struct Icon {}

impl Icon {
    /// Creates a new label.
    ///
    /// # Examples
    ///
    /// ```
    /// # use vizia_core::prelude::*;
    /// #
    /// # let cx = &mut Context::default();
    /// #
    /// Label::new(cx, "Text");
    /// ```
    pub fn new<T>(cx: &mut Context, icon_code: impl Res<T> + Clone) -> Handle<Self>
    where
        T: ToStringLocalized,
    {
        Self {}.build(cx, |_| {}).text(icon_code).role(Role::StaticText)
    }
}

impl View for Icon {
    fn element(&self) -> Option<&'static str> {
        Some("icon")
    }
}
