use crate::Model;
use std::any::TypeId;
use std::fmt::{Debug, Formatter};

/// A Lens allows the construction of a reference to a field of a struct.
///
/// When deriving the `Lens` trait on a struct, the derive macro constructs a static type which implements the `Lens` trait for each field.
/// The `view()` method takes a reference to the struct type as input and outputs a reference to the field.
/// This provides a way to specify a binding to a specific field of some application data.
pub trait Lens: 'static + Clone + Copy + std::fmt::Debug {
    type Source: Model;
    type Target;

    fn view<'a>(&self, source: &'a Self::Source) -> Option<&'a Self::Target>;
}

/// Helpers for constructing more complex `Lens`es.
pub trait LensExt: Lens {
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
    // fn index<I: 'static>(self, index: I) -> Then<Self, Index<Self::Target, I>>
    // where
    //     Self: Sized,
    //     I: Clone,
    //     Self::Target: std::ops::Index<I> + Sized,
    //     <<Self as Lens>::Target as std::ops::Index<I>>::Output: Sized + Clone,
    // {
    //     Then::new(self, Index::new(index))
    // }
}

// Implement LensExt for all types which implement Lens
impl<T: Lens> LensExt for T {}

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

    fn view<'a>(&self, data: &'a Self::Source) -> Option<&'a Self::Target> {
        self.a.view(data).and_then(|intermediate| self.b.view(intermediate))
    }
}

impl<T: Clone, U: Clone> Clone for Then<T, U> {
    fn clone(&self) -> Self {
        Self { a: self.a.clone(), b: self.b.clone() }
    }
}

// pub struct Index<T,I> {
//     index: I,
//     output: PhantomData<T>,
// }

// impl<T,I> Index<T,I> {
//     pub fn new(index: I) -> Self {
//         Self {
//             index,
//             output: PhantomData::default(),
//         }
//     }
// }

// impl<T,I> Lens for Index<T,I>
// where

//     T: 'static + std::ops::Index<I> + Sized,
//     I: 'static + Clone,
//     <T as std::ops::Index<I>>::Output: Sized + Clone,
// {
//     type Source = T;
//     type Target = <T as std::ops::Index<I>>::Output;

//     fn view<'a>(&self, data: &'a Self::Source) -> &'a Self::Target {
//         &data[self.index.clone()]
//     }
// }

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

    fn view<'a>(&self, _source: &'a Self::Source) -> Option<&'a Self::Target> {
        Some(self.data)
    }
}

impl<T> StaticLens<T> {
    pub fn new(data: &'static T) -> Self {
        StaticLens { data }
    }
}
