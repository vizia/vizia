use glutin::window::Window;
use keyboard_types::Code;

use crate::{Context, Handle, MouseButton, View, WindowEvent};




pub struct Textbox {
    text: String,
    edit: bool,
    on_submit: Option<Box<dyn Fn(&mut Context, &Self)>>
}

impl Textbox {
    pub fn new(cx: &mut Context, placeholder: &str) -> Handle<Self> {
        Self {
            text: placeholder.to_owned(),
            edit: false,
            on_submit: None,
        }.build(cx).text(placeholder)
    }
}

impl Handle<Textbox> {
    pub fn on_submit<F>(self, cx: &mut Context, callback: F) -> Self 
    where F: 'static + Fn(&mut Context, &Textbox)
    {
        if let Some(view) = cx.views.get_mut(&self.entity) {
            if let Some(textbox) = view.downcast_mut::<Textbox>() {
                textbox.on_submit = Some(Box::new(callback));
            }
        }

        self
    }
}

impl View for Textbox {
    fn debug(&self, entity: crate::Entity) -> String {
        format!("{} Textbox", entity)
    }

    fn event(&mut self, cx: &mut Context, event: &mut crate::Event) {
        if let Some(window_event) = event.message.downcast() {
            match window_event {
                WindowEvent::MouseDown(button) if *button == MouseButton::Left => {
                    if !self.edit {
                        self.edit = true;
                    }
                }

                WindowEvent::KeyDown(code, key) => {
                    match code {
                        Code::Enter => {
                            self.edit = false;
                        }

                        _=> {}
                    }
                }

                _=> {}
            }
        }
    }
}