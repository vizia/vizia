use vizia_id::{
    impl_generational_id, GenerationalId, GENERATIONAL_ID_GENERATION_MASK,
    GENERATIONAL_ID_INDEX_BITS, GENERATIONAL_ID_INDEX_MASK,
};

/// A rule is an id used to get/set shared style properties in State.
///
/// Rather than having widgets own their data, all state is stored in a single database and
/// is stored and loaded using entities.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct Rule(u64);

impl_generational_id!(Rule);
