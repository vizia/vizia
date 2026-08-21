use vizia_style::AnimationTimelineAxis;

/// Snapshot of a Vizia scroll container used as an animation timeline source.
///
/// ScrollView already stores normalized scroll positions, so timeline sampling is O(1) and does
/// not need to walk or measure the full widget tree on every frame.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub(crate) struct ScrollTimelineSource {
    pub x: f32,
    pub y: f32,
    pub inner_width: f32,
    pub inner_height: f32,
    pub container_width: f32,
    pub container_height: f32,
}

impl ScrollTimelineSource {
    pub fn progress(self, axis: AnimationTimelineAxis) -> f32 {
        match axis {
            AnimationTimelineAxis::Block | AnimationTimelineAxis::Y => self.y,
            AnimationTimelineAxis::Inline | AnimationTimelineAxis::X => self.x,
        }
        .clamp(0.0, 1.0)
    }
}

/// Compute a view-progress timeline from layout coordinates and a normalized scroll source.
///
/// Progress is 0 when the subject's leading edge reaches the far edge of the scrollport and 1
/// when the subject's trailing edge leaves through the near edge. This is intentionally a pure
/// helper so headless tests do not depend on a window backend.
pub(crate) fn view_progress(
    subject_start: f32,
    subject_extent: f32,
    viewport_start: f32,
    viewport_extent: f32,
    scroll_offset: f32,
) -> f32 {
    let subject_start = subject_start - scroll_offset;
    let travel = (viewport_extent + subject_extent).max(f32::MIN_POSITIVE);
    ((viewport_start + viewport_extent - subject_start) / travel).clamp(0.0, 1.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scroll_progress_uses_normalized_axis() {
        let source = ScrollTimelineSource { x: 0.25, y: 0.75, ..Default::default() };
        assert_eq!(source.progress(AnimationTimelineAxis::X), 0.25);
        assert_eq!(source.progress(AnimationTimelineAxis::Block), 0.75);
    }

    #[test]
    fn view_progress_crosses_the_viewport() {
        assert_eq!(view_progress(100.0, 50.0, 0.0, 100.0, 0.0), 0.0);
        assert!((view_progress(25.0, 50.0, 0.0, 100.0, 0.0) - 0.5).abs() < 0.001);
        assert_eq!(view_progress(-50.0, 50.0, 0.0, 100.0, 0.0), 1.0);
    }
}
