pub trait SparseSetIndex: PartialEq + Copy + Clone {
    fn new(index: usize) -> Self;
    fn null() -> Self;
    fn index(&self) -> usize;
    fn is_null(&self) -> bool {
        *self == Self::null()
    }
}

impl SparseSetIndex for usize {
    fn new(index: usize) -> Self {
        index
    }

    fn null() -> Self {
        usize::MAX
    }

    fn index(&self) -> usize {
        *self
    }
}
