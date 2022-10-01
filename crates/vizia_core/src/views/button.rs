use crate::prelude::*;

/// A simple push button with an action and views inside of it.
///
/// # Examples
///
/// ## Button with an action
///
/// A button can be used to call an action when pressed. Usually this is an
/// event that is being emitted.
///
/// ```
/// # use vizia_core::prelude::*;
/// #
/// # enum AppEvent {
/// #     Action,
/// # }
/// #
/// # let cx = &mut Context::new();
/// #
/// Button::new(cx, |cx| cx.emit(AppEvent::Action), |cx| Label::new(cx, "Text"));
/// ```
///
/// ## Button without an action
///
/// A button can be used without an action and therefore do nothing when pressed.
/// This is useful for prototyping and testing out the different styling options of
/// a button without having to add an action.
///
/// ```
/// # use vizia_core::prelude::*;
/// #
/// # let cx = &mut Context::new();
/// #
/// Button::new(cx, |_| {}, |cx| Label::new(cx, "Text"));
/// ```
///
/// ## Button containing multiple views
///
/// A button can contain more than just a single view or label inside of it. This can
/// for example be done by using a [`HStack`](crate::prelude::HStack) or [`VStack`](crate::prelude::VStack).
///
/// ```
/// # use vizia_core::prelude::*;
/// #
/// # let cx = &mut Context::new();
/// #
/// Button::new(
///     cx,
///     |_| {},
///     |cx| {
///         HStack::new(cx, |cx| {
///             Label::new(cx, "Hello");
///             Label::new(cx, "World");
///         })
///     },
/// );
/// ```
pub struct Button {
    action: Option<Box<dyn Fn(&mut EventContext)>>,
}

impl Button {
    /// Creates a new button.
    ///
    /// # Examples
    ///
    /// ```
    /// # use vizia_core::prelude::*;
    /// #
    /// # let cx = &mut Context::new();
    /// #
    /// Button::new(cx, |_| {}, |cx| Label::new(cx, "Text"));
    /// ```
    pub fn new<A, F, V>(cx: &mut Context, action: A, content: F) -> Handle<Self>
    where
        A: 'static + Fn(&mut EventContext),
        F: FnOnce(&mut Context) -> Handle<V>,
        V: 'static + View,
    {
        Self { action: Some(Box::new(action)) }
            .build(cx, move |cx| {
                (content)(cx).hoverable(false);
            })
            .keyboard_navigatable(true)
    }
}

impl View for Button {
    fn element(&self) -> Option<&'static str> {
        Some("button")
    }

    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|window_event, meta| match window_event {
            WindowEvent::PressDown { .. } => {
                cx.capture();
                cx.focus();
            }

            WindowEvent::Press { .. } => {
                if meta.target == cx.current() {
                    if let Some(callback) = &self.action {
                        (callback)(cx);
                    }
                    cx.release();
                }
            }

            _ => {}
        });
    }
}
