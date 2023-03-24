use crate::cache::CachedData;
use morphorm::Cache;

use crate::prelude::*;

impl Cache for CachedData {
    type Node = Entity;

    fn set_bounds(&mut self, node: &Self::Node, posx: f32, posy: f32, width: f32, height: f32) {
        if let Some(bounds) = self.bounds.get_mut(*node) {
            bounds.x = posx;
            bounds.y = posy;
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
