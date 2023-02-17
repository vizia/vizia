/// A trait for any type which can be bound to, i.e. can be cached and compared against previous
/// versions.
///
/// This type is part of the prelude.
pub trait Data: 'static + Clone {
    fn same(&self, other: &Self) -> bool;
}

impl<T: 'static + PartialEq + Clone> Data for T {
    #[inline]
    fn same(&self, other: &Self) -> bool {
        self.eq(other)
    }
}
