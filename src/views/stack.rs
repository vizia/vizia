use morphorm::LayoutType;

use crate::{Color, Context, Entity, Handle, View};
use crate::Units::*;

pub struct VStack {
    pub builder: Option<Box<dyn Fn(&mut Context)>>,
}

impl VStack {
    pub fn new<'a,F>(cx: &'a mut Context, f: F) -> Handle<Self> 
    where F: 'static + Fn(&mut Context)
    {
        Self{
            builder: Some(Box::new(f)),
        }.build(cx)
            //.width(Auto)
            //.height(Auto)
            //.background_color(Color::rgb(50, 50, 50))
    }
}

impl View for VStack {
    fn debug(&self, entity: Entity) -> String {
        format!("{} VStack", entity)
    }

    fn element(&self) -> Option<String> {
        Some("vstack".to_string())
    }

    fn body<'a>(&mut self, cx: &'a mut Context) {
        if let Some(builder) = self.builder.take() {
            (builder)(cx);

            self.builder = Some(builder);
        }
    }
}

pub struct HStack {
    pub builder: Option<Box<dyn Fn(&mut Context)>>,
}

impl HStack {
    pub fn new<F>(cx: &mut Context, f: F) -> Handle<Self> 
    where F: 'static + Fn(&mut Context)
    {
        Self{
            builder: Some(Box::new(f)),
        }.build(cx)
            .layout_type(LayoutType::Row)
            //.width(Auto)
            //.height(Auto)
            // .background_color(Color::rgb(50, 50, 50))
    }

    fn custom_prop(&self, value: f32) {
        println!("{}", value);
    }


}

impl Handle<HStack> {
    pub fn custom_prop(self, cx: &mut Context, value: f32) -> Self {
        if let Some(hstack) = cx.views.get(&self.entity).and_then(|f| f.downcast_ref::<HStack>()) {
            hstack.custom_prop(value);
        }

        self
    }
}

impl View for HStack {
    fn debug(&self, entity: Entity) -> String {
        format!("{} HStack", entity)
    }

    fn element(&self) -> Option<String> {
        Some("hstack".to_string())
    }

    fn body<'a>(&mut self, cx: &'a mut Context) {
        if let Some(builder) = self.builder.take() {
            (builder)(cx);

            self.builder = Some(builder);
        }
    }
}


pub struct ZStack {
    pub builder: Option<Box<dyn Fn(&mut Context)>>,
}

impl ZStack {
    pub fn new<F>(cx: &mut Context, f: F) -> Handle<Self> 
    where F: 'static + Fn(&mut Context)
    {
        Self{
            builder: Some(Box::new(f)),
        }.build(cx)
    }
}

impl View for ZStack {
    fn debug(&self, entity: Entity) -> String {
        format!("{} ZStack", entity)
    }

    fn element(&self) -> Option<String> {
        Some("zstack".to_string())
    }

    fn body<'a>(&mut self, cx: &'a mut Context) {
        if let Some(builder) = self.builder.take() {
            (builder)(cx);

            self.builder = Some(builder);
        }
    }
}