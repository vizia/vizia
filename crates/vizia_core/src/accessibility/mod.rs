use crate::entity::Entity;
use vizia_id::GenerationalId;

/// Trait for converting between an id and an accesskit node.
pub trait IntoNode {
    fn accesskit_id(&self) -> accesskit::NodeId;
}

impl IntoNode for Entity {
    /// Converts an Entity into the corresponding accesskit NodeId.
    fn accesskit_id(&self) -> accesskit::NodeId {
        // Add 1 because the root node has an index of 0 but accesskit uses a `NonZeroU64`.
        std::num::NonZeroU64::new(self.index() as u64 + 1).unwrap().into()
    }
}
