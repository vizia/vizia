use crate::prelude::*;

/// A button which can be toggled between two states.
pub struct ToggleButton {
    on_toggle: Option<Box<dyn Fn(&mut EventContext)>>,
}

impl ToggleButton {
    /// Create a new [ToggleButton] view.
    pub fn new<V: View>(
        cx: &mut Context,
        checked: impl Res<bool>,
        content: impl Fn(&mut Context) -> Handle<V> + 'static,
    ) -> Handle<Self> {
        Self { on_toggle: None }
            .build(cx, |cx| {
                (content)(cx).hoverable(false);
            })
            .role(Role::Button)
            .navigable(true)
            .checkable(true) // To let the accesskit know button is toggleable
            .checked(checked)
    }

    /// Create a new [ToggleButton] view with distinct content for unchecked and checked states.
    pub fn with_contents<V1: View, V2: View>(
        cx: &mut Context,
        checked: impl Res<bool> + Copy + 'static,
        content_unchecked: impl Fn(&mut Context) -> Handle<V1> + 'static,
        content_checked: impl Fn(&mut Context) -> Handle<V2> + 'static,
    ) -> Handle<Self> {
        Self { on_toggle: None }
            .build(cx, move |cx| {
                checked.set_or_bind(cx, move |cx, checked| {
                    if checked.get_value(cx) {
                        (content_checked)(cx).hoverable(false);
                    } else {
                        (content_unchecked)(cx).hoverable(false);
                    }
                });
            })
            .role(Role::Button)
            .navigable(true)
            .checkable(true) // To let the accesskit know button is toggleable
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
}
