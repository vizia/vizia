
use morphorm::GeometryChanged;

use crate::{Binding, Context, Element, Entity, Handle, Lens, Model, MouseButton, Units::*, View, WindowEvent, ZStack};


#[derive(Debug,Default,Lens)]
pub struct SliderData {
    pub value: f32,
}

impl Model for SliderData {
    fn event(&mut self, _: &mut Context, event: &mut crate::Event) {
        if let Some(slider_event) = event.message.downcast() {
            match slider_event {
                SliderEvent::SetValue(value) => {
                    self.value = *value;
                }
            }
        }
    }
}

#[derive(Debug,Default,Lens)]
pub struct SliderDataInternal {
    width: f32,
}

impl Model for SliderDataInternal {
    fn event(&mut self, cx: &mut Context, event: &mut crate::Event) {

        if let Some(window_event) = event.message.downcast() {
            match window_event {
                WindowEvent::GeometryChanged(geo) => {
                    if geo.contains(GeometryChanged::WIDTH_CHANGED) {
                        self.width = cx.cache.get_width(cx.current);
                    }
                }

                _=> {}
            }
        }
        
    }
}

#[derive(Debug)]
pub enum SliderEvent {
    SetValue(f32),
}

pub struct Slider {
    sliding: bool,
}

impl Slider {
    pub fn new(cx: &mut Context, init: f32) -> Handle<Self> {

        Self {
            sliding: false,
        }.build2(cx, move |cx|{
            // Create some slider data
            SliderData {
                value: init.clamp(0.0, 1.0),
            }.build(cx);

            // Create some internal slider data (not exposed to the user)
            SliderDataInternal {
                width: 0.0,
            }.build(cx);

            // Add the various slider components using bindings to the slider data
            Binding::new(cx, SliderData::value, |cx, value|{
                Binding::new(cx, SliderDataInternal::width, move |cx, width|{
                    let value = value.clone();
                    ZStack::new(cx, move |cx|{
                        // TODO - Make this configurable
                        let thumb_width = 30.0;
                        
                        let val = value.get(cx);
                        let width = width.get(cx);
                        let min = thumb_width / width;
                        let max = 1.0;
                        let dx = min + val * (max - min);
                        let px = val * (1.0 -  (thumb_width / width));

                        Element::new(cx)
                            .width(Percentage(dx * 100.0))
                            .height(Stretch(1.0))
                            .class("active");
        
                        Element::new(cx)
                            .width(Pixels(thumb_width))
                            .height(Pixels(thumb_width))
                            .left(Percentage(100.0 * px))
                            .class("thumb");
                    });
                });

            });
        })
    }
}

impl View for Slider {

    fn element(&self) -> Option<String> {
        Some("slider".to_string())
    }

    fn event(&mut self, cx: &mut Context, event: &mut crate::Event) {
        if let Some(window_event) = event.message.downcast() {
            match window_event {
                WindowEvent::MouseDown(button) if *button == MouseButton::Left => {
                    self.sliding = true;
                    cx.captured = cx.current;
                    // Todo - make this configurable
                    let thumb_width = 30.0;
                    let mut dx = (cx.mouse.left.pos_down.0 - cx.cache.get_posx(cx.current) - thumb_width/2.0) / (cx.cache.get_width(cx.current) - thumb_width);
                    dx = dx.clamp(0.0, 1.0);
                    cx.emit(SliderEvent::SetValue(dx));
                }

                WindowEvent::MouseUp(button) if *button == MouseButton::Left => {
                    self.sliding = false;
                    cx.captured = Entity::null();
                }

                WindowEvent::MouseMove(x, _) => {
                    if self.sliding {
                        // Todo - make this configurable
                        let thumb_width = 30.0;
                        let mut dx = (*x - cx.cache.get_posx(cx.current) - thumb_width/2.0) / (cx.cache.get_width(cx.current) - thumb_width);
                        dx = dx.clamp(0.0, 1.0);
                        cx.emit(SliderEvent::SetValue(dx));
                    }
                }

                _=> {}
            }
        }
    }
}