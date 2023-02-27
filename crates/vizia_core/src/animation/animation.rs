use vizia_id::{
    impl_generational_id, GenerationalId, GENERATIONAL_ID_GENERATION_MASK,
    GENERATIONAL_ID_INDEX_BITS, GENERATIONAL_ID_INDEX_MASK,
};

/// An id used to reference style animations stored in context.
///
/// An animation id is returned by `cx.add_animation()` and can be used to configure animations
/// as well as to play, pause, and stop animations on entities (see [`AnimExt`](crate::prelude::AnimExt)).
///
/// This type is part of the prelude.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Animation(u32);

impl_generational_id!(Animation);
