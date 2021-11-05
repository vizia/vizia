use std::{cell::RefCell, collections::{HashMap, VecDeque}, io::Read, rc::Rc};

use fluent_bundle::{FluentBundle, FluentResource};
use unic_langid::LanguageIdentifier;

use crate::{CachedData, Data, Entity, Event, IdManager, Message, MouseState, Propagation, StateData, StateID, Store, Style, Tree, TreeExt, ViewHandler};

pub struct Enviroment {
    // Signifies whether the app should be rebuilt
    // Changing an enviroment variable requires a rebuild of the app
    pub needs_rebuild: bool,
    pub bundle: FluentBundle<FluentResource>,
}

impl Enviroment {
    pub fn new() -> Self {
        let lang =  "en-US".parse::<LanguageIdentifier>().expect("Failed to parse locale");
        let resolved_locales = vec![&lang];
        let mut bundle = FluentBundle::new(resolved_locales.into_iter().cloned().collect());
        let mut file = std::fs::File::open("examples/resources/en-US/hello.ftl").expect("No File Found");
        let mut source: String = String::new();
        file.read_to_string(&mut source);
        let resource = FluentResource::try_new(source).expect("Could not parse an FTL string.");
        bundle
            .add_resource(resource)
            .expect("Failed to add FTL resources to the bundle.");
        Self {
            needs_rebuild: true,
            bundle,
        }
    }

    pub fn set_locale(&mut self, locale: &str) {
        let lang =  locale.parse::<LanguageIdentifier>().expect("Failed to parse locale");
        let resolved_locales = vec![&lang];
        let mut bundle = FluentBundle::new(resolved_locales.into_iter().cloned().collect());
        let mut file = std::fs::File::open(&format!("examples/resources/{}/hello.ftl", locale)).expect("No File Found");
        let mut source: String = String::new();
        file.read_to_string(&mut source);
        let resource = FluentResource::try_new(source).expect("Could not parse an FTL string.");
        bundle
            .add_resource(resource)
            .expect("Failed to add FTL resources to the bundle.");
        self.bundle = bundle;
        self.needs_rebuild = true;
    }
}

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

    pub enviroment: Enviroment,

    pub mouse: MouseState,

    pub hovered: Entity,
    pub focused: Entity,

    pub state_count: u32,
}

impl Context {
    pub fn remove(&mut self, entity: Entity) {
        let delete_list = entity.branch_iter(&self.tree).collect::<Vec<_>>();

        for entity in delete_list.iter().rev() {

            // Remove from observers
            for entry in self.data.model_data.dense.iter_mut() {
                let model_list = &mut entry.value;
                for model in model_list.iter_mut() {
                    model.remove_observer(*entity);
                }
            }

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
            //println!("Current: {} {:?}", entity, entity.parent(&self.tree));
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
