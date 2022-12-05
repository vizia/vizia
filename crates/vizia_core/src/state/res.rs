use std::{marker::PhantomData, process::Output, rc::Rc};

use crate::prelude::*;

use super::Bindable;

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

/// A trait for types that can automatically resolve into other types, with or without consulting
/// the Context.
///
/// This trait is part of the prelude.
pub trait Res<T> {
    fn get_val(&self, cx: &Context) -> T;
    fn get_val_fallible(&self, cx: &Context) -> Option<T> {
        Some(self.get_val(cx))
    }
    fn set_or_bind<F>(&self, cx: &mut Context, entity: Entity, closure: F)
    where
        F: 'static + Clone + Fn(&mut Context, Entity, T);
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
impl_res_simple!(CursorIcon);
impl_res_simple!(Overflow);

#[derive(Clone)]
pub struct BindMap<B, F, T> {
    b: B,
    map: F,
    p: PhantomData<T>,
}

impl<B, F, T> BindMap<B, F, T> {
    pub fn new(b: B, map: F) -> Self
    where
        B: Bindable + Clone,
        F: Fn(&B::Output) -> T,
    {
        Self { b, map, p: PhantomData::default() }
    }
}

impl<B: Copy, F: Copy, T: Clone> Copy for BindMap<B, F, T> {}

impl<B, F, T: Clone + 'static> Bindable for BindMap<B, F, T>
where
    B: 'static + Bindable + Clone,
    F: 'static + Clone + Fn(&B::Output) -> T,
{
    type Output = T;
    fn get_val2<C: DataContext>(&self, cx: &C) -> Self::Output {
        (self.map)(&self.b.get_val2(cx))
    }

    fn insert_store(self, cx: &mut Context, entity: Entity) {
        self.b.insert_store(cx, entity);
    }

    fn name(&self) -> Option<&'static str> {
        self.b.name()
    }
}

#[derive(Clone)]
pub struct BindIndex<B, T> {
    b: B,
    index: usize,
    p: PhantomData<T>,
}

impl<B, T> BindIndex<B, T> {
    pub fn new(b: B, index: usize) -> Self {
        Self { b, index, p: PhantomData::default() }
    }
}

impl<B: Copy, T: Clone> Copy for BindIndex<B, T> {}

impl<B, T: 'static> Bindable for BindIndex<B, T>
where
    B: Bindable,
    <B as Bindable>::Output: std::ops::Deref<Target = [T]>,
    T: Clone,
{
    type Output = T;
    fn get_val2<C: DataContext>(&self, cx: &C) -> Self::Output {
        if let Some(val) = self.b.get_val2(cx).get(self.index) {
            val.clone()
        } else {
            panic!("Failed");
        }
    }

    fn insert_store(self, cx: &mut Context, entity: Entity) {
        self.b.insert_store(cx, entity);
    }

    fn name(&self) -> Option<&'static str> {
        self.b.name()
    }
}

#[derive(Clone)]
pub struct BindThen<B, L, T> {
    b: B,
    l: L,
    p: PhantomData<T>,
}

impl<B, L, T> BindThen<B, L, T> {
    pub fn new(b: B, l: L) -> Self {
        Self { b, l, p: PhantomData::default() }
    }
}

// impl<B: Copy, T: Clone> Copy for BindIndex<B, T> {}

impl<B, L, T: 'static> Bindable for BindThen<B, L, T>
where
    B: Bindable,
    L: Lens<Source = B::Output, Target = T>,
    T: Clone,
{
    type Output = L::Target;
    fn get_val2<C: DataContext>(&self, cx: &C) -> Self::Output {
        self.l.view(&self.b.get_val2(cx), |t| t.cloned().expect("Failed")).clone()
    }

    fn insert_store(self, cx: &mut Context, entity: Entity) {
        self.b.insert_store(cx, entity);
    }

    fn name(&self) -> Option<&'static str> {
        self.b.name()
    }
}
// pub trait BindableExt: Clone {
//     type Map<S, F, T>;
//     type Output;
//     fn map<F, T: 'static>(self, f: F) -> Self::Map<Self, F, T>
//     where
//         F: 'static + Clone + Fn(&Self::Output) -> T;
// }

pub trait BindableExt: Bindable + Clone {
    fn get<C: DataContext>(&self, cx: &C) -> Self::Output {
        self.get_val2(cx)
    }

    // type Output;
    fn map<F, T: 'static>(self, f: F) -> BindMap<Self, F, T>
    where
        F: 'static + Clone + Fn(&Self::Output) -> T,
    {
        BindMap::new(self, f)
    }

    fn index<T: 'static>(self, index: usize) -> BindIndex<Self, T> {
        BindIndex::new(self, index)
    }

    fn then<L: Lens, T: 'static>(self, lens: L) -> BindThen<Self, L, T> {
        BindThen::new(self, lens)
    }
}

impl<B: Bindable + Clone> BindableExt for B {}

// impl<L: Lens> BindableExt for L
// // where
// //     L::Target: Data,
// {
//     fn map<F, T: 'static>(self, f: F) -> BindMap<Self, F, T>
//     where
//         F: 'static + Clone + Fn(&Self::Output) -> T,
//     {
//         self.then(crate::state::lens::Map::new(f))
//     }
// }

// impl<L1: Lens, L2: Lens> BindableExt for (L1, L2)
// where
//     L1::Target: Data,
//     L2::Target: Data,
// {
//     type Output = (L1::Target, L2::Target);
//     type Map<S, F, T> = BindMap<S, F, T>;
//     fn map<F, T: 'static>(self, f: F) -> BindMap<Self, F, T>
//     where
//         F: 'static + Clone + Fn(&Self::Output) -> T,
//     {
//         BindMap::new(self, f)
//     }
// }

// impl<B, F, T> Res<T> for BindMap<B, F, T>
// where
//     B: 'static + Bindable + Clone,
//     F: 'static + Fn(&B::Output) -> T,
// {
//     fn get_val(&self, cx: &Context) -> T {
//         (self.map)(&self.b.get_val2(cx))
//     }

//     fn set_or_bind<G>(&self, cx: &mut Context, entity: Entity, closure: G)
//     where
//         G: 'static + Clone + Fn(&mut Context, Entity, T),
//     {
//         cx.with_current(entity, |cx| {
//             let map = self.map.clone();
//             Binding::new(cx, self.b.clone(), move |cx, b| {
//                 let val = (map)(&b.get_val2(cx));

//                 (closure)(cx, entity, val)
//             });
//         });
//     }
// }

impl<T, B> Res<T> for B
where
    B: 'static + Clone + Bindable<Output = T>,
    T: Clone + Data,
{
    fn get_val(&self, cx: &Context) -> T {
        self.get_val2(cx)
    }

    // fn get_val_fallible(&self, cx: &Context) -> Option<T> {
    //     self.get_fallible(cx).map(|x| x)
    // }

    fn set_or_bind<F>(&self, cx: &mut Context, entity: Entity, closure: F)
    where
        F: 'static + Fn(&mut Context, Entity, T),
    {
        cx.with_current(entity, |cx| {
            Binding::new(cx, self.clone(), move |cx, val| {
                if let Some(v) = val.get_val_fallible(cx) {
                    (closure)(cx, entity, v);
                }
            });
        });
    }
}

// impl<T, L> Res<T> for L
// where
//     L: Lens<Target = T> + LensExt,
//     T: Clone + Data,
// {
//     fn get_val(&self, cx: &Context) -> T {
//         self.get(cx)
//     }

//     fn get_val_fallible(&self, cx: &Context) -> Option<T> {
//         self.get_fallible(cx).map(|x| x)
//     }

//     fn set_or_bind<F>(&self, cx: &mut Context, entity: Entity, closure: F)
//     where
//         F: 'static + Fn(&mut Context, Entity, T),
//     {
//         cx.with_current(entity, |cx| {
//             Binding::new(cx, self.clone(), move |cx, val| {
//                 if let Some(v) = val.get_val_fallible(cx) {
//                     (closure)(cx, entity, v);
//                 }
//             });
//         });
//     }
// }

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

impl Res<(u32, u32)> for (u32, u32) {
    fn get_val(&self, _: &Context) -> (u32, u32) {
        *self
    }

    fn set_or_bind<F>(&self, cx: &mut Context, entity: Entity, closure: F)
    where
        F: 'static + Fn(&mut Context, Entity, Self),
    {
        (closure)(cx, entity, *self);
    }
}

impl<T: Clone + Res<T>> Res<Option<T>> for Option<T> {
    fn get_val(&self, _: &Context) -> Option<T> {
        self.clone()
    }

    fn set_or_bind<F>(&self, cx: &mut Context, entity: Entity, closure: F)
    where
        F: 'static + Clone + Fn(&mut Context, Entity, Option<T>),
    {
        (closure)(cx, entity, self.clone())
    }
}
