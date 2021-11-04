use std::{cell::RefCell, collections::{HashMap, VecDeque}, rc::Rc};

use crate::{CachedData, Data, Entity, Event, IdManager, Message, MouseState, Propagation, StateData, StateID, Store, Style, Tree, TreeExt, ViewHandler};

pub struct Context {
    pub entity_manager: IdManager<Entity>,
    pub tree: Tree,
    pub current: Entity,
    pub count: usize,
    pub views: HashMap<Entity, Box<dyn ViewHandler>>,
    pub state: HashMap<StateID, Box<dyn StateData>>,
    pub data: Data,
    pub event_queue: VecDeque<Event>,
    pub style: Rc<RefCell<Style>>,
    pub cache: CachedData,

    pub mouse: MouseState,

    pub hovered: Entity,

    pub state_count: u32,
}

impl Context {
    pub fn remove(&mut self, entity: Entity) {
        let delete_list = entity.branch_iter(&self.tree).collect::<Vec<_>>();

        for entity in delete_list.iter().rev() {
            //println!("Removing: {}", entity);
            self.tree.remove(*entity).expect("");
            self.cache.remove(*entity);
            self.style.borrow_mut().remove(*entity);
            self.data.model_data.remove(*entity);
            self.entity_manager.destroy(*entity);
        }
    }

    /// Get stored data from the context.
    pub fn data<T: 'static>(&self) -> Option<&T> {
        for entity in self.current.parent_iter(&self.tree) {
            if let Some(data_list) = self.data.model_data.get(entity) {
                for model in data_list.iter() {
                    if let Some(store) = model.downcast_ref::<Store<T>>() {
                        return Some(&store.data);
                    }
                }
            }         
        }

        None

    }

    pub fn emit<M: Message>(&mut self, message: M) {
        self.event_queue.push_back(Event::new(message).target(self.current).origin(self.current).propagate(Propagation::Up));
    }
}
