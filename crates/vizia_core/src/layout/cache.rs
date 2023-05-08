use crate::cache::CachedData;
use morphorm::Cache;

use crate::prelude::*;
use bitflags::bitflags;

bitflags! {
    /// Describes the capabilities of a view with respect to user interaction.
    #[derive(Debug, Clone, Copy)]
    pub struct GeoChanged: u8 {
        // Whether a view will be included in hit tests and receive mouse input events.
        const POSX_CHANGED = 1 << 0;
        // Whether a view can be focused to receive keyboard events.
        const POSY_CHANGED = 1 << 1;
        // Whether a view can be checked.
        const WIDTH_CHANGED = 1 << 2;
        // Whether a view can be focused via keyboard navigation.
        const HEIGHT_CHANGED = 1 << 3;
    }
}

impl Cache for CachedData {
    type Node = Entity;

    fn set_bounds(&mut self, node: &Self::Node, posx: f32, posy: f32, width: f32, height: f32) {
        let mut geo_changed = self.geo_changed.get(*node).copied().unwrap();
        if let Some(bounds) = self.bounds.get_mut(*node) {
            if bounds.w != width {
                geo_changed.set(GeoChanged::WIDTH_CHANGED, true);
            }

            if bounds.h != height {
                geo_changed.set(GeoChanged::HEIGHT_CHANGED, true);
            }

            bounds.x = posx;
            bounds.y = posy;
            bounds.w = width;
            bounds.h = height;
        }

        if let Some(relative_position) = self.relative_position.get_mut(*node) {
            if relative_position.x != posx {
                geo_changed.set(GeoChanged::POSX_CHANGED, true);
            }

            if relative_position.y != posy {
                geo_changed.set(GeoChanged::POSY_CHANGED, true);
            }

            relative_position.x = posx;
            relative_position.y = posy;
        }

        if let Some(geo) = self.geo_changed.get_mut(*node) {
            *geo = geo_changed;
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
