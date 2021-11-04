mod lens;

pub use lens::*;

mod model;
pub use model::*;

mod store;
pub use store::*;

use crate::{Context, Handle, TreeExt, Units, View};


pub struct Binding<L> 
where L: Lens
{
    lens: L,
    builder: Option<Box<dyn Fn(&mut Context, Field<L>)>>,
}

impl<L> Binding<L> 
where 
    L: 'static + Lens,
    <L as Lens>::Source: 'static,
{
    pub fn new<F>(cx: &mut Context, lens: L, builder: F) -> Handle<Self> 
    where 
        F: 'static + Fn(&mut Context, Field<L>),
        <L as Lens>::Source: Model,
    {


        let parent = cx.current;

        

        let handle = Self {
            lens,
            builder: Some(Box::new(builder)),
        }.build(cx)
        .width(Units::Auto)
        .height(Units::Auto);

        for entity in parent.parent_iter(&cx.tree) {
            if let Some(model_list) = cx.data.model_data.get_mut(entity) {
                for model in model_list.iter_mut() {
                    if let Some(store) = model.downcast::<Store<L::Source>>() {
                        store.observers.push(handle.entity);
                    }
                }
            }
        }

        handle
        
        // Use Lens::Source TypeId to look up data
        // 
        //(builder)(cx, Field{lens});
        // if let Some(model) = cx.data.remove(&TypeId::of::<L::Source>()) {
        //     if let Some(store) = model.downcast_ref::<Store<L::Source>>() {
        //         (builder)(cx, lens.view(&store.data));
        //     }

        //     cx.data.insert(TypeId::of::<L::Source>(), model);
        // }
    }
}

impl<L: 'static + Lens> View for Binding<L> {
    fn body<'a>(&mut self, cx: &'a mut Context) {
        if let Some(builder) = self.builder.take() {
            (builder)(cx, Field{lens: self.lens.clone()});
            self.builder = Some(builder);
        }
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

        self.lens.view(cx.data().expect(&format!("Failed to get data for: {:?}", cx.current)))
        // self.lens
        //     .view(&cx.data.model_data
        //     .get(&TypeId::of::<L::Source>())
        //     .unwrap()
        //     .downcast_ref::<Store<L::Source>>()
        //     .unwrap().data)
    }
}

// #[derive(Clone, Copy)]
// pub struct Item<L,T> {
//     pub lens: L,
//     pub index: usize,
//     pub p: PhantomData<T>,
// }

// impl<L: Lens,T> Item<L,T> 
// where 
//     <L as Lens>::Target: Index<usize, Output = T>,
// {
//     pub fn get<'a>(&self, cx: &'a Context) -> &'a T 
//     where 
//         <L as Lens>::Source: 'static,
//         <L as Lens>::Target: 'static
//     {
//         &self.lens.view(&cx.data.model_data.get(&TypeId::of::<L::Source>()).unwrap().downcast_ref::<Store<L::Source>>().unwrap().data)[self.index]
//     }
// }

// impl<L: Lens> View for Binding<L> {
//     fn body<'a>(&mut self, cx: &'a mut Context) {
//         if let Some(builder) = self.builder.take() {

//             (builder)(cx, self.lens.view())
//             self.builder = Some(builder);
//         }
//     }
// } 