use crate::prelude::*;
pub struct Switch {
    on_toggle: Option<Box<dyn Fn(&mut EventContext)>>,
}

impl Switch {
    pub fn new(cx: &mut Context, checked: impl Lens<Target = bool>) -> Handle<Self> {
        Self { on_toggle: None }
            .build(cx, |cx| {
                HStack::new(cx, |cx| {
                    Element::new(cx).class("switch-handle").hoverable(false);
                })
                .class("switch-handle-container")
                .hoverable(false);
            })
            .checked(checked)
            .cursor(CursorIcon::Hand)
            .keyboard_navigatable(true)
    }
}

impl Handle<'_, Switch> {
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
            WindowEvent::TriggerUp { mouse } => {
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
