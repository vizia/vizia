use std::ops::Range;

use parley::Affinity;

use crate::entity::Entity;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct TextSelectionPoint {
    pub entity: Entity,
    pub byte: usize,
    pub affinity: Affinity,
}

impl TextSelectionPoint {
    pub(crate) fn new(entity: Entity, byte: usize) -> Self {
        Self { entity, byte, affinity: Affinity::Downstream }
    }

    pub(crate) fn with_affinity(entity: Entity, byte: usize, affinity: Affinity) -> Self {
        Self { entity, byte, affinity }
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct WindowTextSelection {
    pub anchor: Option<TextSelectionPoint>,
    pub focus: Option<TextSelectionPoint>,
    pub preferred_x: Option<f32>,
    pub dragging: bool,
    pub active: bool,
}

impl WindowTextSelection {
    pub(crate) fn points(&self) -> Option<(TextSelectionPoint, TextSelectionPoint)> {
        Some((self.anchor?, self.focus?))
    }

    pub(crate) fn clear(&mut self) {
        *self = Self::default();
    }
}

pub(crate) fn selected_ranges(
    ordered_labels: &[(Entity, usize)],
    anchor: TextSelectionPoint,
    focus: TextSelectionPoint,
) -> Vec<(Entity, Range<usize>)> {
    let Some(anchor_index) = ordered_labels.iter().position(|(entity, _)| *entity == anchor.entity)
    else {
        return Vec::new();
    };
    let Some(focus_index) = ordered_labels.iter().position(|(entity, _)| *entity == focus.entity)
    else {
        return Vec::new();
    };

    let anchor =
        TextSelectionPoint { byte: anchor.byte.min(ordered_labels[anchor_index].1), ..anchor };
    let focus = TextSelectionPoint { byte: focus.byte.min(ordered_labels[focus_index].1), ..focus };

    let (start_index, start_byte, end_index, end_byte) =
        if (anchor_index, anchor.byte) <= (focus_index, focus.byte) {
            (anchor_index, anchor.byte, focus_index, focus.byte)
        } else {
            (focus_index, focus.byte, anchor_index, anchor.byte)
        };

    (start_index..=end_index)
        .filter_map(|index| {
            let (entity, text_len) = ordered_labels[index];
            let start = if index == start_index { start_byte } else { 0 };
            let end = if index == end_index { end_byte } else { text_len };
            (start < end).then_some((entity, start..end))
        })
        .collect()
}

#[cfg(any(feature = "clipboard", test))]
pub(crate) fn selected_text<'a>(
    ranges: &[(Entity, Range<usize>)],
    mut text_for: impl FnMut(Entity) -> Option<&'a str>,
) -> String {
    ranges
        .iter()
        .filter_map(|(entity, range)| text_for(*entity)?.get(range.clone()))
        .collect::<Vec<_>>()
        .join("\n")
}

#[cfg(test)]
mod tests {
    use vizia_id::GenerationalId;

    use super::*;

    fn entity(index: u64) -> Entity {
        Entity::new(index, 0)
    }

    #[test]
    fn derives_forward_cross_label_ranges() {
        let first = entity(1);
        let middle = entity(2);
        let last = entity(3);
        let labels = [(first, 5), (middle, 4), (last, 6)];

        assert_eq!(
            selected_ranges(
                &labels,
                TextSelectionPoint::new(first, 2),
                TextSelectionPoint::new(last, 3),
            ),
            vec![(first, 2..5), (middle, 0..4), (last, 0..3)]
        );
    }

    #[test]
    fn derives_the_same_ranges_for_reverse_selection() {
        let first = entity(1);
        let middle = entity(2);
        let last = entity(3);
        let labels = [(first, 5), (middle, 4), (last, 6)];

        assert_eq!(
            selected_ranges(
                &labels,
                TextSelectionPoint::new(last, 3),
                TextSelectionPoint::new(first, 2),
            ),
            vec![(first, 2..5), (middle, 0..4), (last, 0..3)]
        );
    }

    #[test]
    fn omits_collapsed_and_empty_ranges() {
        let first = entity(1);
        let second = entity(2);
        let labels = [(first, 5), (second, 0)];

        assert!(
            selected_ranges(
                &labels,
                TextSelectionPoint::new(first, 2),
                TextSelectionPoint::new(first, 2),
            )
            .is_empty()
        );
    }

    #[test]
    fn clamps_points_to_visible_text_lengths() {
        let first = entity(1);
        let labels = [(first, 5)];

        assert_eq!(
            selected_ranges(
                &labels,
                TextSelectionPoint::new(first, 2),
                TextSelectionPoint::new(first, usize::MAX),
            ),
            vec![(first, 2..5)]
        );
    }

    #[test]
    fn copies_label_fragments_with_newline_separators() {
        let first = entity(1);
        let second = entity(2);
        let ranges = [(first, 1..4), (second, 0..3)];

        let text = selected_text(&ranges, |entity| match entity {
            value if value == first => Some("hello"),
            value if value == second => Some("world"),
            _ => None,
        });

        assert_eq!(text, "ell\nwor");
    }
}
