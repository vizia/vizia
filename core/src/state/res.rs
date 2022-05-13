use crate::prelude::*;

macro_rules! impl_res_simple {
    ($t:ty) => {
        impl Res<$t> for $t {
            fn get_val(&self, _: &Context) -> Option<$t> {
                Some(*self)
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
    fn get_val(&self, cx: &Context) -> Option<T>;
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

impl<T, L> Res<T> for L
where
    L: Bindable<Output = T>,
    T: Clone + Data,
{
    fn get_val(&self, cx: &Context) -> Option<T> {
        self.get(cx)
    }

    fn set_or_bind<F>(&self, cx: &mut Context, entity: Entity, closure: F)
    where
        F: 'static + Fn(&mut Context, Entity, T),
    {
        cx.with_current(entity, |cx| {
            Binding::new(cx, self.clone(), move |cx, val| {
                if let Some(v) = val.get_val(cx) {
                    (closure)(cx, entity, v);
                }
            });
        });
    }
}

impl<'s> Res<&'s str> for &'s str {
    fn get_val(&self, _: &Context) -> Option<&'s str> {
        Some(self)
    }

    fn set_or_bind<F>(&self, cx: &mut Context, entity: Entity, closure: F)
    where
        F: 'static + Fn(&mut Context, Entity, Self),
    {
        (closure)(cx, entity, self);
    }
}

impl<'s> Res<&'s String> for &'s String {
    fn get_val(&self, _: &Context) -> Option<&'s String> {
        Some(self)
    }

    fn set_or_bind<F>(&self, cx: &mut Context, entity: Entity, closure: F)
    where
        F: 'static + Fn(&mut Context, Entity, Self),
    {
        (closure)(cx, entity, self);
    }
}

impl Res<Color> for Color {
    fn get_val(&self, _: &Context) -> Option<Color> {
        Some(*self)
    }

    fn set_or_bind<F>(&self, cx: &mut Context, entity: Entity, closure: F)
    where
        F: 'static + Fn(&mut Context, Entity, Self),
    {
        (closure)(cx, entity, *self);
    }
}

impl Res<Units> for Units {
    fn get_val(&self, _: &Context) -> Option<Units> {
        Some(*self)
    }

    fn set_or_bind<F>(&self, cx: &mut Context, entity: Entity, closure: F)
    where
        F: 'static + Fn(&mut Context, Entity, Self),
    {
        (closure)(cx, entity, *self);
    }
}

impl Res<Visibility> for Visibility {
    fn get_val(&self, _: &Context) -> Option<Visibility> {
        Some(*self)
    }

    fn set_or_bind<F>(&self, cx: &mut Context, entity: Entity, closure: F)
    where
        F: 'static + Fn(&mut Context, Entity, Self),
    {
        (closure)(cx, entity, *self);
    }
}

impl Res<Display> for Display {
    fn get_val(&self, _: &Context) -> Option<Display> {
        Some(*self)
    }

    fn set_or_bind<F>(&self, cx: &mut Context, entity: Entity, closure: F)
    where
        F: 'static + Fn(&mut Context, Entity, Self),
    {
        (closure)(cx, entity, *self);
    }
}

impl Res<LayoutType> for LayoutType {
    fn get_val(&self, _: &Context) -> Option<LayoutType> {
        Some(*self)
    }

    fn set_or_bind<F>(&self, cx: &mut Context, entity: Entity, closure: F)
    where
        F: 'static + Fn(&mut Context, Entity, Self),
    {
        (closure)(cx, entity, *self);
    }
}

impl Res<PositionType> for PositionType {
    fn get_val(&self, _: &Context) -> Option<PositionType> {
        Some(*self)
    }

    fn set_or_bind<F>(&self, cx: &mut Context, entity: Entity, closure: F)
    where
        F: 'static + Fn(&mut Context, Entity, Self),
    {
        (closure)(cx, entity, *self);
    }
}

impl<T: Copy> Res<(T, T)> for (T, T) {
    fn get_val(&self, _: &Context) -> Option<(T, T)> {
        Some(*self)
    }

    fn set_or_bind<F>(&self, cx: &mut Context, entity: Entity, closure: F)
    where
        F: 'static + Fn(&mut Context, Entity, Self),
    {
        (closure)(cx, entity, *self);
    }
}
