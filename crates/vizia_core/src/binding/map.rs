use vizia_id::{
    impl_generational_id, GenerationalId, GENERATIONAL_ID_GENERATION_MASK,
    GENERATIONAL_ID_INDEX_BITS, GENERATIONAL_ID_INDEX_MASK,
};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct MapId(u64);

impl_generational_id!(MapId);
