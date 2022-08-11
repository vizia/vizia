/// Represents an entry of a sparse set storing the value and the linked key.
#[derive(Debug, Clone)]
pub struct Entry<I, V> {
    pub key: I,
    pub value: V,
}
