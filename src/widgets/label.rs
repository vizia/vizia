use crate::{Color, Context, Entity, N, Node, Stylable, StyleBuilder, Units};
use crate::Units::*;



pub struct Label;

impl Label {
    pub fn new(text: &str) -> StyleBuilder<Self, N> {
        StyleBuilder::new(Self {})
            .width(Pixels(100.0))
            .height(Pixels(30.0))
            .background_color(Color::blue())
            .text(text.to_owned())
    }
}

impl Node for Label {
    fn debug(&self, entity: Entity) -> String {
        format!("{} Label", entity)
    }
}

impl Stylable for Label {
    type Ret = N;
}