
use crate::{Context, Handle, View};


type Template<T> = Option<Box<dyn Fn(&mut Context, T)>>;


pub struct ForEach {
    length: usize,
    item_template: Template<usize>,
}

impl ForEach {
    pub fn new<F>(cx: &mut Context, length: usize, template: F) -> Handle<Self> 
    where F: 'static + Fn(&mut Context, usize),
    {
        Self{
            length,
            item_template: Some(Box::new(template)),
        }.build(cx)
    }
}

impl View for ForEach {
    fn body(&mut self, cx: &mut Context) {
        if let Some(template) = self.item_template.take() {
            for i in 0..self.length {
                (template)(cx, i);
            }

            self.item_template = Some(template);
        }
    }
}