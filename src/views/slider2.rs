use crate::{Color, Context, Element, Handle, MouseButton, Units::*, View, WindowEvent, events};

pub struct Slider {

}

impl Slider {
    pub fn new(cx: &mut Context) -> Handle<Self> {
        Self{

        }.build(cx)
    }
}

impl View for Slider {
    fn body(&mut self, cx: &mut Context) {
        Element::new(cx)
            .width(Pixels(10.0))
            .height(Pixels(10.0))
            .background_color(Color::red());
    }

    fn event(&mut self, cx: &mut Context, event: &mut crate::Event) {
        if let Some(window_event) = event.message.downcast() {
            match window_event {
                WindowEvent::MouseDown(button) if *button == MouseButton::Left => {
                    if event.target == cx.current {
                        
                    }
                }

                _=> {}
            }
        }
    }
}