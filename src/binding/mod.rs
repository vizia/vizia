mod lens;

use std::collections::HashSet;

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

        let binding = Self {
            lens,
            builder: Some(Box::new(builder)),
        };

        let id = if let Some(id) = cx.tree.get_child(cx.current, cx.count) {
            let prev = cx.current;
            cx.current = id;
            let prev_count = cx.count;
            cx.count = 0;
            //binding.body(cx);
            cx.current = prev;
            cx.count = prev_count;
            id
        } else {
            let id = cx.entity_manager.create();
            cx.tree.add(id, cx.current).expect("Failed to add to tree");
            cx.cache.add(id).expect("Failed to add to cache");
            cx.style.borrow_mut().add(id);
            let prev = cx.current;
            cx.current = id;
            let prev_count = cx.count;
            cx.count = 0;
            //binding.body(cx);
            cx.current = prev;
            cx.count = prev_count;
            cx.views.insert(id, Box::new(binding));
            id  
        };

        cx.count += 1;
        
        let mut ancestors = HashSet::new();
        for entity in parent.parent_iter(&cx.tree) {
            ancestors.insert(entity);

            if let Some(model_list) = cx.data.model_data.get_mut(entity) {
                for (_, model) in model_list.iter_mut() {
                    if let Some(store) = model.downcast::<Store<L::Source>>() {
                        if store.observers.intersection(&ancestors).next().is_some() {
                            break;
                        }
                        store.insert_observer(id);
                    }
                }
            }
        }

        // Call the body of the binding
        if let Some(mut view_handler) = cx.views.remove(&id) {
            let prev = cx.current;
            cx.current = id;
            let prev_count = cx.count;
            cx.count = 0;
            view_handler.body(cx);
            cx.current = prev;
            cx.count = prev_count;
            cx.views.insert(id, view_handler);
        }

        Handle {
            entity: id,
            style: cx.style.clone(),
            p: Default::default(),
        }
        .width(Units::Stretch(1.0))
        .height(Units::Stretch(1.0))
        
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

        self.lens.view(cx.data().expect(&format!("Failed to get {:?} for entity: {:?}", self.lens, cx.current)))
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