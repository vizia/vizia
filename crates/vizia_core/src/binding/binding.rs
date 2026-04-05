use vizia_reactive::{Scope, SignalGet, UpdaterEffect};

use crate::{binding::BindingHandler, context::SIGNAL_REBUILDS, prelude::*};

/// A binding view that observes a reactive [`vizia_reactive`] signal and rebuilds its
/// contents whenever the signal value changes.
///
/// `Binding` subscribes to
/// the signal's reactive graph and is notified immediately when the signal is mutated,
/// even from inside a model's event handler.
///
/// # Example
/// ```ignore
/// pub struct AppData {
///     pub count: Signal<i32>,
/// }
///
/// impl Model for AppData {
///     fn event(&mut self, _cx: &mut EventContext, event: &mut Event) {
///         event.map(|e, _| match e {
///             AppEvent::Increment => self.count.update(|c| *c += 1),
///         });
///     }
/// }
///
/// // In the view tree:
/// Binding::new(cx, app_data.count, |cx| {
///     Label::new(cx, app_data.count.get(cx).to_string());
/// });
/// ```
pub struct Binding<T: 'static + Clone> {
    entity: Entity,
    content: Option<Box<dyn Fn(&mut Context)>>,
    /// Owns the reactive scope; dropping/disposing it cleans up the `UpdaterEffect`.
    scope: Scope,
    marker: std::marker::PhantomData<T>,
}

impl<T: 'static + Clone> Binding<T> {
    /// Creates a new `Binding`.
    ///
    /// * `signal` — any value implementing [`SignalGet<T>`], typically a [`Signal<T>`].
    /// * `builder` — closure called immediately and on every subsequent signal change to
    ///   (re)build child views.
    #[allow(clippy::new_ret_no_self)]
    pub fn new<S, F>(cx: &mut Context, signal: S, builder: F)
    where
        S: SignalGet<T> + 'static,
        F: 'static + Fn(&mut Context),
    {
        let entity = cx.entity_manager.create();
        cx.tree.add(entity, cx.current()).expect("Failed to add to tree");
        cx.tree.set_ignored(entity, true);

        let scope = Scope::new();

        // Create an UpdaterEffect under the scope.
        // `compute` reads the signal (subscribing to it); `on_change` queues a rebuild.
        scope.enter(|| {
            UpdaterEffect::new(
                move || signal.get(),
                move |_new_value| {
                    SIGNAL_REBUILDS.with_borrow_mut(|set| {
                        set.insert(entity);
                    });
                },
            )
        });

        let binding = Self {
            entity,
            content: Some(Box::new(builder)),
            scope,
            marker: std::marker::PhantomData,
        };

        // Build initial content.
        if let Some(b) = &binding.content {
            cx.with_current(entity, |cx| {
                (b)(cx);
            });
        }

        cx.bindings.insert(entity, Box::new(binding));

        let _: Handle<Self> =
            Handle { current: entity, entity, p: Default::default(), cx }.ignore();
    }
}

impl<T: 'static + Clone> BindingHandler for Binding<T> {
    fn update(&mut self, cx: &mut Context) {
        cx.remove_children(cx.current());

        if let Some(b) = &self.content {
            cx.with_current(self.entity, |cx| {
                (b)(cx);
            });
        }
    }

    fn remove(&self, _cx: &mut Context) {
        // Disposing the scope cleans up the UpdaterEffect and unsubscribes from the signal.
        self.scope.dispose();
    }

    fn debug(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str("Binding")
    }
}
