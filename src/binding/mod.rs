mod lens;

use std::{any::TypeId, collections::HashSet};

pub use lens::*;

mod model;
pub use model::*;

mod state;
pub use state::*;

mod store;
pub use store::*;

mod data;
pub use data::*;

use crate::{Color, Context, Display, Entity, Handle, TreeExt, Units, View};


pub struct Binding<L> 
where L: Lens
{
    lens: L,
    parent: Entity,
    count: usize,
    builder: Option<Box<dyn Fn(&mut Context, Field<L>)>>,
}

impl<L> Binding<L> 
where 
    L: 'static + Lens,
    <L as Lens>::Source: 'static,
    <L as Lens>::Target: Data,
{
    pub fn new<F>(cx: &mut Context, lens: L, builder: F) -> Handle<Self> 
    where 
        F: 'static + Fn(&mut Context, Field<L>),
        <L as Lens>::Source: Model,
    {

        println!("New binding: {}", cx.count);
        let parent = cx.current;

        let binding = Self {
            lens,
            parent,
            count: cx.count + 1,
            builder: Some(Box::new(builder)),
        };

        let id = if let Some(id) = cx.tree.get_child(cx.current, cx.count) {
            // let prev = cx.current;
            // cx.current = id;
            // let prev_count = cx.count;
            // cx.count = 0;
            //binding.body(cx);
            // cx.current = prev;
            // cx.count = prev_count;
            //cx.views.insert(id, Box::new(binding));
            id
        } else {
            let id = cx.entity_manager.create();
            cx.tree.add(id, cx.current).expect("Failed to add to tree");
            cx.cache.add(id).expect("Failed to add to cache");
            cx.style.borrow_mut().add(id);
            //let prev = cx.current;
            //cx.current = id;
            //let prev_count = cx.count;
            //cx.count = 0;
            //binding.body(cx);
            //cx.current = prev;
            //cx.count = prev_count;
            
            id  
        };

        
        
        //let mut ancestors = HashSet::new();
        //for entity in parent.parent_iter(&cx.tree) {
            //ancestors.insert(entity);

            let ancestors = parent.parent_iter(&cx.tree).collect::<HashSet<_>>();

            if let Some(lens_wrap) = cx.lenses.get_mut(&TypeId::of::<L>()) {
                let observers = lens_wrap.observers();

                if ancestors.intersection(observers).next().is_none() {
                    lens_wrap.add_observer(id);
                }
                
            } else {
                let mut observers = HashSet::new();
                observers.insert(id);
                let old = lens.view(cx.data().expect("Failed to find model. Has it been built into the tree?"));
                cx.lenses.insert(TypeId::of::<L>(), Box::new(StateStore {
                    lens,
                    old: old.clone(),
                    observers,
                }));
            }

            // if let Some(model_list) = cx.data.model_data.get_mut(entity) {
            //     for (_, model) in model_list.iter_mut() {
            //         if let Some(store) = model.downcast::<Store<L::Source>>() {

            //             let state = binding.lens.view_mut(&mut store.data);

            //             state.add_observer(id);

            //             if store.observers.intersection(&ancestors).next().is_some() {
            //                 break;
            //             }
            //             store.insert_observer(id);
            //         }
            //     }
            // }
        //}

        cx.views.insert(id, Box::new(binding));

        cx.count += 1;

        // Call the body of the binding
        if let Some(mut view_handler) = cx.views.remove(&id) {
            //let prev = cx.current;
            //cx.current = parent;
            //let prev_count = cx.count;
            //cx.count = 0;
            //println!("count: {}", cx.count);
            view_handler.body(cx);
            //cx.current = prev;
            //cx.count = prev_count;
            cx.views.insert(id, view_handler);
        }



        Handle {
            entity: id,
            style: cx.style.clone(),
            p: Default::default(),
        }
        .width(Units::Stretch(1.0))
        .height(Units::Stretch(1.0))
        .background_color(Color::blue())
        .display(Display::None)
        
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
            let prev = cx.current;
            let count = cx.count;
            cx.current = self.parent;
            cx.count = self.count;
            (builder)(cx, Field{lens: self.lens.clone()});
            //cx.current = prev;
            //cx.count = count;
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

fn typeid<T: std::any::Any>(_: &T) {
    println!("{:?}", std::any::TypeId::of::<T>());
}