use vizia_style::{BoxShadow, FontStretch, FontStyle, FontWeight, FontWeightKeyword};

use crate::{
    modifiers::{BoxShadowBuilder, LinearGradientBuilder},
    prelude::*,
};

macro_rules! impl_res_simple {
    ($t:ty) => {
        impl Res<$t> for $t {
            fn get<'a>(&self, _: &impl DataContext) -> Option<LensValue<$t>> {
                Some(LensValue::Borrowed(self))
            }

            fn get_val(&self, _: &impl DataContext) -> $t {
                *self
            }

            fn set_or_bind<F>(self, cx: &mut Context, entity: Entity, closure: F)
            where
                F: 'static + Fn(&mut Context, Self),
            {
                cx.with_current(entity, |cx| {
                    (closure)(cx, self);
                });
            }
        }
    };
}

macro_rules! impl_res_clone {
    ($t:ty) => {
        impl Res<$t> for $t {
            fn get<'a>(&'a self, _: &'a impl DataContext) -> Option<LensValue<'a, $t>> {
                Some(LensValue::Borrowed(self))
            }

            fn get_val(&self, _: &impl DataContext) -> $t {
                self.clone()
            }

            fn set_or_bind<F>(self, cx: &mut Context, entity: Entity, closure: F)
            where
                F: 'static + Fn(&mut Context, Self),
            {
                cx.with_current(entity, |cx| {
                    (closure)(cx, self);
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
    #[allow(unused_variables)]
    fn get<'a>(&'a self, cx: &'a impl DataContext) -> Option<LensValue<'a, T>> {
        None
    }

    fn get_val(&self, _: &impl DataContext) -> T;

    fn set_or_bind<F>(self, cx: &mut Context, entity: Entity, closure: F)
    where
        F: 'static + Clone + Fn(&mut Context, Self);
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
impl_res_simple!(TextAlign);
impl_res_clone!(BoxShadow);
impl_res_clone!(LinearGradientBuilder);
impl_res_clone!(BoxShadowBuilder);
impl_res_clone!(Filter);
impl_res_simple!(Opacity);
impl_res_simple!(FontStretch);
impl_res_clone!(Translate);
impl_res_clone!(Scale);
impl_res_clone!(Position);
impl_res_simple!(PointerEvents);

impl<L> Res<L::Target> for L
where
    L: Lens,
    L::Target: Data,
{
    fn get<'a>(&'a self, cx: &'a impl DataContext) -> Option<LensValue<'a, L::Target>> {
        self.view(cx.data()?)
    }

    fn get_val(&self, cx: &impl DataContext) -> L::Target {
        self.get(cx).unwrap().into_owned()
    }

    fn set_or_bind<F>(self, cx: &mut Context, entity: Entity, closure: F)
    where
        F: 'static + Fn(&mut Context, Self),
    {
        cx.with_current(entity, |cx| {
            Binding::new(cx, self.clone(), move |cx, val| {
                cx.with_current(entity, |cx| {
                    (closure)(cx, val);
                });
            });
        });
    }
}

impl<'i> Res<FontFamily<'i>> for FontFamily<'i> {
    fn get_val(&self, _: &impl DataContext) -> Self {
        self.clone()
    }

    fn set_or_bind<F>(self, cx: &mut Context, entity: Entity, closure: F)
    where
        F: 'static + Fn(&mut Context, Self),
    {
        cx.with_current(entity, |cx| {
            (closure)(cx, self);
        });
    }
}

impl<'i> Res<BackgroundImage<'i>> for BackgroundImage<'i> {
    fn get_val(&self, _: &impl DataContext) -> Self {
        self.clone()
    }

    fn set_or_bind<F>(self, cx: &mut Context, entity: Entity, closure: F)
    where
        F: 'static + Fn(&mut Context, Self),
    {
        cx.with_current(entity, |cx| {
            (closure)(cx, self);
        });
    }
}

impl<'s> Res<&'s str> for &'s str {
    fn get_val(&self, _: &impl DataContext) -> &'s str {
        self
    }

    fn set_or_bind<F>(self, cx: &mut Context, entity: Entity, closure: F)
    where
        F: 'static + Fn(&mut Context, Self),
    {
        cx.with_current(entity, |cx| {
            (closure)(cx, self);
        });
    }
}

impl<'s> Res<&'s String> for &'s String {
    fn get_val(&self, _: &impl DataContext) -> Self {
        self
    }

    fn set_or_bind<F>(self, cx: &mut Context, entity: Entity, closure: F)
    where
        F: 'static + Fn(&mut Context, Self),
    {
        cx.with_current(entity, |cx| {
            (closure)(cx, self);
        });
    }
}

impl Res<String> for String {
    fn get_val(&self, _: &impl DataContext) -> Self {
        self.clone()
    }

    fn set_or_bind<F>(self, cx: &mut Context, entity: Entity, closure: F)
    where
        F: 'static + Fn(&mut Context, Self),
    {
        cx.with_current(entity, |cx| {
            (closure)(cx, self);
        });
    }
}

impl Res<Transform> for Transform {
    fn get_val(&self, _: &impl DataContext) -> Transform {
        self.clone()
    }

    fn set_or_bind<F>(self, cx: &mut Context, entity: Entity, closure: F)
    where
        F: 'static + Fn(&mut Context, Self),
    {
        cx.with_current(entity, |cx| {
            (closure)(cx, self);
        });
    }
}

impl Res<Color> for Color {
    fn get_val(&self, _: &impl DataContext) -> Color {
        *self
    }

    fn set_or_bind<F>(self, cx: &mut Context, entity: Entity, closure: F)
    where
        F: 'static + Fn(&mut Context, Self),
    {
        cx.with_current(entity, |cx| {
            (closure)(cx, self);
        });
    }
}

impl Res<LinearGradient> for LinearGradient {
    fn get_val(&self, _: &impl DataContext) -> LinearGradient {
        self.clone()
    }

    fn set_or_bind<F>(self, cx: &mut Context, entity: Entity, closure: F)
    where
        F: 'static + Fn(&mut Context, Self),
    {
        cx.with_current(entity, |cx| {
            (closure)(cx, self);
        });
    }
}

impl Res<Units> for Units {
    fn get_val(&self, _: &impl DataContext) -> Units {
        *self
    }

    fn set_or_bind<F>(self, cx: &mut Context, entity: Entity, closure: F)
    where
        F: 'static + Fn(&mut Context, Self),
    {
        cx.with_current(entity, |cx| {
            (closure)(cx, self);
        });
    }
}

impl Res<Visibility> for Visibility {
    fn get_val(&self, _: &impl DataContext) -> Visibility {
        *self
    }

    fn set_or_bind<F>(self, cx: &mut Context, entity: Entity, closure: F)
    where
        F: 'static + Fn(&mut Context, Self),
    {
        cx.with_current(entity, |cx| {
            (closure)(cx, self);
        });
    }
}

impl Res<Display> for Display {
    fn get_val(&self, _: &impl DataContext) -> Display {
        *self
    }

    fn set_or_bind<F>(self, cx: &mut Context, entity: Entity, closure: F)
    where
        F: 'static + Fn(&mut Context, Self),
    {
        cx.with_current(entity, |cx| {
            (closure)(cx, self);
        });
    }
}

impl Res<LayoutType> for LayoutType {
    fn get_val(&self, _: &impl DataContext) -> LayoutType {
        *self
    }

    fn set_or_bind<F>(self, cx: &mut Context, entity: Entity, closure: F)
    where
        F: 'static + Fn(&mut Context, Self),
    {
        cx.with_current(entity, |cx| {
            (closure)(cx, self);
        });
    }
}

impl Res<PositionType> for PositionType {
    fn get_val(&self, _: &impl DataContext) -> PositionType {
        *self
    }

    fn set_or_bind<F>(self, cx: &mut Context, entity: Entity, closure: F)
    where
        F: 'static + Fn(&mut Context, Self),
    {
        cx.with_current(entity, |cx| {
            (closure)(cx, self);
        });
    }
}

impl<T: Clone + Res<T>> Res<Option<T>> for Option<T> {
    fn get_val(&self, _: &impl DataContext) -> Option<T> {
        self.clone()
    }

    fn set_or_bind<F>(self, cx: &mut Context, entity: Entity, closure: F)
    where
        F: 'static + Clone + Fn(&mut Context, Option<T>),
    {
        cx.with_current(entity, |cx| (closure)(cx, self));
    }
}

impl Res<Length> for Length {
    fn get_val(&self, _: &impl DataContext) -> Self {
        self.clone()
    }

    fn set_or_bind<F>(self, cx: &mut Context, entity: Entity, closure: F)
    where
        F: 'static + Clone + Fn(&mut Context, Self),
    {
        cx.with_current(entity, |cx| (closure)(cx, self));
    }
}

impl Res<LengthOrPercentage> for LengthOrPercentage {
    fn get_val(&self, _: &impl DataContext) -> Self {
        self.clone()
    }

    fn set_or_bind<F>(self, cx: &mut Context, entity: Entity, closure: F)
    where
        F: 'static + Clone + Fn(&mut Context, Self),
    {
        cx.with_current(entity, |cx| (closure)(cx, self));
    }
}

impl Res<RGBA> for RGBA {
    fn get_val(&self, _: &impl DataContext) -> Self {
        *self
    }

    fn set_or_bind<F>(self, cx: &mut Context, entity: Entity, closure: F)
    where
        F: 'static + Clone + Fn(&mut Context, Self),
    {
        cx.with_current(entity, |cx| (closure)(cx, self));
    }
}

impl<T: Clone + Res<T>> Res<Vec<T>> for Vec<T> {
    fn get_val(&self, _: &impl DataContext) -> Vec<T> {
        self.clone()
    }

    fn set_or_bind<F>(self, cx: &mut Context, entity: Entity, closure: F)
    where
        F: 'static + Clone + Fn(&mut Context, Vec<T>),
    {
        cx.with_current(entity, |cx| (closure)(cx, self));
    }
}

impl<T: Clone + Res<T>, const N: usize> Res<[T; N]> for [T; N] {
    fn get_val(&self, _: &impl DataContext) -> Self {
        self.clone()
    }

    fn set_or_bind<F>(self, cx: &mut Context, entity: Entity, closure: F)
    where
        F: 'static + Clone + Fn(&mut Context, Self),
    {
        cx.with_current(entity, |cx| (closure)(cx, self));
    }
}

impl Res<FamilyOwned> for FamilyOwned {
    fn get_val(&self, _: &impl DataContext) -> FamilyOwned {
        self.clone()
    }

    fn set_or_bind<F>(self, cx: &mut Context, entity: Entity, closure: F)
    where
        F: 'static + Clone + Fn(&mut Context, FamilyOwned),
    {
        cx.with_current(entity, |cx| (closure)(cx, self));
    }
}

impl<T1: Clone, T2: Clone> Res<(T1, T2)> for (T1, T2) {
    fn get_val(&self, _cx: &impl DataContext) -> (T1, T2) {
        self.clone()
    }

    fn set_or_bind<F>(self, cx: &mut Context, entity: Entity, closure: F)
    where
        F: 'static + Clone + Fn(&mut Context, (T1, T2)),
    {
        cx.with_current(entity, |cx| {
            (closure)(cx, self);
        });
    }
}

impl<T1: Clone, T2: Clone, T3: Clone> Res<(T1, T2, T3)> for (T1, T2, T3) {
    fn get_val(&self, _cx: &impl DataContext) -> (T1, T2, T3) {
        self.clone()
    }

    fn set_or_bind<F>(self, cx: &mut Context, entity: Entity, closure: F)
    where
        F: 'static + Clone + Fn(&mut Context, (T1, T2, T3)),
    {
        cx.with_current(entity, |cx| {
            (closure)(cx, self);
        });
    }
}

impl<T1: Clone, T2: Clone, T3: Clone, T4: Clone> Res<(T1, T2, T3, T4)> for (T1, T2, T3, T4) {
    fn get_val(&self, _cx: &impl DataContext) -> (T1, T2, T3, T4) {
        self.clone()
    }

    fn set_or_bind<F>(self, cx: &mut Context, entity: Entity, closure: F)
    where
        F: 'static + Clone + Fn(&mut Context, (T1, T2, T3, T4)),
    {
        cx.with_current(entity, |cx| {
            (closure)(cx, self);
        });
    }
}
