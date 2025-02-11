use std::any::TypeId;
use std::borrow::Borrow;
use std::fmt::{Debug, Formatter};
use std::hash::Hash;
use std::marker::PhantomData;
use std::ops::{BitAnd, BitOr, Deref};
use std::rc::Rc;

use crate::context::{CURRENT, MAPS, MAP_MANAGER};

use super::{MapId, StoreId};

/// A Lens allows the construction of a reference to a piece of some data, e.g. a field of a struct.
///
/// When deriving the `Lens` trait on a struct, the derive macro constructs a static type which implements the `Lens` trait for each field.
/// The `view()` method takes a reference to the struct type as input and outputs a reference to the field.
/// This provides a way to specify a binding to a specific field of some application data.
pub trait Lens: 'static + Copy + Debug {
    /// The type of the source data.
    type Source;
    /// The type of the target data.
    type Target;

    /// View the target data from the source.
    fn view<'a>(&self, source: &'a Self::Source) -> Option<LensValue<'a, Self::Target>>;
    /// Get the store id of the lens.
    fn id(&self) -> StoreId {
        StoreId::Source(TypeId::of::<Self>())
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
    /// Convert the value to an owned value.
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
    fn or<Other>(self, other: Other) -> OrLens<Self, Other>
    where
        Other: Lens<Target = bool>,
        Self: Lens<Target = bool>,
    {
        OrLens::new(self, other)
    }

    fn and<Other>(self, other: Other) -> AndLens<Self, Other>
    where
        Other: Lens<Target = bool>,
        Self: Lens<Target = bool>,
    {
        AndLens::new(self, other)
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

    /// Used to construct a lens to some data contained within an array.
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

// Implement LensExt for all types which implement Lens.
impl<T: Lens> LensExt for T {}

/// The state of a map lens.
pub(crate) struct MapState<T, O> {
    closure: Rc<dyn Fn(&T) -> O>,
}

/// The state of a map ref lens.
pub(crate) struct MapRefState<T, O> {
    closure: Rc<dyn Fn(&T) -> &O>,
}

/// A lens which maps a value to another value.
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

impl<L: Lens, O: 'static> Lens for Map<L, O> {
    type Source = L::Source;
    type Target = O;

    fn view<'a>(&self, source: &'a Self::Source) -> Option<LensValue<'a, Self::Target>> {
        let target = self.lens.view(source)?;
        let closure = MAPS.with_borrow(|f| {
            let (_, any) = f.get(&self.id)?;
            let MapState { closure } = any.downcast_ref()?;
            Some(closure.clone())
        })?;
        Some(LensValue::Owned(closure(&*target)))
    }

    fn id(&self) -> StoreId {
        StoreId::Map(self.id.0)
    }
}

impl<L: Lens, O: 'static> Debug for Map<L, O> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}.map(?)", self.lens))
    }
}

/// A lens which maps a reference value to another reference value.
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

impl<L: Lens, O: 'static + Clone> Lens for MapRef<L, O> {
    type Source = L::Source;
    type Target = O;

    fn view<'a>(&self, source: &'a Self::Source) -> Option<LensValue<'a, Self::Target>> {
        let closure = MAPS.with_borrow(|f| {
            let (_, any) = f.get(&self.id)?;
            let MapRefState { closure } = any.downcast_ref()?;
            Some(closure.clone())
        })?;

        match self.lens.view(source)? {
            LensValue::Borrowed(target) => Some(LensValue::Borrowed(closure(target))),
            LensValue::Owned(target) => Some(LensValue::Owned(closure(&target).clone())),
        }
    }

    fn id(&self) -> StoreId {
        StoreId::Map(self.id.0)
    }
}

impl<L: Lens, O: 'static> Debug for MapRef<L, O> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}.map(?)", self.lens))
    }
}

/// `Lens` composed of two lenses joined together
pub struct Then<A, B> {
    a: A,
    b: B,
}

impl<A, B> Then<A, B> {
    /// Create a new `Then` lens.
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

    fn view<'a>(&self, source: &'a Self::Source) -> Option<LensValue<'a, Self::Target>> {
        if let Some(val) = self.a.view(source) {
            let val = match val {
                LensValue::Borrowed(val) => return self.b.view(val),
                LensValue::Owned(ref val) => val,
            };
            match self.b.view(val) {
                Some(LensValue::Owned(val)) => return Some(LensValue::Owned(val)),
                _ => unreachable!(),
            }
        }

        None
    }

    fn id(&self) -> StoreId {
        StoreId::Recursive((self.a.id(), self.b.id()).into())
    }
}

impl<T: Clone, U: Clone> Clone for Then<T, U> {
    fn clone(&self) -> Self {
        Self { a: self.a.clone(), b: self.b.clone() }
    }
}

impl<A: Lens, B: Lens> Debug for Then<A, B> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}.then({:?})", self.a, self.b))
    }
}

impl<T: Copy, U: Copy> Copy for Then<T, U> {}

/// A lens to a specific index of an array.
pub struct Index<L, T> {
    lens: L,
    index: usize,
    pt: PhantomData<T>,
}

impl<L, T> Index<L, T> {
    /// Create a new `Index` lens.
    pub fn new(lens: L, index: usize) -> Self {
        Self { lens, index, pt: PhantomData }
    }

    /// Get the index the lens.
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

impl<L, T> Lens for Index<L, T>
where
    L: Lens<Target: Deref<Target = [T]>>,
    T: 'static + Clone,
{
    type Source = L::Source;
    type Target = T;

    fn view<'a>(&self, source: &'a Self::Source) -> Option<LensValue<'a, Self::Target>> {
        self.lens.view(source).and_then(|v| match v {
            LensValue::Borrowed(v) => v.get(self.index).map(LensValue::Borrowed),
            LensValue::Owned(v) => v.get(self.index).cloned().map(LensValue::Owned),
        })
    }

    fn id(&self) -> StoreId {
        StoreId::Index(TypeId::of::<Self>(), self.index)
    }
}

/// A lens to static data.
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

impl<T> Lens for StaticLens<T> {
    type Source = ();
    type Target = T;

    fn view<'a>(&self, _: &'a Self::Source) -> Option<LensValue<'a, Self::Target>> {
        Some(LensValue::Borrowed(self.data))
    }
}

impl<T> StaticLens<T> {
    /// Create a new `StaticLens`.
    pub fn new(data: &'static T) -> Self {
        StaticLens { data }
    }
}

#[derive(Default)]
pub struct UnwrapLens<T> {
    t: PhantomData<T>,
}

impl<T> Clone for UnwrapLens<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> UnwrapLens<T> {
    pub fn new() -> Self {
        Self { t: PhantomData }
    }
}

impl<T> Copy for UnwrapLens<T> {}

impl<T: 'static> Lens for UnwrapLens<T> {
    type Source = Option<T>;
    type Target = T;

    fn view<'a>(&self, source: &'a Self::Source) -> Option<LensValue<'a, Self::Target>> {
        source.as_ref().map(LensValue::Borrowed)
    }
}

impl<T: 'static> Debug for UnwrapLens<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("unwrap")
    }
}

#[derive(Default)]
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
        *self
    }
}

impl<T, U> Copy for IntoLens<T, U> {}

impl<T: 'static + Clone + TryInto<U>, U: 'static> Lens for IntoLens<T, U> {
    type Source = T;
    type Target = U;

    fn view<'a>(&self, source: &'a Self::Source) -> Option<LensValue<'a, Self::Target>> {
        source.clone().try_into().ok().map(|t| LensValue::Owned(t))
    }
}

impl<T, U> Debug for IntoLens<T, U> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("into")
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

    fn view<'a>(&self, source: &'a Self::Source) -> Option<LensValue<'a, f32>> {
        let num = self.numerator.view(source)?.into_owned();
        let den = self.denominator.view(source)?.into_owned();
        Some(LensValue::Owned(num / den))
    }
}

#[derive(Debug, Copy)]
pub struct OrLens<L1, L2> {
    lens1: L1,
    lens2: L2,
}

impl<L1, L2> OrLens<L1, L2> {
    pub fn new(lens1: L1, lens2: L2) -> Self
    where
        L1: Lens<Target = bool>,
        L2: Lens<Target = bool>,
    {
        Self { lens1, lens2 }
    }
}

impl<L1, L2> Lens for OrLens<L1, L2>
where
    L1: Lens<Source = L2::Source, Target = bool>,
    L2: Lens<Target = bool>,
{
    type Source = L1::Source;
    type Target = bool;

    fn view<'a>(&self, source: &'a Self::Source) -> Option<LensValue<'a, Self::Target>> {
        let v1 = self.lens1.view(source)?.into_owned();
        let v2 = self.lens2.view(source)?.into_owned();

        Some(LensValue::Owned(v1 | v2))
    }
}

impl<L1: Clone, L2: Clone> Clone for OrLens<L1, L2> {
    fn clone(&self) -> Self {
        Self { lens1: self.lens1.clone(), lens2: self.lens2.clone() }
    }
}

#[derive(Clone)]
pub struct Wrapper<L>(pub L);

impl<L: Copy> Copy for Wrapper<L> {}

impl<L: Lens> Lens for Wrapper<L> {
    type Source = L::Source;
    type Target = L::Target;

    fn view<'a>(&self, source: &'a Self::Source) -> Option<LensValue<'a, Self::Target>> {
        self.0.view(source)
    }
}

impl<L: Lens> Debug for Wrapper<L> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl<L1: Lens<Target = bool>, L2: Lens<Target = bool>> BitOr<L2> for Wrapper<L1>
where
    L1: Lens<Source = L2::Source>,
{
    type Output = OrLens<Self, L2>;
    fn bitor(self, rhs: L2) -> Self::Output {
        OrLens::new(self, rhs)
    }
}

impl<L1, L2, L3: Lens<Target = bool>> BitOr<L3> for OrLens<L1, L2>
where
    Self: Lens<Target = bool>,
    Self: Lens<Source = L3::Source>,
{
    type Output = OrLens<Self, L3>;
    fn bitor(self, rhs: L3) -> Self::Output {
        OrLens::new(self, rhs)
    }
}

impl<A: Lens, L1: Lens<Target = bool>, L2: Lens<Target = bool>> BitOr<L2> for Then<A, L1>
where
    A: Lens<Source = L2::Source>,
    L1: Lens<Source = A::Target>,
{
    type Output = OrLens<Self, L2>;
    fn bitor(self, rhs: L2) -> Self::Output {
        OrLens::new(self, rhs)
    }
}

impl<L, L2: Lens<Target = bool>> BitOr<L2> for Map<L, bool>
where
    L: Lens<Source = L2::Source>,
{
    type Output = OrLens<Self, L2>;
    fn bitor(self, rhs: L2) -> Self::Output {
        OrLens::new(self, rhs)
    }
}

#[derive(Debug, Copy)]
pub struct AndLens<L1, L2> {
    lens1: L1,
    lens2: L2,
}

impl<L1, L2> AndLens<L1, L2> {
    pub fn new(lens1: L1, lens2: L2) -> Self
    where
        L1: Lens<Target = bool>,
        L2: Lens<Target = bool>,
    {
        Self { lens1, lens2 }
    }
}

impl<L1, L2> Lens for AndLens<L1, L2>
where
    L1: Lens<Source = L2::Source, Target = bool>,
    L2: Lens<Target = bool>,
{
    type Source = L1::Source;
    type Target = bool;

    fn view<'a>(&self, source: &'a Self::Source) -> Option<LensValue<'a, Self::Target>> {
        let v1 = self.lens1.view(source)?.into_owned();
        let v2 = self.lens2.view(source)?.into_owned();

        Some(LensValue::Owned(v1 | v2))
    }
}

impl<L1: Clone, L2: Clone> Clone for AndLens<L1, L2> {
    fn clone(&self) -> Self {
        Self { lens1: self.lens1.clone(), lens2: self.lens2.clone() }
    }
}

impl<L1: Lens<Target = bool>, L2: Lens<Target = bool>> BitAnd<L2> for Wrapper<L1>
where
    L1: Lens<Source = L2::Source>,
{
    type Output = AndLens<Self, L2>;
    fn bitand(self, rhs: L2) -> Self::Output {
        AndLens::new(self, rhs)
    }
}

impl<L1, L2, L3: Lens<Target = bool>> BitAnd<L3> for AndLens<L1, L2>
where
    Self: Lens<Target = bool>,
    Self: Lens<Source = L3::Source>,
{
    type Output = AndLens<Self, L3>;
    fn bitand(self, rhs: L3) -> Self::Output {
        AndLens::new(self, rhs)
    }
}

impl<A: Lens, L1: Lens<Target = bool>, L2: Lens<Target = bool>> BitAnd<L2> for Then<A, L1>
where
    A: Lens<Source = L2::Source>,
    L1: Lens<Source = A::Target>,
{
    type Output = AndLens<Self, L2>;
    fn bitand(self, rhs: L2) -> Self::Output {
        AndLens::new(self, rhs)
    }
}

impl<L, L2: Lens<Target = bool>> BitAnd<L2> for Map<L, bool>
where
    L: Lens<Source = L2::Source>,
{
    type Output = AndLens<Self, L2>;
    fn bitand(self, rhs: L2) -> Self::Output {
        AndLens::new(self, rhs)
    }
}

impl<T> Lens for &'static T
where
    T: 'static + Copy + Debug + Hash,
{
    type Source = ();
    type Target = T;

    fn view<'a>(&self, _source: &'a Self::Source) -> Option<LensValue<'a, Self::Target>> {
        Some(LensValue::Borrowed(*self))
    }
}
