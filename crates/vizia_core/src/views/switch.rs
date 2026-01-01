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
///     let label = cx.state("Press me");
///     Label::new(cx, label);
/// });
/// ```
pub struct Switch {
    value: Signal<bool>,
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
    pub fn new(cx: &mut Context, checked: Signal<bool>) -> Handle<Self> {
        let false_signal = cx.state(false);
        let true_signal = cx.state(true);
        let position_absolute = cx.state(PositionType::Absolute);
        Self { value: checked, on_toggle: None }
            .build(cx, |cx| {
                Element::new(cx)
                    .class("switch-handle-bg")
                    .hoverable(false_signal)
                    .position_type(position_absolute);
                Element::new(cx)
                    .class("switch-handle")
                    .hoverable(false_signal)
                    .position_type(position_absolute);
            })
            .checked(checked)
            .navigable(true_signal)
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

    /// Enables two-way binding: toggling the switch automatically updates the bound signal.
    ///
    /// This is a convenience method equivalent to:
    /// ```ignore
    /// .on_toggle(move |cx| signal.update(cx, |v| *v = !*v))
    /// ```
    pub fn two_way(self) -> Self {
        self.modify(|switch| {
            let signal = switch.value;
            switch.on_toggle = Some(Box::new(move |cx| signal.update(cx, |v| *v = !*v)));
        })
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
