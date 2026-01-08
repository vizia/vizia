use crate::prelude::*;
use crate::recoil::NodeId;

pub(crate) trait BindingHandler {
    fn update(&mut self, cx: &mut Context);
    fn remove(&self, cx: &mut Context);
    fn debug(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result;
}

/// A binding that rebuilds its contents when the observed signal changes.
pub struct Binding {
    entity: Entity,
    scope: Option<Entity>,
    signal_id: NodeId,
    content: Box<dyn Fn(&mut Context)>,
}

fn create_scope(cx: &mut Context, parent: Entity) -> Entity {
    let id = cx.entity_manager.create();
    cx.tree.add(id, parent).expect("Failed to add to tree");
    cx.cache.add(id);
    cx.style.add(id);
    cx.tree.set_ignored(id, true);
    id
}

impl Binding {
    /// Create a new binding that rebuilds when `signal` changes.
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>(cx: &mut Context, signal: Signal<T>, content: impl 'static + Fn(&mut Context))
    where
        T: 'static,
    {
        let id = cx.entity_manager.create();
        let current = cx.current();
        cx.tree.add(id, current).expect("Failed to add to tree");
        cx.cache.add(id);
        cx.style.add(id);
        cx.tree.set_ignored(id, true);

        // Observe the signal.
        signal.observe(cx.data.get_store_mut(), id);

        let binding =
            Binding { entity: id, scope: None, signal_id: signal.id(), content: Box::new(content) };
        cx.bindings.insert(id, Box::new(binding));

        // Initial update.
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

impl BindingHandler for Binding {
    fn update(&mut self, cx: &mut Context) {
        if !cx.data.get_store().has_value(&self.signal_id) {
            return;
        }
        if let Some(scope) = self.scope.take() {
            if cx.entity_manager.is_alive(scope) {
                cx.remove(scope);
            }
        }

        let scope = create_scope(cx, self.entity);
        self.scope = Some(scope);

        cx.with_current(scope, |cx| {
            (self.content)(cx);
        });
    }

    fn remove(&self, cx: &mut Context) {
        if let Some(observers) = cx.data.get_store_mut().observers.get_mut(&self.signal_id) {
            observers.retain(|&observer| observer != self.entity);
        }
    }

    fn debug(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Signal({:?})", self.signal_id)
    }
}

impl std::fmt::Debug for dyn BindingHandler {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.debug(f)
    }
}
