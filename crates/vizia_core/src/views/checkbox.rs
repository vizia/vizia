use crate::icons::ICON_CHECK;
use crate::prelude::*;

/// A checkbox used to display and toggle a boolean state.
///
/// Pressing the checkbox triggers the [`on_toggle`](Checkbox::on_toggle) callback.
///
/// # Examples
///
/// ## Basic checkbox
///
/// The checkbox must be bound to a boolean signal.
///
/// ```
/// # use vizia_core::prelude::*;
/// #
/// # let cx = &mut Context::default();
/// # let checked_signal = cx.state(false);
/// #
/// Checkbox::new(cx, checked_signal);
/// ```
///
/// ## Checkbox with an action
///
/// A checkbox can be used to trigger a callback when toggled. Usually this callback
/// updates the signal the checkbox is bound to.
///
/// ```
/// # use vizia_core::prelude::*;
/// #
/// # let cx = &mut Context::default();
/// # let checked_signal = cx.state(false);
/// #
/// Checkbox::new(cx, checked_signal)
///     .on_toggle(move |cx| {
///         checked_signal.upd(cx, |value| *value = !*value);
///     });
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
/// # let cx = &mut Context::default();
/// # let checked_signal = cx.state(false);
/// #
/// HStack::new(cx, |cx| {
///     Checkbox::new(cx, checked_signal)
///         .id("check1");
///     let label = cx.state("Press me");
///     Label::new(cx, label)
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
/// # let cx = &mut Context::default();
/// # let checked_signal = cx.state(false);
/// # use vizia_core::icons::ICON_X;
/// #
/// Checkbox::with_icons(cx, checked_signal, None, Some(ICON_X))
///     .on_toggle(move |cx| {
///         checked_signal.upd(cx, |value| *value = !*value);
///     });
/// ```
pub struct Checkbox {
    value: Signal<bool>,
    on_toggle: Option<Box<dyn Fn(&mut EventContext)>>,
}

impl Checkbox {
    /// Creates a new checkbox.
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
    /// // Static (always checked)
    /// Checkbox::new(cx, true);
    ///
    /// // Reactive with two-way binding
    /// let checked = cx.state(false);
    /// Checkbox::new(cx, checked).two_way();
    /// ```
    pub fn new(cx: &mut Context, checked: impl Res<bool> + 'static) -> Handle<Self> {
        let checked = checked.into_signal(cx);
        let icon_check = cx.state(ICON_CHECK);
        let navigable = cx.state(true);
        Self { value: checked, on_toggle: None }
            .build(cx, move |cx| {
                Binding::new(cx, checked, move |cx| {
                    if *checked.get(cx) {
                        Svg::new(cx, icon_check);
                    }
                })
            })
            .checked(checked)
            .role(Role::CheckBox)
            .navigable(navigable)
    }

    /// Creates a new checkbox with custom icons for both checked and unchecked states.
    ///
    /// # Examples
    ///
    /// ```
    /// # use vizia_core::prelude::*;
    /// #
    /// # let cx = &mut Context::default();
    /// # use vizia_core::icons::ICON_X;
    /// #
    /// let checked = cx.state(false);
    /// Checkbox::with_icons(cx, checked, None, Some(ICON_X));
    /// ```
    pub fn with_icons<T>(
        cx: &mut Context,
        checked: impl Res<bool> + 'static,
        icon_default: Option<Signal<T>>,
        icon_checked: Option<Signal<T>>,
    ) -> Handle<Self>
    where
        T: AsRef<[u8]> + Clone + 'static,
    {
        let checked = checked.into_signal(cx);
        let navigable = cx.state(true);
        Self { value: checked, on_toggle: None }
            .build(cx, move |cx| {
                Binding::new(cx, checked, move |cx| {
                    if *checked.get(cx) {
                        if let Some(icon) = icon_checked {
                            Svg::new(cx, icon);
                        }
                    } else if let Some(icon) = icon_default {
                        Svg::new(cx, icon);
                    }
                })
            })
            .checked(checked)
            .role(Role::CheckBox)
            .navigable(navigable)
    }

    /// Creates a new checkbox in an intermediate state.
    pub fn intermediate(
        cx: &mut Context,
        checked: Signal<bool>,
        intermediate: Signal<bool>,
    ) -> Handle<Self> {
        let dash = cx.state("-");
        let icon_check = cx.state(ICON_CHECK);
        let navigable = cx.state(true);
        let is_intermediate = cx.derived({
            let checked = checked;
            let intermediate = intermediate;
            move |store| !*checked.get(store) && *intermediate.get(store)
        });
        Self { value: checked, on_toggle: None }
            .build(cx, |cx| {
                Binding::new(cx, checked, move |cx| {
                    Binding::new(cx, intermediate, move |cx| {
                        if *checked.get(cx) {
                            Svg::new(cx, icon_check);
                        } else if *intermediate.get(cx) {
                            Label::new(cx, dash);
                        }
                    })
                })
            })
            .checked(checked)
            .toggle_class("intermediate", is_intermediate)
            .role(Role::CheckBox)
            .navigable(navigable)
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
    /// # let cx = &mut Context::default();
    /// # let checked_signal = cx.state(false);
    /// #
    /// Checkbox::new(cx, checked_signal)
    ///     .on_toggle(move |cx| {
    ///         checked_signal.upd(cx, |value| *value = !*value);
    ///     });
    /// ```
    pub fn on_toggle<F>(self, callback: F) -> Self
    where
        F: 'static + Fn(&mut EventContext),
    {
        self.modify(|checkbox| checkbox.on_toggle = Some(Box::new(callback)))
    }

    /// Enables two-way binding: toggling the checkbox automatically updates the bound signal.
    ///
    /// This is a convenience method equivalent to:
    /// ```ignore
    /// .on_toggle(move |cx| signal.upd(cx, |v| *v = !*v))
    /// ```
    pub fn two_way(self) -> Self {
        self.modify(|checkbox| {
            let signal = checkbox.value;
            checkbox.on_toggle = Some(Box::new(move |cx| signal.upd(cx, |v| *v = !*v)));
        })
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
