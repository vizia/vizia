use vizia_id::{
    GENERATIONAL_ID_GENERATION_MASK, GENERATIONAL_ID_INDEX_BITS, GENERATIONAL_ID_INDEX_MASK,
    GenerationalId, impl_generational_id,
};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct MapId(pub(crate) u64);

impl_generational_id!(MapId);
