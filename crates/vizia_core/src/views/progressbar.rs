use crate::prelude::*;

pub struct ProgressBar {}

impl ProgressBar {
    pub fn new<L>(cx: &mut Context, lens: L) -> Handle<Self>
    where
        L: Lens<Target = f32>,
    {
        Self {}
            .build(cx, move |cx| {
                Binding::new(cx, lens, |cx, lens| {
                    let val = lens.get(cx).clamp(0.0, 1.0);

                    Element::new(cx)
                        .width(Units::Percentage(val * 100.0))
                        .class("progressbar-progress");
                })
            })
            .keyboard_navigatable(true)
    }
}

impl View for ProgressBar {
    fn element(&self) -> Option<&'static str> {
        Some("progressbar")
    }

    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|window_event, meta| match window_event {
            WindowEvent::TriggerDown { .. } => {
                cx.set_active(true);
                cx.capture();
                cx.focus();
            }

            WindowEvent::TriggerUp { .. } => {
                if meta.target == cx.current() {
                    cx.release();
                    cx.set_active(false);
                }
            }

            _ => {}
        });
    }
}
