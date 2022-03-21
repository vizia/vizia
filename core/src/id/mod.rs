mod id_manager;
pub(crate) use id_manager::IdManager;

/// Trait implemented by any generational ID
///
/// A generational id has an index, used for indexing into arrays, and a generation, used to check the alive status of the id
pub trait GenerationalId: Copy {
    /// Method for creating an new generational ID from an index and a generation
    fn new(index: usize, generation: usize) -> Self;
    /// Method for retrieving the generational id index
    fn index(&self) -> usize;
    /// Method for retrieving the generational id generation
    fn generation(&self) -> u8;
    /// Returns true is the id is null
    fn is_null(&self) -> bool;
}
