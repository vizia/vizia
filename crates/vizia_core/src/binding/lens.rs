use std::any::TypeId;
use std::borrow::Borrow;
use std::fmt::{Debug, Formatter};
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;
use std::ops::Deref;
use std::rc::Rc;

use crate::context::{DataContext, CURRENT, MAPS, MAP_MANAGER};

use super::{Data, MapId};

/// A Lens allows the construction of a reference to a piece of some data, e.g. a field of a struct.
pub trait Lens: 'static + Copy + Debug + Hash {
    /// The input source data.
    type Source;
    /// The output target data.
    type Target;

    /// Function which views a piece of the source data.
    fn view<'a>(&self, cx: &'a impl DataContext) -> Option<LensValue<'a, Self::Target>>;

    /// Creates a store for the lens used in binding.
    fn bind(&self, cx: &mut impl DataContext)
    where
        Self::Target: Data,
    {
        cx.bind(self);
    }

    /// Returns a list of [TypeId] for the sources of the lens.
    fn sources(&self) -> Vec<TypeId> {
        vec![TypeId::of::<Self::Source>()]
    }
}

/// A type returned by `Lens::view()` which contains either a reference to model data or an owned value.
pub enum LensValue<'a, T> {
    /// A reference to model or local data
    Borrowed(&'a T),
    /// Owned data
    Owned(T),
}

impl<T: Clone> Clone for LensValue<'_, T> {
    fn clone(&self) -> Self {
        match self {
            LensValue::Borrowed(v) => LensValue::Borrowed(*v),
            LensValue::Owned(v) => LensValue::Owned(v.clone()),
        }
    }
}

impl<T: Copy> Copy for LensValue<'_, T> {}

impl<T: Clone> LensValue<'_, T> {
    pub fn into_owned(self) -> T {
        match self {
            LensValue::Borrowed(t) => t.clone(),
            LensValue::Owned(t) => t,
        }
    }
}

impl<T> AsRef<T> for LensValue<'_, T> {
    fn as_ref(&self) -> &T {
        self
    }
}

impl<B> Deref for LensValue<'_, B>
where
    B: Borrow<B>,
{
    type Target = B;

    fn deref(&self) -> &B {
        match *self {
            LensValue::Borrowed(borrowed) => borrowed,
            LensValue::Owned(ref owned) => owned.borrow(),
        }
    }
}

/// Helpers for constructing more complex `Lens`es.
pub trait LensExt: Lens {
    fn idx<T>(self, index: usize) -> Index<Self, T>
    where
        T: 'static,
        Self::Target: Deref<Target = [T]>,
    {
        Index::new(self, index)
    }

    fn map<O: 'static, F: 'static + Fn(&Self::Target) -> O>(self, map: F) -> Map<Self, O> {
        let id = MAP_MANAGER.with_borrow_mut(|f| f.create());
        let entity = CURRENT.with_borrow(|f| *f);
        MAPS.with_borrow_mut(|f| {
            f.insert(id, (entity, Box::new(MapState { closure: Rc::new(map) })))
        });
        Map { id, lens: self, o: PhantomData }
    }

    fn map_ref<O: 'static, F: 'static + Fn(&Self::Target) -> &O>(self, map: F) -> MapRef<Self, O> {
        let id = MAP_MANAGER.with_borrow_mut(|f| f.create());
        let entity = CURRENT.with_borrow(|f| *f);
        MAPS.with_borrow_mut(|f| {
            f.insert(id, (entity, Box::new(MapRefState { closure: Rc::new(map) })))
        });
        MapRef { id, lens: self, o: PhantomData }
    }
}

// Implement LensExt for all types which implement Lens.
impl<T: Lens> LensExt for T {}

pub struct MapState<T, O> {
    closure: Rc<dyn Fn(&T) -> O>,
}

pub struct MapRefState<T, O> {
    closure: Rc<dyn Fn(&T) -> &O>,
}

pub struct Map<L: Lens, O> {
    id: MapId,
    lens: L,
    o: PhantomData<O>,
}

impl<L: Lens, O: 'static> Copy for Map<L, O> {}

impl<L: Lens, O: 'static> Clone for Map<L, O> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<L: Lens, O: 'static + Data> Lens for Map<L, O> {
    type Source = L::Source;
    type Target = O;

    fn view<'a>(&self, cx: &'a impl DataContext) -> Option<LensValue<'a, Self::Target>> {
        let target = self.lens.view(cx)?;
        let closure = MAPS.with_borrow(|f| {
            let (_, any) = f.get(&self.id)?;
            let MapState { closure } = any.downcast_ref()?;
            Some(closure.clone())
        })?;
        Some(LensValue::Owned(closure(&*target)))
    }
    fn sources(&self) -> Vec<TypeId> {
        self.lens.sources()
    }
}

impl<L: Lens, O: 'static> Debug for Map<L, O> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}.map(?)", self.lens))
    }
}

impl<L: Lens, O: 'static> Hash for Map<L, O> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.lens.hash(state);
        self.id.hash(state);
    }
}

pub struct MapRef<L: Lens, O> {
    id: MapId,
    lens: L,
    o: PhantomData<O>,
}

impl<L: Lens, O: 'static> Copy for MapRef<L, O> {}

impl<L: Lens, O: 'static> Clone for MapRef<L, O> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<L: Lens, O: 'static + Data> Lens for MapRef<L, O> {
    type Source = L::Source;
    type Target = O;

    fn view<'a>(&self, cx: &'a impl DataContext) -> Option<LensValue<'a, Self::Target>> {
        let closure = MAPS.with_borrow(|f| {
            let (_, any) = f.get(&self.id)?;
            let MapRefState { closure } = any.downcast_ref()?;
            Some(closure.clone())
        })?;

        match self.lens.view(cx)? {
            LensValue::Borrowed(target) => Some(LensValue::Borrowed(closure(target))),
            LensValue::Owned(target) => Some(LensValue::Owned(closure(&target).clone())),
        }
    }
}

impl<L: Lens, O: 'static> Debug for MapRef<L, O> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}.map(?)", self.lens))
    }
}

impl<L: Lens, O: 'static> Hash for MapRef<L, O> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.lens.hash(state);
        self.id.hash(state);
    }
}

pub struct Index<L, T> {
    lens: L,
    index: usize,
    pt: PhantomData<T>,
}

impl<L, T> Index<L, T> {
    pub fn new(lens: L, index: usize) -> Self {
        Self { lens, index, pt: PhantomData }
    }

    pub fn idx(&self) -> usize {
        self.index
    }
}

impl<L: Lens, T> Clone for Index<L, T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<L: Lens, T> Copy for Index<L, T> {}

impl<L: Lens, T> Debug for Index<L, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}.index({:?})", self.lens, self.index))
    }
}

impl<L: Lens, T> Hash for Index<L, T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.lens.hash(state);
        self.index.hash(state);
    }
}

impl<L, T> Lens for Index<L, T>
where
    L: Lens<Target: Deref<Target = [T]>>,
    T: 'static + Data,
{
    type Source = L::Source;
    type Target = T;

    fn view<'a>(&self, cx: &'a impl DataContext) -> Option<LensValue<'a, Self::Target>> {
        self.lens.view(cx).and_then(|v| match v {
            LensValue::Borrowed(v) => v.get(self.index).map(LensValue::Borrowed),
            LensValue::Owned(v) => v.get(self.index).cloned().map(LensValue::Owned),
        })
    }
}

pub struct StaticLens<T: 'static> {
    data: &'static T,
}

impl<T> Clone for StaticLens<T> {
    fn clone(&self) -> Self {
        *self
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

impl<T> Hash for StaticLens<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let id = TypeId::of::<Self>();
        id.hash(state);
    }
}

impl<T: Data> Lens for StaticLens<T> {
    type Source = ();
    type Target = T;

    fn view<'a>(&self, _: &'a impl DataContext) -> Option<LensValue<'a, Self::Target>> {
        Some(LensValue::Borrowed(self.data))
    }
}

impl<T> StaticLens<T> {
    pub fn new(data: &'static T) -> Self {
        StaticLens { data }
    }
}

#[derive(Hash, Copy, Clone, Debug)]
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

    fn view<'a>(&self, cx: &'a impl DataContext) -> Option<LensValue<'a, f32>> {
        let num = self.numerator.view(cx)?.into_owned();
        let den = self.denominator.view(cx)?.into_owned();
        Some(LensValue::Owned(num / den))
    }
}

#[derive(Hash, Clone)]
pub struct Wrapper<L>(pub L);

impl<L: Copy> Copy for Wrapper<L> {}

impl<L: Lens> Lens for Wrapper<L> {
    type Source = L::Source;
    type Target = L::Target;

    fn view<'a>(&self, cx: &'a impl DataContext) -> Option<LensValue<'a, Self::Target>> {
        self.0.view(cx)
    }
}

impl<L: Lens> Debug for Wrapper<L> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl<T> Lens for &'static T
where
    T: 'static + Copy + Debug + Hash + Data,
{
    type Source = ();
    type Target = T;

    fn view<'a>(&self, _source: &'a impl DataContext) -> Option<LensValue<'a, Self::Target>> {
        Some(LensValue::Borrowed(*self))
    }
}

impl<A: Lens, B: Lens> Lens for (A, B)
where
    A::Target: Data,
    B::Target: Data,
{
    type Source = (A::Source, B::Source);
    type Target = (A::Target, B::Target);

    fn view<'a>(&self, cx: &'a impl DataContext) -> Option<LensValue<'a, Self::Target>> {
        Some(LensValue::Owned((self.0.view(cx)?.into_owned(), self.1.view(cx)?.into_owned())))
    }

    fn sources(&self) -> Vec<TypeId> {
        vec![TypeId::of::<A::Source>(), TypeId::of::<B::Source>()]
    }
}

impl<A: Lens, B: Lens, C: Lens> Lens for (A, B, C)
where
    A::Target: Data,
    B::Target: Data,
    C::Target: Data,
{
    type Source = (A::Source, B::Source, C::Source);
    type Target = (A::Target, B::Target, C::Target);

    fn view<'a>(&self, cx: &'a impl DataContext) -> Option<LensValue<'a, Self::Target>> {
        Some(LensValue::Owned((
            self.0.view(cx)?.into_owned(),
            self.1.view(cx)?.into_owned(),
            self.2.view(cx)?.into_owned(),
        )))
    }

    fn sources(&self) -> Vec<TypeId> {
        vec![TypeId::of::<A::Source>(), TypeId::of::<B::Source>(), TypeId::of::<C::Source>()]
    }
}
