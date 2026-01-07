use crate::prelude::*;

/// A button which can be toggled between two states.
pub struct ToggleButton {
    value: Signal<bool>,
    on_toggle: Option<Box<dyn Fn(&mut EventContext)>>,
}

impl ToggleButton {
    /// Create a new [ToggleButton] view.
    ///
    /// Accepts either a plain boolean value or a `Signal<bool>` for reactive state.
    /// `content` is a closure that creates the content of the button.
    pub fn new<V: View>(
        cx: &mut Context,
        checked: impl Res<bool> + 'static,
        content: impl Fn(&mut Context) -> Handle<V> + 'static,
    ) -> Handle<Self> {
        let checked = checked.into_signal(cx);
        let false_signal = cx.state(false);
        let true_signal = cx.state(true);
        Self { value: checked, on_toggle: None }
            .build(cx, |cx| {
                (content)(cx).hoverable(false_signal);
            })
            .role(Role::Button)
            .navigable(true_signal)
            .checkable(true_signal) // To let the accesskit know button is toggleable
            .checked(checked)
    }
}

impl View for ToggleButton {
    fn element(&self) -> Option<&'static str> {
        Some("toggle-button")
    }

    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|window_event, meta| match window_event {
            WindowEvent::PressDown { mouse } => {
                if *mouse {
                    cx.capture()
                }
                cx.focus();
            }

            WindowEvent::Press { mouse } => {
                let over = if *mouse { cx.mouse().left.pressed } else { cx.focused() };
                if over == cx.current() && meta.target == cx.current() && !cx.is_disabled() {
                    if let Some(callback) = &self.on_toggle {
                        (callback)(cx);
                    }
                }
            }

            WindowEvent::MouseUp(button) if *button == MouseButton::Left => {
                cx.release();
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

impl Handle<'_, ToggleButton> {
    /// Sets the callback triggered when the [ToggleButton] is toggled.
    pub fn on_toggle(self, callback: impl Fn(&mut EventContext) + 'static) -> Self {
        self.modify(|toggle_button| toggle_button.on_toggle = Some(Box::new(callback)))
    }

    /// Enables two-way binding: toggling the button automatically updates the bound signal.
    ///
    /// This is a convenience method equivalent to:
    /// ```ignore
    /// .on_toggle(move |cx| signal.upd(cx, |v| *v = !*v))
    /// ```
    pub fn two_way(self) -> Self {
        self.modify(|toggle_button| {
            let signal = toggle_button.value;
            toggle_button.on_toggle = Some(Box::new(move |cx| signal.upd(cx, |v| *v = !*v)));
        })
    }
}
