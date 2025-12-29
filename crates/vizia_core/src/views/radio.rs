use crate::prelude::*;

/// A radio button used to display and toggle boolean state.
///
/// Clicking on the radio button with the left mouse button triggers the `on_select` callback.
///
/// # Examples
///
/// ## Basic radio button
///
/// The radio button must be bound to a boolean signal.
///
/// ```
/// # use vizia_core::prelude::*;
/// #
/// # let cx = &mut Context::default();
/// # let checked_signal = cx.state(false);
/// #
/// RadioButton::new(cx, checked_signal);
/// ```
///
/// ## Radio button with an action
///
/// A radio button can be used to trigger a callback when selected. Usually this callback
/// updates the signal the radio button is bound to.
///
/// ```
/// # use vizia_core::prelude::*;
/// #
/// # let cx = &mut Context::default();
/// # let checked_signal = cx.state(false);
/// #
/// RadioButton::new(cx, checked_signal)
///     .on_select(move |cx| {
///         checked_signal.set(cx, true);
///     });
/// ```
///
/// ## Radio button with a label
///
/// A radio button is usually used with a label next to it describing what data the radio button
/// is bound to or what the radio button does when pressed. This can be done, for example, by
/// wrapping the radio button in an [`HStack`](crate::prelude::HStack) and adding a [`Label`](crate::prelude::Label)
/// to it.
///
/// ```
/// # use vizia_core::prelude::*;
/// #
/// # let cx = &mut Context::default();
/// # let checked_signal = cx.state(false);
/// #
/// HStack::new(cx, |cx| {
///     RadioButton::new(cx, checked_signal);
///     Label::new(cx, "Press me");
/// });
/// ```
pub struct RadioButton {
    on_select: Option<Box<dyn Fn(&mut EventContext)>>,
}

impl RadioButton {
    /// Creates a new [RadioButton] view.
    pub fn new<L: Res<bool>>(cx: &mut Context, checked: L) -> Handle<Self> {
        Self { on_select: None }
            .build(cx, |cx| {
                Element::new(cx).class("inner").hoverable(false);
            })
            .checked(checked)
            .navigable(true)
            .checkable(true)
            .role(Role::RadioButton)
    }
}

impl View for RadioButton {
    fn element(&self) -> Option<&'static str> {
        Some("radiobutton")
    }

    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|window_event, meta| match window_event {
            WindowEvent::Press { mouse } => {
                let over = if *mouse { cx.mouse.left.pressed } else { cx.focused() };
                if over == cx.current() && meta.target == cx.current() && !cx.is_disabled() {
                    if let Some(callback) = &self.on_select {
                        (callback)(cx);
                    }
                }
            }

            WindowEvent::ActionRequest(request) => match request.action {
                Action::Click => {
                    if let Some(callback) = &self.on_select {
                        (callback)(cx);
                    }
                }

                _ => {}
            },

            _ => {}
        });
    }
}

impl Handle<'_, RadioButton> {
    /// Set the callback triggered when the radio button is selected.
    ///
    /// # Examples
    ///
    /// ```
    /// # use vizia_core::prelude::*;
    /// #
    /// # let cx = &mut Context::default();
    /// # let checked_signal = cx.state(false);
    /// #
    /// RadioButton::new(cx, checked_signal)
    ///     .on_select(move |cx| {
    ///         checked_signal.set(cx, true);
    ///     });
    /// ```
    pub fn on_select<F>(self, callback: F) -> Self
    where
        F: 'static + Fn(&mut EventContext),
    {
        self.modify(|radiobutton| radiobutton.on_select = Some(Box::new(callback)))
    }
}
