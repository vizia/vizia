use crate::entity::Entity;
use vizia_id::GenerationalId;

pub trait IntoNode {
    fn accesskit_id(&self) -> accesskit::NodeId;
}

impl IntoNode for Entity {
    fn accesskit_id(&self) -> accesskit::NodeId {
        // Add 1 because the root node has an index of 0
        std::num::NonZeroU64::new(self.index() as u64 + 1).unwrap().into()
    }
}
