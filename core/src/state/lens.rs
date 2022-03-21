use crate::Context;
use std::any::TypeId;
use std::fmt::{Debug, Formatter};
use std::marker::PhantomData;
use std::ops::Deref;
use std::rc::Rc;

pub struct DerefContainer<T>(T);

impl<T> Deref for DerefContainer<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> AsRef<T> for DerefContainer<T> {
    fn as_ref(&self) -> &T {
        &self.0
    }
}

impl<T> DerefContainer<T> {
    pub fn take(self) -> T {
        self.0
    }
}

impl<T: std::fmt::Debug> std::fmt::Debug for DerefContainer<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

/// A Lens allows the construction of a reference to a piece of some data, e.g. a field of a struct.
///
/// When deriving the `Lens` trait on a struct, the derive macro constructs a static type which implements the `Lens` trait for each field.
/// The `view()` method takes a reference to the struct type as input and outputs a reference to the field.
/// This provides a way to specify a binding to a specific field of some application data.
pub trait Lens: 'static + Clone {
    type Source;
    type Target;

    fn view<O, F: FnOnce(Option<&Self::Target>) -> O>(&self, source: &Self::Source, map: F) -> O;
}

pub(crate) trait LensCache: Lens {
    fn cache_key(&self) -> Option<TypeId> {
        if std::mem::size_of::<Self>() == 0 {
            Some(TypeId::of::<Self>())
        } else {
            None
        }
    }
}

impl<T: Lens> LensCache for T {}

/// Helpers for constructing more complex `Lens`es.
pub trait LensExt: Lens {
    /// Retrieve a `DerefContainer` to the lensed data from context.
    ///
    /// The value can be retrieved by de-referencing the container.  
    ///
    /// Example
    /// ```ignore
    /// let value = lens.get(cx);
    /// ```
    fn get(&self, cx: &Context) -> DerefContainer<Self::Target>
    where
        Self::Target: Clone,
    {
        self.view(
            cx.data().expect("Failed to get data from context. Has it been built into the tree?"),
            |t| {
                DerefContainer(
                    t.expect("Lens failed to resolve. Do you want to use LensExt::get_fallible?")
                        .clone(),
                )
            },
        )
    }

    fn get_fallible(&self, cx: &Context) -> Option<DerefContainer<Self::Target>>
    where
        Self::Target: Clone,
    {
        self.view(
            cx.data().expect("Failed to get data from context. Has it been built into the tree?"),
            |t| t.cloned().map(|v| DerefContainer(v)),
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

    fn unwrap<T: 'static>(self) -> Then<Self, UnwrapLens<T>>
    where
        Self: Lens<Target = Option<T>>,
    {
        self.then(UnwrapLens::new())
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
