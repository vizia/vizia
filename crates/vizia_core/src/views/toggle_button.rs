use crate::prelude::*;

pub struct ToggleButton {
    on_toggle: Option<Box<dyn Fn(&mut EventContext)>>,
}

impl ToggleButton {
    pub fn new<V: View>(
        cx: &mut Context,
        lens: impl Lens<Target = bool>,
        content: impl Fn(&mut Context) -> Handle<V> + 'static,
    ) -> Handle<Self> {
        Self { on_toggle: None }
            .build(cx, |cx| {
                (content)(cx).hoverable(false);
            })
            .role(Role::Button)
            .navigable(true)
            .default_action_verb(DefaultActionVerb::Click)
            .checkable(true) // to let the accesskit know button is toggleable
            .checked(lens)
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

pub trait ToggleButtonModifiers {
    fn on_toggle(self, callback: impl Fn(&mut EventContext) + 'static) -> Self;
}

impl ToggleButtonModifiers for Handle<'_, ToggleButton> {
    fn on_toggle(self, callback: impl Fn(&mut EventContext) + 'static) -> Self {
        self.modify(|toggle_button| toggle_button.on_toggle = Some(Box::new(callback)))
    }
}
