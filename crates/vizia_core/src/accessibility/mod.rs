use crate::entity::Entity;
use accesskit::NodeId;
use vizia_id::GenerationalId;

/// Trait for converting between an [Entity] and an accesskit [NodeId].
pub trait IntoNode {
    /// Converts an [Entity] into the corresponding accesskit [NodeId].
    fn accesskit_id(&self) -> accesskit::NodeId;
}

impl IntoNode for Entity {
    /// Converts an [Entity] into the corresponding accesskit [NodeId].
    fn accesskit_id(&self) -> accesskit::NodeId {
        NodeId(self.index() as u64)
    }
}
