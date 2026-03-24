use crate::prelude::*;
use vizia_reactive::{
    DerivedSignal, Memo, ReadSignal, Signal, SignalGet, SyncDerivedSignal, SyncReadSignal,
    SyncSignal,
};

#[macro_export]
/// A macro for implementing the [Res] trait for simple `Copy` types.
macro_rules! impl_res_simple {
    ($t:ty) => {
        impl Res<$t> for $t {
            fn get_value(&self, _: &impl DataContext) -> $t {
                *self
            }
        }
    };
}

#[macro_export]
/// A macro for implementing the [Res] trait for `Clone` types.
macro_rules! impl_res_clone {
    ($t:ty) => {
        impl Res<$t> for $t {
            fn get_value(&self, _: &impl DataContext) -> $t {
                self.clone()
            }
        }
    };
}

/// A trait which allows passing a value or reactive source to a view or modifier.
///
/// For example, the `Label` view constructor takes a type which implements `Res<T>` where
/// `T` implements `ToString`. This allows the user to pass a type which implements `ToString`,
/// such as `String` or `&str`, or a signal/resource producing a type which implements `ToString`.
pub trait Res<T> {
    /// Returns the value of a resource by value.
    fn get_value(&self, _: &impl DataContext) -> T;

    fn set_or_bind<F>(self, cx: &mut Context, closure: F)
    where
        Self: Sized,
        F: 'static + Fn(&mut Context, Self),
    {
        (closure)(cx, self);
    }

    /// Converts this signal into a signal mirror tied to `entity`.
    ///
    /// Reactive signals keep the returned signal updated over time,
    /// while non-reactive values initialize it once.
    fn to_signal(self, cx: &mut Context) -> Signal<T>
    where
        Self: Sized + 'static,
        T: Clone + 'static,
    {
        let signal = Signal::new(self.get_value(cx));
        self.set_or_bind(cx, move |cx, res| {
            signal.set(res.get_value(cx));
        });
        signal
    }
}

impl<T: Clone + 'static> Res<T> for Signal<T> {
    fn get_value(&self, _: &impl DataContext) -> T {
        SignalGet::get(self)
    }

    fn set_or_bind<F>(self, cx: &mut Context, closure: F)
    where
        F: 'static + Fn(&mut Context, Self),
    {
        Binding::new(cx, self, move |cx| {
            (closure)(cx, self);
        });
    }

    fn to_signal(self, _cx: &mut Context) -> Signal<T>
    where
        Self: Sized + 'static,
        T: Clone + 'static,
    {
        self
    }
}

impl<T: Clone + Send + Sync + 'static> Res<T> for SyncSignal<T> {
    fn get_value(&self, _: &impl DataContext) -> T {
        SignalGet::get(self)
    }

    fn set_or_bind<F>(self, cx: &mut Context, closure: F)
    where
        F: 'static + Fn(&mut Context, Self),
    {
        Binding::new(cx, self, move |cx| {
            (closure)(cx, self);
        });
    }
}

impl<T: Clone + 'static> Res<T> for ReadSignal<T> {
    fn get_value(&self, _: &impl DataContext) -> T {
        SignalGet::get(self)
    }

    fn set_or_bind<F>(self, cx: &mut Context, closure: F)
    where
        F: 'static + Fn(&mut Context, Self),
    {
        Binding::new(cx, self, move |cx| {
            (closure)(cx, self);
        });
    }
}

impl<T: Clone + Send + Sync + 'static> Res<T> for SyncReadSignal<T> {
    fn get_value(&self, _: &impl DataContext) -> T {
        SignalGet::get(self)
    }

    fn set_or_bind<F>(self, cx: &mut Context, closure: F)
    where
        F: 'static + Fn(&mut Context, Self),
    {
        Binding::new(cx, self, move |cx| {
            (closure)(cx, self);
        });
    }
}

impl<T: Clone + PartialEq + 'static> Res<T> for Memo<T> {
    fn get_value(&self, _: &impl DataContext) -> T {
        SignalGet::get(self)
    }

    fn set_or_bind<F>(self, cx: &mut Context, closure: F)
    where
        F: 'static + Fn(&mut Context, Self),
    {
        Binding::new(cx, self, move |cx| {
            (closure)(cx, self);
        });
    }
}

impl<T, O, GF, UF> Res<O> for DerivedSignal<T, O, GF, UF>
where
    T: Clone + 'static,
    O: Clone + 'static,
    GF: Fn(&T) -> O + Copy + 'static,
    UF: Fn(&O) -> T + Copy + 'static,
{
    fn get_value(&self, _: &impl DataContext) -> O {
        SignalGet::get(self)
    }

    fn set_or_bind<F>(self, cx: &mut Context, closure: F)
    where
        F: 'static + Fn(&mut Context, Self),
    {
        Binding::new(cx, self, move |cx| {
            (closure)(cx, self);
        });
    }
}

impl<T, O, GF, UF> Res<O> for SyncDerivedSignal<T, O, GF, UF>
where
    T: Clone + Send + Sync + 'static,
    O: Clone + Send + Sync + 'static,
    GF: Fn(&T) -> O + Copy + Send + Sync + 'static,
    UF: Fn(&O) -> T + Copy + Send + Sync + 'static,
{
    fn get_value(&self, _: &impl DataContext) -> O {
        SignalGet::get(self)
    }

    fn set_or_bind<F>(self, cx: &mut Context, closure: F)
    where
        F: 'static + Fn(&mut Context, Self),
    {
        Binding::new(cx, self, move |cx| {
            (closure)(cx, self);
        });
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
impl_res_simple!(LengthValue);
impl_res_simple!(FontWeight);
impl_res_simple!(FontWeightKeyword);
impl_res_simple!(FontSlant);
impl_res_simple!(CornerShape);
impl_res_simple!(Angle);
impl_res_simple!(TextAlign);
impl_res_simple!(TextOverflow);
impl_res_simple!(LineClamp);
impl_res_clone!(Shadow);
impl_res_clone!(LinearGradientBuilder);
impl_res_clone!(ShadowBuilder);
impl_res_simple!(FontVariation);
impl_res_clone!(Filter);
impl_res_simple!(Opacity);
impl_res_simple!(FontWidth);
impl_res_clone!(Translate);
impl_res_clone!(Scale);
impl_res_clone!(Position);
impl_res_simple!(PointerEvents);
impl_res_simple!(ButtonVariant);
impl_res_simple!(AvatarVariant);
impl_res_clone!(FamilyOwned);
impl_res_simple!(TextDecorationLine);
impl_res_clone!(TextStroke);
impl_res_clone!(TextStrokeStyle);
impl_res_simple!(Alignment);
impl_res_simple!(WindowPosition);
impl_res_simple!(Anchor);
impl_res_simple!(AnchorTarget);
impl_res_clone!(std::ops::Range<f32>);

impl<'i> Res<FontFamily<'i>> for FontFamily<'i> {
    fn get_value(&self, _: &impl DataContext) -> Self {
        self.clone()
    }
}

impl<'i> Res<BackgroundImage<'i>> for BackgroundImage<'i> {
    fn get_value(&self, _: &impl DataContext) -> Self {
        self.clone()
    }
}

impl<'s> Res<&'s str> for &'s str {
    fn get_value(&self, _: &impl DataContext) -> &'s str {
        self
    }
}

impl<'s> Res<&'s String> for &'s String {
    fn get_value(&self, _: &impl DataContext) -> Self {
        self
    }
}

impl Res<String> for String {
    fn get_value(&self, _: &impl DataContext) -> Self {
        self.clone()
    }
}

impl Res<Transform> for Transform {
    fn get_value(&self, _: &impl DataContext) -> Transform {
        self.clone()
    }
}

impl Res<Color> for Color {
    fn get_value(&self, _: &impl DataContext) -> Color {
        *self
    }
}

impl Res<LinearGradient> for LinearGradient {
    fn get_value(&self, _: &impl DataContext) -> LinearGradient {
        self.clone()
    }
}

impl Res<Units> for Units {
    fn get_value(&self, _: &impl DataContext) -> Units {
        *self
    }
}

impl Res<Visibility> for Visibility {
    fn get_value(&self, _: &impl DataContext) -> Visibility {
        *self
    }
}

impl Res<Display> for Display {
    fn get_value(&self, _: &impl DataContext) -> Display {
        *self
    }
}

impl Res<LayoutType> for LayoutType {
    fn get_value(&self, _: &impl DataContext) -> LayoutType {
        *self
    }
}

impl Res<PositionType> for PositionType {
    fn get_value(&self, _: &impl DataContext) -> PositionType {
        *self
    }
}

impl<T: Clone + Res<T>> Res<Option<T>> for Option<T> {
    fn get_value(&self, _: &impl DataContext) -> Option<T> {
        self.clone()
    }
}

impl Res<Length> for Length {
    fn get_value(&self, _: &impl DataContext) -> Self {
        self.clone()
    }
}

impl Res<LengthOrPercentage> for LengthOrPercentage {
    fn get_value(&self, _: &impl DataContext) -> Self {
        self.clone()
    }
}

impl Res<RGBA> for RGBA {
    fn get_value(&self, _: &impl DataContext) -> Self {
        *self
    }
}

impl<T: Clone + Res<T>> Res<Vec<T>> for Vec<T> {
    fn get_value(&self, _: &impl DataContext) -> Vec<T> {
        self.clone()
    }
}

impl<T: Clone + Res<T>, const N: usize> Res<[T; N]> for [T; N] {
    fn get_value(&self, _: &impl DataContext) -> Self {
        self.clone()
    }
}

impl<T1: Clone, T2: Clone> Res<(T1, T2)> for (T1, T2) {
    fn get_value(&self, _cx: &impl DataContext) -> (T1, T2) {
        self.clone()
    }
}

impl<T1: Clone, T2: Clone, T3: Clone> Res<(T1, T2, T3)> for (T1, T2, T3) {
    fn get_value(&self, _cx: &impl DataContext) -> (T1, T2, T3) {
        self.clone()
    }
}

impl<T1: Clone, T2: Clone, T3: Clone, T4: Clone> Res<(T1, T2, T3, T4)> for (T1, T2, T3, T4) {
    fn get_value(&self, _cx: &impl DataContext) -> (T1, T2, T3, T4) {
        self.clone()
    }
}
