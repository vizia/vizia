use crate::binding::get_storeid;
use crate::context::{CURRENT, MAPS, MAP_MANAGER};
use crate::prelude::*;

/// A view with a binding which rebuilds its contents when the observed data changes.
///
/// This view is typically used to switch between two or more views when the bound data changes. The binding view will destroy and then recreate its
/// contents whenever the bound data changes, so it is usually preferable to bind a view directly to the data (if supported) or to bind to a view modifier,
/// which will update the properties of a view without rebuilding it.
pub struct Binding<L>
where
    L: Lens,
{
    entity: Entity,
    lens: L,
    #[allow(clippy::type_complexity)]
    content: Option<Box<dyn Fn(&mut Context, L)>>,
}

impl<L> Binding<L>
where
    L: 'static + Lens<Source: 'static, Target: Data>,
{
    /// Creates a new binding view.
    ///
    /// A binding view observes application data through a lens and rebuilds its contents if the data changes.
    ///
    /// # Example
    /// When the value of `AppData::some_data` changes, the label inside of the binding will be rebuilt.
    /// ```ignore
    /// Binding::new(cx, AppData::some_data, |cx, lens|{
    ///     // Retrieve the data from context
    ///     let value = *lens.get(cx);
    ///     Label::new(cx, value.to_string());
    /// });
    /// ```
    #[allow(clippy::new_ret_no_self)]
    pub fn new<F>(cx: &mut Context, lens: L, builder: F)
    where
        F: 'static + Fn(&mut Context, L),
    {
        let id = cx.entity_manager.create();
        let current = cx.current();
        cx.tree.add(id, current).expect("Failed to add to tree");
        cx.cache.add(id);
        cx.style.add(id);
        cx.tree.set_ignored(id, true);

        let binding = Self { entity: id, lens, content: Some(Box::new(builder)) };

        CURRENT.with_borrow_mut(|f| *f = id);

        cx.with_current(id, |cx| {
            lens.bind(cx);
        });

        cx.bindings.insert(id, Box::new(binding));

        cx.with_current(id, |cx| {
            // Call the body of the binding
            if let Some(mut binding) = cx.bindings.remove(&id) {
                binding.update(cx);
                cx.bindings.insert(id, binding);
            }
        });

        let _: Handle<Self> =
            Handle { current: id, entity: id, p: Default::default(), cx }.ignore();
    }
}

pub(crate) trait BindingHandler {
    fn update(&mut self, cx: &mut Context);
    fn remove(&self, cx: &mut Context);
    fn debug(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result;
}

impl<L: 'static + Lens> BindingHandler for Binding<L> {
    fn update(&mut self, cx: &mut Context) {
        cx.remove_children(cx.current());

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

        if let Some(builder) = &self.content {
            CURRENT.with_borrow_mut(|f| *f = self.entity);
            (builder)(cx, self.lens);
        }
    }

    fn remove(&self, cx: &mut Context) {
        for entity in self.entity.parent_iter(&cx.tree) {
            let key = self.lens.id();

            if let Some(stores) = cx.stores.get_mut(&entity) {
                if let Some(store) = stores.get_mut(&key) {
                    let source = store.source();
                    if cx.views.get(&entity).filter(|view| view.id() == source).is_some()
                        || cx
                            .models
                            .get(&entity)
                            .filter(|models| models.contains_key(&source))
                            .is_some()
                    {
                        store.remove_observer(&self.entity);

                        if store.num_observers() == 0 {
                            stores.remove(&key);
                        }

                        break;
                    }
                }
            }
        }
    }

    fn debug(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.lens.fmt(f)
    }
}

impl std::fmt::Debug for dyn BindingHandler {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.debug(f)
    }
}
