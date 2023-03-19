use std::borrow::Borrow;

use crate::prelude::*;

macro_rules! impl_res_simple {
    ($t:ty) => {
        impl GenericRes<$t, $t> for $t {
            fn get_ref(&self, _: &impl DataContext) -> Option<LensValue<'_, $t>> {
                Some(LensValue::Borrowed(self))
            }

            fn get_val(&self, _: &impl DataContext) -> $t {
                *self
            }

            fn set_or_bind<F>(&self, cx: &mut Context, entity: Entity, closure: F)
            where
                F: 'static + Fn(&mut Context, Entity, &Self),
            {
                (closure)(cx, entity, self);
            }
        }
    };
}

/// A trait for types that can automatically resolve into other types, with or without consulting
/// the Context.
///
/// This trait is part of the prelude.
pub trait Res<T> {
    #[allow(unused_variables)]
    fn get_ref<'a>(&'a self, cx: &'a impl DataContext) -> Option<LensValue<'a, T>> {
        None
    }

    fn get_val(&self, _: &impl DataContext) -> T;

    fn set_or_bind<F>(&self, cx: &mut Context, entity: Entity, closure: F)
    where
        F: 'static + Clone + Fn(&mut Context, Entity, &Self);
}

pub trait GenericRes<T, B>
where
    T: Borrow<B>,
{
    #[allow(unused_variables)]
    fn get_ref<'a>(&'a self, cx: &'a impl DataContext) -> Option<LensValue<'a, B, T>> {
        None
    }

    fn get_val(&self, _: &impl DataContext) -> T;

    fn set_or_bind<F>(&self, cx: &mut Context, entity: Entity, closure: F)
    where
        F: 'static + Clone + Fn(&mut Context, Entity, &Self);
}

impl<R, T> Res<T> for R
where
    R: GenericRes<T, T>,
{
    fn get_val(&self, cx: &impl DataContext) -> T {
        <Self as GenericRes<T, T>>::get_val(&self, cx)
    }

    fn set_or_bind<F>(&self, cx: &mut Context, entity: Entity, closure: F)
    where
        F: 'static + Clone + Fn(&mut Context, Entity, &Self),
    {
        <Self as GenericRes<T, T>>::set_or_bind(&self, cx, entity, closure)
    }

    fn get_ref<'a>(&'a self, cx: &'a impl DataContext) -> Option<LensValue<'a, T, T>> {
        <Self as GenericRes<T, T>>::get_ref(&self, cx)
    }
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
impl_res_simple!(Weight);
impl_res_simple!(FontStyle);

impl<T, L, B> GenericRes<T, B> for L
// translation lookaside buffer
where
    L: Lens<TargetOwned = T, Target = B>,
    T: Clone + Data + Borrow<B>,
    B: Data + ToOwned<Owned = T>,
    L::Source: Sized,
{
    fn get_ref<'a>(&'a self, cx: &'a impl DataContext) -> Option<LensValue<'a, B, T>> {
        self.view(LensValue::Borrowed(cx.data()?))
    }

    fn get_val(&self, cx: &impl DataContext) -> T {
        self.get_ref(cx).unwrap().into_owned()
    }

    fn set_or_bind<F>(&self, cx: &mut Context, entity: Entity, closure: F)
    where
        F: 'static + Fn(&mut Context, Entity, &Self),
    {
        cx.with_current(entity, |cx| {
            Binding::new(cx, self.clone(), move |cx, val| {
                (closure)(cx, entity, &val);
            });
        });
    }
}

impl<'s> Res<&'s str> for &'s str {
    fn get_val(&self, _: &impl DataContext) -> &'s str {
        self
    }

    fn set_or_bind<F>(&self, cx: &mut Context, entity: Entity, closure: F)
    where
        F: 'static + Fn(&mut Context, Entity, &Self),
    {
        (closure)(cx, entity, self);
    }
}

impl<'s> Res<&'s String> for &'s String {
    fn get_val(&self, _: &impl DataContext) -> Self {
        *self
    }

    fn set_or_bind<F>(&self, cx: &mut Context, entity: Entity, closure: F)
    where
        F: 'static + Fn(&mut Context, Entity, &Self),
    {
        (closure)(cx, entity, self);
    }
}

impl<'s> Res<String> for String {
    fn get_val(&self, _: &impl DataContext) -> Self {
        self.clone()
    }

    fn set_or_bind<F>(&self, cx: &mut Context, entity: Entity, closure: F)
    where
        F: 'static + Fn(&mut Context, Entity, &Self),
    {
        (closure)(cx, entity, self);
    }
}

impl Res<Color> for Color {
    fn get_val(&self, _: &impl DataContext) -> Color {
        *self
    }

    fn set_or_bind<F>(&self, cx: &mut Context, entity: Entity, closure: F)
    where
        F: 'static + Fn(&mut Context, Entity, &Self),
    {
        (closure)(cx, entity, self);
    }
}

impl Res<Units> for Units {
    fn get_val(&self, _: &impl DataContext) -> Units {
        *self
    }

    fn set_or_bind<F>(&self, cx: &mut Context, entity: Entity, closure: F)
    where
        F: 'static + Fn(&mut Context, Entity, &Self),
    {
        (closure)(cx, entity, self);
    }
}

impl Res<Visibility> for Visibility {
    fn get_val(&self, _: &impl DataContext) -> Visibility {
        *self
    }

    fn set_or_bind<F>(&self, cx: &mut Context, entity: Entity, closure: F)
    where
        F: 'static + Fn(&mut Context, Entity, &Self),
    {
        (closure)(cx, entity, self);
    }
}

impl Res<Display> for Display {
    fn get_val(&self, _: &impl DataContext) -> Display {
        *self
    }

    fn set_or_bind<F>(&self, cx: &mut Context, entity: Entity, closure: F)
    where
        F: 'static + Fn(&mut Context, Entity, &Self),
    {
        (closure)(cx, entity, self);
    }
}

impl Res<LayoutType> for LayoutType {
    fn get_val(&self, _: &impl DataContext) -> LayoutType {
        *self
    }

    fn set_or_bind<F>(&self, cx: &mut Context, entity: Entity, closure: F)
    where
        F: 'static + Fn(&mut Context, Entity, &Self),
    {
        (closure)(cx, entity, self);
    }
}

impl Res<PositionType> for PositionType {
    fn get_val(&self, _: &impl DataContext) -> PositionType {
        *self
    }

    fn set_or_bind<F>(&self, cx: &mut Context, entity: Entity, closure: F)
    where
        F: 'static + Fn(&mut Context, Entity, &Self),
    {
        (closure)(cx, entity, self);
    }
}

impl Res<(u32, u32)> for (u32, u32) {
    fn get_val(&self, _: &impl DataContext) -> (u32, u32) {
        *self
    }

    fn set_or_bind<F>(&self, cx: &mut Context, entity: Entity, closure: F)
    where
        F: 'static + Fn(&mut Context, Entity, &Self),
    {
        (closure)(cx, entity, self);
    }
}

impl<T: Clone + Res<T>> Res<Option<T>> for Option<T> {
    fn get_val(&self, _: &impl DataContext) -> Option<T> {
        self.clone()
    }

    fn set_or_bind<F>(&self, cx: &mut Context, entity: Entity, closure: F)
    where
        F: 'static + Clone + Fn(&mut Context, Entity, &Option<T>),
    {
        (closure)(cx, entity, self)
    }
}

impl<T: Clone + Res<T>> Res<Vec<T>> for Vec<T> {
    fn get_val(&self, _: &impl DataContext) -> Vec<T> {
        self.clone()
    }

    fn set_or_bind<F>(&self, cx: &mut Context, entity: Entity, closure: F)
    where
        F: 'static + Clone + Fn(&mut Context, Entity, &Vec<T>),
    {
        (closure)(cx, entity, self)
    }
}

impl Res<FamilyOwned> for FamilyOwned {
    fn get_val(&self, _: &impl DataContext) -> FamilyOwned {
        self.clone()
    }

    fn set_or_bind<F>(&self, cx: &mut Context, entity: Entity, closure: F)
    where
        F: 'static + Clone + Fn(&mut Context, Entity, &FamilyOwned),
    {
        (closure)(cx, entity, self)
    }
}
