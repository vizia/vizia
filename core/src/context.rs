use std::{
    cell::RefCell,
    collections::{HashMap, VecDeque},
    rc::Rc,
};

use femtovg::{FontId, TextContext};
// use fluent_bundle::{FluentBundle, FluentResource};
// use unic_langid::LanguageIdentifier;

use crate::{
    AppData, CachedData, Entity, Enviroment, Event, FontOrId, IdManager, Message, Modifiers,
    MouseState, Propagation, ResourceManager, Store, Style, Tree, TreeExt, View, ViewHandler,
};

static DEFAULT_THEME: &str = include_str!("default_theme.css");

#[derive(Default)]
pub struct Context {
    pub entity_manager: IdManager<Entity>,
    pub tree: Tree,
    pub current: Entity,
    pub count: usize,
    pub views: HashMap<Entity, Box<dyn ViewHandler>>,
    //pub lenses: HashMap<TypeId, Box<dyn LensWrap>>,
    pub data: AppData,
    pub event_queue: VecDeque<Event>,
    pub listeners: HashMap<Entity, Box<dyn Fn(&mut dyn ViewHandler, &mut Context, &mut Event)>>,
    pub style: Style,
    pub cache: CachedData,

    pub enviroment: Enviroment,

    pub mouse: MouseState,
    pub modifiers: Modifiers,

    pub captured: Entity,
    pub hovered: Entity,
    pub focused: Entity,

    // pub state_count: u32,
    pub resource_manager: ResourceManager,

    // Temp
    pub fonts: Vec<FontId>,

    pub text_context: TextContext,
}

impl Context {

    pub fn remove_children(&mut self, entity: Entity) {
        let children = entity.child_iter(&self.tree).collect::<Vec<_>>();
        for child in children.into_iter() {
            self.remove(child);
        }
    }

    pub fn remove(&mut self, entity: Entity) {
        let delete_list = entity.branch_iter(&self.tree).collect::<Vec<_>>();

        if !delete_list.is_empty() {
            self.style.needs_restyle = true;
            self.style.needs_relayout = true;
            self.style.needs_redraw = true;
        }

        for entity in delete_list.iter().rev() {
            // Remove from observers
            for entry in self.data.model_data.dense.iter_mut() {
                let model_list = &mut entry.value;
                for (_, model) in model_list.data.iter_mut() {
                    model.remove_observer(*entity);
                }
            }

            //println!("Removing: {}", entity);
            self.tree.remove(*entity).expect("");
            self.cache.remove(*entity);
            self.style.remove(*entity);
            self.data.model_data.remove(*entity);
            self.entity_manager.destroy(*entity);
            self.views.remove(entity);
        }


    }

    /// Get stored data from the context.
    pub fn data<T: 'static>(&self) -> Option<&T> {
        for entity in self.current.parent_iter(&self.tree) {
            //println!("Current: {} {:?}", entity, entity.parent(&self.tree));
            if let Some(data_list) = self.data.model_data.get(entity) {
                for (_, model) in data_list.data.iter() {
                    if let Some(store) = model.downcast_ref::<Store<T>>() {
                        return Some(&store.data);
                    }
                }
            }
        }

        None
    }

    pub fn emit<M: Message>(&mut self, message: M) {
        self.event_queue.push_back(
            Event::new(message)
                .target(self.current)
                .origin(self.current)
                .propagate(Propagation::Up),
        );
    }

    pub fn add_listener<F, W>(&mut self, listener: F)
    where
        W: View,
        F: 'static + Fn(&mut W, &mut Context, &mut Event),
    {
        self.listeners.insert(
            self.current,
            Box::new(move |event_handler, context, event| {
                if let Some(widget) = event_handler.downcast_mut::<W>() {
                    (listener)(widget, context, event);
                }
            }),
        );
    }

    pub fn emit_trace<M: Message>(&mut self, message: M) {
        self.event_queue.push_back(
            Event::new(message)
                .target(self.current)
                .origin(self.current)
                .propagate(Propagation::Up)
                .trace(),
        );
    }

    /// Add a font from memory to the application
    pub fn add_font_mem(&mut self, name: &str, data: &[u8]) {
        // TODO - return error
        if self.resource_manager.fonts.contains_key(name) {
            println!("Font already exists");
            return;
        }
        //let id = self.text_context.add_font_mem(&data.clone()).expect("failed");
        //println!("{} {:?}", name, id);
        self.resource_manager.fonts.insert(name.to_owned(), FontOrId::Font(data.to_vec()));
    }

    /// Sets the global default font for the application
    pub fn set_default_font(&mut self, name: &str) {
        self.style.default_font = name.to_string();
    }

    pub fn add_theme(&mut self, theme: &str) {
        self.resource_manager.themes.push(theme.to_owned());

        self.reload_styles().expect("Failed to reload styles");
    }

    pub fn remove_user_themes(&mut self) {
        self.resource_manager.themes.clear();

        self.add_theme(DEFAULT_THEME);
    }

    pub fn add_stylesheet(&mut self, path: &str) -> Result<(), std::io::Error> {
        let style_string = std::fs::read_to_string(path.clone())?;
        self.resource_manager.stylesheets.push(path.to_owned());
        self.style.parse_theme(&style_string);

        Ok(())
    }

    pub fn reload_styles(&mut self) -> Result<(), std::io::Error> {
        if self.resource_manager.themes.is_empty() && self.resource_manager.stylesheets.is_empty() {
            return Ok(());
        }

        self.style.remove_rules();

        self.style.rules.clear();

        self.style.remove_all();

        let mut overall_theme = String::new();

        // Reload the stored themes
        for (index, theme) in self.resource_manager.themes.iter().enumerate() {
            if !self.enviroment.include_default_theme && index == 0 {
                continue;
            }

            //self.style.parse_theme(theme);
            overall_theme += theme;
        }

        // Reload the stored stylesheets
        for stylesheet in self.resource_manager.stylesheets.iter() {
            let theme = std::fs::read_to_string(stylesheet)?;
            overall_theme += &theme;
        }

        self.style.parse_theme(&overall_theme);

        self.enviroment.needs_rebuild = true;

        // Entity::root().restyle(self);
        // Entity::root().relayout(self);
        // Entity::root().redraw(self);

        Ok(())
    }
}
