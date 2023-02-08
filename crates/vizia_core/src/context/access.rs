use accesskit::{NodeBuilder, NodeId, Rect, TextDirection, TextSelection};

use crate::{cache::CachedData, prelude::*, text::TextContext};

// A context used for configuring the accessibility features of a view.
pub struct AccessContext<'a> {
    pub(crate) current: Entity,
    pub(crate) cache: &'a CachedData,
    pub(crate) text_context: &'a mut TextContext,
}

/// Wrapper around an accesskit node builder, a node id, and a list of children to be added to the node
pub struct AccessNode {
    pub(crate) node_id: NodeId,
    pub(crate) node_builder: NodeBuilder,
    pub(crate) children: Vec<AccessNode>,
}

impl AccessNode {
    pub fn new_from_parent(parent_id: NodeId, index: usize) -> Self {
        // Concatenate the parent id with the index of the text line to form a unique node id
        let mut node_id = (parent_id.0.get() as u64) << 32;
        node_id |= index as u64;
        let node_id: NodeId = std::num::NonZeroU64::new(node_id).unwrap().into();

        Self { node_id, node_builder: NodeBuilder::default(), children: Vec::new() }
    }

    pub fn node_id(&self) -> NodeId {
        self.node_id
    }

    pub fn add_child(&mut self, child: AccessNode) {
        self.children.push(child);
    }

    pub fn set_role(&mut self, role: Role) {
        self.node_builder.set_role(role);
    }

    pub fn set_text_direction(&mut self, text_direction: TextDirection) {
        self.node_builder.set_text_direction(text_direction);
    }

    pub fn set_text_selection(&mut self, text_selection: TextSelection) {
        self.node_builder.set_text_selection(text_selection);
    }

    // TODO: switch to bounding box
    pub fn set_bounds(&mut self, bounds: Rect) {
        self.node_builder.set_bounds(bounds);
    }

    pub fn set_value(&mut self, value: impl Into<Box<str>>) {
        self.node_builder.set_value(value);
    }

    pub fn set_character_lengths(&mut self, character_lengths: impl Into<Box<[u8]>>) {
        self.node_builder.set_character_lengths(character_lengths);
    }

    pub fn set_character_positions(&mut self, character_positions: impl Into<Box<[f32]>>) {
        self.node_builder.set_character_positions(character_positions);
    }

    pub fn set_character_widths(&mut self, character_widths: impl Into<Box<[f32]>>) {
        self.node_builder.set_character_widths(character_widths);
    }

    pub fn set_word_lengths(&mut self, word_lengths: impl Into<Box<[u8]>>) {
        self.node_builder.set_word_lengths(word_lengths);
    }
}
