use crate::{Color, Context, Entity, Handle, View, Units};
use crate::Units::*;



pub struct Label;

impl Label {
    pub fn new(cx: &mut Context, text: &str) -> Handle<Self> {
        Self{}.build(cx)
             .width(Pixels(100.0))
             .height(Pixels(30.0))
             .child_space(Stretch(1.0))
             .child_left(Pixels(5.0))
             //.background_color(Color::blue())
             .text(text)
    }
}

impl View for Label {
    fn debug(&self, entity: Entity) -> String {
        format!("{} Label", entity)
    }
}