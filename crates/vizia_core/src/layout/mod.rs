//! # Layout
//! Layout determines the size and position of entities on the screen.
//!
//! All layout calculations are handled by the Morphorm crate.
pub(crate) mod cache;
pub(crate) mod node;

use crate::prelude::*;
use crate::style::SystemFlags;

pub(crate) fn geometry_changed(cx: &mut Context) {
    if cx.style.system_flags.contains(SystemFlags::RELAYOUT) {
        for node in cx.tree.into_iter() {
            cx.event_queue.push_back(
                Event::new(WindowEvent::GeometryChanged(true))
                    .target(node)
                    .propagate(Propagation::Up),
            );
        }
    }

    if cx.style.system_flags.contains(SystemFlags::RELAYOUT) {
        // A relayout, retransform, or reclip, can cause the element under the cursor to change. So we push a mouse move event here to force
        // a new event cycle and the hover system to trigger.
        #[cfg(feature = "winit")]
        if let Some(proxy) = &cx.event_proxy {
            let event = Event::new(WindowEvent::MouseMove(cx.mouse.cursorx, cx.mouse.cursory))
                .target(Entity::root())
                .origin(Entity::root())
                .propagate(Propagation::Up);

            proxy.send(event).expect("Failed to send event");
        }

        cx.style.system_flags.set(SystemFlags::RELAYOUT, false);
    }
}
