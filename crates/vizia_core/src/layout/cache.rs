use crate::cache::CachedData;
use morphorm::Cache;

use crate::prelude::*;
use bitflags::bitflags;

bitflags! {
    ///
    #[derive(Debug, Clone, Copy)]
    pub struct GeoChanged: u8 {
        //
        const POSX_CHANGED = 1 << 0;
        //
        const POSY_CHANGED = 1 << 1;
        //
        const WIDTH_CHANGED = 1 << 2;
        //
        const HEIGHT_CHANGED = 1 << 3;
    }
}

impl Cache for CachedData {
    type Node = Entity;

    fn set_bounds(&mut self, node: &Self::Node, posx: f32, posy: f32, width: f32, height: f32) {
        if let Some(bounds) = self.relative_bounds.get_mut(*node) {
            bounds.x = posx.round();
            bounds.y = posy.round();
            bounds.w = width;
            bounds.h = height;
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
