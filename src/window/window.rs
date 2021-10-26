use crate::{Entity, Event, State, Widget, WindowEvent, apply_hover};

// use crate::systems::{apply_styles, apply_visibility, apply_z_ordering, apply_transform};
// use crate::layout::geometry_changed;


/// A widget which represents the main window.
#[derive(Clone)]
pub struct WindowWidget {}

impl WindowWidget {
    pub fn new() -> Self {
        WindowWidget {}
    }

    pub fn build_window(self, state: &mut State) {
        state.build(Entity::root(), self);
    }
}

impl Widget for WindowWidget {
    type Ret = Entity;
    type Data = ();
    fn on_build(&mut self, _state: &mut State, entity: Entity) -> Self::Ret {
        entity
    }

    fn on_event(&mut self, state: &mut State, _entity: Entity, event: &mut Event) {
        
        if let Some(window_event) = event.message.downcast::<WindowEvent>() {
            match window_event {
                WindowEvent::WindowClose => {
                    println!("Window Close Event");
                }

                WindowEvent::Debug(val) => {
                    println!("{}", val);
                }

                WindowEvent::Restyle => {
                    //state.needs_restyle = true;
                    //println!("Restyle");
                    //apply_styles2(state, &state.tree.clone(), event.origin);
                    // apply_styles(state, &state.tree.clone());
                    // apply_visibility(state, &state.tree.clone());
                    // let start = std::time::Instant::now();
                    let tree = state.tree.clone();
                    apply_styles(state, &tree);
                    // println!("{:.2?} seconds to restyle. {}", start.elapsed(), event.origin);
                }

                WindowEvent::Relayout => {
                    //state.needs_relayout = true;
                    //let start = std::time::Instant::now();
                    let tree = state.tree.clone();
                    state.needs_redraw = true;
                    //println!("Relayout");
                    // apply_z_ordering(state, &state.tree.clone());
                    // apply_visibility(state, &state.tree.clone());
                    // apply_clipping(state, &state.tree.clone());
                    // apply_layout(state, &state.tree.clone());
                    // apply_hover(state);
                    apply_z_ordering(state, &tree);
                    //apply_transform(state, &tree);
                    apply_visibility(state, &tree);
                    //apply_layout(state, &tree);
                    //apply_layout2(state, &tree);
                    morphorm::layout(&mut state.data, &state.tree, &mut state.style);
                    apply_transform(state, &tree);
                    
                    geometry_changed(state, &tree);
                    
                    apply_hover(state);
                    //println!("{:.2?} seconds to relayout. {}", start.elapsed(), event.origin);
                }

                WindowEvent::Redraw => {
                    let tree = state.tree.clone();
                    //apply_z_ordering(state, &tree);
                    apply_transform(state, &tree);
                    state.needs_redraw = true;
                }

                _ => {}
            }
        }
    }
}
