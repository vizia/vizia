//! The `Context` is where the retained data of a vizia application lives.

mod access;
#[doc(hidden)]
pub mod backend;
mod draw;
mod event;
mod proxy;
mod resource;

use instant::Instant;
use std::any::{Any, TypeId};
use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet, VecDeque};
use std::iter::once;
use std::path::Path;
use std::sync::Mutex;

#[cfg(all(feature = "clipboard", feature = "x11"))]
use copypasta::ClipboardContext;
#[cfg(feature = "clipboard")]
use copypasta::{nop_clipboard::NopClipboardContext, ClipboardProvider};
use cosmic_text::{fontdb::Database, Attrs, AttrsList, BufferLine, FamilyOwned};
use fnv::FnvHashMap;
use replace_with::replace_with_or_abort;

use unic_langid::LanguageIdentifier;

pub use access::*;
pub use draw::*;
pub use event::*;
pub use proxy::*;
pub use resource::*;

use crate::binding::BindingHandler;
use crate::cache::CachedData;
use crate::environment::{Environment, ThemeMode};
use crate::events::ViewHandler;
#[cfg(feature = "embedded_fonts")]
use crate::fonts;
use crate::model::ModelDataStore;
use crate::prelude::*;
use crate::resource::{ImageOrId, ImageRetentionPolicy, ResourceManager, StoredImage};
use crate::style::{PseudoClassFlags, Style};
use crate::text::{TextConfig, TextContext};
use vizia_id::{GenerationalId, IdManager};
use vizia_input::{Modifiers, MouseState};
use vizia_storage::TreeExt;
use vizia_storage::{ChildIterator, SparseSet};

static DEFAULT_LAYOUT: &str = include_str!("../../resources/themes/default_layout.css");
static DARK_THEME: &str = include_str!("../../resources/themes/dark_theme.css");
static LIGHT_THEME: &str = include_str!("../../resources/themes/light_theme.css");

type Views = FnvHashMap<Entity, Box<dyn ViewHandler>>;
type Models = SparseSet<ModelDataStore>;
type Bindings = FnvHashMap<Entity, Box<dyn BindingHandler>>;

/// The main storage and control object for a Vizia application.
pub struct Context {
    pub(crate) entity_manager: IdManager<Entity>,
    pub(crate) entity_identifiers: HashMap<String, Entity>,
    pub(crate) tree: Tree<Entity>,
    pub(crate) current: Entity,
    pub(crate) views: Views,
    pub(crate) data: Models,
    pub(crate) bindings: Bindings,
    pub(crate) event_queue: VecDeque<Event>,
    pub(crate) tree_updates: Vec<accesskit::TreeUpdate>,
    pub(crate) listeners:
        HashMap<Entity, Box<dyn Fn(&mut dyn ViewHandler, &mut EventContext, &mut Event)>>,
    pub(crate) global_listeners: Vec<Box<dyn Fn(&mut EventContext, &mut Event)>>,
    pub(crate) style: Style,
    pub(crate) cache: CachedData,

    pub(crate) canvases: HashMap<Entity, crate::prelude::Canvas>,
    pub(crate) mouse: MouseState<Entity>,
    pub(crate) modifiers: Modifiers,

    pub(crate) captured: Entity,
    pub(crate) triggered: Entity,
    pub(crate) hovered: Entity,
    pub(crate) focused: Entity,
    pub(crate) focus_stack: Vec<Entity>,
    pub(crate) cursor_icon_locked: bool,

    pub(crate) resource_manager: ResourceManager,

    pub(crate) text_context: TextContext,
    pub(crate) text_config: TextConfig,

    pub(crate) event_proxy: Option<Box<dyn EventProxy>>,

    /// The window's size in logical pixels, before `user_scale_factor` gets applied to it. If this
    /// value changed during a frame then the window will be resized and a
    /// [`WindowEvent::GeometryChanged`] will be emitted.
    pub(crate) window_size: WindowSize,
    /// A scale factor used for uniformly scaling the window independently of any HiDPI scaling.
    /// `window_size` gets multplied with this factor to get the actual logical window size. If this
    /// changes during a frame, then the window will be resized at the end of the frame and a
    /// [`WindowEvent::GeometryChanged`] will be emitted. This can be initialized using
    /// [`WindowDescription::user_scale_factor`][crate::WindowDescription::user_scale_factor].
    pub(crate) user_scale_factor: f64,

    #[cfg(feature = "clipboard")]
    pub(crate) clipboard: Box<dyn ClipboardProvider>,

    pub(crate) click_time: Instant,
    pub(crate) clicks: usize,
    pub(crate) click_pos: (f32, f32),

    pub ignore_default_theme: bool,
    pub window_has_focus: bool,

    pub(crate) drop_data: Option<DropData>,
}

impl Default for Context {
    fn default() -> Self {
        Context::new(WindowSize::new(800, 600), 1.0)
    }
}

impl Context {
    /// Creates a new context.
    pub fn new(window_size: WindowSize, user_scale_factor: f64) -> Self {
        let mut cache = CachedData::default();
        cache.add(Entity::root()).expect("Failed to add entity to cache");

        let mut db = Database::new();
        db.load_system_fonts();

        // Add default fonts if the feature is enabled.
        #[cfg(feature = "embedded_fonts")]
        {
            db.load_font_data(Vec::from(fonts::ROBOTO_REGULAR));
            db.load_font_data(Vec::from(fonts::ROBOTO_BOLD));
            db.load_font_data(Vec::from(fonts::ROBOTO_ITALIC));
            db.load_font_data(Vec::from(fonts::TABLER_ICONS));
        }

        let mut result = Self {
            entity_manager: IdManager::new(),
            entity_identifiers: HashMap::new(),
            tree: Tree::new(),
            current: Entity::root(),
            views: FnvHashMap::default(),
            data: SparseSet::new(),
            bindings: FnvHashMap::default(),
            style: Style::default(),
            cache,
            canvases: HashMap::new(),
            event_queue: VecDeque::new(),
            tree_updates: Vec::new(),
            listeners: HashMap::default(),
            global_listeners: vec![],
            mouse: MouseState::default(),
            modifiers: Modifiers::empty(),
            captured: Entity::null(),
            triggered: Entity::null(),
            hovered: Entity::root(),
            focused: Entity::root(),
            focus_stack: Vec::new(),
            cursor_icon_locked: false,
            resource_manager: ResourceManager::new(),
            text_context: TextContext::new_from_locale_and_db(
                sys_locale::get_locale().unwrap_or_else(|| "en-US".to_owned()),
                db,
            ),

            text_config: TextConfig::default(),

            event_proxy: None,

            window_size,
            user_scale_factor,

            #[cfg(feature = "clipboard")]
            clipboard: {
                #[cfg(feature = "x11")]
                if let Ok(context) = ClipboardContext::new() {
                    Box::new(context)
                } else {
                    Box::new(NopClipboardContext::new().unwrap())
                }
                #[cfg(not(feature = "x11"))]
                Box::new(NopClipboardContext::new().unwrap())
            },
            click_time: Instant::now(),
            clicks: 0,
            click_pos: (0.0, 0.0),

            ignore_default_theme: false,
            window_has_focus: true,

            drop_data: None,
        };

        result.style.needs_restyle();
        result.style.needs_relayout();
        result.style.needs_redraw();

        // Build the environment model at the root.
        Environment::new().build(&mut result);

        result.entity_manager.create();
        result.set_default_font(&["Roboto Regular"]);

        result.style.roles.insert(Entity::root(), Role::Window).unwrap();

        result
    }

    /// The "current" entity, generally the entity which is currently being built or the entity
    /// which is currently having an event dispatched to it.
    pub fn current(&self) -> Entity {
        self.current
    }

    /// Set the current entity. This is useful in user code when you're performing black magic and
    /// want to trick other parts of the code into thinking you're processing some other part of the
    /// tree.
    pub(crate) fn set_current(&mut self, e: Entity) {
        self.current = e;
    }

    /// Makes the above black magic more explicit
    pub fn with_current(&mut self, e: Entity, f: impl FnOnce(&mut Context)) {
        let prev = self.current;
        self.current = e;
        f(self);
        self.current = prev;
    }

    /// Returns a reference to the [Environment] model.
    pub fn environment(&self) -> &Environment {
        self.data::<Environment>().unwrap()
    }

    /// The window's size in logical pixels, before
    /// [`user_scale_factor()`][Self::user_scale_factor()] gets applied to it. If this value changed
    /// during a frame then the window will be resized and a [`WindowEvent::GeometryChanged`] will be
    /// emitted.
    pub fn window_size(&self) -> WindowSize {
        self.window_size
    }

    /// A scale factor used for uniformly scaling the window independently of any HiDPI scaling.
    /// `window_size` gets multplied with this factor to get the actual logical window size. If this
    /// changes during a frame, then the window will be resized at the end of the frame and a
    /// [`WindowEvent::GeometryChanged`] will be emitted. This can be initialized using
    /// [`WindowDescription::user_scale_factor`][crate::WindowDescription::user_scale_factor].
    pub fn user_scale_factor(&self) -> f64 {
        self.user_scale_factor
    }

    /// Returns the scale factor of the display.
    pub fn scale_factor(&self) -> f32 {
        self.style.dpi_factor as f32
    }

    /// Mark the application as needing to rerun the draw method
    pub fn needs_redraw(&mut self) {
        self.style.needs_redraw();
    }

    /// Mark the application as needing to recompute view styles
    pub fn needs_restyle(&mut self) {
        self.style.needs_restyle();
    }

    /// Mark the application as needing to rerun layout computations
    pub fn needs_relayout(&mut self) {
        self.style.needs_relayout();
    }

    /// Enables or disables PseudoClasses for the focus of an entity
    pub(crate) fn set_focus_pseudo_classes(
        &mut self,
        focused: Entity,
        enabled: bool,
        focus_visible: bool,
    ) {
        #[cfg(debug_assertions)]
        if enabled {
            println!(
            "Focus changed to {:?} parent: {:?}, view: {}, posx: {}, posy: {} width: {} height: {}",
            focused,
            self.tree.get_parent(focused),
            self.views
                .get(&focused)
                .map_or("<None>", |view| view.element().unwrap_or("<Unnamed>")),
            self.cache.get_posx(focused),
            self.cache.get_posy(focused),
            self.cache.get_width(focused),
            self.cache.get_height(focused),
        );
        }

        if let Some(pseudo_classes) = self.style.pseudo_classes.get_mut(focused) {
            pseudo_classes.set(PseudoClassFlags::FOCUS, enabled);
            if !enabled || focus_visible {
                pseudo_classes.set(PseudoClassFlags::FOCUS_VISIBLE, enabled);
            }
        }

        for ancestor in focused.parent_iter(&self.tree) {
            let entity = ancestor;
            if let Some(pseudo_classes) = self.style.pseudo_classes.get_mut(entity) {
                pseudo_classes.set(PseudoClassFlags::FOCUS_WITHIN, enabled);
            }
        }
    }

    /// Sets application focus to the current entity with the specified focus visiblity
    pub fn focus_with_visibility(&mut self, focus_visible: bool) {
        let old_focus = self.focused;
        let new_focus = self.current;
        self.set_focus_pseudo_classes(old_focus, false, focus_visible);
        if self.current != self.focused {
            self.emit_to(old_focus, WindowEvent::FocusOut);
            self.emit_to(new_focus, WindowEvent::FocusIn);
            self.focused = self.current;
        }
        self.set_focus_pseudo_classes(new_focus, true, focus_visible);

        self.style.needs_restyle();
    }

    /// Sets application focus to the current entity using the previous focus visibility
    pub fn focus(&mut self) {
        let focused = self.focused;
        let old_focus_visible = self
            .style
            .pseudo_classes
            .get_mut(focused)
            .filter(|class| class.contains(PseudoClassFlags::FOCUS_VISIBLE))
            .is_some();
        self.focus_with_visibility(old_focus_visible)
    }

    /// Removes the children of the provided entity from the application.
    pub(crate) fn remove_children(&mut self, entity: Entity) {
        let child_iter = ChildIterator::new(&self.tree, entity);
        let children = child_iter.collect::<Vec<_>>();
        for child in children.into_iter() {
            self.remove(child);
        }
    }

    /// Removes the provided entity from the application.
    pub(crate) fn remove(&mut self, entity: Entity) {
        let delete_list = entity.branch_iter(&self.tree).collect::<Vec<_>>();

        if !delete_list.is_empty() {
            self.style.needs_restyle();
            self.style.needs_relayout();
            self.style.needs_redraw();
        }

        for entity in delete_list.iter().rev() {
            if let Some(binding) = self.bindings.remove(entity) {
                binding.remove(self);

                self.bindings.insert(*entity, binding);
            }

            for image in self.resource_manager.images.values_mut() {
                // no need to drop them here. garbage collection happens after draw (policy based)
                image.observers.remove(entity);
            }

            if let Some(identifier) = self.style.ids.get(*entity) {
                self.entity_identifiers.remove(identifier);
            }

            if let Some(index) = self.focus_stack.iter().position(|r| r == entity) {
                self.focus_stack.remove(index);
            }

            if self.focused == *entity {
                if let Some(new_focus) = self.focus_stack.pop() {
                    self.with_current(new_focus, |cx| cx.focus());
                } else {
                    self.with_current(Entity::root(), |cx| cx.focus());
                }
            }

            if self.captured == *entity {
                self.captured = Entity::null();
            }

            self.tree.remove(*entity).expect("");
            self.cache.remove(*entity);
            self.style.remove(*entity);
            self.data.remove(*entity);
            self.views.remove(entity);
            self.entity_manager.destroy(*entity);
            self.text_context.clear_buffer(*entity);
        }
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

    /// Adds a global listener to the application.
    ///
    /// Global listeners have the first opportunity to handle every event that is sent in an
    /// application. They will *never* be removed. If you need a listener tied to the lifetime of a
    /// view, use `add_listener`.
    pub fn add_global_listener<F>(&mut self, listener: F)
    where
        F: 'static + Fn(&mut EventContext, &mut Event),
    {
        self.global_listeners.push(Box::new(listener));
    }

    /// Add a font from memory to the application.
    ///
    ///
    /// The `include_bytes!()` macro can be used to embed a font into the application binary.
    /// # Example
    /// ```
    /// # use vizia_core::prelude::*;
    /// # let cx = &mut Context::default();
    /// cx.add_fonts_mem(&[include_bytes!("../../resources/fonts/Roboto-Regular.ttf")]);
    /// ```
    pub fn add_fonts_mem(&mut self, data: &[&[u8]]) {
        self.text_context.take_buffers();
        replace_with_or_abort(&mut self.text_context, |mut ccx| {
            let buffers = ccx.take_buffers();
            let (locale, mut db) = ccx.into_font_system().into_locale_and_db();
            for font_data in data {
                db.load_font_data(Vec::from(*font_data));
            }
            let mut new_ccx = TextContext::new_from_locale_and_db(locale, db);
            for (entity, lines) in buffers {
                new_ccx.with_buffer(entity, move |_, buf| {
                    buf.lines = lines
                        .into_iter()
                        .map(|line| BufferLine::new(line, AttrsList::new(Attrs::new())))
                        .collect();
                });
            }
            new_ccx
        });
    }

    /// Sets the global default font for the application.
    pub fn set_default_font(&mut self, names: &[&str]) {
        self.style.default_font = names
            .iter()
            .map(|x| FamilyOwned::Name(x.to_string()))
            .chain(once(FamilyOwned::SansSerif))
            .collect();
    }

    /// Add a style string to the application.
    pub fn add_theme(&mut self, theme: &str) {
        self.resource_manager.themes.push(theme.to_owned());

        EventContext::new(self).reload_styles().expect("Failed to reload styles");
    }

    pub fn add_stylesheet(&mut self, path: impl AsRef<Path>) -> Result<(), std::io::Error> {
        let style_string = std::fs::read_to_string(path.as_ref())?;
        self.resource_manager.stylesheets.push(path.as_ref().to_owned());
        self.style.parse_theme(&style_string);
        Ok(())
    }

    /// Remove all user themes from the application.
    pub fn remove_user_themes(&mut self) {
        self.resource_manager.themes.clear();

        self.add_theme(DEFAULT_LAYOUT);
        if !self.ignore_default_theme {
            let environment = self.data::<Environment>().expect("Failed to get environment");
            match environment.theme_mode {
                ThemeMode::LightMode => self.add_theme(LIGHT_THEME),
                ThemeMode::DarkMode => self.add_theme(DARK_THEME),
            }
        }
    }

    pub fn reload_styles(&mut self) -> Result<(), std::io::Error> {
        if self.resource_manager.themes.is_empty() && self.resource_manager.stylesheets.is_empty() {
            return Ok(());
        }

        self.style.remove_rules();

        self.style.clear_style_rules();

        let mut overall_theme = String::new();

        // Reload the stored themes
        for theme in self.resource_manager.themes.iter() {
            overall_theme += theme;
        }

        // Reload the stored stylesheets
        for stylesheet in self.resource_manager.stylesheets.iter() {
            let theme = std::fs::read_to_string(stylesheet)?;
            overall_theme += &theme;
        }

        self.style.parse_theme(&overall_theme);

        self.style.needs_restyle();
        self.style.needs_relayout();
        self.style.needs_redraw();

        Ok(())
    }

    pub fn set_image_loader<F: 'static + Fn(&mut ResourceContext, &str)>(&mut self, loader: F) {
        self.resource_manager.image_loader = Some(Box::new(loader));
    }

    pub fn add_translation(&mut self, lang: LanguageIdentifier, ftl: impl ToString) {
        self.resource_manager.add_translation(lang, ftl.to_string());
        self.emit(EnvironmentEvent::SetLocale(self.resource_manager.language.clone()));
    }

    pub fn load_image(
        &mut self,
        path: &str,
        image: image::DynamicImage,
        policy: ImageRetentionPolicy,
    ) {
        match self.resource_manager.images.entry(path.to_string()) {
            Entry::Occupied(mut occ) => {
                occ.get_mut().image = ImageOrId::Image(
                    image,
                    femtovg::ImageFlags::REPEAT_X | femtovg::ImageFlags::REPEAT_Y,
                );
                occ.get_mut().dirty = true;
                occ.get_mut().retention_policy = policy;
            }
            Entry::Vacant(vac) => {
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
        self.style.needs_relayout();
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

    pub fn get_proxy(&self) -> ContextProxy {
        ContextProxy {
            current: self.current,
            event_proxy: self.event_proxy.as_ref().map(|p| p.make_clone()),
        }
    }

    /// Finds the entity that identifier identifies
    pub fn resolve_entity_identifier(&self, identity: &str) -> Option<Entity> {
        self.entity_identifiers.get(identity).cloned()
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
    /// Get model/view data from the context. Returns `None` if the data does not exist.
    fn data<T: 'static>(&self) -> Option<&T>;
}

pub trait EmitContext {
    /// Send an event containing the provided message up the tree from the current entity.
    fn emit<M: Any + Send>(&mut self, message: M);
    /// Send an event containing the provided message directly to a specified entity from the current entity.
    fn emit_to<M: Any + Send>(&mut self, target: Entity, message: M);
    /// Send a custom event with custom origin and propagation information.
    fn emit_custom(&mut self, event: Event);
}

impl DataContext for Context {
    fn data<T: 'static>(&self) -> Option<&T> {
        // return data for the static model.
        if let Some(t) = <dyn Any>::downcast_ref::<T>(&()) {
            return Some(t);
        }

        for entity in self.current.parent_iter(&self.tree) {
            // Return any model data.
            if let Some(model_data_store) = self.data.get(entity) {
                if let Some(model) = model_data_store.models.get(&TypeId::of::<T>()) {
                    return model.downcast_ref::<T>();
                }
            }

            // Return any view data.
            if let Some(view_handler) = self.views.get(&entity) {
                if let Some(data) = view_handler.downcast_ref::<T>() {
                    return Some(data);
                }
            }
        }

        None
    }
}

impl EmitContext for Context {
    fn emit<M: Any + Send>(&mut self, message: M) {
        self.event_queue.push_back(
            Event::new(message)
                .target(self.current)
                .origin(self.current)
                .propagate(Propagation::Up),
        );
    }

    fn emit_to<M: Any + Send>(&mut self, target: Entity, message: M) {
        self.event_queue.push_back(
            Event::new(message).target(target).origin(self.current).propagate(Propagation::Direct),
        );
    }

    fn emit_custom(&mut self, event: Event) {
        self.event_queue.push_back(event);
    }
}
