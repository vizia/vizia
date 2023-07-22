use std::any::TypeId;
use std::fmt::{Debug, Formatter};
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;
use std::ops::{BitAnd, BitOr, Deref};
use std::rc::Rc;

use crate::context::{CURRENT, MAPS, MAP_MANAGER};
use crate::prelude::*;

use super::MapId;

/// A Lens allows the construction of a reference to a piece of some data, e.g. a field of a struct.
///
/// When deriving the `Lens` trait on a struct, the derive macro constructs a static type which implements the `Lens` trait for each field.
/// The `view()` method takes a reference to the struct type as input and outputs a reference to the field.
/// This provides a way to specify a binding to a specific field of some application data.
pub trait Lens: 'static + Copy + std::fmt::Debug + std::hash::Hash {
    type Source;
    type Target;

    fn view<O, F: FnOnce(Option<&Self::Target>) -> O>(&self, source: &Self::Source, map: F) -> O;
}

/// Helpers for constructing more complex `Lens`es.
pub trait LensExt: Lens {
    /// Retrieve a copy of the lensed data from context. This will clone the data.
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
            |t| t.cloned(),
        )
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
        Other: Lens<Source = Self::Target>,
    {
        Then::new(self, other)
    }

    fn index<T>(self, index: usize) -> Index<Self, T>
    where
        T: 'static,
        Self::Target: Deref<Target = [T]>,
    {
        Index::new(self, index)
    }

    fn map<O: 'static, F: 'static + Fn(&Self::Target) -> O>(self, map: F) -> Map<Self, O> {
        let id = MAP_MANAGER.with(|f| f.borrow_mut().create());
        let entity = CURRENT.with(|f| *f.borrow());
        MAPS.with(|f| {
            f.borrow_mut().insert(id, (entity, Box::new(MapState { closure: Rc::new(map) })))
        });
        Map { id, lens: self, o: PhantomData }
    }

    fn map_ref<O: 'static, F: 'static + Fn(&Self::Target) -> &O>(self, map: F) -> MapRef<Self, O> {
        let id = MAP_MANAGER.with(|f| f.borrow_mut().create());
        let entity = CURRENT.with(|f| *f.borrow());
        MAPS.with(|f| {
            f.borrow_mut().insert(id, (entity, Box::new(MapRefState { closure: Rc::new(map) })))
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

impl<L: Lens, O: 'static> std::marker::Copy for Map<L, O> {}

impl<L: Lens, O: 'static> Clone for Map<L, O> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<L: Lens, O: 'static> Lens for Map<L, O> {
    type Source = L::Source;
    type Target = O;

    fn view<VO, F: FnOnce(Option<&Self::Target>) -> VO>(
        &self,
        source: &Self::Source,
        map: F,
    ) -> VO {
        self.lens.view(source, |t| {
            let closure = MAPS.with(|f| {
                if let Some(lens_map) = f.borrow().get(&self.id) {
                    if let Some(mapping) = lens_map.1.downcast_ref::<MapState<L::Target, O>>() {
                        return Some(mapping.closure.clone());
                    }
                }

                None
            });
            map(t.map(|tt| (closure.unwrap())(tt)).as_ref())
        })
    }
}

impl<L: Lens, O: 'static> std::fmt::Debug for Map<L, O> {
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

// #[derive(Debug)]
pub struct MapRef<L: Lens, O> {
    id: MapId,
    lens: L,
    o: PhantomData<O>,
}

impl<L: Lens, O: 'static> std::marker::Copy for MapRef<L, O> {}

impl<L: Lens, O: 'static> Clone for MapRef<L, O> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<L: Lens, O: 'static> Lens for MapRef<L, O> {
    type Source = L::Source;
    type Target = O;

    fn view<VO, F: FnOnce(Option<&Self::Target>) -> VO>(
        &self,
        source: &Self::Source,
        map: F,
    ) -> VO {
        self.lens.view(source, |t| {
            let closure = MAPS.with(|f| {
                if let Some(lens_map) = f.borrow().get(&self.id) {
                    if let Some(mapping) = lens_map.1.downcast_ref::<MapRefState<L::Target, O>>() {
                        return Some(mapping.closure.clone());
                    }
                }

                None
            });
            map(t.map(|tt| (closure.unwrap())(tt)))
        })
    }
}

impl<L: Lens, O: 'static> std::fmt::Debug for MapRef<L, O> {
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

/// `Lens` composed of two lenses joined together
#[derive(Hash)]
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

impl<A: Lens, B: Lens> std::fmt::Debug for Then<A, B> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}.then({:?})", self.a, self.b))
    }
}

impl<T: Copy, U: Copy> Copy for Then<T, U> {}

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

impl<L: Lens, T: 'static> Lens for Index<L, T>
where
    <L as Lens>::Target: std::ops::Deref<Target = [T]>,
{
    type Source = L::Source;
    type Target = T;

    fn view<O, F: FnOnce(Option<&Self::Target>) -> O>(&self, source: &Self::Source, map: F) -> O {
        self.lens.view(source, |t| if let Some(t) = t { map(t.get(self.index)) } else { map(None) })
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

    fn view<O, F: FnOnce(Option<&Self::Target>) -> O>(&self, source: &Self::Source, map: F) -> O {
        map(source.as_ref())
    }
}

impl<T: 'static> std::fmt::Debug for UnwrapLens<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("unwrap")
    }
}

impl<T: 'static> Hash for UnwrapLens<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let id = TypeId::of::<Self>();
        id.hash(state);
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

    fn view<O, F: FnOnce(Option<&Self::Target>) -> O>(&self, source: &Self::Source, map: F) -> O {
        let converted = source.clone().try_into().ok();
        map(converted.as_ref())
    }
}

impl<T, U> std::fmt::Debug for IntoLens<T, U> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("into")
    }
}

impl<T: 'static, U: 'static> Hash for IntoLens<T, U> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let id = TypeId::of::<Self>();
        id.hash(state);
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

#[derive(Hash, Debug, Copy)]
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

    fn view<O, F: FnOnce(Option<&Self::Target>) -> O>(&self, source: &Self::Source, map: F) -> O {
        self.lens1.view(source, |t1| {
            if let Some(l1) = t1 {
                self.lens2.view(source, |t2| {
                    if let Some(l2) = t2 {
                        map(Some(&(*l1 | *l2)))
                    } else {
                        map(None)
                    }
                })
            } else {
                map(None)
            }
        })
    }
}

impl<L1: Clone, L2: Clone> Clone for OrLens<L1, L2> {
    fn clone(&self) -> Self {
        Self { lens1: self.lens1.clone(), lens2: self.lens2.clone() }
    }
}

#[derive(Hash, Clone)]
pub struct Wrapper<L>(pub L);

impl<L: Copy> Copy for Wrapper<L> {}

impl<L: Lens> Lens for Wrapper<L> {
    type Source = L::Source;
    type Target = L::Target;
    fn view<O, F: FnOnce(Option<&Self::Target>) -> O>(&self, source: &Self::Source, map: F) -> O {
        self.0.view(source, map)
    }
}

impl<L: Lens> std::fmt::Debug for Wrapper<L> {
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

#[derive(Hash, Debug, Copy)]
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

    fn view<O, F: FnOnce(Option<&Self::Target>) -> O>(&self, source: &Self::Source, map: F) -> O {
        self.lens1.view(source, |t1| {
            if let Some(l1) = t1 {
                self.lens2.view(source, |t2| {
                    if let Some(l2) = t2 {
                        map(Some(&(*l1 & *l2)))
                    } else {
                        map(None)
                    }
                })
            } else {
                map(None)
            }
        })
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
