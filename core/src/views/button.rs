use crate::{Handle, MouseButton, WindowEvent};

use crate::{Context, Entity, Event, View};

pub struct Button {
    action: Option<Box<dyn Fn(&mut Context)>>,
}

impl Button {
    pub fn new<A, L, Label>(cx: &mut Context, action: A, label: L) -> Handle<Self>
    where
        A: 'static + Fn(&mut Context),
        L: 'static + Fn(&mut Context) -> Handle<Label>,
        Label: 'static + View,
    {
        Self { action: Some(Box::new(action)) }.build2(cx, move |cx| {
            (label)(cx);
        })
    }
}

impl View for Button {
    fn debug(&self, entity: Entity) -> String {
        format!("{} Button", entity)
    }

    fn element(&self) -> Option<String> {
        Some("button".to_string())
    }

    fn event(&mut self, cx: &mut Context, event: &mut Event) {
        if let Some(window_event) = event.message.downcast() {
            match window_event {
                WindowEvent::MouseDown(button) if *button == MouseButton::Left => {
                    //println!("Mouse Down");
                    if let Some(callback) = self.action.take() {
                        (callback)(cx);

                        self.action = Some(callback);
                    }
                }

                WindowEvent::MouseUp(button) if *button == MouseButton::Left => {
                    // if let Some(callback) = self.action.take() {
                    //     (callback)(cx);
                    // }
                }

                _ => {}
            }
        }
    }
}
