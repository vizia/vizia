use morphorm::LayoutType;

use crate::{C, Color, Container, Context, Entity, Handle, Node, Stylable, StyleBuilder};
use crate::Units::*;

pub struct VStack;

impl VStack {
    pub fn new() -> StyleBuilder<Self, C> {
        StyleBuilder::new(Self{})
            .width(Auto)
            .height(Auto)
            .background_color(Color::rgb(50, 50, 50))
    }
}

impl Container for VStack {
    fn debug(&self, entity: Entity) -> String {
        format!("{} VStack", entity)
    }
}

impl Stylable for VStack {
    type Ret = C;
}

pub struct HStack;

impl HStack {
    pub fn new() -> StyleBuilder<Self, C> {
        StyleBuilder::new(Self{})
            .layout_type(LayoutType::Row)
            .width(Auto)
            .height(Auto)
            .background_color(Color::rgb(50, 50, 50))
    }
}

impl Container for HStack {
    fn debug(&self, entity: Entity) -> String {
        format!("{} HStack", entity)
    }
}

impl Stylable for HStack {
    type Ret = C;
}

pub struct NewStack {
    pub builder: Option<Box<dyn Fn(&mut Context)>>,
}

impl NewStack {
    pub fn new<'a, F>(cx: &'a mut Context, f: F) -> Handle<'a> 
    where F: 'static + Fn(&mut Context)
    {
        let stack = Self {
            builder: Some(Box::new(f)),
        };

        let entity = Node::build(stack, cx);

        Handle { entity, cx }
    }
}

impl Node for NewStack {
    fn body(&mut self, cx: &mut Context) {
        if let Some(builder) = self.builder.take() {
            (builder)(cx);
            self.builder = Some(builder);
        }
    } 
}
