mod build;
mod draw;
mod event;
mod methods;
mod proxy;

use femtovg::TextContext;
use instant::Instant;
use std::any::Any;
use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet, VecDeque};
use std::error::Error;
use std::sync::Mutex;

#[cfg(feature = "clipboard")]
use copypasta::{nop_clipboard::NopClipboardContext, ClipboardContext, ClipboardProvider};
use fnv::FnvHashMap;
use unic_langid::LanguageIdentifier;

pub use build::*;
pub use draw::*;
pub use event::*;
pub use proxy::*;

use crate::cache::CachedData;
use crate::environment::Environment;
use crate::events::ViewHandler;
use crate::id::IdManager;
use crate::input::{Modifiers, MouseState};
use crate::prelude::*;
use crate::resource::{FontOrId, ImageOrId, ImageRetentionPolicy, ResourceManager, StoredImage};
use crate::state::ModelDataStore;
use crate::storage::sparse_set::SparseSet;
use crate::style::Style;
use crate::tree::TreeExt;

static DEFAULT_THEME: &str = include_str!("../../resources/themes/default_theme.css");
static DEFAULT_LAYOUT: &str = include_str!("../../resources/themes/default_layout.css");
// const DOUBLE_CLICK_INTERVAL: Duration = Duration::from_millis(500);

/// The main storage and control object for a Vizia application.
pub struct Context {
    /// Creates and destroys entities.
    pub(crate) entity_manager: IdManager<Entity>,
    /// The tree of entities.
    pub(crate) tree: Tree,
    /// The current entity being processed.
    pub(crate) current: Entity,
    /// TODO make this private when there's no longer a need to mutate views after building
    /// List of views.
    pub(crate) views: FnvHashMap<Entity, Box<dyn ViewHandler>>,
    /// List of model data.
    pub(crate) data: SparseSet<ModelDataStore>,
    /// The event queue.
    pub(crate) event_queue: VecDeque<Event>,

    pub(crate) listeners:
        HashMap<Entity, Box<dyn Fn(&mut dyn ViewHandler, &mut EventContext, &mut Event)>>,
    pub(crate) style: Style,
    pub(crate) cache: CachedData,
    pub(crate) draw_cache: DrawCache,

    pub(crate) canvases: HashMap<Entity, femtovg::Canvas<femtovg::renderer::OpenGl>>,

    pub(crate) environment: Environment,

    pub(crate) mouse: MouseState,
    pub(crate) modifiers: Modifiers,

    pub(crate) captured: Entity,
    pub(crate) hovered: Entity,
    pub(crate) focused: Entity,
    pub(crate) cursor_icon_locked: bool,

    pub(crate) resource_manager: ResourceManager,

    pub(crate) text_context: TextContext,

    pub(crate) event_proxy: Option<Box<dyn EventProxy>>,

    #[cfg(feature = "clipboard")]
    pub(crate) clipboard: Box<dyn ClipboardProvider>,

    pub(crate) click_time: Instant,
    pub(crate) double_click: bool,
    pub(crate) click_pos: (f32, f32),
}

impl Context {
    /// Creates a new default context.
    pub fn new() -> Self {
        let mut cache = CachedData::default();
        cache.add(Entity::root()).expect("Failed to add entity to cache");

        let mut result = Self {
            entity_manager: IdManager::new(),
            tree: Tree::new(),
            current: Entity::root(),
            views: FnvHashMap::default(),
            data: SparseSet::new(),
            canvases: HashMap::new(),
            style: Style::default(),
            cache,
            draw_cache: DrawCache::new(),
            environment: Environment::new(),
            event_queue: VecDeque::new(),
            listeners: HashMap::default(),
            mouse: MouseState::default(),
            modifiers: Modifiers::empty(),
            captured: Entity::null(),
            hovered: Entity::root(),
            focused: Entity::root(),
            cursor_icon_locked: false,
            resource_manager: ResourceManager::new(),
            text_context: TextContext::default(),

            event_proxy: None,

            #[cfg(feature = "clipboard")]
            clipboard: if let Ok(context) = ClipboardContext::new() {
                Box::new(context)
            } else {
                Box::new(NopClipboardContext::new().unwrap())
            },
            click_time: Instant::now(),
            double_click: false,
            click_pos: (0.0, 0.0),
        };

        result.entity_manager.create();
        result.add_theme(DEFAULT_LAYOUT);
        result.add_theme(DEFAULT_THEME);

        result
    }

    /// Returns the current entity.
    ///
    /// The "current" entity, generally the entity which is currently being built or the entity
    /// which is currently having an event dispatched to it.
    pub fn current(&self) -> Entity {
        self.current
    }

    /// Set the current entity. This is useful in user code when you're performing black magic and
    /// want to trick other parts of the code into thinking you're processing some other part of the
    /// tree.
    pub fn set_current(&mut self, e: Entity) {
        self.current = e;
    }

    /// Temporarily sets the current entity, calls the provided closure, and then resets the current entity back to previous.
    pub(crate) fn with_current(&mut self, e: Entity, f: impl FnOnce(&mut Context)) {
        let prev = self.current;
        self.current = e;
        f(self);
        self.current = prev;
    }

    /// Mark the application as needing to rerun the draw method
    pub fn need_redraw(&mut self) {
        self.style.needs_redraw = true;
    }

    /// Mark the application as needing to recompute view styles
    pub fn need_restyle(&mut self) {
        self.style.needs_restyle = true;
    }

    /// Mark the application as needing to rerun layout computations
    pub fn need_relayout(&mut self) {
        self.style.needs_relayout = true;
    }

    pub fn toggle_class(&mut self, class_name: &str, applied: bool) {
        let current = self.current();
        if let Some(class_list) = self.style.classes.get_mut(current) {
            if applied {
                class_list.insert(class_name.to_string());
            } else {
                class_list.remove(class_name);
            }
        } else if applied {
            let mut class_list = HashSet::new();
            class_list.insert(class_name.to_string());
            self.style.classes.insert(current, class_list).expect("Failed to insert class name");
        }

        self.need_restyle();
        self.style.needs_relayout = true;
        self.style.needs_redraw = true;
    }

    /// Get the contents of the system clipboard. This may fail for a variety of backend-specific
    /// reasons.
    #[cfg(feature = "clipboard")]
    pub fn get_clipboard(&mut self) -> Result<String, Box<dyn Error + Send + Sync + 'static>> {
        self.clipboard.get_contents()
    }

    /// Set the contents of the system clipboard. This may fail for a variety of backend-specific
    /// reasons.
    #[cfg(feature = "clipboard")]
    pub fn set_clipboard(
        &mut self,
        text: String,
    ) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
        self.clipboard.set_contents(text)
    }

    /// Check whether an entity has a given pseudoclass, e.g. detecting if it's checked, selected,
    /// etc.
    pub fn has_pseudo_class(&self, entity: Entity, cls: PseudoClass) -> bool {
        self.style.pseudo_classes.get(entity).copied().unwrap_or_default().contains(cls)
    }

    pub(crate) fn remove_children(&mut self, entity: Entity) {
        let children = entity.child_iter(&self.tree).collect::<Vec<_>>();
        for child in children.into_iter() {
            self.remove(child);
        }
    }

    pub(crate) fn remove(&mut self, entity: Entity) {
        let delete_list = entity.branch_iter(&self.tree).collect::<Vec<_>>();

        if !delete_list.is_empty() {
            self.style.needs_restyle = true;
            self.style.needs_relayout = true;
            self.style.needs_redraw = true;
        }

        for entity in delete_list.iter().rev() {
            for model_store in self.data.dense.iter_mut().map(|entry| &mut entry.value) {
                for (_, lens) in model_store.lenses_dedup.iter_mut() {
                    lens.remove_observer(entity);
                }
                for lens in model_store.lenses_dup.iter_mut() {
                    lens.remove_observer(entity);
                }

                model_store.lenses_dedup.retain(|_, store| store.num_observers() != 0);
                model_store.lenses_dup.retain(|store| store.num_observers() != 0);
            }

            for image in self.resource_manager.images.values_mut() {
                // no need to drop them here. garbage collection happens after draw (policy based)
                image.observers.remove(entity);
            }

            self.tree.remove(*entity).expect("");
            self.cache.remove(*entity);
            self.style.remove(*entity);
            self.data.remove(*entity);
            self.views.remove(entity);
            self.entity_manager.destroy(*entity);

            if self.captured == *entity {
                self.captured = Entity::null();
            }
        }
    }

    /// Send an event containing a message up the tree from the current entity.
    pub fn emit<M: Message>(&mut self, message: M) {
        self.event_queue.push_back(
            Event::new(message)
                .target(self.current)
                .origin(self.current)
                .propagate(Propagation::Up),
        );
    }

    /// Send an event containing a message directly to a specified entity.
    pub fn emit_to<M: Message>(&mut self, target: Entity, message: M) {
        self.event_queue.push_back(
            Event::new(message).target(target).origin(self.current).propagate(Propagation::Direct),
        );
    }

    /// Send an event with custom origin and propagation information.
    pub fn send_event(&mut self, event: Event) {
        self.event_queue.push_back(event);
    }

    /// Add a listener to an entity.
    ///
    /// A listener can be used to handle events which would not normally propagate to the entity.
    /// For example, mouse events when a different entity has captured them. Useful for things like
    /// closing a popup when clicking outside of its bounding box.
    pub fn add_listener<F, W>(&mut self, listener: F)
    where
        W: View,
        F: 'static + Fn(&mut W, &mut EventContext, &mut Event),
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

    /// Add a font from memory to the application.
    pub fn add_font_mem(&mut self, name: &str, data: &[u8]) {
        // TODO - return error
        if self.resource_manager.fonts.contains_key(name) {
            println!("Font already exists");
            return;
        }

        self.resource_manager.fonts.insert(name.to_owned(), FontOrId::Font(data.to_vec()));
    }

    /// Sets the global default font for the application.
    pub fn set_default_font(&mut self, name: &str) {
        self.style.default_font = name.to_string();
    }

    pub fn add_theme(&mut self, theme: &str) {
        self.resource_manager.themes.push(theme.to_owned());

        self.reload_styles().expect("Failed to reload styles");
    }

    pub fn reload_styles(&mut self) -> Result<(), std::io::Error> {
        if self.resource_manager.themes.is_empty() && self.resource_manager.stylesheets.is_empty() {
            return Ok(());
        }

        self.style.remove_rules();

        self.style.rules.clear();

        self.style.clear_style_rules();

        let mut overall_theme = String::new();

        // Reload the stored themes
        for (index, theme) in self.resource_manager.themes.iter().enumerate() {
            if !self.environment.include_default_theme && index == 1 {
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

        //self.environment.needs_rebuild = true;

        Ok(())
    }

    pub fn remove_user_themes(&mut self) {
        self.resource_manager.themes.clear();

        self.add_theme(DEFAULT_LAYOUT);
        self.add_theme(DEFAULT_THEME);
    }

    pub fn add_stylesheet(&mut self, path: &str) -> Result<(), std::io::Error> {
        let style_string = std::fs::read_to_string(path)?;
        self.resource_manager.stylesheets.push(path.to_owned());
        self.style.parse_theme(&style_string);

        Ok(())
    }

    /// Adds a new property animation returning an animation builder
    ///
    /// # Example
    /// Create an animation which animates the `left` property from 0 to 100 pixels in 5 seconds
    /// and play the animation on an entity:
    /// ```ignore
    /// let animation_id = cx.add_animation(instant::Duration::from_secs(5))
    ///     .add_keyframe(0.0, |keyframe| keyframe.set_left(Pixels(0.0)))
    ///     .add_keyframe(1.0, |keyframe| keyframe.set_left(Pixels(100.0)))
    ///     .build();
    /// ```
    pub fn add_animation(&mut self, duration: std::time::Duration) -> AnimationBuilder {
        let id = self.style.animation_manager.create();
        AnimationBuilder::new(id, self, duration)
    }

    // pub fn play_animation(&mut self, animation: Animation) {
    //     self.current.play_animation(self, animation);
    // }

    pub fn set_image_loader<F: 'static + Fn(&mut Context, &str)>(&mut self, loader: F) {
        self.resource_manager.image_loader = Some(Box::new(loader));
    }

    pub fn load_image(
        &mut self,
        path: String,
        image: image::DynamicImage,
        policy: ImageRetentionPolicy,
    ) {
        println!("Load image: {}", path);
        match self.resource_manager.images.entry(path) {
            Entry::Occupied(mut occ) => {
                occ.get_mut().image = ImageOrId::Image(
                    image,
                    femtovg::ImageFlags::REPEAT_X | femtovg::ImageFlags::REPEAT_Y,
                );
                occ.get_mut().dirty = true;
                occ.get_mut().retention_policy = policy;
            }
            Entry::Vacant(vac) => {
                println!("Insert the image into resource manager");
                vac.insert(StoredImage {
                    image: ImageOrId::Image(
                        image,
                        femtovg::ImageFlags::REPEAT_X | femtovg::ImageFlags::REPEAT_Y,
                    ),
                    retention_policy: policy,
                    used: true,
                    dirty: false,
                    observers: HashSet::new(),
                });
            }
        }
        self.style.needs_redraw = true;
        self.style.needs_relayout = true;
    }

    pub fn evict_image(&mut self, path: &str) {
        self.resource_manager.images.remove(path);
        self.style.needs_redraw = true;
        self.style.needs_relayout = true;
    }

    pub fn add_translation(&mut self, lang: LanguageIdentifier, ftl: String) {
        self.resource_manager.add_translation(lang, ftl)
    }

    pub fn spawn<F>(&self, target: F)
    where
        F: 'static + Send + FnOnce(&mut ContextProxy),
    {
        let mut cxp = ContextProxy {
            current: self.current,
            event_proxy: self.event_proxy.as_ref().map(|p| p.make_clone()),
        };

        std::thread::spawn(move || target(&mut cxp));
    }
}

pub(crate) enum InternalEvent {
    Redraw,
    LoadImage {
        path: String,
        image: Mutex<Option<image::DynamicImage>>,
        policy: ImageRetentionPolicy,
    },
}

/// A trait for any Context-like object that lets you access stored model data.
///
/// This lets e.g Lens::get be generic over any of these types.
pub trait DataContext {
    /// Get stored data from the context.
    fn data<T: 'static>(&self) -> Option<&T>;
}

impl DataContext for Context {
    fn data<T: 'static>(&self) -> Option<&T> {
        // return data for the static model
        if let Some(t) = <dyn Any>::downcast_ref::<T>(&()) {
            return Some(t);
        }

        for entity in self.current.parent_iter(&self.tree) {
            //println!("Current: {} {:?}", entity, entity.parent(&self.tree));
            if let Some(data_list) = self.data.get(entity) {
                for (_, model) in data_list.data.iter() {
                    if let Some(data) = model.downcast_ref::<T>() {
                        return Some(data);
                    }
                }
            }

            if let Some(view_handler) = self.views.get(&entity) {
                if let Some(data) = view_handler.downcast_ref::<T>() {
                    return Some(data);
                }
            }
        }

        None
    }
}
