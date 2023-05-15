use vizia_id::{
    impl_generational_id, GenerationalId, GENERATIONAL_ID_GENERATION_MASK,
    GENERATIONAL_ID_INDEX_BITS, GENERATIONAL_ID_INDEX_MASK,
};

/// An id used to reference style animations stored in style.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Animation(u32);

impl_generational_id!(Animation);
