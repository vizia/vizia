//! # Layout
//! Layout determines the size and position of entities on the screen and is performed by [morphorm](https://github.com/vizia/morphorm).
//!
pub(crate) mod cache;
pub(crate) mod node;

use crate::prelude::*;
use morphorm::{Cache, Hierarchy};
pub use morphorm::{GeometryChanged, LayoutType, PositionType, Units};

pub(crate) fn geometry_changed(cx: &mut Context, tree: &Tree<Entity>) {
    for node in tree.down_iter() {
        let geometry_changed = cx.cache.geometry_changed(node);
        if !geometry_changed.is_empty() {
            cx.event_queue.push_back(
                Event::new(WindowEvent::GeometryChanged(geometry_changed))
                    .target(node)
                    .propagate(Propagation::Up),
            );
        }

        cx.cache.set_geo_changed(node, morphorm::GeometryChanged::POSX_CHANGED, false);
        cx.cache.set_geo_changed(node, morphorm::GeometryChanged::POSY_CHANGED, false);
        cx.cache.set_geo_changed(node, morphorm::GeometryChanged::WIDTH_CHANGED, false);
        cx.cache.set_geo_changed(node, morphorm::GeometryChanged::HEIGHT_CHANGED, false);
    }
}
