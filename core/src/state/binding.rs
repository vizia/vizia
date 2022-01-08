use std::any::TypeId;
use std::collections::HashSet;

use morphorm::{PositionType, LayoutType};

use crate::{Color, Context, Display, Entity, Handle, StateStore, Store, TreeExt, Units, View, Visibility};

use crate::{Data, Lens, Model};

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
            cx.style.add(id);
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

        let _: Handle<Self> = Handle { entity: id, p: Default::default(), cx }
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
        self.lens.view(cx.data().expect(&format!(
            "Failed to get {:?} for entity: {:?}. Is the data in the tree?",
            self.lens, cx.current
        )))
        // self.lens
        //     .view(&cx.data.model_data
        //     .get(&TypeId::of::<L::Source>())
        //     .unwrap()
        //     .downcast_ref::<Store<L::Source>>()
        //     .unwrap().data)
    }
}

pub trait Res<T> {
    fn get<'a>(&'a self, cx: &'a Context) -> &'a T;
}

impl<T,L> Res<T> for Field<L>
where
    L: Lens<Target = T>,
{
    fn get<'a>(&'a self, cx: &'a Context) -> &'a T
    {
        self.get(cx)
    }
}

impl Res<Color> for Color {
    fn get<'a>(&'a self, cx: &'a Context) -> &'a Color {
        self
    }
}

impl Res<Units> for Units {
    fn get<'a>(&'a self, cx: &'a Context) -> &'a Units {
        self
    }
}

impl Res<Visibility> for Visibility {
    fn get<'a>(&'a self, cx: &'a Context) -> &'a Visibility {
        self
    }
}

impl Res<Display> for Display {
    fn get<'a>(&'a self, cx: &'a Context) -> &'a Display {
        self
    }
}

impl Res<LayoutType> for LayoutType {
    fn get<'a>(&'a self, cx: &'a Context) -> &'a LayoutType {
        self
    }
}

impl Res<PositionType> for PositionType {
    fn get<'a>(&'a self, cx: &'a Context) -> &'a PositionType {
        self
    }
}

impl Res<usize> for usize {
    fn get<'a>(&'a self, cx: &'a Context) -> &'a usize {
        self
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
