use crate::{Context, Event, PropSet, View};
use crate::{Handle, MouseButton, WindowEvent};

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
/// # use vizia_core::*;
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
/// # use vizia_core::*;
/// #
/// # let cx = &mut Context::new();
/// #
/// Button::new(cx, |_| {}, |cx| Label::new(cx, "Text"));
/// ```
///
/// ## Button containing multiple views
///
/// A button can contain more than just a single view or label inside of it. This can
/// for example be done by using a [`HStack`](crate::HStack) or [`VStack`](crate::VStack).
///
/// ```
/// # use vizia_core::*;
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
    action: Option<Box<dyn Fn(&mut Context)>>,
}

impl Button {
    /// Creates a new button.
    ///
    /// # Examples
    ///
    /// ```
    /// # use vizia_core::*;
    /// #
    /// # let cx = &mut Context::new();
    /// #
    /// Button::new(cx, |_| {}, |cx| Label::new(cx, "Text"));
    /// ```
    pub fn new<A, L, Label>(cx: &mut Context, action: A, label: L) -> Handle<Self>
    where
        A: 'static + Fn(&mut Context),
        L: FnOnce(&mut Context) -> Handle<Label>,
        Label: 'static + View,
    {
        Self { action: Some(Box::new(action)) }.build(cx, move |cx| {
            (label)(cx).hoverable(false).focusable(false);
        })
    }
}

impl View for Button {
    fn element(&self) -> Option<String> {
        Some(String::from("button"))
    }

    fn event(&mut self, cx: &mut Context, event: &mut Event) {
        if let Some(window_event) = event.message.downcast() {
            match window_event {
                WindowEvent::MouseDown(button) if *button == MouseButton::Left => {
                    cx.current.set_active(cx, true);
                    cx.capture();
                    if let Some(callback) = self.action.take() {
                        (callback)(cx);

                        self.action = Some(callback);
                    }
                }

                WindowEvent::MouseUp(button) if *button == MouseButton::Left => {
                    if event.target == cx.current {
                        cx.release();
                        cx.current.set_active(cx, false);
                    }
                }

                _ => {}
            }
        }
    }
}
