use crate::SparseSetIndex;

/// Represents an entry of a sparse set storing the value and the linked key.
#[derive(Debug, Clone)]
pub struct Entry<I, T> {
    pub key: I,
    pub value: T,
}

impl<I, T> Entry<I, T>
where
    I: SparseSetIndex,
{
    pub fn key(&self) -> I {
        self.key
    }

    pub fn value(&self) -> &T {
        &self.value
    }

    pub fn value_mut(&mut self) -> &mut T {
        &mut self.value
    }
}
