pub mod backend;
mod draw;
mod event;
mod proxy;

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

pub use draw::*;
pub use event::*;
pub use proxy::*;

use crate::cache::CachedData;
use crate::environment::Environment;
use crate::events::ViewHandler;
use crate::fonts;
use crate::prelude::*;
use crate::resource::{ImageOrId, ImageRetentionPolicy, ResourceManager, StoredImage};
use crate::state::{BindingHandler, ModelDataStore};
use crate::style::Style;
use crate::text::TextContext;
use vizia_id::{GenerationalId, IdManager};
use vizia_input::{Modifiers, MouseState};
use vizia_storage::SparseSet;
use vizia_storage::TreeExt;

static DEFAULT_THEME: &str = include_str!("../../resources/themes/default_theme.css");
static DEFAULT_LAYOUT: &str = include_str!("../../resources/themes/default_layout.css");

/// The main storage and control object for a Vizia application.
///
/// This type is part of the prelude.
pub struct Context {
    pub(crate) entity_manager: IdManager<Entity>,
    pub(crate) entity_identifiers: HashMap<String, Entity>,
    pub(crate) tree: Tree<Entity>,
    pub(crate) current: Entity,
    /// TODO make this private when there's no longer a need to mutate views after building
    pub views: FnvHashMap<Entity, Box<dyn ViewHandler>>,
    pub(crate) data: SparseSet<ModelDataStore>,
    pub(crate) bindings: FnvHashMap<Entity, Box<dyn BindingHandler>>,
    pub(crate) event_queue: VecDeque<Event>,
    pub(crate) listeners:
        HashMap<Entity, Box<dyn Fn(&mut dyn ViewHandler, &mut EventContext, &mut Event)>>,
    pub(crate) global_listeners: Vec<Box<dyn Fn(&mut EventContext, &mut Event)>>,
    pub(crate) style: Style,
    pub(crate) cache: CachedData,
    pub(crate) draw_cache: DrawCache,

    pub(crate) canvases: HashMap<Entity, crate::prelude::Canvas>,
    //environment: Environment,
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
}

impl Default for Context {
    fn default() -> Self {
        Context::new(WindowSize::new(800, 600), 1.0)
    }
}

impl Context {
    pub fn new(window_size: WindowSize, user_scale_factor: f64) -> Self {
        let mut cache = CachedData::default();
        cache.add(Entity::root()).expect("Failed to add entity to cache");

        // Add default fonts
        let mut db = Database::new();
        db.load_system_fonts();
        db.load_font_data(Vec::from(fonts::ROBOTO_REGULAR));
        db.load_font_data(Vec::from(fonts::ROBOTO_BOLD));
        db.load_font_data(Vec::from(fonts::ROBOTO_ITALIC));
        db.load_font_data(Vec::from(fonts::ENTYPO));
        db.load_font_data(Vec::from(fonts::OPEN_SANS_EMOJI));
        db.load_font_data(Vec::from(fonts::AMIRI_REGULAR));
        db.load_font_data(Vec::from(fonts::MATERIAL_ICONS_REGULAR));

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
            draw_cache: DrawCache::new(),
            canvases: HashMap::new(),
            // environment: Environment::new(),
            event_queue: VecDeque::new(),
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
        };

        Environment::new().build(&mut result);

        result.entity_manager.create();
        result.set_default_font(&["Roboto"]);

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
    pub fn set_current(&mut self, e: Entity) {
        self.current = e;
    }

    /// Makes the above black magic more explicit
    pub(crate) fn with_current(&mut self, e: Entity, f: impl FnOnce(&mut Context)) {
        let prev = self.current;
        self.current = e;
        f(self);
        self.current = prev;
    }

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

    /// Enables or disables pseudoclasses for the focus of an entity
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
            pseudo_classes.set(PseudoClass::FOCUS, enabled);
            if !enabled || focus_visible {
                pseudo_classes.set(PseudoClass::FOCUS_VISIBLE, enabled);
            }
        }

        for ancestor in focused.parent_iter(&self.tree) {
            let entity = ancestor;
            if let Some(pseudo_classes) = self.style.pseudo_classes.get_mut(entity) {
                pseudo_classes.set(PseudoClass::FOCUS_WITHIN, enabled);
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

        self.style.needs_relayout = true;
        self.style.needs_redraw = true;
        self.style.needs_restyle = true;
    }

    /// Sets application focus to the current entity using the previous focus visibility
    pub fn focus(&mut self) {
        let focused = self.focused;
        let old_focus_visible = self
            .style
            .pseudo_classes
            .get_mut(focused)
            .filter(|class| class.contains(PseudoClass::FOCUS_VISIBLE))
            .is_some();
        self.focus_with_visibility(old_focus_visible)
    }

    /// Sets the checked flag of the current entity
    pub fn set_selected(&mut self, flag: bool) {
        let current = self.current();
        if let Some(pseudo_classes) = self.style.pseudo_classes.get_mut(current) {
            pseudo_classes.set(PseudoClass::SELECTED, flag);
        }

        self.style.needs_restyle = true;
        self.style.needs_relayout = true;
        self.style.needs_redraw = true;
    }

    pub(crate) fn remove_children(&mut self, entity: Entity) {
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

            if self.focused == *entity
                && delete_list.contains(&self.tree.lock_focus_within(*entity))
            {
                if let Some(new_focus) = self.focus_stack.pop() {
                    self.with_current(new_focus, |cx| cx.focus());
                }
            }

            self.tree.remove(*entity).expect("");
            self.cache.remove(*entity);
            self.draw_cache.remove(*entity);
            self.style.remove(*entity);
            self.data.remove(*entity);
            self.views.remove(entity);
            self.entity_manager.destroy(*entity);
            self.text_context.clear_buffer(*entity);

            if self.captured == *entity {
                self.captured = Entity::null();
            }
        }
    }

    /// Check whether there are any events in the queue waiting for the next event dispatch cycle.
    // pub fn has_queued_events(&self) -> bool {
    //     !self.event_queue.is_empty()
    // }

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
                new_ccx.with_buffer(entity, move |buf| {
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

    pub fn add_theme(&mut self, theme: &str) {
        self.resource_manager.themes.push(theme.to_owned());

        EventContext::new(self).reload_styles().expect("Failed to reload styles");
    }

    pub fn remove_user_themes(&mut self) {
        self.resource_manager.themes.clear();

        self.add_theme(DEFAULT_LAYOUT);
        if !self.ignore_default_theme {
            self.add_theme(DEFAULT_THEME);
        }
    }

    pub fn add_stylesheet(&mut self, path: impl AsRef<Path>) -> Result<(), std::io::Error> {
        let style_string = std::fs::read_to_string(path.as_ref())?;
        self.resource_manager.stylesheets.push(path.as_ref().to_owned());
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

    pub fn set_image_loader<F: 'static + Fn(&mut Context, &str)>(&mut self, loader: F) {
        self.resource_manager.image_loader = Some(Box::new(loader));
    }

    pub fn load_image(
        &mut self,
        path: String,
        image: image::DynamicImage,
        policy: ImageRetentionPolicy,
    ) {
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

    pub fn add_translation(&mut self, lang: LanguageIdentifier, ftl: String) {
        self.resource_manager.add_translation(lang, ftl);
        self.emit(EnvironmentEvent::SetLocale(self.resource_manager.language.clone()));
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
///
/// This type is part of the prelude.
pub trait DataContext {
    /// Get stored data from the context.
    fn data<T: 'static>(&self) -> Option<&T>;
}

pub trait EmitContext {
    fn emit<M: Any + Send>(&mut self, message: M);
    fn emit_to<M: Any + Send>(&mut self, target: Entity, message: M);
    fn emit_custom(&mut self, event: Event);
}

impl DataContext for Context {
    fn data<T: 'static>(&self) -> Option<&T> {
        // return data for the static model
        if let Some(t) = <dyn Any>::downcast_ref::<T>(&()) {
            return Some(t);
        }

        for entity in self.current.parent_iter(&self.tree) {
            if let Some(model_data_store) = self.data.get(entity) {
                if let Some(model) = model_data_store.models.get(&TypeId::of::<T>()) {
                    return model.downcast_ref::<T>();
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

impl EmitContext for Context {
    /// Send an event containing a message up the tree from the current entity.
    fn emit<M: Any + Send>(&mut self, message: M) {
        self.event_queue.push_back(
            Event::new(message)
                .target(self.current)
                .origin(self.current)
                .propagate(Propagation::Up),
        );
    }

    /// Send an event containing a message directly to a specified entity.
    fn emit_to<M: Any + Send>(&mut self, target: Entity, message: M) {
        self.event_queue.push_back(
            Event::new(message).target(target).origin(self.current).propagate(Propagation::Direct),
        );
    }

    /// Send an event with custom origin and propagation information.
    fn emit_custom(&mut self, event: Event) {
        self.event_queue.push_back(event);
    }
}
