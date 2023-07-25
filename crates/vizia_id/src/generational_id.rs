/// The bits used for the index.
pub const GENERATIONAL_ID_INDEX_BITS: u64 = 48;

/// The mask of the bits used for the index.
pub const GENERATIONAL_ID_INDEX_MASK: u64 = (1 << GENERATIONAL_ID_INDEX_BITS) - 1;

/// The bits used for the generation.
pub const GENERATIONAL_ID_GENERATION_BITS: u64 = 16;

/// The mask of the bits used for the generation.
pub const GENERATIONAL_ID_GENERATION_MASK: u64 = (1 << GENERATIONAL_ID_GENERATION_BITS) - 1;

/// A trait implemented by any generational id.
///
/// A generational id has an index and a generation. The index is used for accessing
/// arrays and the generation is used to check if the id is still valid or alive.
pub trait GenerationalId: Copy + PartialEq {
    /// Creates a new generational id from an index and a generation.
    fn new(index: u64, generation: u64) -> Self;

    /// Returns the index of the generational id.
    ///
    /// This is used to access the data of the generational id inside of an array.
    fn index(&self) -> usize;

    /// Returns the generation of the generational id.
    ///
    /// This is used to determine whether this generational id is still valid.
    fn generation(&self) -> u16;

    /// Creates a null or invalid generational id.
    ///
    /// A null id can be used as a place holder.
    fn null() -> Self;

    /// Returns `true` is the generational id is null.
    fn is_null(&self) -> bool;

    /// Returns the root id usually referring to the first id (e.g. Entity(0)).
    fn root() -> Self;
}

#[macro_export]
macro_rules! impl_generational_id {
    ($impl_type: ty) => {
        impl Default for $impl_type {
            fn default() -> Self {
                GenerationalId::null()
            }
        }

        impl std::fmt::Display for $impl_type {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "{}", GenerationalId::index(self))
            }
        }

        impl std::fmt::Debug for $impl_type {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(
                    f,
                    concat!(stringify!($impl_type), " (index: {}, generation: {})"),
                    GenerationalId::index(self),
                    GenerationalId::generation(self),
                )
            }
        }

        impl GenerationalId for $impl_type {
            fn new(index: u64, generation: u64) -> Self {
                assert!(index < GENERATIONAL_ID_INDEX_MASK);
                assert!(generation < GENERATIONAL_ID_GENERATION_MASK);
                Self(index | generation << GENERATIONAL_ID_INDEX_BITS)
            }

            fn index(&self) -> usize {
                (self.0 & GENERATIONAL_ID_INDEX_MASK) as usize
            }

            fn generation(&self) -> u16 {
                ((self.0 >> GENERATIONAL_ID_INDEX_BITS) & GENERATIONAL_ID_GENERATION_MASK) as u16
            }

            fn null() -> Self {
                Self(u64::MAX)
            }

            fn is_null(&self) -> bool {
                *self == Self::null()
            }

            fn root() -> Self {
                Self(0)
            }
        }
    };
}

pub use impl_generational_id;
