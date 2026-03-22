use crate::prelude::*;

/// A Switch used to display and toggle a boolean state.
///
/// Clicking on the Switch with the left mouse button triggers the `on_toggle` callback.
///
/// # Examples
///
/// ## Basic Switch
///
/// The Switch takes a boolean signal (for example `Signal<bool>`).
///
/// ```
/// # use vizia_core::prelude::*;
/// #
/// #
/// # struct AppData {
/// #     value: bool,
/// # }
/// #
/// # impl Model for AppData {}
/// #
/// # let cx = &mut Context::default();
/// #
/// # let value = Signal::new(false);
/// #
/// Switch::new(cx, value);
/// ```
///
/// ## Switch with an action
///
/// A Switch can be used to trigger a callback when toggled. Usually this updates
/// the underlying boolean state.
///
/// ```
/// # use vizia_core::prelude::*;
/// #
/// #
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
/// # let value = Signal::new(false);
/// #
/// Switch::new(cx, value).on_toggle(|cx| cx.emit(AppEvent::ToggleValue));
/// ```
///
/// ## Switch with a label
///
/// A Switch is usually used with a label next to it describing what state the Switch
/// controls or what the Switch does when pressed. This can be done, for example, by
/// wrapping the Switch in an [`HStack`](crate::prelude::HStack) and adding a [`Label`](crate::prelude::Label)
/// to it.
///
/// ```
/// # use vizia_core::prelude::*;
/// #
/// #
/// # struct AppData {
/// #     value: bool,
/// # }
/// #
/// # impl Model for AppData {}
/// #
/// # let cx = &mut Context::default();
/// #
/// # let value = Signal::new(false);
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
    /// #
    /// # struct AppData {
    /// #     value: bool,
    /// # }
    /// #
    /// # impl Model for AppData {}
    /// #
    /// # let cx = &mut Context::default();
    /// #
    /// # let value = Signal::new(false);
    /// #
    /// Switch::new(cx, value);
    /// ```
    pub fn new(cx: &mut Context, checked: impl Res<bool>) -> Handle<Self> {
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
            .role(Role::Switch)
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
    /// #
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
    /// # let value = Signal::new(false);
    /// #
    /// Switch::new(cx, value)
    ///     .on_toggle(|cx| cx.emit(AppEvent::ToggleValue));
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
            WindowEvent::ActionRequest(action) => match action.action {
                Action::Click if !cx.is_disabled() => {
                    if let Some(callback) = &self.on_toggle {
                        (callback)(cx);
                    }
                }
                _ => {}
            },
            _ => {}
        });
    }
}
