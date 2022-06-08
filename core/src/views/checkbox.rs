use crate::fonts::unicode_names::CHECK;
use crate::prelude::*;

/// A checkbox used to display and toggle boolean state.
///
/// Clicking on the checkbox with the left mouse button triggers the `on_toggle` callback.
/// The checkbox cannot be used without being bound to some data, because it will not accept
/// a raw bool as an argument.
///
/// # Examples
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
/// # let cx = &mut Context::new();
/// #
/// # AppData { value: false }.build(cx);
/// #
/// Checkbox::new(cx, AppData::value).on_toggle(|cx| cx.emit(AppEvent::ToggleValue));
/// ```
///
/// ## Checkbox without an action
///
/// A checkbox can be used without a callback and therefore do nothing when pressed.
/// This is useful for prototyping and testing out the different styling options of
/// a checkbox without having to add a callback.
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
/// # let cx = &mut Context::new();
/// #
/// # AppData { value: false }.build(cx);
/// #
/// Checkbox::new(cx, AppData::value);
/// ```
///
/// ## Checkbox with a label
///
/// A checkbox is usually used with a label next to it describing what data the checkbox
/// is bound to or what the checkbox does when pressed. This can for example be done by
/// wrapping the checkbox in an [`HStack`](crate::prelude::HStack) and adding a [`Label`](crate::prelude::Label)
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
/// # let cx = &mut Context::new();
/// #
/// # AppData { value: false }.build(cx);
/// #
/// HStack::new(cx, |cx| {
///     Checkbox::new(cx, AppData::value);
///     Label::new(cx, "Press me");
/// });
/// ```
pub struct Checkbox {
    on_toggle: Option<Box<dyn Fn(&mut Context)>>,
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
    /// # let cx = &mut Context::new();
    /// #
    /// # AppData { value: false }.build(cx);
    /// #
    /// Checkbox::new(cx, AppData::value);
    /// ```
    pub fn new(cx: &mut Context, checked: impl Lens<Target = bool>) -> Handle<Self> {
        //let checked = checked.get_val_fallible(cx).unwrap_or(false);
        Self { on_toggle: None }.build(cx, |_| {}).bind(checked, |handle, checked| {
            if let Some(flag) = checked.get_val_fallible(handle.cx) {
                handle.text(if flag { CHECK } else { "" }).checked(flag);
            }
        })
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
    /// # let cx = &mut Context::new();
    /// #
    /// # AppData { value: false }.build(cx);
    /// #
    /// Checkbox::new(cx, AppData::value).on_toggle(|cx| cx.emit(AppEvent::ToggleValue));
    /// ```
    pub fn on_toggle<F>(self, callback: F) -> Self
    where
        F: 'static + Fn(&mut Context),
    {
        self.modify(|checkbox| checkbox.on_toggle = Some(Box::new(callback)))
    }
}

impl View for Checkbox {
    fn element(&self) -> Option<&'static str> {
        Some("checkbox")
    }

    fn event(&mut self, cx: &mut Context, event: &mut Event) {
        event.map(|window_event, meta| match window_event {
            WindowEvent::MouseDown(MouseButton::Left) => {
                if meta.target == cx.current() {
                    if let Some(callback) = &self.on_toggle {
                        (callback)(cx);
                    }
                }
            }

            _ => {}
        });
    }
}
