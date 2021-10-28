use std::any::TypeId;
use std::marker::PhantomData;

use crate::{Context, Handle, Item, Lens, Store, View};
use crate::Units::*;


// pub struct List {
//     num_items: usize,
//     builder: Option<Box<dyn Fn(&mut Context, usize)>>,
// }

// impl List {
//     pub fn new<'a,F>(cx: &'a mut Context, num_items: usize, item: F) -> Handle<'a, Self> 
//     where F: 'static + Fn(&mut Context, usize),
//     {
//         Self {
//             num_items,
//             builder: Some(Box::new(item)),
//         }.build(cx)
//     }
// }

// impl View for List {
//     fn body(&mut self, cx: &mut Context) 
//     {
//         if let Some(builder) = self.builder.take() {
//             for i in 0..self.num_items {
//                 (builder)(cx, i);
//             }

//             self.builder = Some(builder);
//         }
//     }
// }



pub struct List<L,T: 'static> 
where 
    L: Lens,
    <L as Lens>::Target: Clone + IntoIterator<Item = T>,
{
    lens: L,
    builder: Option<Box<dyn Fn(&mut Context, usize, Item<L,T>)>>,
}

impl<L: 'static + Lens, T> List<L,T> 
where 
    <L as Lens>::Target: Clone + IntoIterator<Item = T>,
{
    pub fn new<'b,F>(cx: &'b mut Context, lens: L, item: F) -> Handle<'b, Self> 
    where F: 'static + Fn(&mut Context, usize, Item<L,T>),
    {
        Self {
            lens,
            builder: Some(Box::new(item)),
        }.build(cx)
    }
}

impl<L: 'static + Lens, T> View for List<L,T> 
where 
    <L as Lens>::Target: Clone + IntoIterator<Item = T>,
{
    fn body(&mut self, cx: &mut Context) 
    {
        if let Some(builder) = self.builder.take() {
            // for i in 0..self.num_items {
            //     (builder)(cx, i);
            // }

            if let Some(store) = cx.data.get(&TypeId::of::<L::Source>()).and_then(|model| model.downcast_ref::<Store<L::Source>>()) {
                let list_data = self.lens.view(&store.data);
                for (index, item) in list_data.clone().into_iter().enumerate() {
                    (builder)(cx, index, Item {
                        lens: self.lens.clone(),
                        index,
                        p: PhantomData::default(),
                    });
                }
            }

            self.builder = Some(builder);
        }
    }
}