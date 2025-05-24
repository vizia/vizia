use crate::cache::CachedData;
use morphorm::Cache;

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

fn to_finite_number(value: f32) -> f32 {
    if value.is_finite() {
        value
    } else {
        0.0
    }
}

impl Cache for CachedData {
    type Node = Entity;

    fn set_bounds(&mut self, node: &Self::Node, posx: f32, posy: f32, width: f32, height: f32) {
        if let Some(bounds) = self.relative_bounds.get_mut(*node) {
            bounds.x = to_finite_number(posx.round());
            bounds.y = to_finite_number(posy.round());
            bounds.w = to_finite_number(width);
            bounds.h = to_finite_number(height);
        }
    }

    fn posx(&self, node: &Self::Node) -> f32 {
        self.get_posx(*node)
    }

    fn posy(&self, node: &Self::Node) -> f32 {
        self.get_posy(*node)
    }

    fn width(&self, node: &Self::Node) -> f32 {
        self.get_width(*node)
    }

    fn height(&self, node: &Self::Node) -> f32 {
        self.get_height(*node)
    }
}
