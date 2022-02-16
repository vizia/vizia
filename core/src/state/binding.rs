use std::any::TypeId;
use std::collections::HashSet;

use morphorm::{LayoutType, PositionType};

use crate::{
    Color, Context, Display, Entity, Handle, LensExt, StateStore, TreeExt, Units, View, Visibility,
};

use crate::{Data, Lens};

pub struct Binding<L>
where
    L: Lens,
{
    lens: L,
    // parent: Entity,
    // count: usize,
    builder: Option<Box<dyn Fn(&mut Context, L)>>,
}

impl<L> Binding<L>
where
    L: 'static + Lens,
    <L as Lens>::Source: 'static,
    <L as Lens>::Target: Data,
{
    pub fn new<F>(cx: &mut Context, lens: L, builder: F)
    where
        F: 'static + Fn(&mut Context, L),
    {
        let binding = Self { lens: lens.clone(), builder: Some(Box::new(builder)) };

        let id = if let Some(id) = cx.tree.get_child(cx.current, cx.count) {
            id
        } else {
            let id = cx.entity_manager.create();
            cx.tree.add(id, cx.current).expect("Failed to add to tree");
            cx.cache.add(id).expect("Failed to add to cache");
            cx.style.add(id);
            id
        };

        let ancestors = cx.current.parent_iter(&cx.tree).collect::<HashSet<_>>();

        for entity in id.parent_iter(&cx.tree) {
            if let Some(model_data_store) = cx.data.get_mut(entity) {
                if let Some(model_data) = model_data_store.data.get(&TypeId::of::<L::Source>()) {
                    if let Some(lens_wrap) = model_data_store.lenses.get_mut(&TypeId::of::<L>()) {
                        let observers = lens_wrap.observers();

                        if ancestors.intersection(observers).next().is_none() {
                            lens_wrap.add_observer(id);
                        }
                    } else {
                        let mut observers = HashSet::new();
                        observers.insert(id);

                        let model = model_data.downcast_ref::<L::Source>().unwrap();

                        let old = lens.view(model, |t| t.cloned());

                        model_data_store.lenses.insert(
                            TypeId::of::<L>(),
                            Box::new(StateStore { entity: id, lens, old, observers }),
                        );
                    }

                    break;
                }
            }
        }

        cx.views.insert(id, Box::new(binding));

        cx.count += 1;

        let prev = cx.current;
        let prev_count = cx.count;
        cx.current = id;
        cx.count = 0;
        // Call the body of the binding
        if let Some(mut view_handler) = cx.views.remove(&id) {
            view_handler.body(cx);
            cx.views.insert(id, view_handler);
        }
        cx.current = prev;
        cx.count = prev_count;

        let _: Handle<Self> = Handle { entity: id, p: Default::default(), cx }
            .width(Units::Stretch(1.0))
            .height(Units::Stretch(1.0))
            .ignore();
        //.background_color(Color::blue());
        //.display(Display::None);
    }
}

impl<L: 'static + Lens> View for Binding<L> {
    fn body<'a>(&mut self, cx: &'a mut Context) {
        cx.remove_trailing_children();
        if let Some(builder) = self.builder.take() {
            //let prev = cx.current;
            //let count = cx.count;
            //cx.current = self.parent;
            //cx.count = self.count;
            (builder)(cx, self.lens.clone());
            //cx.current = prev;
            //cx.count = count;
            self.builder = Some(builder);
        }
    }
}

// #[derive(Clone, Copy)]
// pub struct Field<L> {
//     lens: L,
// }

// impl<L: Lens> Field<L>
// where
//     <L as Lens>::Source: 'static,
// {
//     pub fn get<'a>(&self, cx: &'a Context) -> &'a L::Target {
//         self.lens.view(cx.data().expect(&format!(
//             "Failed to get {:?} for entity: {:?}. Is the data in the tree?",
//             self.lens, cx.current
//         )))
//     }
// }

macro_rules! impl_res_simple {
    ($t:ty) => {
        impl Res<$t> for $t {
            fn get_val(&self, _: &Context) -> $t {
                *self
            }

            fn set_or_bind<F>(&self, cx: &mut Context, entity: Entity, closure: F)
            where
                F: 'static + Fn(&mut Context, Entity, Self),
            {
                (closure)(cx, entity, *self);
            }
        }
    };
}

pub trait Res<T> {
    fn get_val(&self, cx: &Context) -> T;
    fn set_or_bind<F>(&self, cx: &mut Context, entity: Entity, closure: F)
    where
        F: 'static + Fn(&mut Context, Entity, T);
}

impl_res_simple!(i8);
impl_res_simple!(i16);
impl_res_simple!(i32);
impl_res_simple!(i64);
impl_res_simple!(i128);
impl_res_simple!(isize);
impl_res_simple!(u8);
impl_res_simple!(u16);
impl_res_simple!(u32);
impl_res_simple!(u64);
impl_res_simple!(u128);
impl_res_simple!(usize);
impl_res_simple!(char);
impl_res_simple!(bool);
impl_res_simple!(f32);
impl_res_simple!(f64);

impl<T, L> Res<T> for L
where
    L: Lens<Target = T>,
    T: Clone + Data,
{
    fn get_val(&self, cx: &Context) -> T {
        (*self.get(cx)).clone()
    }

    fn set_or_bind<F>(&self, cx: &mut Context, entity: Entity, closure: F)
    where
        F: 'static + Fn(&mut Context, Entity, T),
    {
        let prev_current = cx.current;
        let prev_count = cx.count;
        cx.current = entity;
        cx.count = 0;
        Binding::new(cx, self.clone(), move |cx, val| {
            let v = val.get_val(cx);
            (closure)(cx, entity, v);
        });
        cx.current = prev_current;
        cx.count = prev_count;
    }
}

impl<'s> Res<&'s str> for &'s str {
    fn get_val(&self, _: &Context) -> &'s str {
        self
    }

    fn set_or_bind<F>(&self, cx: &mut Context, entity: Entity, closure: F)
    where
        F: 'static + Fn(&mut Context, Entity, Self),
    {
        (closure)(cx, entity, self);
    }
}

impl<'s> Res<&'s String> for &'s String {
    fn get_val(&self, _: &Context) -> &'s String {
        self
    }

    fn set_or_bind<F>(&self, cx: &mut Context, entity: Entity, closure: F)
    where
        F: 'static + Fn(&mut Context, Entity, Self),
    {
        (closure)(cx, entity, self);
    }
}

impl Res<Color> for Color {
    fn get_val(&self, _: &Context) -> Color {
        *self
    }

    fn set_or_bind<F>(&self, cx: &mut Context, entity: Entity, closure: F)
    where
        F: 'static + Fn(&mut Context, Entity, Self),
    {
        (closure)(cx, entity, *self);
    }
}

impl Res<Units> for Units {
    fn get_val(&self, _: &Context) -> Units {
        *self
    }

    fn set_or_bind<F>(&self, cx: &mut Context, entity: Entity, closure: F)
    where
        F: 'static + Fn(&mut Context, Entity, Self),
    {
        (closure)(cx, entity, *self);
    }
}

impl Res<Visibility> for Visibility {
    fn get_val(&self, _: &Context) -> Visibility {
        *self
    }

    fn set_or_bind<F>(&self, cx: &mut Context, entity: Entity, closure: F)
    where
        F: 'static + Fn(&mut Context, Entity, Self),
    {
        (closure)(cx, entity, *self);
    }
}

impl Res<Display> for Display {
    fn get_val(&self, _: &Context) -> Display {
        *self
    }

    fn set_or_bind<F>(&self, cx: &mut Context, entity: Entity, closure: F)
    where
        F: 'static + Fn(&mut Context, Entity, Self),
    {
        (closure)(cx, entity, *self);
    }
}

impl Res<LayoutType> for LayoutType {
    fn get_val(&self, _: &Context) -> LayoutType {
        *self
    }

    fn set_or_bind<F>(&self, cx: &mut Context, entity: Entity, closure: F)
    where
        F: 'static + Fn(&mut Context, Entity, Self),
    {
        (closure)(cx, entity, *self);
    }
}

impl Res<PositionType> for PositionType {
    fn get_val(&self, _: &Context) -> PositionType {
        *self
    }

    fn set_or_bind<F>(&self, cx: &mut Context, entity: Entity, closure: F)
    where
        F: 'static + Fn(&mut Context, Entity, Self),
    {
        (closure)(cx, entity, *self);
    }
}

impl<T: Copy> Res<(T, T)> for (T, T) {
    fn get_val(&self, _: &Context) -> (T, T) {
        *self
    }

    fn set_or_bind<F>(&self, cx: &mut Context, entity: Entity, closure: F)
    where
        F: 'static + Fn(&mut Context, Entity, Self),
    {
        (closure)(cx, entity, *self);
    }
}
