//! # Layout
//! Layout determines the size and position of entities on the screen.
//!
//! All layout calculations are handled by the Morphorm crate.
pub(crate) mod cache;
pub(crate) mod node;

use crate::prelude::*;
use crate::style::SystemFlags;
pub use morphorm::GeometryChanged;
use morphorm::{Cache, Hierarchy};

pub(crate) fn geometry_changed(cx: &mut Context) {
    if cx.style.system_flags.contains(SystemFlags::RELAYOUT) {
        for node in cx.tree.down_iter() {
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

    if cx.style.system_flags.contains(SystemFlags::RELAYOUT)
        || cx.style.system_flags.contains(SystemFlags::RETRANSFORM)
        || cx.style.system_flags.contains(SystemFlags::RECLIP)
        || cx.style.system_flags.contains(SystemFlags::REORDER)
    {
        // A relayout, retransform, or reclip, can cause the element under the cursor to change. So we push a mouse move event here to force
        // a new event cycle and the hover system to trigger.
        if let Some(proxy) = &cx.event_proxy {
            let event = Event::new(WindowEvent::MouseMove(cx.mouse.cursorx, cx.mouse.cursory))
                .target(Entity::root())
                .origin(Entity::root())
                .propagate(Propagation::Up);

            proxy.send(event).expect("Failed to send event");
        }

        cx.style.system_flags.set(SystemFlags::RELAYOUT, false);
        cx.style.system_flags.set(SystemFlags::RETRANSFORM, false);
        cx.style.system_flags.set(SystemFlags::RECLIP, false);
        cx.style.system_flags.set(SystemFlags::REORDER, false);
    }
}
