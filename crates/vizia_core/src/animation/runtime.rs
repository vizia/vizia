use crate::entity::Entity;

/// Stable runtime identity for one CSS animation occurrence.
///
/// This is intentionally distinct from [`Animation`](super::Animation), which identifies a declared
/// keyframe definition. Repeated animation names on an entity therefore still have independent IDs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CssAnimationId(pub(crate) u64);

impl CssAnimationId {
    /// Return the stable numeric runtime identifier.
    pub fn get(self) -> u64 {
        self.0
    }
}

/// Observable runtime state for a CSS animation occurrence.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CssAnimationPlaybackState {
    Pending,
    Running,
    Paused,
    Finished,
}

/// Read-only snapshot of a running CSS animation occurrence.
#[derive(Debug, Clone, PartialEq)]
pub struct CssAnimationSnapshot {
    pub id: CssAnimationId,
    pub name: String,
    pub entity: Entity,
    /// Current local effect time in seconds. For progress timelines this is the sampled
    /// progress mapped onto the effect's active duration rather than wall-clock time.
    pub current_time: f32,
    /// Current directed keyframe progress when the effect has a sampled value.
    pub progress: Option<f32>,
    pub playback_rate: f32,
    pub state: CssAnimationPlaybackState,
    /// True when progress comes from a scroll/view/named progress timeline rather than wall time.
    pub timeline_driven: bool,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum CssAnimationControl {
    Pause,
    Resume,
    Seek(f32),
    SetPlaybackRate(f32),
    Reverse,
    Finish,
}
