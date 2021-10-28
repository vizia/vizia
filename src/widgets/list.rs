use std::any::TypeId;
use std::marker::PhantomData;

use crate::{Context, Handle, Lens, Store, View};


pub struct List<L,T: 'static> 
where 
    L: Lens<Target = Vec<T>>,
{
    lens: L,
    builder: Option<Box<dyn Fn(&mut Context, usize, &T)>>,
}

impl<L: 'static + Lens<Target = Vec<T>>, T> List<L,T>
{
    pub fn new<F>(cx: &mut Context, lens: L, item: F) -> Handle<'_, Self> 
    where F: 'static + Fn(&mut Context, usize, &T),
    {
        Self {
            lens,
            builder: Some(Box::new(item)),
        }.build(cx)
    }
}

impl<L: 'static + Lens<Target = Vec<T>>, T> View for List<L,T> 
{
    fn body(&mut self, cx: &mut Context) 
    {
        if let Some(builder) = self.builder.take() {

            if let Some(model) = cx.data.remove(&TypeId::of::<L::Source>()) {
                if let Some(store) = model.downcast_ref::<Store<L::Source>>() {
                    let list_data = self.lens.view(&store.data);
                    for (index, item) in list_data.iter().enumerate() {
                        (builder)(cx, index, item);
                    }
                }

                cx.data.insert(TypeId::of::<L::Source>(), model);
            }

            // if let Some(store) = cx.data.get(&TypeId::of::<L::Source>()).and_then(|model| model.downcast_ref::<Store<L::Source>>()) {
            //     let list_data = self.lens.view(&store.data);
            //     for (index, item) in list_data.iter().enumerate() {
            //         (builder)(cx, index, item);
            //     }
            // }

            self.builder = Some(builder);
        }
    }
}