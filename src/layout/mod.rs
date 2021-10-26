//! # Layout
//! Layout determines the size and position of entities on the screen.
//! 
//! All layout calculations are handled by the Morphorm crate.
pub(crate) mod cache;

pub use morphorm::{GeometryChanged};
pub(crate) use morphorm::Cache;

pub(crate) mod node;

pub(crate) mod hierarchy;

use morphorm::{Hierarchy};
// use crate::{Event, Propagation, State, Tree, WindowEvent};

// pub(crate) fn geometry_changed(state: &mut State, tree: &Tree) {
//     for node in tree.down_iter() {
//         let geometry_changed = state.data.geometry_changed(node);
//         if !geometry_changed.is_empty() {
//             state.insert_event(Event::new(WindowEvent::GeometryChanged(geometry_changed)).target(node).propagate(Propagation::Down));
//         }

//         state.data.set_geo_changed(node, morphorm::GeometryChanged::POSX_CHANGED, false);
//         state.data.set_geo_changed(node, morphorm::GeometryChanged::POSY_CHANGED, false);
//         state.data.set_geo_changed(node, morphorm::GeometryChanged::WIDTH_CHANGED, false);
//         state.data.set_geo_changed(node, morphorm::GeometryChanged::HEIGHT_CHANGED, false);
//     }
// }

