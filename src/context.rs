use std::{collections::{HashMap, VecDeque}, rc::Rc, sync::Arc};

use crate::{CachedData, ContainerHandler, Entity, Event, IdManager, MouseState, NodeHandler, State, StateData, StateID, Style, Tree, TreeExt};



pub struct Context {
    pub entity_manager: IdManager<Entity>,
    pub tree: Tree,
    pub current: Entity,
    pub count: usize,
    pub containers: HashMap<Entity, Box<dyn ContainerHandler>>,
    pub nodes: HashMap<Entity, Box<dyn NodeHandler>>,
    pub container_builders: HashMap<Entity, Rc<dyn Fn(&mut Context)>>,
    pub state: HashMap<StateID, Box<dyn StateData>>,
    pub event_queue: VecDeque<Event>,
    pub style: Style,
    pub cache: CachedData,

    pub mouse: MouseState,

    pub hovered: Entity,

    pub state_count: u32,
    
    //pub data: HashMap<u32, Box<dyn Model>>,
    //pub handlers: HashMap<i32, Box<dyn View>>,
}

impl Context {
    pub fn remove(&mut self, entity: Entity) {
        let delete_list = entity.branch_iter(&self.tree).collect::<Vec<_>>();

        for entity in delete_list.iter().rev() {
            println!("Removing: {}", entity);
            self.tree.remove(*entity).expect("");
            self.cache.remove(*entity);
            //self.style.remove(*entity); TODO
            self.entity_manager.destroy(*entity);
        }
    }
}