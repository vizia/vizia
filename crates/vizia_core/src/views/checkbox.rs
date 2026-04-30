use crate::icons::{ICON_CHECK, ICON_MINUS};
use crate::prelude::*;

/// A checkbox used to display and toggle a boolean state.
///
/// Pressing the checkbox triggers the [`on_toggle`](Checkbox::on_toggle) callback.
///
/// # Examples
///
/// ## Basic checkbox
///
/// The checkbox takes a boolean signal (for example `Signal<bool>`).
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
/// Checkbox::new(cx, value);
/// ```
///
/// ## Checkbox with an action
///
/// A checkbox can be used to trigger a callback when toggled. Usually this updates
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
/// Checkbox::new(cx, value)
///     .on_toggle(|cx| cx.emit(AppEvent::ToggleValue));
/// ```
///
/// ## Checkbox with a label
///
/// A checkbox is usually used with a label next to it describing what state the checkbox
/// controls or what the checkbox does when pressed. This can be done, for example, by
/// wrapping the checkbox in an [`HStack`](crate::prelude::HStack) and adding a [`Label`](crate::prelude::Label)
/// to it.
///
/// The Label can be used to trigger the checkbox by assigning the checkbox an id name and using it with the `describing` modifier on the label.
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
///     Checkbox::new(cx, value)
///         .id("check1");
///     Label::new(cx, "Press me")
///         .describing("check1");
/// });
/// ```
///
/// ## Custom checkbox
///
/// The `with_icons` constructor can be used to create a checkbox with custom icons for both checked and unchecked states.
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
/// # use vizia_core::icons::ICON_X;
///
/// Checkbox::with_icons(cx, value, Some(""), Some(ICON_X))
///     .on_toggle(|cx| cx.emit(AppEvent::ToggleValue));
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
    /// Checkbox::new(cx, value);
    /// ```
    pub fn new(cx: &mut Context, checked: impl Res<bool> + Copy + 'static) -> Handle<Self> {
        Self { on_toggle: None }
            .build(cx, move |cx| {
                checked.set_or_bind(cx, |cx, checked| {
                    if checked.get_value(cx) {
                        Svg::new(cx, ICON_CHECK);
                    }
                });
            })
            .checked(checked)
            .role(Role::CheckBox)
            .navigable(true)
    }

    /// Creates a new checkbox with custom icons for both checked and unchecked states.
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
    /// # use vizia_core::icons::ICON_X;
    ///
    /// Checkbox::with_icons(cx, value, Some(""), Some(ICON_X))
    ///     .on_toggle(|cx| cx.emit(AppEvent::ToggleValue));
    /// ```
    pub fn with_icons<T>(
        cx: &mut Context,
        checked: impl Res<bool> + Copy + 'static,
        icon_default: Option<impl Res<T> + Copy + 'static>,
        icon_checked: Option<impl Res<T> + Copy + 'static>,
    ) -> Handle<Self>
    where
        T: AsRef<[u8]> + 'static,
    {
        Self { on_toggle: None }
            .build(cx, move |cx| {
                checked.set_or_bind(cx, move |cx, checked| {
                    if checked.get_value(cx) {
                        if let Some(icon) = icon_checked {
                            Svg::new(cx, icon);
                        }
                    } else if let Some(icon) = icon_default {
                        Svg::new(cx, icon);
                    }
                });
            })
            .checked(checked)
            .role(Role::CheckBox)
            .navigable(true)
    }

    /// Creates a new checkbox in an intermediate state.
    pub fn intermediate(
        cx: &mut Context,
        checked: impl Res<bool> + Clone + 'static,
        intermediate: impl Res<bool> + Clone + 'static,
    ) -> Handle<Self> {
        let checked_state_for_icon = checked.clone().to_signal(cx);
        let intermediate_state_for_icon = intermediate.clone().to_signal(cx);
        let checked_state_for_class = checked.clone().to_signal(cx);
        let intermediate_state_for_class = intermediate.clone().to_signal(cx);

        let is_intermediate_memo = Memo::new(move |_| {
            let checked = checked_state_for_class.get();
            let intermediate = intermediate_state_for_class.get();
            !checked && intermediate
        });

        Self { on_toggle: None }
            .build(cx, move |cx| {
                let icon_memo = Memo::new(move |_| {
                    if checked_state_for_icon.get() {
                        Some(ICON_CHECK)
                    } else if intermediate_state_for_icon.get() {
                        Some(ICON_MINUS)
                    } else {
                        None
                    }
                });

                Binding::new(cx, icon_memo, move |cx| {
                    if let Some(icon) = icon_memo.get() {
                        Svg::new(cx, icon);
                    }
                });
            })
            .toggle_class("intermediate", is_intermediate_memo)
            .checked(checked)
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
    /// Checkbox::new(cx, value)
    ///     .on_toggle(|cx| cx.emit(AppEvent::ToggleValue));
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
            WindowEvent::PressDown { mouse: _ } => {
                if meta.target == cx.current {
                    cx.focus();
                }
            }

            WindowEvent::Press { mouse: _ } => {
                if meta.target == cx.current {
                    if let Some(callback) = &self.on_toggle {
                        (callback)(cx);
                    }
                }
            }

            WindowEvent::ActionRequest(action) => match action.action {
                Action::Click => {
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
