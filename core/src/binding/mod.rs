//! # Data Binding
//!
//! Binding provides a way to add reactivity to a vizia application. Rather than sending events back and forth between widgets
//! to update local widget data, widgets can instead `bind` to application data.
//!
//! # Example
//! Fist we declare the data for our application. The [Lens] trait has been derived for the data, which allows us to bind to fields of the struct:
//! ```
//! #[derive(Default, Lens)]
//! struct AppData {
//!     some_data: bool,
//! }
//! ```
//! Next we'll declare some events which will be sent by widgets to modify the app data. Data binding in vizia is one-way, events are sent up the tree
//! to the app data to mutate it and updated values are sent to observer [Binding] views.
//! ```
//! struct AppEvent {
//!     SetTrue,
//!     SetFalse,   
//! }
//! ```
//! Next we implement the [Model] trait on our app data, which allows us to modify the data in response to an [Event]:
//! ```
//! impl Model for AppData {
//!     fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
//!         if let Some(app_event) = event.message.downcast() {
//!             match app_event {
//!                 AppEvent::SetTrue => {
//!                     self.some_data = true;
//!                 }
//!
//!                 AppEvent::SetFalse => {
//!                     self.some_data = false;
//!                 }
//!             }   
//!         }
//!     }
//! }
//! ```.
//! This trait also allows data to be built into the [Tree]:
//! ```
//! fn main() {
//!     Application::new(WindowDescription::new(), |cx|{
//!         AppData::default().build(cx);
//!     }).run();  
//! }
//! ```
//! A [Binding] view allows the data to be used by widgets. A [Lens] is used to determine what data the binding should react to:
//! ```
//! fn main() {
//!     Application::new(WindowDescription::new(), |cx|{
//!         AppData::default().build(cx);
//!
//!         Binding::new(cx, AppData::some_data, |cx, some_data|{
//!             Label::new(cx, &some_data.get(cx).to_string());
//!         });
//!     }).run();
//! }
//! ```
//! The second parameter to a [Binding] view is a [Lens] on the application data, allowing us to bind to some field of the application data.
//! The third parameter is a closure which provides the context and a [Field] parameter which can be used to retrieve the bound data using the `.get()`
//! method, which takes the [Context] as an argument.
//!
//! Now when the data is modified by another widget, the label will update, for example:
//! ```
//! //! fn main() {
//!     Application::new(WindowDescription::new(), |cx|{
//!         AppData::default().build(cx);
//!
//!         Binding::new(cx, AppData::some_data, |cx, some_data|{
//!             Label::new(cx, &some_data.get(cx).to_string());
//!         });
//!
//!         Checkbox::new(cx, false)
//!             .on_checked(cx, |cx| cx.emit(AppEvent::SetTrue))
//!             .on_unchecked(cx, |cx| cx.emit(AppEvent::SetFalse));
//!     }).run();
//! }
//! ```
//! Note, the checkbox does not need to be bound to the data to send an event to it. By default events will propagate up the tree.
//!
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
where
    L: Lens,
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
    pub fn new<F>(cx: &mut Context, lens: L, builder: F)
    where
        F: 'static + Fn(&mut Context, Field<L>),
        <L as Lens>::Source: Model,
    {
        let parent = cx.current;

        let binding = Self { lens, parent, count: cx.count + 1, builder: Some(Box::new(builder)) };

        let id = if let Some(id) = cx.tree.get_child(cx.current, cx.count) {
            id
        } else {
            let id = cx.entity_manager.create();
            cx.tree.add(id, cx.current).expect("Failed to add to tree");
            cx.cache.add(id).expect("Failed to add to cache");
            cx.style.borrow_mut().add(id);
            id
        };

        let ancestors = parent.parent_iter(&cx.tree).collect::<HashSet<_>>();

        for entity in id.parent_iter(&cx.tree) {
            if let Some(model_data_store) = cx.data.model_data.get_mut(entity) {
                if let Some(model_data) = model_data_store.data.get(&TypeId::of::<L::Source>()) {
                    if let Some(lens_wrap) = model_data_store.lenses.get_mut(&TypeId::of::<L>()) {
                        let observers = lens_wrap.observers();

                        if ancestors.intersection(observers).next().is_none() {
                            lens_wrap.add_observer(id);
                        }
                    } else {
                        let mut observers = HashSet::new();
                        observers.insert(id);

                        let model = model_data.downcast_ref::<Store<L::Source>>().unwrap();

                        let old = lens.view(&model.data);

                        model_data_store.lenses.insert(
                            TypeId::of::<L>(),
                            Box::new(StateStore { entity: id, lens, old: old.clone(), observers }),
                        );
                    }

                    break;
                }
            }
        }

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

        let _: Handle<Self> = Handle { entity: id, style: cx.style.clone(), p: Default::default() }
            .width(Units::Stretch(1.0))
            .height(Units::Stretch(1.0))
            .background_color(Color::blue())
            .display(Display::None);

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
            //let prev = cx.current;
            //let count = cx.count;
            cx.current = self.parent;
            cx.count = self.count;
            (builder)(cx, Field { lens: self.lens.clone() });
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
where
    <L as Lens>::Source: 'static,
{
    pub fn get<'a>(&self, cx: &'a Context) -> &'a L::Target {
        self.lens.view(
            cx.data()
                .expect(&format!("Failed to get {:?} for entity: {:?}", self.lens, cx.current)),
        )
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
