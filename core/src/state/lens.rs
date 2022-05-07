use std::any::TypeId;
use std::collections::HashSet;
use std::fmt::{Debug, Formatter};
use std::marker::PhantomData;
use std::ops::Deref;
use std::rc::Rc;

use crate::prelude::*;
use crate::state::{PubStore, State};

/// A Lens allows the construction of a reference to a piece of some data, e.g. a field of a struct.
///
/// When deriving the `Lens` trait on a struct, the derive macro constructs a static type which implements the `Lens` trait for each field.
/// The `view()` method takes a reference to the struct type as input and outputs a reference to the field.
/// This provides a way to specify a binding to a specific field of some application data.
///
/// This trait is part of the prelude.
pub trait Lens: 'static + Clone {
    type Source;
    type Target;

    fn view<O, F: FnOnce(Option<&Self::Target>) -> O>(&self, source: &Self::Source, map: F) -> O;

    fn make_store(&self, source: &Self::Source, entity: Entity) -> PubStore
    where
        Self::Target: Data,
    {
        PubStore(Box::new(State {
            entity,
            lens: self.clone(),
            old: self.view(source, |t| t.cloned().map(|v| v)),
            observers: HashSet::from([entity]),
        }))
    }

    fn cache_key(&self) -> Option<TypeId> {
        if std::mem::size_of::<Self>() == 0 {
            Some(TypeId::of::<Self>())
        } else {
            None
        }
    }
}

/// Helpers for constructing more complex `Lens`es.
///
/// This trait is part of the prelude.
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
        Self: Sized,
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

    fn map<G, B: 'static>(self, get: G) -> Then<Self, Map<Self::Target, B>>
    where
        G: 'static + Fn(&Self::Target) -> B,
    {
        self.then(Map::new(get))
    }

    fn map_shallow<G, B: 'static>(self, get: G) -> MapShallow<Self, Self::Target, B>
        where
            G: 'static + Fn(&Self::Target) -> B,
    {
        MapShallow { child: self, mapper: Rc::new(get) }
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

pub struct Map<I, O> {
    get: Rc<dyn Fn(&I) -> O>,
}

impl<I, O> Clone for Map<I, O> {
    fn clone(&self) -> Self {
        Map { get: self.get.clone() }
    }
}

impl<I, O> Map<I, O> {
    pub fn new<F>(get: F) -> Self
    where
        F: 'static + Fn(&I) -> O,
    {
        Self { get: Rc::new(get) }
    }
}

impl<I: 'static, O: 'static> Lens for Map<I, O> {
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
    T: Sized,
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

#[derive(Clone)]
pub struct MapShallow<L, I, O> {
    child: L,
    mapper: Rc<dyn Fn(&I) -> O>,
}

impl<L, I, O> Lens for MapShallow<L, I, O>
where
    L: Lens<Target = I>,
    I: Data,
    O: Data,
{
    type Source = L::Source;
    type Target = O;

    fn view<OO, F: FnOnce(Option<&Self::Target>) -> OO>(
        &self,
        source: &Self::Source,
        map: F,
    ) -> OO {
        self.child.view(source, |t| map(t.map(self.mapper.as_ref()).as_ref()))
    }

    fn cache_key(&self) -> Option<TypeId> {
        self.child.cache_key()
    }

    fn make_store(&self, source: &Self::Source, entity: Entity) -> PubStore
    where
        Self::Target: Data,
    {
        self.child.make_store(source, entity)
    }
}
