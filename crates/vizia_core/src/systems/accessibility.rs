use std::sync::Arc;

use crate::{accessibility::IntoNode, prelude::*};
use accesskit::{
    kurbo::Rect, Node, NodeId, TextDirection, TextPosition, TextSelection, TreeUpdate,
};
use cosmic_text::Edit;
use unicode_segmentation::UnicodeSegmentation;
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
                // This is a dirty hack because we need the bounds of the inner inner text content
                // which we know is going to be 3 more than the id of the textbox
                let text_content_id = Entity::new(entity.index() as u32 + 3, 0);
                let bounds = cx.cache.get_bounds(text_content_id);

                // We need a child node per line
                let mut children = Vec::new();
                cx.text_context.with_editor(text_content_id, |editor| {
                    let cursor = editor.cursor();
                    let selection = editor.select_opt().unwrap_or(cursor);

                    let mut selection_active_line = node_id;
                    let mut selection_anchor_line = node_id;

                    for (index, line) in editor.buffer().layout_runs().enumerate() {
                        // Concatenate the parent id with the index of the text line to form a unique node id
                        let mut line_id = (entity.index() as u64 + 1) << 32;
                        line_id |= index as u64;
                        let line_id: NodeId = std::num::NonZeroU64::new(line_id).unwrap().into();

                        children.push(line_id);

                        let text = line.text;

                        let mut line_node = Node::default();

                        line_node.role = Role::InlineTextBox;
                        let line_height = editor.buffer().metrics().line_height as f64;
                        line_node.bounds = Some(Rect {
                            x0: bounds.x as f64,
                            y0: bounds.y as f64 + line.line_y as f64
                                - editor.buffer().metrics().font_size as f64,
                            x1: bounds.x as f64 + line.line_w as f64,
                            y1: bounds.y as f64 + line.line_y as f64
                                - editor.buffer().metrics().font_size as f64
                                + line_height,
                        });
                        line_node.text_direction = if line.rtl {
                            Some(TextDirection::RightToLeft)
                        } else {
                            Some(TextDirection::LeftToRight)
                        };

                        let mut character_lengths = Vec::with_capacity(line.glyphs.len());
                        let mut character_positions = Vec::with_capacity(line.glyphs.len());
                        let mut character_widths = Vec::with_capacity(line.glyphs.len());

                        // Get the actual text in the line
                        let first_glyph_pos =
                            line.glyphs.first().map(|glyph| glyph.start).unwrap_or_default();
                        let last_glyph_pos =
                            line.glyphs.last().map(|glyph| glyph.end).unwrap_or_default();

                        let line_text = &text[first_glyph_pos..last_glyph_pos];

                        let word_lengths = line_text
                            .unicode_words()
                            .map(|word| word.len() as u8)
                            .collect::<Vec<_>>();

                        for glyph in line.glyphs.iter() {
                            let length = (glyph.end - glyph.start) as u8;
                            let position = glyph.x;
                            let width = glyph.w;

                            character_lengths.push(length);
                            character_positions.push(position);
                            character_widths.push(width);
                        }

                        line_node.value = Some(line_text.into());
                        line_node.character_lengths = character_lengths.into();
                        line_node.character_positions = Some(character_positions.into());
                        line_node.character_widths = Some(character_widths.into());
                        line_node.word_lengths = word_lengths.into();
                        child_nodes.push((line_id, Arc::new(line_node)));

                        // Check if this line contains the cursor or selection
                        if cursor.line == index {
                            selection_active_line = line_id;
                        }

                        if selection.line == index {
                            selection_anchor_line = line_id;
                        }
                    }

                    let cursor = editor.cursor();

                    node.text_selection = Some(TextSelection {
                        anchor: TextPosition {
                            node: selection_anchor_line,
                            character_index: selection.index,
                        },
                        focus: TextPosition {
                            node: selection_active_line,
                            character_index: cursor.index,
                        },
                    });

                    node.children = children;
                });
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
