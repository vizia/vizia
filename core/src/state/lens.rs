use crate::{Context, Model};
use std::any::TypeId;
use std::fmt::{Debug, Formatter};
use std::marker::PhantomData;
use std::ops::Deref;
use std::rc::Rc;

/// A Lens allows the construction of a reference to a field of a struct.
///
/// When deriving the `Lens` trait on a struct, the derive macro constructs a static type which implements the `Lens` trait for each field.
/// The `view()` method takes a reference to the struct type as input and outputs a reference to the field.
/// This provides a way to specify a binding to a specific field of some application data.
pub trait Lens: 'static + Clone {
    type Source;
    type Target;

    fn view<'a, O, F: FnOnce(&'a Self::Target) -> O>(&self, source: &'a Self::Source, map: F) -> O;
}

/// Helpers for constructing more complex `Lens`es.
pub trait LensExt: Lens {
    fn get<'a>(&self, cx: &'a Context) -> &'a Self::Target {
        self.view(cx.data().expect("Failed to get data from context. Has it been built into the tree?"), |target| target)
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
        Other: Lens + Sized,
        Self: Sized,
    {
        Then::new(self, other)
    }

    // fn and<Other>(self, other: Other) -> And<Self, Other>
    // where
    //     Other: Lens + Sized,
    //     Self: Sized,
    // {
    //     And::new(self, other)
    // }

    // TODO
    fn index<A,I>(self, index: I) -> Then<Self,Index<A,I>>
    where
        A: 'static + std::ops::Index<I>,
        I: 'static + Clone,
        <A as std::ops::Index<I>>::Output: Sized,
    {
        self.then(Index::new(index))
    }

    fn map<L,G,B: Clone>(self, get: G) -> Map<Self,B>
    where
        G: 'static + Fn(&Self::Target) -> B,
    {
        Map::new(self, get)
    }
}

// Implement LensExt for all types which implement Lens
impl<T: Lens> LensExt for T {}

#[derive(Clone)]
pub struct Map<L: Lens, B> {
    get: Rc<dyn Fn(&L::Target) -> B>,
    lens: L,
    p: PhantomData<B>,
}

impl<L: Lens, B> Map<L,B> {
    pub fn new<F>(lens: L, get: F) -> Self 
    where
        F: 'static + Fn(&L::Target) -> B,
    {
        Self {
            get: Rc::new(get),
            lens,
            p: PhantomData::default(),
        }
    }
}

impl<L: Lens, B: 'static + Clone> Lens for Map<L,B>
where
    L: Lens,
    <L as Lens>::Target: Clone,
{

    type Source = L::Source;
    type Target = B;

    fn view<'a, O, F: FnOnce(&'a Self::Target) -> O>(&self, source: &'a Self::Source, map: F) -> O {
        
        map(&(self.get)(self.lens.view(source, |t| t)))
    }
}

/// `Lens` composed of two lenses joined together
#[derive(Debug, Copy)]
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

    fn view<'a,O,F: FnOnce(&'a Self::Target) -> O>(&self, data: &'a Self::Source, map: F) -> O{
        self.a.view(data, |t| self.b.view(t, map))
    }
}

impl<T: Clone, U: Clone> Clone for Then<T, U> {
    fn clone(&self) -> Self {
        Self { a: self.a.clone(), b: self.b.clone() }
    }
}

pub struct Index<A,I> {
    index: I,
    p: PhantomData<A>,
}

impl<A,I> Index<A,I> {
    pub fn new(index: I) -> Self {
        Self { index, p: PhantomData::default() }
    }

    pub fn idx(&self) -> I 
    where I: Clone,
    {
        self.index.clone()
    }
}

impl<A,I: Clone> Clone for Index<A,I> {
    fn clone(&self) -> Self {
        Self { index: self.index.clone(), p: PhantomData::default() }
    }
}

// impl<A,I> Copy for Index<A,I> {}

// impl<A,I> Debug for Index<A,I> {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         f.debug_struct("Index").field("index", &self.index).finish()
//     }
// }

impl<A,I> Lens for Index<A,I>
where
    A: 'static + std::ops::Index<I>,
    I: 'static + Clone,
    <A as std::ops::Index<I>>::Output: Sized,
{
    type Source = A;
    type Target = A::Output;

    fn view<'a,O,F: FnOnce(&'a Self::Target) -> O>(&self, data: &'a Self::Source, map: F) -> O {
        // &self.input.view(data)[self.index]
        map(&data[self.index.clone()])

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

    fn view<'a,O,F: FnOnce(&'a Self::Target) -> O>(&self, _source: &'a Self::Source, map: F) -> O {
        map(self.data)
    }
}

impl<T> StaticLens<T> {
    pub fn new(data: &'static T) -> Self {
        StaticLens { data }
    }
}
