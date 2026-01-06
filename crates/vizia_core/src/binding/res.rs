use super::Binding;
use crate::context::{Context, DataContext};
use crate::entity::Entity;
use crate::recoil::Signal;
use chrono::{NaiveDate, NaiveTime};
use morphorm::{LayoutType, PositionType, Units};
use std::ops::Range;
use vizia_style::{
    Alignment, Angle, BackgroundImage, BackgroundSize, BorderStyleKeyword, ClipPath, Color,
    CornerRadius, CornerShape, CursorIcon, Display, Filter, FontFamily, FontSize, FontSlant,
    FontVariation, FontWeight, FontWeightKeyword, FontWidth, Gradient, Length, LengthOrPercentage,
    LengthValue, LineClamp, LinearGradient, Opacity, Overflow, PointerEvents, Position, Rect,
    Scale, Shadow, TextAlign, TextDecorationLine, TextOverflow, TextStroke, TextStrokeStyle,
    Transform, Translate, Visibility, RGBA,
};
use vizia_window::{Anchor, AnchorTarget, WindowPosition, WindowSize};
use crate::views::Placement;

#[macro_export]
/// A macro for implementing the [Res] trait for simple `Copy` types.
macro_rules! impl_res_simple {
    ($t:ty) => {
        impl $crate::binding::Res<$t> for $t {
            fn resolve(&self, _: &impl $crate::context::DataContext) -> $t {
                *self
            }
        }
    };
}

#[macro_export]
/// A macro for implementing the [Res] trait for `Clone` types.
macro_rules! impl_res_clone {
    ($t:ty) => {
        impl $crate::binding::Res<$t> for $t {
            fn resolve(&self, _: &impl $crate::context::DataContext) -> $t {
                self.clone()
            }
        }
    };
}

/// A trait which allows passing a value or a signal to a view or modifier.
pub trait Res<T>: Sized {
    /// Returns the current value.
    fn resolve(&self, cx: &impl DataContext) -> T;

    /// Either set immediately (value) or bind to changes (signal).
    fn set_or_bind<F>(self, cx: &mut Context, entity: Entity, closure: F)
    where
        F: 'static + Fn(&mut Context, Self),
    {
        cx.with_current(entity, |cx| {
            (closure)(cx, self);
        });
    }

    /// Converts the resource into a signal. Values become constant signals.
    fn into_signal(self, cx: &mut Context) -> Signal<T>
    where
        T: Clone + 'static,
    {
        cx.state(self.resolve(cx))
    }
}

impl<T: Clone + 'static> Res<T> for Signal<T> {
    fn resolve(&self, cx: &impl DataContext) -> T {
        Signal::get(self, cx).clone()
    }

    fn set_or_bind<F>(self, cx: &mut Context, entity: Entity, closure: F)
    where
        F: 'static + Fn(&mut Context, Self),
    {
        let signal = self;
        Binding::new(cx, signal, move |cx| {
            cx.with_current(entity, |cx| {
                (closure)(cx, signal);
            });
        });
    }

    fn into_signal(self, _cx: &mut Context) -> Signal<T> {
        self
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

impl_res_clone!(CursorIcon);
impl_res_clone!(Overflow);
impl_res_clone!(LengthValue);
impl_res_clone!(FontWeight);
impl_res_clone!(FontWeightKeyword);
impl_res_clone!(FontSlant);
impl_res_clone!(CornerShape);
impl_res_clone!(Angle);
impl_res_clone!(TextAlign);
impl_res_clone!(TextOverflow);
impl_res_clone!(LineClamp);
impl_res_clone!(FontSize);
impl_res_clone!(FontVariation);
impl_res_clone!(Filter);
impl_res_clone!(Opacity);
impl_res_clone!(FontWidth);
impl_res_clone!(Translate);
impl_res_clone!(Scale);
impl_res_clone!(Position);
impl_res_clone!(PointerEvents);
impl_res_clone!(Alignment);
impl_res_clone!(TextDecorationLine);
impl_res_clone!(TextStroke);
impl_res_clone!(TextStrokeStyle);
impl_res_clone!(Color);
impl_res_clone!(LinearGradient);
impl_res_clone!(Gradient);
impl_res_clone!(Shadow);
impl_res_clone!(Transform);
impl_res_clone!(LayoutType);
impl_res_clone!(PositionType);
impl_res_clone!(Units);
impl_res_clone!(Visibility);
impl_res_clone!(Display);
impl_res_clone!(Length);
impl_res_clone!(LengthOrPercentage);
impl_res_clone!(RGBA);
impl_res_clone!(CornerRadius);
impl_res_clone!(BackgroundSize);
impl_res_clone!(ClipPath);
impl_res_clone!(BorderStyleKeyword);
impl_res_clone!(WindowPosition);
impl_res_clone!(WindowSize);
impl_res_clone!(Anchor);
impl_res_clone!(AnchorTarget);
impl_res_clone!(Range<f32>);
impl_res_clone!(NaiveDate);
impl_res_clone!(NaiveTime);
impl_res_simple!(Placement);

impl<'i> Res<FontFamily<'i>> for FontFamily<'i> {
    fn resolve(&self, _: &impl DataContext) -> Self {
        self.clone()
    }
}

impl<'i> Res<BackgroundImage<'i>> for BackgroundImage<'i> {
    fn resolve(&self, _: &impl DataContext) -> Self {
        self.clone()
    }
}

impl<'s> Res<&'s str> for &'s str {
    fn resolve(&self, _: &impl DataContext) -> &'s str {
        self
    }
}

impl<'s> Res<&'s String> for &'s String {
    fn resolve(&self, _: &impl DataContext) -> &'s String {
        self
    }
}

impl Res<String> for String {
    fn resolve(&self, _: &impl DataContext) -> String {
        self.clone()
    }
}

impl<T: Clone> Res<Option<T>> for Option<T> {
    fn resolve(&self, _: &impl DataContext) -> Option<T> {
        self.clone()
    }
}

impl<T: Clone> Res<Vec<T>> for Vec<T> {
    fn resolve(&self, _: &impl DataContext) -> Vec<T> {
        self.clone()
    }
}

impl<T: Clone, const N: usize> Res<[T; N]> for [T; N] {
    fn resolve(&self, _: &impl DataContext) -> [T; N] {
        self.clone()
    }
}

impl<T: Clone> Res<Rect<T>> for Rect<T> {
    fn resolve(&self, _: &impl DataContext) -> Rect<T> {
        self.clone()
    }
}

impl<T1: Clone, T2: Clone> Res<(T1, T2)> for (T1, T2) {
    fn resolve(&self, _: &impl DataContext) -> (T1, T2) {
        self.clone()
    }
}

impl<T1: Clone, T2: Clone, T3: Clone> Res<(T1, T2, T3)> for (T1, T2, T3) {
    fn resolve(&self, _: &impl DataContext) -> (T1, T2, T3) {
        self.clone()
    }
}

impl<T1: Clone, T2: Clone, T3: Clone, T4: Clone> Res<(T1, T2, T3, T4)> for (T1, T2, T3, T4) {
    fn resolve(&self, _: &impl DataContext) -> (T1, T2, T3, T4) {
        self.clone()
    }
}
