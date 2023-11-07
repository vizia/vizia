use crate::entity::Entity;
use vizia_id::GenerationalId;

/// Trait for converting between an id and an accesskit node.
pub trait IntoNode {
    fn accesskit_id(&self) -> accesskit::NodeId;
}

impl IntoNode for Entity {
    /// Converts an Entity into the corresponding accesskit NodeId.
    fn accesskit_id(&self) -> accesskit::NodeId {
        (self.index() as u64).into()
    }
}
