use crate::{C, Color, MouseButton, StyleBuilder, WindowEvent};

use crate::{Container, Context, Entity, Event, N, Node, Stylable};
use crate::Units::*;




pub struct Button {
    action: Option<Box<dyn Fn(&mut Context)>>,
}

impl Button {
    pub fn new<F>(f: F) -> StyleBuilder<Self, C> 
    where F: 'static + Fn(&mut Context)
    {
        StyleBuilder::new(Self {
            action: Some(Box::new(f)),
        }).width(Pixels(100.0)).height(Pixels(50.0)).background_color(Color::rgb(150,150,150))
        
    }
}

impl Container for Button {
    fn debug(&self, entity: Entity) -> String {
        format!("{} Button", entity)
    }
    
    fn on_event(&mut self, cx: &mut Context, event: &mut Event) {
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

                _=> {}
            }
        }
    }
}

impl Stylable for Button {
    type Ret = N;
}