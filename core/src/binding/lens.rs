use crate::Model;

pub trait Lens: 'static + Clone + Copy + std::fmt::Debug {
    type Source: Model;
    type Target;

    fn view<'a>(&self, source: &'a Self::Source) -> &'a Self::Target;
}

/// Helpers for constructing more complex `Lens`es.
pub trait LensExt: Lens {
    /// Used to construct a lens to some data contained within some other lensed data.
    ///
    /// # Example
    /// Binds a label to `other_data`, which is a field of a struct `SomeData`, which is a field of the root `AppData` model:
    /// ```
    /// Label::new("").bind(AppData::some_data.then(SomeData::other_data))
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

    fn view<'a>(&self, data: &'a Self::Source) -> &'a Self::Target {
        &self.b.view(&self.a.view(data))
    }
}

impl<T: Clone, U: Clone> Clone for Then<T, U> {
    fn clone(&self) -> Self {
        Self { a: self.a.clone(), b: self.b.clone() }
    }
}

// pub struct And<A,B> {
//     a: A,
//     b: B,
// }

// impl<A,B> And<A,B> {
//     pub fn new(a: A, b: B) -> Self
//     where
//         A: Lens,
//         B: Lens,
//     {
//         Self {
//             a,
//             b,
//         }
//     }
// }

// impl<A,B> Lens for And<A,B>
// where
//     A: Lens,
//     B: Lens<Source = A::Source>,
// {
//     type Source = A::Source;
//     type Target = (A::Target, B::Target);

//     fn view<'a>(&self, data: &'a Self::Source) -> &Self::Target {
//         &(self.a.view(data), self.b.view(data))
//     }
// }

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
