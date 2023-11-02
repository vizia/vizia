use std::marker::PhantomData;

use vizia_style::{BoxShadow, FontStretch, FontStyle, FontWeight, FontWeightKeyword};

use crate::{
    modifiers::{BoxShadowBuilder, LinearGradientBuilder},
    prelude::*,
};

use super::Map;

macro_rules! impl_res_simple {
    ($t:ty) => {
        impl Res<$t> for $t {
            type M<O> = ResMap<$t, Self, O> where O: Data;
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

            fn map_res<F, O: Data>(self, mapping: F) -> Self::M<O>
            where
                F: 'static + Fn(&$t) -> O,
            {
                ResMap { val: self, o: PhantomData::default(), closure: Box::new(mapping) }
            }
        }
    };
}

macro_rules! impl_res_clone {
    ($t:ty) => {
        impl Res<$t> for $t {
            type M<O> = ResMap<$t, Self, O> where O: Data;
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

            fn map_res<F, O: Data>(self, mapping: F) -> Self::M<O>
            where
                F: 'static + Fn(&$t) -> O,
            {
                ResMap { val: self, o: PhantomData::default(), closure: Box::new(mapping) }
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
    type M<O>: Res<O>
    where
        O: Data;
    #[allow(unused_variables)]
    fn get<'a>(&'a self, cx: &'a impl DataContext) -> Option<LensValue<'a, T>> {
        None
    }

    fn get_val(&self, _: &impl DataContext) -> T;

    fn set_or_bind<F>(self, cx: &mut Context, entity: Entity, closure: F)
    where
        F: 'static + Clone + Fn(&mut Context, Self);

    fn map_res<F, O: Data>(self, mapping: F) -> Self::M<O>
    where
        F: 'static + Fn(&T) -> O;
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

pub struct ResMap<T, R: Res<T>, O> {
    pub val: R,
    pub o: PhantomData<O>,
    pub closure: Box<dyn Fn(&T) -> O>,
}

impl<T, O, R: Res<T>> Res<O> for ResMap<T, R, O> {
    type M<P> = ResMap<O, Self, P> where P: Data;
    fn get<'a>(&'a self, cx: &'a impl DataContext) -> Option<LensValue<'a, O>> {
        Some(LensValue::Owned((self.closure)(&self.val.get_val(cx))))
    }

    fn get_val(&self, cx: &impl DataContext) -> O {
        (self.closure)(&self.val.get_val(cx))
    }

    fn set_or_bind<F>(self, cx: &mut Context, entity: Entity, closure: F)
    where
        F: 'static + Clone + Fn(&mut Context, Self),
    {
        cx.with_current(entity, |cx| {
            (closure)(cx, self);
        });
    }

    fn map_res<F, P: Data>(self, mapping: F) -> Self::M<P>
    where
        F: 'static + Fn(&O) -> P,
    {
        ResMap { val: self, o: PhantomData, closure: Box::new(mapping) }
    }
}

impl<L> Res<L::Target> for L
where
    L: Lens,
    L::Target: Data,
{
    type M<O> = Map<L, O> where O: Data;

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

    fn map_res<F, O: Data>(self, mapping: F) -> Self::M<O>
    where
        F: 'static + Fn(&L::Target) -> O,
    {
        LensExt::map(self, move |t| (mapping)(t))
    }
}

impl<'i> Res<FontFamily<'i>> for FontFamily<'i> {
    type M<O> = ResMap<Self, Self, O> where O: Data;
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

    fn map_res<F, O: Data>(self, mapping: F) -> Self::M<O>
    where
        F: 'static + Fn(&Self) -> O,
    {
        ResMap { val: self, o: PhantomData, closure: Box::new(mapping) }
    }
}

impl<'i> Res<BackgroundImage<'i>> for BackgroundImage<'i> {
    type M<O> = ResMap<Self, Self, O> where O: Data;
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

    fn map_res<F, O: Data>(self, mapping: F) -> Self::M<O>
    where
        F: 'static + Fn(&Self) -> O,
    {
        ResMap { val: self, o: PhantomData, closure: Box::new(mapping) }
    }
}

impl<'s> Res<&'s str> for &'s str {
    type M<O> = ResMap<Self, Self, O> where O: Data;
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

    fn map_res<F, O: Data>(self, mapping: F) -> Self::M<O>
    where
        F: 'static + Fn(&Self) -> O,
    {
        ResMap { val: self, o: PhantomData, closure: Box::new(mapping) }
    }
}

impl<'s> Res<&'s String> for &'s String {
    type M<O> = ResMap<Self, Self, O> where O: Data;
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

    fn map_res<F, O: Data>(self, mapping: F) -> Self::M<O>
    where
        F: 'static + Fn(&Self) -> O,
    {
        ResMap { val: self, o: PhantomData, closure: Box::new(mapping) }
    }
}

impl Res<String> for String {
    type M<O> = ResMap<Self, Self, O> where O: Data;
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

    fn map_res<F, O: Data>(self, mapping: F) -> Self::M<O>
    where
        F: 'static + Fn(&Self) -> O,
    {
        ResMap { val: self, o: PhantomData, closure: Box::new(mapping) }
    }
}

impl Res<Transform> for Transform {
    type M<O> = ResMap<Self, Self, O> where O: Data;
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

    fn map_res<F, O: Data>(self, mapping: F) -> Self::M<O>
    where
        F: 'static + Fn(&Self) -> O,
    {
        ResMap { val: self, o: PhantomData, closure: Box::new(mapping) }
    }
}

impl Res<Color> for Color {
    type M<O> = ResMap<Self, Self, O> where O: Data;

    fn get<'a>(&'a self, _: &'a impl DataContext) -> Option<LensValue<'a, Color>> {
        Some(LensValue::Borrowed(self))
    }

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

    fn map_res<F, O: Data>(self, mapping: F) -> Self::M<O>
    where
        F: 'static + Fn(&Self) -> O,
    {
        ResMap { val: self, o: PhantomData, closure: Box::new(mapping) }
    }
}

impl Res<LinearGradient> for LinearGradient {
    type M<O> = ResMap<Self, Self, O> where O: Data;
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

    fn map_res<F, O: Data>(self, mapping: F) -> Self::M<O>
    where
        F: 'static + Fn(&Self) -> O,
    {
        ResMap { val: self, o: PhantomData, closure: Box::new(mapping) }
    }
}

impl Res<Units> for Units {
    type M<O> = ResMap<Self, Self, O> where O: Data;
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

    fn map_res<F, O: Data>(self, mapping: F) -> Self::M<O>
    where
        F: 'static + Fn(&Self) -> O,
    {
        ResMap { val: self, o: PhantomData, closure: Box::new(mapping) }
    }
}

impl Res<Visibility> for Visibility {
    type M<O> = ResMap<Self, Self, O> where O: Data;
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

    fn map_res<F, O: Data>(self, mapping: F) -> Self::M<O>
    where
        F: 'static + Fn(&Self) -> O,
    {
        ResMap { val: self, o: PhantomData, closure: Box::new(mapping) }
    }
}

impl Res<Display> for Display {
    type M<O> = ResMap<Self, Self, O> where O: Data;
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

    fn map_res<F, O: Data>(self, mapping: F) -> Self::M<O>
    where
        F: 'static + Fn(&Self) -> O,
    {
        ResMap { val: self, o: PhantomData, closure: Box::new(mapping) }
    }
}

impl Res<LayoutType> for LayoutType {
    type M<O> = ResMap<Self, Self, O> where O: Data;
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

    fn map_res<F, O: Data>(self, mapping: F) -> Self::M<O>
    where
        F: 'static + Fn(&Self) -> O,
    {
        ResMap { val: self, o: PhantomData, closure: Box::new(mapping) }
    }
}

impl Res<PositionType> for PositionType {
    type M<O> = ResMap<Self, Self, O> where O: Data;
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

    fn map_res<F, O: Data>(self, mapping: F) -> Self::M<O>
    where
        F: 'static + Fn(&Self) -> O,
    {
        ResMap { val: self, o: PhantomData, closure: Box::new(mapping) }
    }
}

impl<T: Clone + Res<T>> Res<Option<T>> for Option<T> {
    type M<O> = ResMap<Self, Self, O> where O: Data;
    fn get_val(&self, _: &impl DataContext) -> Option<T> {
        self.clone()
    }

    fn set_or_bind<F>(self, cx: &mut Context, entity: Entity, closure: F)
    where
        F: 'static + Clone + Fn(&mut Context, Option<T>),
    {
        cx.with_current(entity, |cx| (closure)(cx, self));
    }

    fn map_res<F, O: Data>(self, mapping: F) -> Self::M<O>
    where
        F: 'static + Fn(&Self) -> O,
    {
        ResMap { val: self, o: PhantomData, closure: Box::new(mapping) }
    }
}

impl Res<Length> for Length {
    type M<O> = ResMap<Self, Self, O> where O: Data;
    fn get_val(&self, _: &impl DataContext) -> Self {
        self.clone()
    }

    fn set_or_bind<F>(self, cx: &mut Context, entity: Entity, closure: F)
    where
        F: 'static + Clone + Fn(&mut Context, Self),
    {
        cx.with_current(entity, |cx| (closure)(cx, self));
    }

    fn map_res<F, O: Data>(self, mapping: F) -> Self::M<O>
    where
        F: 'static + Fn(&Self) -> O,
    {
        ResMap { val: self, o: PhantomData, closure: Box::new(mapping) }
    }
}

impl Res<LengthOrPercentage> for LengthOrPercentage {
    type M<O> = ResMap<Self, Self, O> where O: Data;
    fn get_val(&self, _: &impl DataContext) -> Self {
        self.clone()
    }

    fn set_or_bind<F>(self, cx: &mut Context, entity: Entity, closure: F)
    where
        F: 'static + Clone + Fn(&mut Context, Self),
    {
        cx.with_current(entity, |cx| (closure)(cx, self));
    }

    fn map_res<F, O: Data>(self, mapping: F) -> Self::M<O>
    where
        F: 'static + Fn(&Self) -> O,
    {
        ResMap { val: self, o: PhantomData, closure: Box::new(mapping) }
    }
}

impl Res<RGBA> for RGBA {
    type M<O> = ResMap<Self, Self, O> where O: Data;
    fn get_val(&self, _: &impl DataContext) -> Self {
        *self
    }

    fn set_or_bind<F>(self, cx: &mut Context, entity: Entity, closure: F)
    where
        F: 'static + Clone + Fn(&mut Context, Self),
    {
        cx.with_current(entity, |cx| (closure)(cx, self));
    }

    fn map_res<F, O: Data>(self, mapping: F) -> Self::M<O>
    where
        F: 'static + Fn(&Self) -> O,
    {
        ResMap { val: self, o: PhantomData, closure: Box::new(mapping) }
    }
}

impl<T: Clone + Res<T>> Res<Vec<T>> for Vec<T> {
    type M<O> = ResMap<Self, Self, O> where O: Data;
    fn get_val(&self, _: &impl DataContext) -> Vec<T> {
        self.clone()
    }

    fn set_or_bind<F>(self, cx: &mut Context, entity: Entity, closure: F)
    where
        F: 'static + Clone + Fn(&mut Context, Vec<T>),
    {
        cx.with_current(entity, |cx| (closure)(cx, self));
    }

    fn map_res<F, O: Data>(self, mapping: F) -> Self::M<O>
    where
        F: 'static + Fn(&Self) -> O,
    {
        ResMap { val: self, o: PhantomData, closure: Box::new(mapping) }
    }
}

impl<T: Clone + Res<T>, const N: usize> Res<[T; N]> for [T; N] {
    type M<O> = ResMap<Self, Self, O> where O: Data;
    fn get_val(&self, _: &impl DataContext) -> Self {
        self.clone()
    }

    fn set_or_bind<F>(self, cx: &mut Context, entity: Entity, closure: F)
    where
        F: 'static + Clone + Fn(&mut Context, Self),
    {
        cx.with_current(entity, |cx| (closure)(cx, self));
    }

    fn map_res<F, O: Data>(self, mapping: F) -> Self::M<O>
    where
        F: 'static + Fn(&Self) -> O,
    {
        ResMap { val: self, o: PhantomData, closure: Box::new(mapping) }
    }
}

impl Res<FamilyOwned> for FamilyOwned {
    type M<O> = ResMap<Self, Self, O> where O: Data;
    fn get_val(&self, _: &impl DataContext) -> FamilyOwned {
        self.clone()
    }

    fn set_or_bind<F>(self, cx: &mut Context, entity: Entity, closure: F)
    where
        F: 'static + Clone + Fn(&mut Context, FamilyOwned),
    {
        cx.with_current(entity, |cx| (closure)(cx, self));
    }

    fn map_res<F, O: Data>(self, mapping: F) -> Self::M<O>
    where
        F: 'static + Fn(&Self) -> O,
    {
        ResMap { val: self, o: PhantomData, closure: Box::new(mapping) }
    }
}

impl<T1: Clone, T2: Clone> Res<(T1, T2)> for (T1, T2) {
    type M<O> = ResMap<Self, Self, O> where O: Data;
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

    fn map_res<F, O: Data>(self, mapping: F) -> Self::M<O>
    where
        F: 'static + Fn(&Self) -> O,
    {
        ResMap { val: self, o: PhantomData, closure: Box::new(mapping) }
    }
}

impl<T1: Clone, T2: Clone, T3: Clone> Res<(T1, T2, T3)> for (T1, T2, T3) {
    type M<O> = ResMap<Self, Self, O> where O: Data;
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

    fn map_res<F, O: Data>(self, mapping: F) -> Self::M<O>
    where
        F: 'static + Fn(&Self) -> O,
    {
        ResMap { val: self, o: PhantomData, closure: Box::new(mapping) }
    }
}

impl<T1: Clone, T2: Clone, T3: Clone, T4: Clone> Res<(T1, T2, T3, T4)> for (T1, T2, T3, T4) {
    type M<O> = ResMap<Self, Self, O> where O: Data;
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

    fn map_res<F, O: Data>(self, mapping: F) -> Self::M<O>
    where
        F: 'static + Fn(&Self) -> O,
    {
        ResMap { val: self, o: PhantomData, closure: Box::new(mapping) }
    }
}
