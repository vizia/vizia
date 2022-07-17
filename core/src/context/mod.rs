mod draw;
mod proxy;

use instant::{Duration, Instant};
use std::any::{Any, TypeId};
use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet, VecDeque};
use std::error::Error;
use std::sync::Mutex;

#[cfg(feature = "clipboard")]
use copypasta::{nop_clipboard::NopClipboardContext, ClipboardContext, ClipboardProvider};
use femtovg::TextContext;
use fnv::FnvHashMap;
use keyboard_types::Code;
use morphorm::layout;
use unic_langid::LanguageIdentifier;

pub use draw::*;
pub use proxy::*;

use crate::cache::CachedData;
use crate::environment::Environment;
use crate::events::ViewHandler;
use crate::hover_system::apply_hover;
use crate::id::{GenerationalId, IdManager};
use crate::input::{Modifiers, MouseState};
use crate::layout::geometry_changed;
use crate::prelude::*;
use crate::resource::{FontOrId, ImageOrId, ImageRetentionPolicy, ResourceManager, StoredImage};
use crate::state::ModelDataStore;
use crate::storage::sparse_set::SparseSet;
use crate::style::{apply_transform, Style};
use crate::style_system::{
    apply_clipping, apply_inline_inheritance, apply_shared_inheritance, apply_styles,
    apply_text_constraints, apply_visibility, apply_z_ordering,
};
use crate::tree::{
    focus_backward, focus_forward, is_focusable, TreeDepthIterator, TreeExt, TreeIterator,
};

static DEFAULT_THEME: &str = include_str!("../../resources/themes/default_theme.css");
static DEFAULT_LAYOUT: &str = include_str!("../../resources/themes/default_layout.css");
const DOUBLE_CLICK_INTERVAL: Duration = Duration::from_millis(500);

/// The main storage and control object for a Vizia application.
///
/// This type is part of the prelude.
pub struct Context {
    pub(crate) entity_manager: IdManager<Entity>,
    pub(crate) tree: Tree,
    current: Entity,
    /// TODO make this private when there's no longer a need to mutate views after building
    pub views: FnvHashMap<Entity, Box<dyn ViewHandler>>,
    pub(crate) data: SparseSet<ModelDataStore>,
    pub(crate) event_queue: VecDeque<Event>,
    pub(crate) listeners:
        HashMap<Entity, Box<dyn Fn(&mut dyn ViewHandler, &mut Context, &mut Event)>>,
    style: Style,
    cache: CachedData,

    //environment: Environment,
    mouse: MouseState,
    modifiers: Modifiers,

    captured: Entity,
    pub(crate) hovered: Entity,
    focused: Entity,
    cursor_icon_locked: bool,

    resource_manager: ResourceManager,

    text_context: TextContext,

    event_proxy: Option<Box<dyn EventProxy>>,

    #[cfg(feature = "clipboard")]
    clipboard: Box<dyn ClipboardProvider>,

    click_time: Instant,
    double_click: bool,
    click_pos: (f32, f32),
}

impl Context {
    pub fn new() -> Self {
        let mut cache = CachedData::default();
        cache.add(Entity::root()).expect("Failed to add entity to cache");

        let mut result = Self {
            entity_manager: IdManager::new(),
            tree: Tree::new(),
            current: Entity::root(),
            views: FnvHashMap::default(),
            data: SparseSet::new(),
            style: Style::default(),
            cache,
            // environment: Environment::new(),
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

        Environment::new().build(&mut result);

        result.entity_manager.create();

        result
    }

    /// Access to the tree of entities
    pub fn tree(&mut self) -> &mut Tree {
        &mut self.tree
    }

    pub fn tree_ref(&self) -> &Tree {
        &self.tree
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

    /// The style storage for the application. Used to get properties set through inline style
    /// attributes or CSS.
    pub fn style(&mut self) -> &mut Style {
        &mut self.style
    }

    pub fn style_ref(&self) -> &Style {
        &self.style
    }

    /// The cache storage for the application. Used to get intermediate data computed during the
    /// layout and rendering processes.
    pub fn cache(&mut self) -> &mut CachedData {
        &mut self.cache
    }

    pub fn cache_ref(&self) -> &CachedData {
        &self.cache
    }

    pub fn environment(&self) -> &Environment {
        //&mut self.environment
        self.data::<Environment>().unwrap()
    }

    /// The current femtovg text context. Useful when measuring or rendering fonts.
    pub fn text_context(&mut self) -> &mut TextContext {
        &mut self.text_context
    }

    /// The current mouse state, i.e. the button and motion information.
    pub fn mouse(&self) -> &MouseState {
        &self.mouse
    }

    pub fn resource_manager(&mut self) -> &mut ResourceManager {
        &mut self.resource_manager
    }

    pub fn resource_manager_ref(&self) -> &ResourceManager {
        &self.resource_manager
    }

    /// The current keyboard modifiers state.
    pub fn modifiers(&self) -> Modifiers {
        self.modifiers
    }

    pub fn modifiers_mut(&mut self) -> &mut Modifiers {
        &mut self.modifiers
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

    /// Causes mouse events to propagate to the current entity until released
    pub fn capture(&mut self) {
        self.captured = self.current;
    }

    /// Releases the mouse events capture
    pub fn release(&mut self) {
        self.captured = Entity::null();
    }

    /// Prevents the cursor icon from changing until the lock is released
    pub fn lock_cursor_icon(&mut self) {
        self.cursor_icon_locked = true;
    }

    /// Releases any cursor icon lock, allowing the cursor icon to be changed
    pub fn unlock_cursor_icon(&mut self) {
        self.cursor_icon_locked = false;
        let hovered = self.hovered;
        let cursor = self.style().cursor.get(hovered).cloned().unwrap_or_default();
        self.emit(WindowEvent::SetCursor(cursor));
    }

    /// Returns true if the cursor icon is locked
    pub fn is_cursor_icon_locked(&self) -> bool {
        self.cursor_icon_locked
    }

    /// Sets application focus to the current entity
    pub fn focus(&mut self) {
        self.focused = self.current;
    }

    /// Sets the active flag of the current entity
    pub fn set_active(&mut self, flag: bool) {
        let current = self.current();
        if let Some(pseudo_classes) = self.style().pseudo_classes.get_mut(current) {
            pseudo_classes.set(PseudoClass::ACTIVE, flag);
        }

        self.style().needs_restyle = true;
        self.style().needs_relayout = true;
        self.style().needs_redraw = true;
    }

    /// Sets the hover flag of the current entity
    pub fn set_hover(&mut self, flag: bool) {
        let current = self.current();
        if let Some(pseudo_classes) = self.style().pseudo_classes.get_mut(current) {
            pseudo_classes.set(PseudoClass::HOVER, flag);
        }

        self.style().needs_restyle = true;
        self.style().needs_relayout = true;
        self.style().needs_redraw = true;
    }

    /// Sets the checked flag of the current entity
    pub fn set_checked(&mut self, flag: bool) {
        let current = self.current();
        if let Some(pseudo_classes) = self.style().pseudo_classes.get_mut(current) {
            pseudo_classes.set(PseudoClass::CHECKED, flag);
        }

        self.style().needs_restyle = true;
        self.style().needs_relayout = true;
        self.style().needs_redraw = true;
    }

    /// Sets the checked flag of the current entity
    pub fn set_selected(&mut self, flag: bool) {
        let current = self.current();
        if let Some(pseudo_classes) = self.style().pseudo_classes.get_mut(current) {
            pseudo_classes.set(PseudoClass::SELECTED, flag);
        }

        self.style().needs_restyle = true;
        self.style().needs_relayout = true;
        self.style().needs_redraw = true;
    }

    pub fn toggle_class(&mut self, class_name: &str, applied: bool) {
        let current = self.current();
        if let Some(class_list) = self.style().classes.get_mut(current) {
            if applied {
                class_list.insert(class_name.to_string());
            } else {
                class_list.remove(class_name);
            }
        } else if applied {
            let mut class_list = HashSet::new();
            class_list.insert(class_name.to_string());
            self.style().classes.insert(current, class_list).expect("Failed to insert class name");
        }

        self.need_restyle();
        self.style().needs_relayout = true;
        self.style().needs_redraw = true;
    }

    /// Returns true if the current entity is disabled
    pub fn is_disabled(&self) -> bool {
        self.style_ref().disabled.get(self.current()).cloned().unwrap_or_default()
    }

    /// Returns true if the mouse cursor is over the current entity
    pub fn is_over(&self) -> bool {
        if let Some(pseudo_classes) = self.style_ref().pseudo_classes.get(self.current) {
            pseudo_classes.contains(PseudoClass::OVER)
        } else {
            false
        }
    }

    pub fn hovered(&self) -> Entity {
        self.hovered
    }

    pub fn captured(&self) -> Entity {
        self.captured
    }

    /// You should not call this method unless you are writing a windowing backend, in which case
    /// you should consult the existing windowing backends for usage information.
    pub fn set_event_proxy(&mut self, proxy: Box<dyn EventProxy>) {
        if self.event_proxy.is_some() {
            panic!("Set the event proxy twice. This should never happen.");
        }

        self.event_proxy = Some(proxy);
    }

    /// You should not call this method unless you are writing a windowing backend, in which case
    /// you should consult the existing windowing backends for usage information.
    #[cfg(feature = "clipboard")]
    pub fn set_clipboard_provider(&mut self, clipboard: Box<dyn ClipboardProvider>) {
        self.clipboard = clipboard;
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
    pub fn emit_custom(&mut self, event: Event) {
        self.event_queue.push_back(event);
    }

    /// Check whether there are any events in the queue waiting for the next event dispatch cycle.
    pub fn has_queued_events(&self) -> bool {
        !self.event_queue.is_empty()
    }

    /// Add a listener to an entity.
    ///
    /// A listener can be used to handle events which would not normally propagate to the entity.
    /// For example, mouse events when a different entity has captured them. Useful for things like
    /// closing a popup when clicking outside of its bounding box.
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

    /// Ensure all FontOrId entires are loaded into the contexts and become Ids.
    pub fn synchronize_fonts(&mut self, canvas: &mut Canvas) {
        for (name, font) in self.resource_manager.fonts.iter_mut() {
            match font {
                FontOrId::Font(data) => {
                    let id1 = canvas
                        .add_font_mem(&data.clone())
                        .expect(&format!("Failed to load font file for: {}", name));
                    let id2 = self.text_context.add_font_mem(&data.clone()).expect("failed");
                    if id1 != id2 {
                        panic!(
                            "Fonts in canvas must have the same id as fonts in the text context"
                        );
                    }
                    *font = FontOrId::Id(id1);
                }

                _ => {}
            }
        }
    }

    pub fn add_theme(&mut self, theme: &str) {
        self.resource_manager.themes.push(theme.to_owned());

        self.reload_styles().expect("Failed to reload styles");
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

    pub fn has_animations(&mut self) -> bool {
        self.style.display.has_animations()
            | self.style.visibility.has_animations()
            | self.style.opacity.has_animations()
            | self.style.rotate.has_animations()
            | self.style.translate.has_animations()
            | self.style.scale.has_animations()
            | self.style.border_width.has_animations()
            | self.style.border_color.has_animations()
            | self.style.border_radius_top_left.has_animations()
            | self.style.border_radius_top_right.has_animations()
            | self.style.border_radius_bottom_left.has_animations()
            | self.style.border_radius_bottom_right.has_animations()
            | self.style.background_color.has_animations()
            | self.style.outer_shadow_h_offset.has_animations()
            | self.style.outer_shadow_v_offset.has_animations()
            | self.style.outer_shadow_blur.has_animations()
            | self.style.outer_shadow_color.has_animations()
            | self.style.font_color.has_animations()
            | self.style.font_size.has_animations()
            | if self.style.left.has_animations()
                | self.style.right.has_animations()
                | self.style.top.has_animations()
                | self.style.bottom.has_animations()
                | self.style.width.has_animations()
                | self.style.height.has_animations()
                | self.style.max_width.has_animations()
                | self.style.max_height.has_animations()
                | self.style.min_width.has_animations()
                | self.style.min_height.has_animations()
                | self.style.min_left.has_animations()
                | self.style.max_left.has_animations()
                | self.style.min_right.has_animations()
                | self.style.max_right.has_animations()
                | self.style.min_top.has_animations()
                | self.style.max_top.has_animations()
                | self.style.min_bottom.has_animations()
                | self.style.max_bottom.has_animations()
                | self.style.row_between.has_animations()
                | self.style.col_between.has_animations()
                | self.style.child_left.has_animations()
                | self.style.child_right.has_animations()
                | self.style.child_top.has_animations()
                | self.style.child_bottom.has_animations()
            {
                self.style.needs_relayout = true;
                true
            } else {
                false
            }
    }

    pub fn apply_animations(&mut self) {
        let time = instant::Instant::now();

        self.style.display.tick(time);
        self.style.visibility.tick(time);
        self.style.opacity.tick(time);
        self.style.rotate.tick(time);
        self.style.translate.tick(time);
        self.style.scale.tick(time);
        self.style.border_width.tick(time);
        self.style.border_color.tick(time);
        self.style.border_radius_top_left.tick(time);
        self.style.border_radius_top_right.tick(time);
        self.style.border_radius_bottom_left.tick(time);
        self.style.border_radius_bottom_right.tick(time);
        self.style.background_color.tick(time);
        self.style.outer_shadow_h_offset.tick(time);
        self.style.outer_shadow_v_offset.tick(time);
        self.style.outer_shadow_blur.tick(time);
        self.style.outer_shadow_color.tick(time);
        self.style.font_color.tick(time);
        self.style.font_size.tick(time);
        self.style.left.tick(time);
        self.style.right.tick(time);
        self.style.top.tick(time);
        self.style.bottom.tick(time);
        self.style.width.tick(time);
        self.style.height.tick(time);
        self.style.max_width.tick(time);
        self.style.max_height.tick(time);
        self.style.min_width.tick(time);
        self.style.min_height.tick(time);
        self.style.min_left.tick(time);
        self.style.max_left.tick(time);
        self.style.min_right.tick(time);
        self.style.max_right.tick(time);
        self.style.min_top.tick(time);
        self.style.max_top.tick(time);
        self.style.min_bottom.tick(time);
        self.style.max_bottom.tick(time);
        self.style.row_between.tick(time);
        self.style.col_between.tick(time);
        self.style.child_left.tick(time);
        self.style.child_right.tick(time);
        self.style.child_top.tick(time);
        self.style.child_bottom.tick(time);
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

    pub fn play_animation(&mut self, animation: Animation) {
        self.current.play_animation(self, animation);
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
            if !self.environment().include_default_theme && index == 1 {
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

        Ok(())
    }

    pub fn set_image_loader<F: 'static + Fn(&mut Context, &str)>(&mut self, loader: F) {
        self.resource_manager.image_loader = Some(Box::new(loader));
    }

    fn get_image_internal(&mut self, path: &str) -> &mut StoredImage {
        if let Some(img) = self.resource_manager.images.get_mut(path) {
            img.used = true;
            // borrow checker hack
            return self.resource_manager.images.get_mut(path).unwrap();
        }

        if let Some(callback) = self.resource_manager.image_loader.take() {
            callback(self, path);
            self.resource_manager.image_loader = Some(callback);
        }

        if let Some(img) = self.resource_manager.images.get_mut(path) {
            img.used = true;
            // borrow checker hack
            return self.resource_manager.images.get_mut(path).unwrap();
        } else {
            self.resource_manager.images.insert(
                path.to_owned(),
                StoredImage {
                    image: ImageOrId::Image(
                        image::load_from_memory_with_format(
                            include_bytes!("../../resources/images/broken_image.png"),
                            image::ImageFormat::Png,
                        )
                        .unwrap(),
                        femtovg::ImageFlags::NEAREST,
                    ),
                    retention_policy: ImageRetentionPolicy::Forever,
                    used: true,
                    dirty: false,
                    observers: HashSet::new(),
                },
            );
            self.resource_manager.images.get_mut(path).unwrap()
        }
    }

    pub fn get_image(&mut self, path: &str) -> &mut ImageOrId {
        &mut self.get_image_internal(path).image
    }

    pub fn add_image_observer(&mut self, path: &str, observer: Entity) {
        self.get_image_internal(path).observers.insert(observer);
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

    pub fn evict_image(&mut self, path: &str) {
        self.resource_manager.images.remove(path);
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

    /// For each binding or data observer, check if its data has changed, and if so, rerun its
    /// builder/body.
    pub fn process_data_updates(&mut self) {
        let mut observers: Vec<Entity> = Vec::new();

        for entity in self.tree.into_iter() {
            if let Some(model_store) = self.data.get_mut(entity) {
                for (_, model) in model_store.data.iter() {
                    for lens in model_store.lenses_dup.iter_mut() {
                        if lens.update(model) {
                            observers.extend(lens.observers().iter())
                        }
                    }

                    for (_, lens) in model_store.lenses_dedup.iter_mut() {
                        if lens.update(model) {
                            observers.extend(lens.observers().iter());
                        }
                    }
                }

                for lens in model_store.lenses_dup.iter_mut() {
                    if let Some(view_handler) = self.views.get(&entity) {
                        if lens.update_view(view_handler) {
                            observers.extend(lens.observers().iter())
                        }
                    }
                }

                for (_, lens) in model_store.lenses_dedup.iter_mut() {
                    if let Some(view_handler) = self.views.get(&entity) {
                        if lens.update_view(view_handler) {
                            observers.extend(lens.observers().iter())
                        }
                    }
                }
            }
        }

        for img in self.resource_manager.images.values_mut() {
            if img.dirty {
                observers.extend(img.observers.iter());
                img.dirty = false;
            }
        }

        for observer in observers.iter() {
            if let Some(mut view) = self.views.remove(observer) {
                let prev = self.current;
                self.current = *observer;
                view.body(self);
                self.current = prev;
                self.views.insert(*observer, view);
            }
        }
    }

    pub fn process_style_updates(&mut self) {
        // Not ideal
        let tree = self.tree.clone();

        apply_inline_inheritance(self, &tree);

        if self.style.needs_restyle {
            apply_styles(self, &tree);
            self.style.needs_restyle = false;
        }

        apply_shared_inheritance(self, &tree);
    }

    /// Massages the style system until everything is coherent
    pub fn process_visual_updates(&mut self) {
        // Not ideal
        let tree = self.tree.clone();

        apply_z_ordering(self, &tree);
        apply_visibility(self, &tree);

        // Layout
        if self.style.needs_relayout {
            apply_text_constraints(self, &tree);

            // hack!
            let mut store = (Style::default(), TextContext::default(), ResourceManager::default());
            std::mem::swap(&mut store.0, &mut self.style);
            std::mem::swap(&mut store.1, &mut self.text_context);
            std::mem::swap(&mut store.2, &mut self.resource_manager);

            layout(&mut self.cache, &self.tree, &store);
            std::mem::swap(&mut store.0, &mut self.style);
            std::mem::swap(&mut store.1, &mut self.text_context);
            std::mem::swap(&mut store.2, &mut self.resource_manager);
            self.style.needs_relayout = false;
        }

        apply_transform(self, &tree);
        apply_hover(self);
        apply_clipping(self, &tree);

        // Emit any geometry changed events
        geometry_changed(self, &tree);
    }

    pub fn draw(&mut self, canvas: &mut Canvas) {
        self.resource_manager.mark_images_unused();

        let window_width = self.cache.get_width(Entity::root());
        let window_height = self.cache.get_height(Entity::root());

        canvas.set_size(window_width as u32, window_height as u32, 1.0);
        let clear_color =
            self.style.background_color.get(Entity::root()).cloned().unwrap_or(Color::white());
        canvas.clear_rect(0, 0, window_width as u32, window_height as u32, clear_color.into());

        // filter for widgets that should be drawn
        let tree_iter = self.tree.into_iter();
        let mut draw_tree: Vec<Entity> = tree_iter
            .filter(|&entity| {
                entity != Entity::root()
                    && self.cache.get_visibility(entity) != Visibility::Invisible
                    && self.cache.get_display(entity) != Display::None
                    && !self.tree.is_ignored(entity)
                    && self.cache.get_opacity(entity) > 0.0
                    && {
                        let bounds = self.cache.get_bounds(entity);
                        !(bounds.x > window_width
                            || bounds.y > window_height
                            || bounds.x + bounds.w <= 0.0
                            || bounds.y + bounds.h <= 0.0)
                    }
            })
            .collect();

        // Sort the tree by z order
        draw_tree.sort_by_cached_key(|entity| self.cache.get_z_index(*entity));

        for entity in draw_tree.into_iter() {
            // Apply clipping
            let clip_region = self.cache.get_clip_region(entity);

            // Skips drawing views with zero-sized clip regions
            // This skips calling the `draw` method of the view
            if clip_region.height() == 0.0 || clip_region.width() == 0.0 {
                continue;
            }

            canvas.scissor(clip_region.x, clip_region.y, clip_region.w, clip_region.h);

            // Apply transform
            let transform = self.cache.get_transform(entity);
            canvas.save();
            canvas.set_transform(
                transform[0],
                transform[1],
                transform[2],
                transform[3],
                transform[4],
                transform[5],
            );

            if let Some(view) = self.views.remove(&entity) {
                self.current = entity;
                view.draw(&mut DrawContext::new(self), canvas);

                self.views.insert(entity, view);
            }

            canvas.restore();

            // Uncomment this for debug outlines
            // TODO - Hook this up to a key in debug mode
            // let mut path = Path::new();
            // path.rect(bounds.x, bounds.y, bounds.w, bounds.h);
            // let mut paint = Paint::color(femtovg::Color::rgb(255, 0, 0));
            // paint.set_line_width(1.0);
            // canvas.stroke_path(&mut path, paint);
        }

        canvas.flush();

        self.resource_manager.evict_unused_images();
    }

    /// This method is in charge of receiving raw WindowEvents and dispatching them to the
    /// appropriate points in the tree.
    pub fn dispatch_system_event(&mut self, event: WindowEvent) {
        match &event {
            WindowEvent::MouseMove(x, y) => {
                self.mouse.previous_cursorx = self.mouse.cursorx;
                self.mouse.previous_cursory = self.mouse.cursory;
                self.mouse.cursorx = *x;
                self.mouse.cursory = *y;

                apply_hover(self);

                self.dispatch_direct_or_hovered(event, self.captured, false);
            }
            WindowEvent::MouseDown(button) => {
                match button {
                    MouseButton::Left => self.mouse.left.state = MouseButtonState::Pressed,
                    MouseButton::Right => self.mouse.right.state = MouseButtonState::Pressed,
                    MouseButton::Middle => self.mouse.middle.state = MouseButtonState::Pressed,
                    _ => {}
                }

                let new_click_time = Instant::now();
                let click_duration = new_click_time - self.click_time;
                let new_click_pos = (self.mouse.cursorx, self.mouse.cursory);

                if click_duration <= DOUBLE_CLICK_INTERVAL && new_click_pos == self.click_pos {
                    if !self.double_click {
                        self.dispatch_direct_or_hovered(
                            WindowEvent::MouseDoubleClick(*button),
                            self.captured,
                            true,
                        );
                        self.double_click = true;
                    }
                } else {
                    self.double_click = false;
                }

                self.click_time = new_click_time;
                self.click_pos = new_click_pos;

                match button {
                    MouseButton::Left => {
                        self.mouse.left.pos_down = (self.mouse.cursorx, self.mouse.cursory);
                        self.mouse.left.pressed = self.hovered;
                    }
                    MouseButton::Right => {
                        self.mouse.right.pos_down = (self.mouse.cursorx, self.mouse.cursory);
                        self.mouse.right.pressed = self.hovered;
                    }
                    MouseButton::Middle => {
                        self.mouse.middle.pos_down = (self.mouse.cursorx, self.mouse.cursory);
                        self.mouse.middle.pressed = self.hovered;
                    }
                    _ => {}
                }

                self.dispatch_direct_or_hovered(event, self.captured, true);
            }
            WindowEvent::MouseUp(button) => {
                match button {
                    MouseButton::Left => {
                        self.mouse.left.pos_up = (self.mouse.cursorx, self.mouse.cursory);
                        self.mouse.left.released = self.hovered;
                        self.mouse.left.state = MouseButtonState::Released;
                    }
                    MouseButton::Right => {
                        self.mouse.right.pos_up = (self.mouse.cursorx, self.mouse.cursory);
                        self.mouse.right.released = self.hovered;
                        self.mouse.right.state = MouseButtonState::Released;
                    }
                    MouseButton::Middle => {
                        self.mouse.middle.pos_up = (self.mouse.cursorx, self.mouse.cursory);
                        self.mouse.middle.released = self.hovered;
                        self.mouse.middle.state = MouseButtonState::Released;
                    }
                    _ => {}
                }
                self.dispatch_direct_or_hovered(event, self.captured, true);
            }
            WindowEvent::MouseScroll(_, _) => {
                self.event_queue.push_back(Event::new(event).target(self.hovered));
            }
            WindowEvent::KeyDown(code, _) => {
                #[cfg(debug_assertions)]
                if *code == Code::KeyH {
                    for entity in self.tree.into_iter() {
                        println!("Entity: {} Parent: {:?} View: {} posx: {} posy: {} width: {} height: {}", entity, entity.parent(&self.tree), self.views.get(&entity).map_or("<None>", |view| view.element().unwrap_or("<Unnamed>")), self.cache.get_posx(entity), self.cache.get_posy(entity), self.cache.get_width(entity), self.cache.get_height(entity));
                    }
                }

                #[cfg(debug_assertions)]
                if *code == Code::KeyI {
                    let iter = TreeDepthIterator::full(&self.tree);
                    for (entity, level) in iter {
                        if let Some(element_name) = self.views.get(&entity).unwrap().element() {
                            println!(
                                "{:indent$} {} {} {:?} {:?} {:?} {:?}",
                                "",
                                entity,
                                element_name,
                                self.cache.get_visibility(entity),
                                self.cache.get_display(entity),
                                self.cache.get_bounds(entity),
                                self.cache.get_clip_region(entity),
                                indent = level
                            );
                        }
                    }
                }

                if *code == Code::F5 {
                    self.reload_styles().unwrap();
                }

                if *code == Code::Tab {
                    let focused = self.focused;
                    if let Some(pseudo_classes) = self.style().pseudo_classes.get_mut(focused) {
                        pseudo_classes.set(PseudoClass::FOCUS, false);
                    }

                    if self.modifiers.contains(Modifiers::SHIFT) {
                        let prev_focused = if let Some(prev_focused) =
                            focus_backward(&self.tree, &self.style, self.focused)
                        {
                            prev_focused
                        } else {
                            TreeIterator::full(&self.tree)
                                .filter(|node| is_focusable(&self.style, *node))
                                .next_back()
                                .unwrap_or(Entity::root())
                        };

                        if prev_focused != self.focused {
                            self.event_queue
                                .push_back(Event::new(WindowEvent::FocusOut).target(self.focused));
                            self.event_queue
                                .push_back(Event::new(WindowEvent::FocusIn).target(prev_focused));
                            self.focused = prev_focused;
                        }
                    } else {
                        let next_focused = if let Some(next_focused) =
                            focus_forward(&self.tree, &self.style, self.focused)
                        {
                            next_focused
                        } else {
                            Entity::root()
                        };

                        if next_focused != self.focused {
                            self.event_queue
                                .push_back(Event::new(WindowEvent::FocusOut).target(self.focused));
                            self.event_queue
                                .push_back(Event::new(WindowEvent::FocusIn).target(next_focused));
                            self.focused = next_focused;
                        }
                    }

                    let focused = self.focused;
                    if let Some(pseudo_classes) = self.style().pseudo_classes.get_mut(focused) {
                        pseudo_classes.set(PseudoClass::FOCUS, true);
                    }

                    self.style().needs_relayout = true;
                    self.style().needs_redraw = true;
                }

                self.event_queue.push_back(Event::new(event).target(self.focused));
            }
            WindowEvent::KeyUp(_, _) | WindowEvent::CharInput(_) => {
                self.event_queue.push_back(Event::new(event).target(self.focused));
            }
            _ => {}
        }
    }

    fn dispatch_direct_or_hovered(&mut self, event: WindowEvent, target: Entity, root: bool) {
        if target != Entity::null() {
            self.event_queue
                .push_back(Event::new(event).target(target).propagate(Propagation::Direct));
        } else if self.hovered != Entity::root() || root {
            self.event_queue
                .push_back(Event::new(event).target(self.hovered).propagate(Propagation::Up));
        }
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

impl DataContext for Context {
    fn data<T: 'static>(&self) -> Option<&T> {
        // return data for the static model
        if let Some(t) = <dyn Any>::downcast_ref::<T>(&()) {
            return Some(t);
        }

        for entity in self.current.parent_iter(&self.tree) {
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
