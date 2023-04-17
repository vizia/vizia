use vizia_style::{FontStyle, FontWeight, FontWeightKeyword};

use crate::prelude::*;

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
                cx.with_current(entity, |cx| {
                    (closure)(cx, entity, *self);
                });
            }
        }
    };
}

/// A trait which allows passing a value or a lens to a view or modifier.
///
/// For example, the `Label` view constructor takes a type which implements `Res<T>` where
/// `T` implements `ToString`. This allows the user to pass a type which implements `ToString`,
/// such as `String` or `&str`, or a lens to a type which implements `ToString`.
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
impl_res_simple!(LengthValue);
impl_res_simple!(FontWeight);
impl_res_simple!(FontWeightKeyword);
impl_res_simple!(FontStyle);
impl_res_simple!(BorderCornerShape);
impl_res_simple!(Angle);

impl<T, L> Res<T> for L
where
    L: Lens<Target = T> + LensExt,
    T: Clone + Data,
{
    fn get_val(&self, cx: &Context) -> T {
        self.get(cx)
    }

    fn get_val_fallible(&self, cx: &Context) -> Option<T> {
        self.get_fallible(cx)
    }

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

impl<'i> Res<BackgroundImage<'i>> for BackgroundImage<'i> {
    fn get_val(&self, _: &Context) -> Self {
        self.clone()
    }

    fn set_or_bind<F>(&self, cx: &mut Context, entity: Entity, closure: F)
    where
        F: 'static + Fn(&mut Context, Entity, Self),
    {
        (closure)(cx, entity, self.clone());
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

impl<'s> Res<Transform> for Transform {
    fn get_val(&self, _: &Context) -> Transform {
        self.clone()
    }

    fn set_or_bind<F>(&self, cx: &mut Context, entity: Entity, closure: F)
    where
        F: 'static + Fn(&mut Context, Entity, Self),
    {
        (closure)(cx, entity, self.clone());
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

impl Res<LinearGradient> for LinearGradient {
    fn get_val(&self, _: &Context) -> LinearGradient {
        self.clone()
    }

    fn set_or_bind<F>(&self, cx: &mut Context, entity: Entity, closure: F)
    where
        F: 'static + Fn(&mut Context, Entity, Self),
    {
        (closure)(cx, entity, self.clone());
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

impl Res<Length> for Length {
    fn get_val(&self, _: &Context) -> Self {
        self.clone()
    }

    fn set_or_bind<F>(&self, cx: &mut Context, entity: Entity, closure: F)
    where
        F: 'static + Clone + Fn(&mut Context, Entity, Self),
    {
        (closure)(cx, entity, self.clone())
    }
}

impl Res<LengthOrPercentage> for LengthOrPercentage {
    fn get_val(&self, _: &Context) -> Self {
        self.clone()
    }

    fn set_or_bind<F>(&self, cx: &mut Context, entity: Entity, closure: F)
    where
        F: 'static + Clone + Fn(&mut Context, Entity, Self),
    {
        (closure)(cx, entity, self.clone())
    }
}

impl Res<RGBA> for RGBA {
    fn get_val(&self, _: &Context) -> Self {
        self.clone()
    }

    fn set_or_bind<F>(&self, cx: &mut Context, entity: Entity, closure: F)
    where
        F: 'static + Clone + Fn(&mut Context, Entity, Self),
    {
        (closure)(cx, entity, self.clone())
    }
}

impl<T: Clone + Res<T>> Res<Vec<T>> for Vec<T> {
    fn get_val(&self, _: &Context) -> Vec<T> {
        self.clone()
    }

    fn set_or_bind<F>(&self, cx: &mut Context, entity: Entity, closure: F)
    where
        F: 'static + Clone + Fn(&mut Context, Entity, Vec<T>),
    {
        (closure)(cx, entity, self.clone())
    }
}

impl<T: Clone + Res<T>, const N: usize> Res<[T; N]> for [T; N] {
    fn get_val(&self, _: &Context) -> Self {
        self.clone()
    }

    fn set_or_bind<F>(&self, cx: &mut Context, entity: Entity, closure: F)
    where
        F: 'static + Clone + Fn(&mut Context, Entity, Self),
    {
        (closure)(cx, entity, self.clone())
    }
}

impl Res<FamilyOwned> for FamilyOwned {
    fn get_val(&self, _: &Context) -> FamilyOwned {
        self.clone()
    }

    fn set_or_bind<F>(&self, cx: &mut Context, entity: Entity, closure: F)
    where
        F: 'static + Clone + Fn(&mut Context, Entity, FamilyOwned),
    {
        (closure)(cx, entity, self.clone())
    }
}

impl<T1: Clone, T2: Clone> Res<(T1, T2)> for (T1, T2) {
    fn get_val(&self, _cx: &Context) -> (T1, T2) {
        self.clone()
    }

    fn set_or_bind<F>(&self, cx: &mut Context, entity: Entity, closure: F)
    where
        F: 'static + Clone + Fn(&mut Context, Entity, (T1, T2)),
    {
        (closure)(cx, entity, self.clone());
    }
}

impl<T1: Clone, T2: Clone, T3: Clone> Res<(T1, T2, T3)> for (T1, T2, T3) {
    fn get_val(&self, _cx: &Context) -> (T1, T2, T3) {
        self.clone()
    }

    fn set_or_bind<F>(&self, cx: &mut Context, entity: Entity, closure: F)
    where
        F: 'static + Clone + Fn(&mut Context, Entity, (T1, T2, T3)),
    {
        (closure)(cx, entity, self.clone());
    }
}

impl<T1: Clone, T2: Clone, T3: Clone, T4: Clone> Res<(T1, T2, T3, T4)> for (T1, T2, T3, T4) {
    fn get_val(&self, _cx: &Context) -> (T1, T2, T3, T4) {
        self.clone()
    }

    fn set_or_bind<F>(&self, cx: &mut Context, entity: Entity, closure: F)
    where
        F: 'static + Clone + Fn(&mut Context, Entity, (T1, T2, T3, T4)),
    {
        (closure)(cx, entity, self.clone());
    }
}
