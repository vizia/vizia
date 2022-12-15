use std::sync::Arc;

use crate::{
    accessibility::IntoNode,
    prelude::*,
    text::{measure_text_lines, text_layout, text_paint_general, Selection},
};
use accesskit::{
    kurbo::Rect, Node, NodeId, TextDirection, TextPosition, TextSelection, TreeUpdate,
};
use vizia_storage::LayoutTreeIterator;

// Updates node properties from view properties
// Should be run after layout so that things like bounding box are correct
// This system doesn't change the structure of the accessibility tree as this is done when views are built/removed
// TODO: Change this to incrementally update nodes when required instead of updating all nodes every frame
pub fn accessibility_system(cx: &mut Context, tree: &Tree<Entity>) {
    let iterator = LayoutTreeIterator::full(tree);

    for entity in iterator {
        let node_id = entity.accesskit_id();
        let mut node = cx.get_node(entity);

        let navigable = cx
            .style
            .abilities
            .get(entity)
            .copied()
            .unwrap_or_default()
            .contains(Abilities::NAVIGABLE);

        if cx.style.roles.get(entity).is_none() && !navigable {
            continue;
        }

        // println!("ENTITY: {} NODE: {:?} \n", entity, node);

        let mut child_nodes = Vec::new();

        // Here we need to construct the correct text edit nodes for each wrapped line of text
        if let Some(role) = cx.style.roles.get(entity) {
            if *role == Role::TextField {
                if let Some(text) = cx.style.text_value.get(entity) {
                    // This is a dirty hack because we need the bounds of the inner inner text content
                    // which we know is going to be 3 more than the id of the textbox
                    let text_content_id = Entity::new(entity.index() as u32 + 3, 0);
                    let bounds = cx.cache.get_bounds(text_content_id);
                    let selection = cx
                        .style
                        .text_selection
                        .get(text_content_id)
                        .copied()
                        .unwrap_or(Selection::caret(0));
                    let mut selection_active_line = node_id;
                    let mut selection_anchor_line = node_id;
                    let mut selection_active_cursor = 0;
                    let mut selection_anchor_cursor = 0;
                    // Compute the rows of text
                    let text_paint = text_paint_general(&cx.style, &cx.resource_manager, entity);
                    if let Ok((text_layout, new_lines)) =
                        text_layout(bounds.width(), &text, &text_paint, &cx.text_context)
                    {
                        let text_lines = measure_text_lines(
                            text,
                            &text_paint,
                            text_layout.as_ref(),
                            bounds.x,
                            bounds.y,
                            &cx.text_context,
                        );

                        let mut children = Vec::new();

                        let mut cursor = 0;

                        for ((index, line), has_new_line) in
                            text_lines.iter().enumerate().zip(new_lines.iter())
                        {
                            // Concatenate the parent id with the index of the text line to form a unique node id
                            let mut line_id = (entity.index() as u64 + 1) << 32;
                            line_id |= index as u64;

                            let line_id: NodeId =
                                std::num::NonZeroU64::new(line_id).unwrap().into();

                            children.push(line_id);

                            let range = &text_layout[index];

                            // println!(
                            //     "line: {} range: {:?} active: {} anchor: {}",
                            //     index, range, selection.active, selection.anchor
                            // );

                            if range.contains(&selection.active) {
                                selection_active_line = line_id;
                                selection_active_cursor = selection.active - cursor;
                            }

                            if range.contains(&selection.anchor) {
                                selection_anchor_line = line_id;
                                selection_anchor_cursor = selection.anchor - cursor;
                            }

                            let txt = text.as_str();
                            let mut line_str =
                                (&txt[text_layout[index].start..text_layout[index].end]).to_owned();

                            let mut line_node = Node::default();

                            line_node.role = Role::InlineTextBox;
                            line_node.bounds = Some(Rect {
                                x0: line.x as f64,
                                y0: line.y as f64,
                                x1: (line.x + line.width()) as f64,
                                y1: (line.y + line.height()) as f64,
                            });
                            line_node.text_direction = Some(TextDirection::LeftToRight);

                            let mut word_lengths = Vec::new();

                            let mut character_lengths = Vec::with_capacity(line.glyphs.len());
                            let mut character_positions = Vec::with_capacity(line.glyphs.len());
                            let mut character_widths = Vec::with_capacity(line.glyphs.len());

                            let mut was_at_word_end = false;
                            let mut last_word_start = 0usize;

                            for glyph in line.glyphs.iter() {
                                let length = glyph.c.len_utf8() as u8;
                                if index != text_lines.len() - 1 {
                                    cursor += length as usize;
                                }
                                let position = glyph.x - bounds.x;
                                let width = glyph.width;

                                let is_word_char = glyph.c.is_alphanumeric();
                                if is_word_char && was_at_word_end {
                                    word_lengths
                                        .push((character_lengths.len() - last_word_start) as u8);
                                    last_word_start = character_lengths.len();
                                }

                                was_at_word_end = !is_word_char;

                                character_lengths.push(length);
                                character_positions.push(position);
                                character_widths.push(width);
                            }

                            if *has_new_line {
                                line_str += "\n";
                                character_lengths.push(1);
                                character_positions.push(line.width());
                                character_widths.push(0.0);
                            }

                            word_lengths.push((character_lengths.len() - last_word_start) as u8);

                            line_node.value = Some(line_str.into());
                            line_node.character_lengths = character_lengths.into();
                            line_node.character_positions = Some(character_positions.into());
                            line_node.character_widths = Some(character_widths.into());
                            line_node.word_lengths = word_lengths.into();
                            child_nodes.push((line_id, Arc::new(line_node)));
                        }

                        if selection_active_line == node_id {
                            selection_active_line = child_nodes.last().unwrap().0;
                            selection_active_cursor = selection.active - cursor;
                        }

                        if selection_anchor_line == node_id {
                            selection_anchor_line = child_nodes.last().unwrap().0;
                            selection_anchor_cursor = selection.anchor - cursor;
                        }

                        node.text_selection = Some(TextSelection {
                            anchor: TextPosition {
                                node: selection_anchor_line,
                                character_index: selection_anchor_cursor,
                            },
                            focus: TextPosition {
                                node: selection_active_line,
                                character_index: selection_active_cursor,
                            },
                        });

                        // println!("{:?}", node.text_selection);

                        node.children = children;
                    }
                }
            }
        }

        let mut nodes = vec![(node_id, Arc::new(node))];

        // If child nodes were generated then append them to the nodes list
        if !child_nodes.is_empty() {
            nodes.extend(child_nodes.into_iter());
        }

        cx.tree_updates.push(TreeUpdate {
            nodes,
            tree: None,
            focus: cx.window_has_focus.then_some(cx.focused.accesskit_id()),
        });
    }
}
