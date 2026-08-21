use crate::cache::CachedData;
use morphorm::{Cache, CachedLayout};

use crate::prelude::*;
use bitflags::bitflags;

bitflags! {
    /// Bitflag representing whether the bounds of a view has changed after relayout.
    #[derive(Debug, Clone, Copy)]
    pub struct GeoChanged: u8 {
        /// Flag representing whether the X position of a view has changed.
        const POSX_CHANGED = 1 << 0;
        /// Flag representing whether the Y position of a view has changed.
        const POSY_CHANGED = 1 << 1;
        /// Flag representing whether the width position of a view has changed.
        const WIDTH_CHANGED = 1 << 2;
        /// Flag representing whether the height position of a view has changed.
        const HEIGHT_CHANGED = 1 << 3;
    }
}

impl Cache for CachedData {
    type Node = Entity;

    fn set_bounds(&mut self, node: &Self::Node, posx: f32, posy: f32, width: f32, height: f32) {
        let new_bounds = BoundingBox { x: posx.round(), y: posy.round(), w: width, h: height };
        if let Some(bounds) = self.relative_bounds.get_mut(*node) {
            *bounds = new_bounds;
        }
    }

    fn posx(&self, node: &Self::Node) -> f32 {
        // Morphorm works with positions relative to the parent, and `set_bounds` writes into
        // `relative_bounds`, so these getters must read from `relative_bounds` too. Reading the
        // absolute `bounds` here would make incremental relayout (which restarts from a non-root
        // node and feeds its cached position back through `set_bounds`) double-count the parent
        // offset, shifting the subtree off-screen.
        self.relative_bounds.get(*node).map_or(0.0, |b| b.x)
    }

    fn posy(&self, node: &Self::Node) -> f32 {
        self.relative_bounds.get(*node).map_or(0.0, |b| b.y)
    }

    fn width(&self, node: &Self::Node) -> f32 {
        self.relative_bounds.get(*node).map_or(0.0, |b| b.w)
    }

    fn height(&self, node: &Self::Node) -> f32 {
        self.relative_bounds.get(*node).map_or(0.0, |b| b.h)
    }

    fn cached_layout(&self, node: &Self::Node) -> Option<CachedLayout> {
        self.cached_layout.get(*node).copied()
    }

    fn set_cached_layout(&mut self, node: &Self::Node, layout: CachedLayout) {
        self.cached_layout.insert(*node, layout);
    }

    fn invalidate_cached_layout(&mut self, node: &Self::Node) {
        self.cached_layout.remove(*node);
    }
}
