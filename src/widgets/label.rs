use crate::{Color, Context, Entity, Handle, View, Units};
use crate::Units::*;



pub struct Label;

impl Label {
    pub fn new<'a>(cx: &'a mut Context, text: &str) -> Handle<'a, Self> {
        Self{}.build(cx)
             .width(Pixels(100.0))
             .height(Pixels(30.0))
             .background_color(Color::blue())
             .text(text)
    }
}

impl View for Label {
    fn debug(&self, entity: Entity) -> String {
        format!("{} Label", entity)
    }
}