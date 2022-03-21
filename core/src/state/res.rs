use crate::{
    Binding, Color, Context, Data, Display, Entity, LayoutType, Lens, LensExt, PositionType, Units,
    Visibility,
};

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

impl<T, L> Res<T> for L
where
    L: Lens<Target = T> + LensExt,
    T: Clone + Data,
{
    fn get_val(&self, cx: &Context) -> T {
        self.get(cx).take()
    }

    fn get_val_fallible(&self, cx: &Context) -> Option<T> {
        self.get_fallible(cx).map(|x| x.take())
    }

    fn set_or_bind<F>(&self, cx: &mut Context, entity: Entity, closure: F)
    where
        F: 'static + Fn(&mut Context, Entity, T),
    {
        let prev_current = cx.current;
        cx.current = entity;
        Binding::new(cx, self.clone(), move |cx, val| {
            if let Some(v) = val.get_val_fallible(cx) {
                (closure)(cx, entity, v);
            }
        });
        cx.current = prev_current;
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
