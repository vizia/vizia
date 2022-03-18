use crate::{Context, Element, Event, Handle, Lens, MouseButton, View, WindowEvent};

use morphorm::PositionType;

pub struct RadioButton {
    on_select: Option<Box<dyn Fn(&mut Context)>>,
}

impl RadioButton {
    pub fn new(cx: &mut Context, checked: impl Lens<Target = bool>) -> Handle<Self> {
        Self { on_select: None }
            .build2(cx, |cx| {
                Element::new(cx).class("inner").position_type(PositionType::SelfDirected);
            })
            .checked(checked)
    }
}

impl View for RadioButton {
    fn element(&self) -> Option<String> {
        Some("radiobutton".to_string())
    }

    fn event(&mut self, cx: &mut Context, event: &mut Event) {
        if let Some(WindowEvent::MouseDown(MouseButton::Left)) = event.message.downcast() {
            if event.target == cx.current {
                if let Some(callback) = &self.on_select {
                    (callback)(cx);
                }
            }
        }
    }
}

impl Handle<'_, RadioButton> {
    pub fn on_select<F>(self, callback: F) -> Self
    where
        F: 'static + Fn(&mut Context),
    {
        if let Some(view) = self.cx.views.get_mut(&self.entity) {
            if let Some(checkbox) = view.downcast_mut::<RadioButton>() {
                checkbox.on_select = Some(Box::new(callback));
            }
        }

        self
    }
}
