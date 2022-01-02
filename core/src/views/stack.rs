use morphorm::LayoutType;

use crate::{Context, Entity, Handle, View};

pub struct VStack {
    //pub builder: Option<Box<dyn Fn(&mut Context)>>,
}

impl VStack {
    pub fn new<'a, F>(cx: &'a mut Context, builder: F) -> Handle<Self>
    where
        F: 'static + FnOnce(&mut Context),
    {
        Self{
            //builder: Some(Box::new(f)),
        }
        .build2(cx, |cx| {
            (builder)(cx);
        })
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
}

pub struct HStack {
    //pub builder: Option<Box<dyn Fn(&mut Context)>>,
}

impl HStack {
    pub fn new<F>(cx: &mut Context, builder: F) -> Handle<Self>
    where
        F: 'static + FnOnce(&mut Context),
    {
        Self{
            //builder: Some(Box::new(f)),
        }
        .build2(cx, |cx| {
            (builder)(cx);
        })
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
}

pub struct ZStack {
    //pub builder: Option<Box<dyn Fn(&mut Context)>>,
}

impl ZStack {
    pub fn new<F>(cx: &mut Context, builder: F) -> Handle<Self>
    where
        F: 'static + FnOnce(&mut Context),
    {
        Self{
            //builder: Some(Box::new(f)),
        }
        .build2(cx, |cx| {
            (builder)(cx);
        })
    }
}

impl View for ZStack {
    fn debug(&self, entity: Entity) -> String {
        format!("{} ZStack", entity)
    }

    fn element(&self) -> Option<String> {
        Some("zstack".to_string())
    }
}
