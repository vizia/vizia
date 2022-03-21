use std::any::TypeId;
use std::collections::HashSet;

use crate::{Context, Handle, LensCache, StateStore, TreeExt, Units, View};

use crate::{Data, Lens};

/// A binding view which rebuilds its contents when its observed data changes.
pub struct Binding<L>
where
    L: Lens,
{
    lens: L,
    builder: Option<Box<dyn Fn(&mut Context, L)>>,
}

impl<L> Binding<L>
where
    L: 'static + Lens,
    <L as Lens>::Source: 'static,
    <L as Lens>::Target: Data,
{
    /// Creates a new binding view.
    ///
    /// A binding view observes application data through a lens and rebuilds its contents if the data changes.
    ///
    /// Example
    /// When the value of `AppData::some_data` changes, the label inside of the binding will be rebuilt.
    /// ```ignore
    /// Binding::new(cx, AppData::some_data, |cx, lens|{
    ///     // Retrieve the data from context
    ///     let value = *lens.get(cx);
    ///     Label::new(cx, value.to_string());
    /// });
    pub fn new<F>(cx: &mut Context, lens: L, builder: F)
    where
        F: 'static + Fn(&mut Context, L),
    {
        let binding = Self { lens: lens.clone(), builder: Some(Box::new(builder)) };

        let id = cx.entity_manager.create();
        cx.tree.add(id, cx.current).expect("Failed to add to tree");
        cx.cache.add(id).expect("Failed to add to cache");
        cx.style.add(id);

        let ancestors = cx.current.parent_iter(&cx.tree).collect::<HashSet<_>>();

        for entity in id.parent_iter(&cx.tree) {
            if let Some(model_data_store) = cx.data.get_mut(entity) {
                if let Some(model_data) = model_data_store.data.get(&TypeId::of::<L::Source>()) {
                    if let Some(lens_wrap) =
                        lens.cache_key().and_then(|key| model_data_store.lenses_dedup.get_mut(&key))
                    {
                        let observers = lens_wrap.observers();

                        if ancestors.intersection(observers).next().is_none() {
                            lens_wrap.add_observer(id);
                        }
                    } else {
                        let mut observers = HashSet::new();
                        observers.insert(id);

                        let model = model_data.downcast_ref::<L::Source>().unwrap();

                        let old = lens.view(model, |t| t.cloned());

                        let state = Box::new(StateStore { entity: id, lens, old, observers });

                        if let Some(key) = state.lens.cache_key() {
                            model_data_store.lenses_dedup.insert(key, state);
                        } else {
                            model_data_store.lenses_dup.push(state);
                        }
                    }

                    break;
                }
            }
        }

        cx.views.insert(id, Box::new(binding));

        let prev = cx.current;
        cx.current = id;

        // Call the body of the binding
        if let Some(mut view_handler) = cx.views.remove(&id) {
            view_handler.body(cx);
            cx.views.insert(id, view_handler);
        }
        cx.current = prev;

        let _: Handle<Self> = Handle { entity: id, p: Default::default(), cx }
            .width(Units::Stretch(1.0))
            .height(Units::Stretch(1.0))
            .focusable(false)
            .ignore();
    }
}

impl<L: 'static + Lens> View for Binding<L> {
    fn element(&self) -> Option<String> {
        Some("binding".to_string())
    }

    fn body<'a>(&mut self, cx: &'a mut Context) {
        cx.remove_children(cx.current);
        if let Some(builder) = self.builder.take() {
            (builder)(cx, self.lens.clone());
            self.builder = Some(builder);
        }
    }
}
