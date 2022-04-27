use crate::{Context, Element, Event, Handle, Lens, MouseButton, View, WindowEvent};

use morphorm::PositionType;

pub struct RadioButton {
    on_select: Option<Box<dyn Fn(&mut Context)>>,
}

impl RadioButton {
    pub fn new(cx: &mut Context, checked: impl Lens<Target = bool>) -> Handle<Self> {
        Self { on_select: None }
            .build(cx, |cx| {
                Element::new(cx).class("inner").position_type(PositionType::SelfDirected);
            })
            .checked(checked)
    }
}

impl View for RadioButton {
    fn element(&self) -> Option<String> {
        Some(String::from("radiobutton"))
    }

    fn event(&mut self, cx: &mut Context, event: &mut Event) {
        event.map(|window_event, meta| match window_event {
            WindowEvent::MouseDown(MouseButton::Left) => {
                if meta.target == cx.current {
                    if let Some(callback) = &self.on_select {
                        (callback)(cx);
                    }
                }
            }
            _ => {}
        });
    }
}

impl Handle<'_, RadioButton> {
    pub fn on_select<F>(self, callback: F) -> Self
    where
        F: 'static + Fn(&mut Context),
    {
        self.modify(|radiobutton| radiobutton.on_select = Some(Box::new(callback)))
    }
}
