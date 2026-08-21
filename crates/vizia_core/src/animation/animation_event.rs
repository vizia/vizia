use crate::entity::Entity;

/// Lifecycle stage emitted by a CSS keyframe animation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnimationEventKind {
    Start,
    Iteration,
    End,
    Cancel,
}

/// Event emitted once per `animation-name`, not once per animated property.
#[derive(Debug, Clone, PartialEq)]
pub struct AnimationEvent {
    pub kind: AnimationEventKind,
    pub name: String,
    pub elapsed_time: f32,
    pub entity: Entity,
}
