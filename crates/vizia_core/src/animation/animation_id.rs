use vizia_id::{
    impl_generational_id, GenerationalId, GENERATIONAL_ID_GENERATION_MASK,
    GENERATIONAL_ID_INDEX_BITS, GENERATIONAL_ID_INDEX_MASK,
};

use crate::context::EventContext;

/// An ID used to reference style animations stored in the style store.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct AnimationId(u64);

impl_generational_id!(AnimationId);

/// Trait for getting the animation id From an [Animation] or an animation name.
pub trait AnimId {
    /// Returns the animation associated with the id.
    fn get(&self, cx: &EventContext) -> Option<AnimationId>;
}

impl AnimId for AnimationId {
    fn get(&self, _cx: &EventContext) -> Option<AnimationId> {
        Some(*self)
    }
}

impl AnimId for &'static str {
    fn get(&self, cx: &EventContext) -> Option<AnimationId> {
        cx.style.get_animation(self).copied()
    }
}
