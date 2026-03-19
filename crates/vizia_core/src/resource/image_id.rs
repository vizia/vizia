use vizia_id::{
    GENERATIONAL_ID_GENERATION_MASK, GENERATIONAL_ID_INDEX_BITS, GENERATIONAL_ID_INDEX_MASK,
    GenerationalId, impl_generational_id,
};

/// An ID used to reference images in the resource manager.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct ImageId(u64);

impl_generational_id!(ImageId);
