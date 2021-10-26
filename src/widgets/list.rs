use crate::{C, Color, Container, Context, Entity, Node, Stylable, StyleBuilder};
use crate::Units::*;


pub struct List {
    num_items: usize,
    builder: Option<Box<dyn Fn(&mut Context, usize)>>,
}

impl List {
    pub fn new(num_items: usize) -> Self {
        Self {
            num_items,
            builder: None,
        }
    }

    pub fn build<F>(mut self, cx: &mut Context, f: F)
    where F: 'static + Fn(&mut Context, usize)
    {
        self.builder = Some(Box::new(f));

        Node::build(self, cx);
    }
}

impl Node for List {
    fn body(&mut self, cx: &mut Context) 
    {
        if let Some(builder) = self.builder.take() {
            for i in 0..self.num_items {
                (builder)(cx, i);
            }

            self.builder = Some(builder);
        }
    }
}