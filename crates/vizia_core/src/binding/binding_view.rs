use hashbrown::{HashMap, HashSet};
use std::any::TypeId;

use crate::binding::{BasicStore, Store, StoreId};
use crate::context::{MAPS, MAP_MANAGER};
use crate::model::ModelOrView;
use crate::prelude::*;

// /// A view with a binding which rebuilds its contents when the observed data changes.
// ///
// /// This view is typically used to switch between two or more views when the bound data changes. The binding view will destroy and then recreate its
// /// contents whenever the bound data changes, so it is usually preferable to bind a view directly to the data (if supported) or to bind to a view modifier,
// /// which will update the properties of a view without rebuilding it.

pub(crate) trait BindingHandler {
    fn update(&mut self, cx: &mut Context);
    fn remove(&self, cx: &mut Context);
    fn debug(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result;
}

/// Trait that abstracts over different data observation mechanisms
pub trait Observable {
    /// The type of data this observable produces
    type Data;

    /// Set up observation for this data source
    fn setup_observation(&self, cx: &mut Context, entity: Entity);

    /// Clean up observation for this data source  
    fn cleanup_observation(&self, cx: &mut Context, entity: Entity);

    /// Get debug information for this observable
    fn debug_info(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result;

    /// Whether this observable provides data to the content function
    /// Lenses provide data, signals use dependency tracking
    fn provides_data() -> bool;
}

/// Implementation for lens-based observables
impl<L> Observable for L
where
    L: 'static + Lens<Source: 'static, Target: Data> + Clone,
{
    type Data = L::Target;

    fn setup_observation(&self, cx: &mut Context, entity: Entity) {
        // Use the existing lens store setup logic
        let current = cx.current();
        let ancestors = current.parent_iter(&cx.tree).collect::<HashSet<_>>();
        let new_ancestors = entity.parent_iter(&cx.tree).collect::<Vec<_>>();

        for ancestor in new_ancestors {
            // Check for view store
            if let Some(view_handler) = cx.views.get(&ancestor) {
                if view_handler.as_any_ref().is::<L::Source>() {
                    insert_store_for_lens(
                        ancestor,
                        &ancestors,
                        &mut cx.stores,
                        ModelOrView::View(view_handler.as_ref()),
                        self.clone(),
                        entity,
                    );
                    break;
                }
            }

            if let Some(models) = cx.models.get_mut(&ancestor) {
                if let Some(model_data) = models.get(&TypeId::of::<L::Source>()) {
                    insert_store_for_lens(
                        ancestor,
                        &ancestors,
                        &mut cx.stores,
                        ModelOrView::Model(model_data.as_ref()),
                        self.clone(),
                        entity,
                    );
                    break;
                }
            }
        }
    }

    fn cleanup_observation(&self, cx: &mut Context, entity: Entity) {
        for ancestor in entity.parent_iter(&cx.tree) {
            let key = self.id();

            if let Some(stores) = cx.stores.get_mut(&ancestor) {
                if let Some(store) = stores.get_mut(&key) {
                    let source = store.source();
                    if cx.views.get(&ancestor).filter(|view| view.id() == source).is_some()
                        || cx
                            .models
                            .get(&ancestor)
                            .filter(|models| models.contains_key(&source))
                            .is_some()
                    {
                        store.remove_observer(&entity);

                        if store.num_observers() == 0 {
                            stores.remove(&key);
                        }

                        break;
                    }
                }
            }
        }
    }

    fn debug_info(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.fmt(f)
    }

    fn provides_data() -> bool {
        true
    }
}

// Helper function to insert store for lens types
fn insert_store_for_lens<L>(
    entity: Entity,
    ancestors: &HashSet<Entity>,
    stores: &mut HashMap<Entity, HashMap<StoreId, Box<dyn Store>>>,
    model_data: ModelOrView,
    lens: L,
    id: Entity,
) where
    L: Lens<Target: Data> + Clone,
{
    if !stores.contains_key(&entity) {
        stores.insert(entity, HashMap::new());
    }

    if let Some(stores) = stores.get_mut(&entity) {
        let key = lens.id();

        if let Some(store) = stores.get_mut(&key) {
            let observers = store.observers();

            if ancestors.intersection(observers).next().is_none() {
                store.add_observer(id);
            }
        } else {
            let mut observers = HashSet::new();
            observers.insert(id);

            let model = model_data.downcast_ref::<L::Source>().unwrap();
            let old = lens.view(model).map(|val| val.into_owned());
            let store = Box::new(BasicStore { lens, old, observers });

            stores.insert(key, store);
        }
    }
}

/// Implementation for signal-based observables
impl<T> Observable for Signal<T>
where
    T: 'static,
{
    type Data = T;

    fn setup_observation(&self, cx: &mut Context, entity: Entity) {
        self.observe(cx.data.get_store_mut(), entity);
    }

    fn cleanup_observation(&self, cx: &mut Context, entity: Entity) {
        cx.data.get_store_mut().observers.get_mut(&self.id()).map(|observers| {
            observers.retain(|&observer| observer != entity);
        });
    }

    fn debug_info(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Signal({:?})", self.id())
    }

    fn provides_data() -> bool {
        false
    }
}

/// Trait for content functions that can handle different argument patterns
pub trait ContentFunction {
    fn call(&self, cx: &mut Context);
}

/// Wrapper for lens-based content functions that need the lens data
pub struct LensContentFunction<L, F> {
    lens: L,
    func: F,
}

impl<L, F> LensContentFunction<L, F>
where
    L: Clone,
    F: Fn(&mut Context, L),
{
    pub fn create(lens: L, func: F) -> Self {
        Self { lens, func }
    }
}

impl<L, F> ContentFunction for LensContentFunction<L, F>
where
    L: Clone,
    F: Fn(&mut Context, L),
{
    fn call(&self, cx: &mut Context) {
        (self.func)(cx, self.lens.clone());
    }
}

/// Wrapper for signal-based content functions
pub struct SignalContentFunction<F> {
    func: F,
}

impl<F> SignalContentFunction<F>
where
    F: Fn(&mut Context),
{
    pub fn create(func: F) -> Self {
        Self { func }
    }
}

impl<F> ContentFunction for SignalContentFunction<F>
where
    F: Fn(&mut Context),
{
    fn call(&self, cx: &mut Context) {
        (self.func)(cx);
    }
}

/// The unified binding struct
pub struct Binding {
    entity: Entity,
    observable: Box<dyn ObservableErasure>,
    content: Box<dyn ContentFunction>,
}

/// Type-erased version of Observable for storage
trait ObservableErasure {
    fn setup_observation(&self, cx: &mut Context, entity: Entity);
    fn cleanup_observation(&self, cx: &mut Context, entity: Entity);
    fn debug_info(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result;
}

impl<O: Observable + 'static> ObservableErasure for O {
    fn setup_observation(&self, cx: &mut Context, entity: Entity) {
        <Self as Observable>::setup_observation(self, cx, entity);
    }

    fn cleanup_observation(&self, cx: &mut Context, entity: Entity) {
        <Self as Observable>::cleanup_observation(self, cx, entity);
    }

    fn debug_info(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        <Self as Observable>::debug_info(self, f)
    }
}

/// Trait for binding creation that can handle different observable types and content functions
pub trait BindingBuilder<Args> {
    fn build_binding(self, cx: &mut Context, args: Args);
}

/// Implementation for lens-based bindings (Observable + content function that takes observable)
/// Explicitly excludes Signal types to avoid conflicts
impl<L, F> BindingBuilder<F> for L
where
    L: Observable + Clone + 'static + Lens,
    F: 'static + Fn(&mut Context, L),
{
    fn build_binding(self, cx: &mut Context, content: F) {
        let id = cx.entity_manager.create();
        let current = cx.current();
        cx.tree.add(id, current).expect("Failed to add to tree");
        cx.cache.add(id);
        cx.style.add(id);
        cx.tree.set_ignored(id, true);

        // Set up observation
        Observable::setup_observation(&self, cx, id);

        let lens_content = LensContentFunction::create(self.clone(), content);
        let binding =
            Binding { entity: id, observable: Box::new(self), content: Box::new(lens_content) };

        cx.bindings.insert(id, Box::new(binding));

        // Initial update
        cx.with_current(id, |cx| {
            if let Some(mut binding) = cx.bindings.remove(&id) {
                binding.update(cx);
                cx.bindings.insert(id, binding);
            }
        });

        let _: Handle<Binding> =
            Handle { current: id, entity: id, p: Default::default(), cx }.ignore();
    }
}

/// Implementation for signal-based bindings (Signal + content function that takes only context)
impl<T, F> BindingBuilder<F> for Signal<T>
where
    T: 'static,
    F: 'static + Fn(&mut Context),
{
    fn build_binding(self, cx: &mut Context, content: F) {
        let id = cx.entity_manager.create();
        let current = cx.current();
        cx.tree.add(id, current).expect("Failed to add to tree");
        cx.cache.add(id);
        cx.style.add(id);
        cx.tree.set_ignored(id, true);

        // Set up observation
        Observable::setup_observation(&self, cx, id);

        let signal_content = SignalContentFunction::create(content);
        let binding =
            Binding { entity: id, observable: Box::new(self), content: Box::new(signal_content) };

        cx.bindings.insert(id, Box::new(binding));

        // Initial update
        cx.with_current(id, |cx| {
            if let Some(mut binding) = cx.bindings.remove(&id) {
                binding.update(cx);
                cx.bindings.insert(id, binding);
            }
        });

        let _: Handle<Binding> =
            Handle { current: id, entity: id, p: Default::default(), cx }.ignore();
    }
}

impl Binding {
    /// Unified constructor that works with both lens and signal observables
    /// The function signature determines which implementation is used
    pub fn new<O, F>(cx: &mut Context, observable: O, content: F)
    where
        O: BindingBuilder<F>,
    {
        observable.build_binding(cx, content);
    }
}

impl BindingHandler for Binding {
    fn update(&mut self, cx: &mut Context) {
        cx.remove_children(cx.current());

        // Remove maps for lens bindings
        MAP_MANAGER.with_borrow_mut(|manager| {
            MAPS.with_borrow_mut(|maps| {
                maps.retain(|id, (e, _)| {
                    if *e == self.entity {
                        manager.destroy(*id);
                        false
                    } else {
                        true
                    }
                });
            });
        });

        // Call content function
        self.content.call(cx);
    }

    fn remove(&self, cx: &mut Context) {
        self.observable.cleanup_observation(cx, self.entity);
    }

    fn debug(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.observable.debug_info(f)
    }
}

impl std::fmt::Debug for dyn BindingHandler {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.debug(f)
    }
}
