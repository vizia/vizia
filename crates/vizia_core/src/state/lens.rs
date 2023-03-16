use std::any::TypeId;
use std::borrow::Borrow;
use std::fmt::{Debug, Formatter};
use std::marker::PhantomData;
use std::ops::{BitAnd, BitOr, Deref};

use crate::context::DataContext;

use super::{next_uuid, StoreId};

/// A Lens allows the construction of a reference to a piece of some data, e.g. a field of a struct.
///
/// When deriving the `Lens` trait on a struct, the derive macro constructs a static type which implements the `Lens` trait for each field.
/// The `view()` method takes a reference to the struct type as input and outputs a reference to the field.
/// This provides a way to specify a binding to a specific field of some application data.
///
/// This trait is part of the prelude.
pub trait Lens: 'static + Clone {
    type Source: ?Sized;
    type SourceOwned: Borrow<Self::Source>;
    type Target: ?Sized;
    type TargetOwned: Borrow<Self::Target>;

    fn view<'a>(
        &'a self,
        source: impl Into<LensValue<'a, Self::Source, Self::SourceOwned>>,
    ) -> Option<LensValue<'a, Self::Target, Self::TargetOwned>>;

    fn name(&self) -> Option<&'static str> {
        None
    }
}

pub trait StatelessLens: Lens {
    fn view_stateless<'a>(
        &self,
        source: impl Into<LensValue<'a, Self::Source, Self::SourceOwned>>,
    ) -> Option<LensValue<'a, Self::Target, Self::TargetOwned>>;
}

/// A type returned by `Lens::view()` which contains either a reference to model data or an owned value.
pub enum LensValue<'a, T: ?Sized, U: Borrow<T> = T> {
    /// A reference to model or local data
    Borrowed(&'a T),
    /// Owned data
    Owned(U),
}

impl<T, U> Clone for LensValue<'_, T, U>
where
    T: ?Sized,
    U: Clone + Borrow<T>,
{
    fn clone(&self) -> Self {
        match self {
            LensValue::Borrowed(v) => LensValue::Borrowed(*v),
            LensValue::Owned(v) => LensValue::Owned(v.clone()),
        }
    }
}

impl<T: Copy> Copy for LensValue<'_, T> {}

impl<T> LensValue<'_, T, T::Owned>
where
    T: ToOwned + ?Sized,
{
    pub fn into_owned(self) -> T::Owned {
        match self {
            LensValue::Borrowed(t) => t.to_owned(),
            LensValue::Owned(t) => t,
        }
    }
}

impl<B, O> AsRef<B> for LensValue<'_, B, O>
where
    O: Borrow<B>,
    B: ?Sized,
{
    fn as_ref(&self) -> &B {
        self
    }
}

impl<B, O> Deref for LensValue<'_, B, O>
where
    O: Borrow<B>,
    B: ?Sized,
{
    type Target = B;

    fn deref(&self) -> &B {
        match *self {
            LensValue::Borrowed(borrowed) => borrowed,
            LensValue::Owned(ref owned) => owned.borrow(),
        }
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

impl<T, U: Borrow<T>> From<U> for LensValue<'_, T, U> {
    fn from(value: U) -> Self {
        LensValue::Owned(value)
    }
}

impl<'a, T> From<&'a T> for LensValue<'a, T> {
    fn from(reference: &'a T) -> Self {
        LensValue::Borrowed(reference)
    }
}

/// Helpers for constructing more complex `Lens`es.
///
/// This trait is part of the prelude.
pub trait LensExt: Lens {
    fn get(&self, cx: &impl DataContext) -> Self::TargetOwned
    where
        Self::Target: ToOwned<Owned = Self::TargetOwned>,
        Self::Source: Sized,
        Self::SourceOwned: Sized,
    {
        self.get_fallible(cx).unwrap()
    }

    fn get_fallible(&self, cx: &impl DataContext) -> Option<Self::TargetOwned>
    where
        Self::Target: ToOwned<Owned = Self::TargetOwned>,
        Self::Source: Sized,
        Self::SourceOwned: Sized,
    {
        self.view(LensValue::Borrowed(cx.data()?)).map(|t| t.into_owned())
    }

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
        Other: Lens<SourceOwned = Self::TargetOwned>,
        Self::Target: Borrow<Other::Source>,
    {
        Then::new(self, other)
    }

    fn index<T>(self, index: usize) -> Then<Self, Index<Self::TargetOwned, (), T>>
    where
        T: 'static,
        Self: Lens<TargetOwned = Vec<T>>,
        Self::Target: Borrow<[T]>,
    {
        self.then(Index::new(index))
    }

    fn map<G: Clone, B: 'static + Clone>(
        self,
        get: G,
    ) -> Then<Self, Map<G, Self::Target, Self::TargetOwned, B>>
    where
        G: 'static + Fn(&Self::Target) -> B,
    {
        self.then(Map::new(get))
    }

    fn unwrap<T: 'static>(self) -> Then<Self, UnwrapLens<T>>
    where
        Self: Lens<Target = Option<T>, TargetOwned = Option<T>>,
    {
        self.then(UnwrapLens::new())
    }

    fn into_lens<T: 'static>(self) -> Then<Self, IntoLens<Self::Target, T>>
    where
        Self::Target: ToOwned<Owned = Self::TargetOwned>,
        <Self::Target as ToOwned>::Owned: TryInto<T>,
    {
        self.then(IntoLens::new())
    }
}

// Implement LensExt for all types which implement Lens
impl<T: Lens> LensExt for T {}

pub struct Borrowed<B, O> {
    b: PhantomData<B>,
    o: PhantomData<O>,
}

pub struct Map<G, I: ?Sized, IO, O> {
    get: G,
    i: PhantomData<I>,
    io: PhantomData<IO>,
    o: PhantomData<O>,
}

impl<G: Clone, I: ?Sized, IO, O> Clone for Map<G, I, IO, O> {
    fn clone(&self) -> Self {
        Map {
            get: self.get.clone(),
            i: PhantomData::default(),
            o: PhantomData::default(),
            io: PhantomData,
        }
    }
}

impl<G, I: ?Sized, IO, O> Map<G, I, IO, O> {
    pub fn new(get: G) -> Self
    where
        G: Fn(&I) -> O,
    {
        Self { get, i: PhantomData::default(), o: PhantomData::default(), io: PhantomData }
    }
}

impl<
        G: 'static + Clone + Fn(&I) -> O,
        I: 'static + ?Sized,
        IO: 'static + Borrow<I>,
        O: 'static,
    > Lens for Map<G, I, IO, O>
{
    // TODO can we get rid of these static bounds?
    type Source = I;
    type SourceOwned = IO;
    type Target = O;
    type TargetOwned = O;

    fn view<'a>(
        &self,
        source: impl Into<LensValue<'a, Self::Source, Self::SourceOwned>>,
    ) -> Option<LensValue<'a, Self::Target>> {
        Some(LensValue::Owned((self.get)(source.into().deref())))
    }
}

/// `Lens` composed of two lenses joined together
pub struct Then<A, B>
where
    A: Lens,
    B: Lens,
    A::TargetOwned: Into<B::SourceOwned>,
    A::Target: Borrow<B::Source>,
{
    a: A,
    b: B,
}

impl<A, B> Then<A, B>
where
    A: Lens,
    B: Lens,
    A::TargetOwned: Into<B::SourceOwned>,
    A::Target: Borrow<B::Source>,
{
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
    B: Lens,
    A::TargetOwned: Into<B::SourceOwned>,
    A::Target: Borrow<B::Source>,
{
    type Source = A::Source;
    type SourceOwned = A::SourceOwned;
    type Target = B::Target;
    type TargetOwned = B::TargetOwned;

    fn view<'a>(
        &'a self,
        source: impl Into<LensValue<'a, Self::Source, Self::SourceOwned>>,
    ) -> Option<LensValue<'a, Self::Target, Self::TargetOwned>> {
        match self.a.view(source)? {
            LensValue::Borrowed(b) => self.b.view(LensValue::Borrowed(b.borrow())),
            LensValue::Owned(o) => self.b.view(LensValue::Owned(o.into())),
        }
    }

    fn name(&self) -> Option<&'static str> {
        self.a.name()
    }
}

impl<T: Clone, U: Clone> Clone for Then<T, U>
where
    T: Lens,
    U: Lens,
    T::TargetOwned: Into<U::SourceOwned>,
    T::Target: Borrow<U::Source>,
{
    fn clone(&self) -> Self {
        Self { a: self.a.clone(), b: self.b.clone() }
    }
}

impl<T: Copy, U: Copy> Copy for Then<T, U>
where
    T: Lens,
    U: Lens,
    T::TargetOwned: Into<U::SourceOwned>,
    T::Target: Borrow<U::Source>,
{
}

pub struct Index<A, B, T> {
    index: usize,
    pa: PhantomData<A>,
    pb: PhantomData<B>,
    pt: PhantomData<T>,
}

impl<A, B, T> Default for Index<A, B, T> {
    fn default() -> Self {
        Self {
            index: Default::default(),
            pa: Default::default(),
            pb: Default::default(),
            pt: Default::default(),
        }
    }
}

impl<A, B, T> Index<A, B, T> {
    pub fn new(index: usize) -> Self {
        Self { index, ..Default::default() }
    }

    pub fn idx(&self) -> usize {
        self.index
    }
}

impl<A, B, T> Clone for Index<A, B, T> {
    fn clone(&self) -> Self {
        Self { index: self.index, ..Default::default() }
    }
}

impl<A, B, T> Copy for Index<A, B, T> {}

impl<A, B, T: 'static> Lens for Index<A, B, T>
where
    A: 'static + Into<Vec<T>> + Borrow<[T]>,
    B: 'static,
{
    type Source = [T];
    type SourceOwned = A;
    type Target = T;
    type TargetOwned = T;

    fn view<'a>(
        &'a self,
        source: impl Into<LensValue<'a, Self::Source, Self::SourceOwned>>,
    ) -> Option<LensValue<'a, Self::Target, Self::TargetOwned>> {
        Some(match source.into() {
            LensValue::Borrowed(source) => LensValue::Borrowed(<[T]>::get(source, self.index)?),
            LensValue::Owned(source) => LensValue::Owned(source.into().swap_remove(self.index)),
        })
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
    type SourceOwned = ();
    type Target = T;
    type TargetOwned = T;

    fn view<'a>(
        &'a self,
        source: impl Into<LensValue<'a, Self::Source>>,
    ) -> Option<LensValue<'a, Self::Target>> {
        Some(LensValue::Borrowed(self.data))
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
    type SourceOwned = Self::Source;
    type TargetOwned = Self::Target;

    fn view<'a>(
        &'a self,
        source: impl Into<LensValue<'a, Self::Source>>,
    ) -> Option<LensValue<'a, Self::Target>> {
        match source.into() {
            LensValue::Owned(o) => o.map(LensValue::Owned),
            LensValue::Borrowed(b) => b.as_ref().map(LensValue::Borrowed),
        }
    }
}

#[derive(Debug)]
pub struct IntoLens<T: ?Sized, U> {
    t: PhantomData<T>,
    u: PhantomData<U>,
}

impl<T: ?Sized, U> IntoLens<T, U> {
    pub fn new() -> Self {
        Self { t: Default::default(), u: Default::default() }
    }
}

impl<T: ?Sized, U> Clone for IntoLens<T, U> {
    fn clone(&self) -> Self {
        Self::new()
    }
}

impl<T: ?Sized, U> Copy for IntoLens<T, U> {}

impl<T: ?Sized, U> Lens for IntoLens<T, U>
where
    T: 'static + ToOwned,
    U: 'static,
    T::Owned: TryInto<U>,
{
    type Source = T;
    type SourceOwned = T::Owned;
    type Target = U;
    type TargetOwned = U;

    fn view<'a>(
        &'a self,
        source: impl Into<LensValue<'a, Self::Source, Self::SourceOwned>>,
    ) -> Option<LensValue<'a, Self::Target>> {
        match source.into() {
            LensValue::Owned(o) => o.try_into().ok().map(LensValue::Owned),
            LensValue::Borrowed(b) => b.to_owned().try_into().ok().map(LensValue::Owned),
        }
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
    L2: 'static
        + Clone
        + Lens<Target = f32, Source = <L1 as Lens>::Source, SourceOwned = <L1 as Lens>::SourceOwned>,
{
    type Source = L1::Source;
    type SourceOwned = L1::SourceOwned;
    type Target = f32;
    type TargetOwned = f32;

    fn view<'a>(
        &'a self,
        source: impl Into<LensValue<'a, Self::Source, Self::SourceOwned>>,
    ) -> Option<LensValue<'a, Self::Target>> {
        let source = source.into();
        let num = *self.numerator.view(source)?;
        let den = *self.denominator.view(source)?;
        Some(LensValue::Owned(num / den))
    }
}

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
    type SourceOwned = L2::SourceOwned;
    type TargetOwned = Self::Target;

    fn view<'a>(
        &'a self,
        source: impl Into<LensValue<'a, Self::Source, Self::SourceOwned>>,
    ) -> Option<LensValue<'a, Self::Target>> {
        let source = source.into();
        let v1 = *self.lens1.view(LensValue::Borrowed(source.deref()))?;
        let v2 = *self.lens2.view(source)?;

        Some(LensValue::Owned(v1 | v2))
    }

    fn name(&self) -> Option<&'static str> {
        self.lens1.name()
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
    type SourceOwned = L::SourceOwned;
    type Target = L::Target;
    type TargetOwned = L::TargetOwned;

    fn view<'a>(
        &'a self,
        source: impl Into<LensValue<'a, Self::Source, Self::SourceOwned>>,
    ) -> Option<LensValue<'a, Self::Target, Self::TargetOwned>> {
        self.0.view(source)
    }

    fn name(&self) -> Option<&'static str> {
        self.0.name()
    }
}

impl<L: StatelessLens> StatelessLens for Wrapper<L> {
    fn view_stateless<'a>(
        &self,
        source: impl Into<LensValue<'a, Self::Source, Self::SourceOwned>>,
    ) -> Option<LensValue<'a, Self::Target, Self::TargetOwned>> {
        self.0.view_stateless(source)
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
    A: Lens<Source = L2::Source, SourceOwned = L2::SourceOwned>,
    L1: Lens<Source = A::Target, SourceOwned = A::TargetOwned>,
{
    type Output = OrLens<Self, L2>;
    fn bitor(self, rhs: L2) -> Self::Output {
        OrLens::new(self, rhs)
    }
}

impl<G: 'static + Clone + Fn(&I) -> bool, I: 'static, L2: Lens<Target = bool>> BitOr<L2>
    for Map<G, I, I, bool>
where
    I: Lens<Source = L2::Source>,
{
    type Output = OrLens<Self, L2>;
    fn bitor(self, rhs: L2) -> Self::Output {
        OrLens::new(self, rhs)
    }
}

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
    type SourceOwned = L2::SourceOwned;
    type TargetOwned = bool;

    fn view<'a>(
        &'a self,
        source: impl Into<LensValue<'a, Self::Source, Self::SourceOwned>>,
    ) -> Option<LensValue<'a, Self::Target>> {
        let source = source.into();
        let v1 = *self.lens1.view(LensValue::Borrowed(source.deref()))?;
        let v2 = *self.lens2.view(source)?;

        Some(LensValue::Owned(v1 | v2))
    }

    fn name(&self) -> Option<&'static str> {
        self.lens1.name()
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
    A: Lens<Source = L2::Source, SourceOwned = L2::SourceOwned>,
    L1: Lens<Source = A::Target, SourceOwned = A::TargetOwned>,
{
    type Output = AndLens<Self, L2>;
    fn bitand(self, rhs: L2) -> Self::Output {
        AndLens::new(self, rhs)
    }
}

impl<G: 'static + Clone + Fn(&I) -> bool, I: 'static, IO: Borrow<I>, L2: Lens<Target = bool>>
    BitAnd<L2> for Map<G, I, IO, bool>
where
    I: Lens<Source = L2::Source>,
{
    type Output = AndLens<Self, L2>;
    fn bitand(self, rhs: L2) -> Self::Output {
        AndLens::new(self, rhs)
    }
}

impl<L1, L2> Lens for (L1, L2)
where
    L1: Lens<Source = L2::Source, SourceOwned = L2::SourceOwned>,
    L2: Lens,
    L1::SourceOwned: Clone,
    L2::SourceOwned: Clone,
    L1::Target: ToOwned<Owned = L1::TargetOwned>,
    L2::Target: ToOwned<Owned = L2::TargetOwned>,
{
    type Source = L1::Source;
    type SourceOwned = L1::SourceOwned;
    type Target = (L1::TargetOwned, L2::TargetOwned);
    type TargetOwned = (L1::TargetOwned, L2::TargetOwned);

    fn view<'a>(
        &'a self,
        source: impl Into<LensValue<'a, Self::Source, Self::SourceOwned>>,
    ) -> Option<LensValue<'a, Self::Target, Self::TargetOwned>> {
        let source = source.into();
        let v1 = self.0.view(source.clone())?.into_owned();
        let v2 = self.1.view(source)?.into_owned();

        Some(LensValue::Owned((v1, v2)))
    }

    fn name(&self) -> Option<&'static str> {
        self.0.name()
    }
}
