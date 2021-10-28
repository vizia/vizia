mod lens;
use std::{any::TypeId, marker::PhantomData, ops::Index};

pub use lens::*;

mod model;
pub use model::*;

mod store;
pub use store::*;

use crate::{Context, View};


pub struct Binding<L> 
where L: Lens
{
    lens: L,
    builder: Option<Box<dyn Fn(&mut Context, Field<L>)>>,
}

impl<L> Binding<L> 
where 
    L: Lens,
    <L as Lens>::Source: 'static,
{
    pub fn new<F>(cx: &mut Context, lens: L, builder: F) 
    where 
        F: 'static + Fn(&mut Context, Field<L>),
    {
        // Use Lens::Source TypeId to look up data
        // 
        (builder)(cx, Field{lens});
        // if let Some(model) = cx.data.remove(&TypeId::of::<L::Source>()) {
        //     if let Some(store) = model.downcast_ref::<Store<L::Source>>() {
        //         (builder)(cx, lens.view(&store.data));
        //     }

        //     cx.data.insert(TypeId::of::<L::Source>(), model);
        // }
    }
}

#[derive(Clone, Copy)]
pub struct Field<L> {
    lens: L,
}

impl<L: Lens> Field<L> 
where <L as Lens>::Source: 'static,
{
    pub fn get<'a>(&self, cx: &'a Context) -> &'a L::Target {
        self.lens.view(&cx.data.get(&TypeId::of::<L::Source>()).unwrap().downcast_ref::<Store<L::Source>>().unwrap().data)
    }
}

#[derive(Clone, Copy)]
pub struct Item<L,T> {
    pub lens: L,
    pub index: usize,
    pub p: PhantomData<T>,
}

impl<L: Lens,T> Item<L,T> 
where 
    <L as Lens>::Target: Index<usize, Output = T>,
{
    pub fn get<'a>(&self, cx: &'a Context) -> &'a T 
    where 
        <L as Lens>::Source: 'static,
        <L as Lens>::Target: 'static
    {
        &self.lens.view(&cx.data.get(&TypeId::of::<L::Source>()).unwrap().downcast_ref::<Store<L::Source>>().unwrap().data)[self.index]
    }
}

// impl<L: Lens> View for Binding<L> {
//     fn body<'a>(&mut self, cx: &'a mut Context) {
//         if let Some(builder) = self.builder.take() {

//             (builder)(cx, self.lens.view())
//             self.builder = Some(builder);
//         }
//     }
// } 