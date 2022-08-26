use std::any::TypeId;
use std::fmt::{Debug, Formatter};
use std::marker::PhantomData;
use std::ops::Deref;

use crate::prelude::*;

use super::{next_uuid, StoreId};

/// A Lens allows the construction of a reference to a piece of some data, e.g. a field of a struct.
///
/// When deriving the `Lens` trait on a struct, the derive macro constructs a static type which implements the `Lens` trait for each field.
/// The `view()` method takes a reference to the struct type as input and outputs a reference to the field.
/// This provides a way to specify a binding to a specific field of some application data.
pub trait Lens: 'static + Clone {
    type Source;
    type Target;

    fn view<O, F: FnOnce(Option<&Self::Target>) -> O>(&self, source: &Self::Source, map: F) -> O;

    fn name(&self) -> Option<&'static str> {
        None
    }
}

pub(crate) trait LensCache: Lens {
    fn cache_key(&self) -> StoreId {
        if std::mem::size_of::<Self>() == 0 {
            StoreId::Type(TypeId::of::<Self>())
        } else {
            StoreId::UUID(next_uuid())
        }
    }
}

impl<T: Lens> LensCache for T {}

/// Helpers for constructing more complex `Lens`es.
pub trait LensExt: Lens {
    /// Retrieve a copy of the lensed data from context.
    ///
    /// Example
    /// ```ignore
    /// let value = lens.get(cx);
    /// ```
    fn get<C: DataContext>(&self, cx: &C) -> Self::Target
    where
        Self::Target: Clone,
    {
        self.view(
            cx.data().expect("Failed to get data from context. Has it been built into the tree?"),
            |t| {
                t.expect("Lens failed to resolve. Do you want to use LensExt::get_fallible?")
                    .clone()
            },
        )
    }

    fn get_fallible<C: DataContext>(&self, cx: &C) -> Option<Self::Target>
    where
        Self::Target: Clone,
    {
        self.view(
            cx.data().expect("Failed to get data from context. Has it been built into the tree?"),
            |t| t.cloned().map(|v| v),
        )
    }

    /// Used to construct a lens to some data contained within some other lensed data.
    ///
    /// # Example
    /// Binds a label to `other_data`, which is a field of a struct `SomeData`, which is a field of the root `AppData` model:
    /// ```compile_fail
    /// Binding::new(cx, AppData::some_data.then(SomeData::other_data), |cx, data|{
    ///
    /// });
    /// ```
    fn then<Other>(self, other: Other) -> Then<Self, Other>
    where
        Other: Lens<Source = Self::Target>,
    {
        Then::new(self, other)
    }

    fn index<T>(self, index: usize) -> Then<Self, Index<Self::Target, T>>
    where
        T: 'static,
        Self::Target: Deref<Target = [T]>,
    {
        self.then(Index::new(index))
    }

    fn map<G: Clone, B: 'static + Clone>(self, get: G) -> Then<Self, Map<G, Self::Target, B>>
    where
        G: 'static + Fn(&Self::Target) -> B,
    {
        self.then(Map::new(get))
    }

    fn unwrap<T: 'static>(self) -> Then<Self, UnwrapLens<T>>
    where
        Self: Lens<Target = Option<T>>,
    {
        self.then(UnwrapLens::new())
    }

    fn into_lens<T: 'static>(self) -> Then<Self, IntoLens<Self::Target, T>>
    where
        Self::Target: Clone + Into<T>,
    {
        self.then(IntoLens::new())
    }
}

// Implement LensExt for all types which implement Lens
impl<T: Lens> LensExt for T {}

pub struct Map<G, I, O> {
    get: G,
    i: PhantomData<I>,
    o: PhantomData<O>,
}

impl<G: Clone, I, O> Clone for Map<G, I, O> {
    fn clone(&self) -> Self {
        Map { get: self.get.clone(), i: PhantomData::default(), o: PhantomData::default() }
    }
}

impl<G, I, O> Map<G, I, O> {
    pub fn new(get: G) -> Self
    where
        G: Fn(&I) -> O,
    {
        Self { get, i: PhantomData::default(), o: PhantomData::default() }
    }
}

impl<G: 'static + Clone + Fn(&I) -> O, I: 'static, O: 'static> Lens for Map<G, I, O> {
    // TODO can we get rid of these static bounds?
    type Source = I;
    type Target = O;

    fn view<VO, F: FnOnce(Option<&Self::Target>) -> VO>(
        &self,
        source: &Self::Source,
        map: F,
    ) -> VO {
        let data = (self.get)(source);
        map(Some(&data))
    }
}

/// `Lens` composed of two lenses joined together
pub struct Then<A, B> {
    a: A,
    b: B,
}

impl<A, B> Then<A, B> {
    pub fn new(a: A, b: B) -> Self
    where
        A: Lens,
        B: Lens,
    {
        Self { a, b }
    }
}

impl<A, B> Lens for Then<A, B>
where
    A: Lens,
    B: Lens<Source = A::Target>,
{
    type Source = A::Source;
    type Target = B::Target;

    fn view<O, F: FnOnce(Option<&Self::Target>) -> O>(&self, source: &Self::Source, map: F) -> O {
        self.a.view(source, |t| if let Some(t) = t { self.b.view(t, map) } else { map(None) })
    }

    fn name(&self) -> Option<&'static str> {
        self.a.name()
    }
}

impl<T: Clone, U: Clone> Clone for Then<T, U> {
    fn clone(&self) -> Self {
        Self { a: self.a.clone(), b: self.b.clone() }
    }
}

impl<T: Copy, U: Copy> Copy for Then<T, U> {}

pub struct Index<A, T> {
    index: usize,
    pa: PhantomData<A>,
    pt: PhantomData<T>,
}

impl<A, T> Index<A, T> {
    pub fn new(index: usize) -> Self {
        Self { index, pa: PhantomData::default(), pt: PhantomData::default() }
    }

    pub fn idx(&self) -> usize {
        self.index.clone()
    }
}

impl<A, T> Clone for Index<A, T> {
    fn clone(&self) -> Self {
        Self { index: self.index.clone(), pa: PhantomData::default(), pt: PhantomData::default() }
    }
}

impl<A, T> Copy for Index<A, T> {}

// impl<A,I> Debug for Index<A,I> {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         f.debug_struct("Index").field("index", &self.index).finish()
//     }
// }

impl<A, T: 'static> Lens for Index<A, T>
where
    A: 'static + std::ops::Deref<Target = [T]>,
{
    type Source = A;
    type Target = T;

    fn view<O, F: FnOnce(Option<&Self::Target>) -> O>(&self, source: &Self::Source, map: F) -> O {
        let data = source.get(self.index.clone());
        map(data)
    }
}

pub struct StaticLens<T: 'static> {
    data: &'static T,
}

impl<T> Clone for StaticLens<T> {
    fn clone(&self) -> Self {
        StaticLens { data: self.data }
    }
}

impl<T> Copy for StaticLens<T> {}

impl<T> Debug for StaticLens<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("Static Lens: ")?;
        TypeId::of::<T>().fmt(f)?;
        Ok(())
    }
}

impl<T> Lens for StaticLens<T> {
    type Source = ();
    type Target = T;

    fn view<O, F: FnOnce(Option<&Self::Target>) -> O>(&self, _: &Self::Source, map: F) -> O {
        map(Some(self.data))
    }
}

impl<T> StaticLens<T> {
    pub fn new(data: &'static T) -> Self {
        StaticLens { data }
    }
}

pub struct UnwrapLens<T> {
    t: PhantomData<T>,
}

impl<T> Clone for UnwrapLens<T> {
    fn clone(&self) -> Self {
        UnwrapLens::new()
    }
}

impl<T> UnwrapLens<T> {
    pub fn new() -> Self {
        Self { t: PhantomData::default() }
    }
}

impl<T> Copy for UnwrapLens<T> {}

impl<T: 'static> Lens for UnwrapLens<T> {
    type Source = Option<T>;
    type Target = T;

    fn view<O, F: FnOnce(Option<&Self::Target>) -> O>(&self, source: &Self::Source, map: F) -> O {
        map(source.as_ref())
    }
}

#[derive(Debug)]
pub struct IntoLens<T, U> {
    t: PhantomData<T>,
    u: PhantomData<U>,
}

impl<T, U> IntoLens<T, U> {
    pub fn new() -> Self {
        Self { t: Default::default(), u: Default::default() }
    }
}

impl<T, U> Clone for IntoLens<T, U> {
    fn clone(&self) -> Self {
        Self::new()
    }
}

impl<T, U> Copy for IntoLens<T, U> {}

impl<T: 'static + Clone + TryInto<U>, U: 'static> Lens for IntoLens<T, U> {
    type Source = T;
    type Target = U;

    fn view<O, F: FnOnce(Option<&Self::Target>) -> O>(&self, source: &Self::Source, map: F) -> O {
        let converted = source.clone().try_into().ok();
        map(converted.as_ref())
    }
}

#[derive(Copy, Clone, Debug)]
pub struct RatioLens<L1, L2> {
    numerator: L1,
    denominator: L2,
}

impl<L1, L2> RatioLens<L1, L2> {
    pub fn new(numerator: L1, denominator: L2) -> Self {
        Self { numerator, denominator }
    }
}

impl<L1, L2> Lens for RatioLens<L1, L2>
where
    L1: 'static + Clone + Lens<Target = f32>,
    L2: 'static + Clone + Lens<Target = f32, Source = <L1 as Lens>::Source>,
{
    type Source = L1::Source;
    type Target = f32;

    fn view<O, F: FnOnce(Option<&Self::Target>) -> O>(&self, source: &Self::Source, map: F) -> O {
        let num = self.numerator.view(source, |num| num.copied());
        if let Some(num) = num {
            let den = self.denominator.view(source, |den| den.copied());
            if let Some(den) = den {
                map(Some(&(num / den)))
            } else {
                map(None)
            }
        } else {
            map(None)
        }
    }
}
