use crate::icons::ICON_CHECK;
use crate::prelude::*;

/// A checkbox used to display and toggle a boolean state.
///
/// Clicking on the checkbox with the left mouse button triggers the `on_toggle` callback.
///
/// # Examples
///
/// ## Basic checkbox
///
/// The checkbox must bound to some boolean data.
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
/// Checkbox::new(cx, AppData::value);
/// ```
///
/// ## Checkbox with an action
///
/// A checkbox can be used to trigger a callback when toggled. Usually this is emitting an
/// event responsible for changing the data the checkbox is bound to.
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
/// Checkbox::new(cx, AppData::value).on_toggle(|cx| cx.emit(AppEvent::ToggleValue));
/// ```
///
/// ## Checkbox with a label
///
/// A checkbox is usually used with a label next to it describing what data the checkbox
/// is bound to or what the checkbox does when pressed. This can be done, for example, by
/// wrapping the checkbox in an [`HStack`](crate::prelude::HStack) and adding a [`Label`](crate::prelude::Label)
/// to it.
///
/// The Label can be used to trigger the checkbox by assigning the checkbox an id name and using it with the `describing` modifier on the label.
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
///     Checkbox::new(cx, AppData::value).id("check1");
///     Label::new(cx, "Press me").describing("check1");
/// });
/// ```
///
/// ## Custom checkbox
///
/// The `text` modifier combined with a `map` on the lens can be used to customize the icon used by the checkbox.
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
/// # use vizia_core::icons::ICON_X;
///
/// Checkbox::new(cx, AppData::value)
///     .on_toggle(|cx| cx.emit(AppEvent::ToggleValue))
///     .text(AppData::value.map(|flag| if *flag {ICON_X} else {""}));
/// ```
pub struct Checkbox {
    on_toggle: Option<Box<dyn Fn(&mut EventContext)>>,
}

impl Checkbox {
    /// Creates a new checkbox.
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
    /// Checkbox::new(cx, AppData::value);
    /// ```
    pub fn new(cx: &mut Context, checked: impl Lens<Target = bool>) -> Handle<Self> {
        Self { on_toggle: None }
            .build(cx, |_| {})
            .text(checked.map(|flag| if *flag { ICON_CHECK } else { "" }))
            .checked(checked)
            .role(Role::CheckBox)
            .default_action_verb(DefaultActionVerb::Click)
            .cursor(CursorIcon::Hand)
            .navigable(true)
    }

    pub fn intermediate(
        cx: &mut Context,
        checked: impl Lens<Target = bool>,
        intermediate: impl Lens<Target = bool>,
    ) -> Handle<Self> {
        Self { on_toggle: None }
            .build(cx, |_| {})
            .bind(checked, move |handle, c| {
                handle.bind(intermediate, move |handle, i| {
                    if c.get(handle.cx) {
                        handle.text(ICON_CHECK).toggle_class("intermediate", false);
                    } else if i.get(handle.cx) {
                        handle.text("-").toggle_class("intermediate", true);
                    } else {
                        handle.text("").toggle_class("intermediate", false);
                    }
                });
            })
            .checked(checked)
            .cursor(CursorIcon::Hand)
            .navigable(true)
    }
}

impl Handle<'_, Checkbox> {
    /// Set the callback triggered when the checkbox is pressed.
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
    /// Checkbox::new(cx, AppData::value).on_toggle(|cx| cx.emit(AppEvent::ToggleValue));
    /// ```
    pub fn on_toggle<F>(self, callback: F) -> Self
    where
        F: 'static + Fn(&mut EventContext),
    {
        self.modify(|checkbox| checkbox.on_toggle = Some(Box::new(callback)))
    }
}

impl View for Checkbox {
    fn element(&self) -> Option<&'static str> {
        Some("checkbox")
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
                Action::Default => {
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
