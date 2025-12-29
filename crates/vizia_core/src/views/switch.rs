use crate::prelude::*;

/// A Switch used to display and toggle a boolean state.
///
/// Clicking on the Switch with the left mouse button triggers the `on_toggle` callback.
///
/// # Examples
///
/// ## Basic Switch
///
/// The Switch must be bound to a boolean signal.
///
/// ```
/// # use vizia_core::prelude::*;
/// #
/// # let cx = &mut Context::default();
/// # let value = cx.state(false);
/// #
/// Switch::new(cx, value);
/// ```
///
/// ## Switch with an action
///
/// A Switch can be used to trigger a callback when toggled. Usually this callback
/// updates the signal the Switch is bound to.
///
/// ```
/// # use vizia_core::prelude::*;
/// #
/// # let cx = &mut Context::default();
/// # let value = cx.state(false);
/// #
/// Switch::new(cx, value)
///     .on_toggle(move |cx| {
///         value.update(cx, |v| *v = !*v);
///     });
/// ```
///
/// ## Switch with a label
///
/// A Switch is usually used with a label next to it describing what data the Switch
/// is bound to or what the Switch does when pressed. This can be done, for example, by
/// wrapping the Switch in an [`HStack`](crate::prelude::HStack) and adding a [`Label`](crate::prelude::Label)
/// to it.
///
/// ```
/// # use vizia_core::prelude::*;
/// #
/// # let cx = &mut Context::default();
/// # let value = cx.state(false);
/// #
/// HStack::new(cx, |cx| {
///     Switch::new(cx, value);
///     Label::new(cx, "Press me");
/// });
/// ```
pub struct Switch {
    on_toggle: Option<Box<dyn Fn(&mut EventContext)>>,
}

impl Switch {
    /// Creates a new Switch.
    ///
    /// # Examples
    ///
    /// ```
    /// # use vizia_core::prelude::*;
    /// #
    /// # let cx = &mut Context::default();
    /// # let checked_signal = cx.state(false);
    /// #
    /// Switch::new(cx, checked_signal);
    /// ```
    pub fn new<L: Res<bool>>(cx: &mut Context, checked: L) -> Handle<Self> {
        Self { on_toggle: None }
            .build(cx, |cx| {
                Element::new(cx)
                    .class("switch-handle-bg")
                    .hoverable(false)
                    .position_type(PositionType::Absolute);
                Element::new(cx)
                    .class("switch-handle")
                    .hoverable(false)
                    .position_type(PositionType::Absolute);
            })
            .checked(checked)
            .navigable(true)
    }
}

impl Handle<'_, Switch> {
    /// Set the callback triggered when the Switch is pressed.
    ///
    /// # Examples
    ///
    /// ```
    /// # use vizia_core::prelude::*;
    /// #
    /// # let cx = &mut Context::default();
    /// # let checked_signal = cx.state(false);
    /// #
    /// Switch::new(cx, checked_signal)
    ///     .on_toggle(move |cx| {
    ///         checked_signal.update(cx, |value| *value = !*value);
    ///     });
    /// ```
    pub fn on_toggle<F>(self, callback: F) -> Self
    where
        F: 'static + Fn(&mut EventContext),
    {
        self.modify(|switch| switch.on_toggle = Some(Box::new(callback)))
    }
}

impl View for Switch {
    fn element(&self) -> Option<&'static str> {
        Some("switch")
    }

    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|window_event, meta| match window_event {
            WindowEvent::Press { mouse } => {
                let over = if *mouse { cx.mouse.left.pressed } else { cx.focused() };
                if over == cx.current() && meta.target == cx.current() && !cx.is_disabled() {
                    if let Some(callback) = &self.on_toggle {
                        (callback)(cx);
                    }
                }
            }

            _ => {}
        });
    }
}
