use crate::prelude::*;

/// A Switch used to display and toggle a boolean state.
///
/// Clicking on the Switch with the left mouse button triggers the `on_toggle` callback.
///
/// # Examples
///
/// ## Basic Switch
///
/// The Switch must bound to some boolean data.
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
/// # let cx = &mut Context::default();
/// #
/// # AppData { value: false }.build(cx);
/// #
/// Switch::new(cx, AppData::value);
/// ```
///
/// ## Switch with an action
///
/// A Switch can be used to trigger a callback when toggled. Usually this is emitting an
/// event responsible for changing the data the Switch is bound to.
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
/// Switch::new(cx, AppData::value).on_toggle(|cx| cx.emit(AppEvent::ToggleValue));
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
/// # #[derive(Lens)]
/// # struct AppData {
/// #     value: bool,
/// # }
/// #
/// # impl Model for AppData {}
/// #
/// # let cx = &mut Context::default();
/// #
/// # AppData { value: false }.build(cx);
/// #
/// HStack::new(cx, |cx| {
///     Switch::new(cx, AppData::value);
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
    /// # #[derive(Lens)]
    /// # struct AppData {
    /// #     value: bool,
    /// # }
    /// #
    /// # impl Model for AppData {}
    /// #
    /// # let cx = &mut Context::default();
    /// #
    /// # AppData { value: false }.build(cx);
    /// #
    /// Switch::new(cx, AppData::value);
    /// ```
    pub fn new(cx: &mut Context, checked: impl Lens<Target = bool>) -> Handle<Self> {
        Self { on_toggle: None }
            .build(cx, |cx| {
                Element::new(cx)
                    .class("switch-handle-bg")
                    .hoverable(false)
                    .position_type(PositionType::SelfDirected);
                Element::new(cx)
                    .class("switch-handle")
                    .hoverable(false)
                    .position_type(PositionType::SelfDirected);
            })
            .checked(checked)
            .cursor(CursorIcon::Hand)
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
    /// Switch::new(cx, AppData::value).on_toggle(|cx| cx.emit(AppEvent::ToggleValue));
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
