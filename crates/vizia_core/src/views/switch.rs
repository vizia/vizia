use crate::prelude::*;

/// A Switch used to display and toggle a boolean state.
///
/// Clicking on the Switch with the left mouse button triggers the `on_toggle` callback.
///
/// # Examples
///
/// ## Basic Switch
///
/// ```
/// # use vizia_core::prelude::*;
/// #
/// # let cx = &mut Context::default();
/// #
/// // Static (always on)
/// Switch::new(cx, true);
///
/// // Reactive with two-way binding
/// let value = cx.state(false);
/// Switch::new(cx, value).two_way();
/// ```
///
/// ## Switch with a label
///
/// ```
/// # use vizia_core::prelude::*;
/// #
/// # let cx = &mut Context::default();
/// # let value = cx.state(false);
/// #
/// HStack::new(cx, |cx| {
///     Switch::new(cx, value).two_way();
///     Label::new(cx, "Press me");
/// });
/// ```
pub struct Switch {
    value: Signal<bool>,
    on_toggle: Option<Box<dyn Fn(&mut EventContext)>>,
}

impl Switch {
    /// Creates a new Switch.
    ///
    /// Accepts either a plain boolean value or a `Signal<bool>` for reactive state.
    /// Use `.two_way()` to enable automatic state updates on toggle.
    ///
    /// # Examples
    ///
    /// ```
    /// # use vizia_core::prelude::*;
    /// #
    /// # let cx = &mut Context::default();
    /// #
    /// let checked = cx.state(false);
    /// Switch::new(cx, checked).two_way();
    /// ```
    pub fn new(cx: &mut Context, checked: impl Res<bool> + 'static) -> Handle<Self> {
        let checked = checked.into_signal(cx);
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
