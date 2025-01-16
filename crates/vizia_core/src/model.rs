//! Models are used to store application data and can be bound to by views to visually display the data.

use crate::prelude::*;
use hashbrown::HashMap;
use std::any::{Any, TypeId};

/// A trait implemented by application data in order to respond to events and mutate state.
///
/// # Examples
///
/// ```
/// # use vizia_core::prelude::*;
/// #
/// pub struct AppData {
///     count: i32,
/// }
///
/// enum AppEvent {
///     Increment,
///     Decrement,
/// }
///
/// impl Model for AppData {
///     fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
///         event.map(|app_event, _| match app_event {
///             AppEvent::Increment => {
///                 self.count += 1;
///             }
///
///             AppEvent::Decrement => {
///                 self.count -= 1;
///             }
///         });
///     }
/// }
/// ```
pub trait Model: 'static + Sized {
    /// Build the model data into the application tree.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use vizia_core::prelude::*;
    /// # use vizia_winit::application::Application;
    /// #
    /// # #[derive(Default, Lens)]
    /// # pub struct AppData {
    /// #     count: i32,
    /// # }
    /// #
    /// # impl Model for AppData {}
    /// #
    /// fn main() {
    ///     Application::new(|cx|{
    ///         AppData::default().build(cx);
    ///     }).run();  
    /// }
    /// ```
    fn build(self, cx: &mut Context) {
        let current = if cx.tree.is_ignored(cx.current) {
            cx.tree.get_layout_parent(cx.current).unwrap()
        } else {
            cx.current
        };

        if let Some(models) = cx.models.get_mut(&current) {
            models.insert(TypeId::of::<Self>(), Box::new(self));
        } else {
            let mut models: HashMap<TypeId, Box<dyn ModelData>> = HashMap::new();
            models.insert(TypeId::of::<Self>(), Box::new(self));
            cx.models.insert(current, models);
        }
    }

    /// Respond to events in order to mutate the model data.
    ///
    /// # Examples
    ///
    /// ```
    /// # use vizia_core::prelude::*;
    /// # use vizia_derive::*;
    /// # use vizia_winit::application::Application;
    /// #
    /// # #[derive(Default, Lens)]
    /// # pub struct AppData {
    /// #     count: i32,
    /// # }
    /// #
    /// # enum AppEvent {
    /// #     Increment,
    /// #     Decrement,
    /// # }
    /// #
    /// impl Model for AppData {
    ///     fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
    ///         event.map(|app_event, _| match app_event {
    ///             AppEvent::Increment => {
    ///                 self.count += 1;
    ///             }
    ///
    ///             AppEvent::Decrement => {
    ///                 self.count -= 1;
    ///             }
    ///         });
    ///     }
    /// }
    /// ```
    #[allow(unused_variables)]
    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {}

    /// Returns the name of the [Model] if it has one.
    #[cfg(debug_assertions)]
    fn name(&self) -> Option<&'static str> {
        None
    }
}

pub(crate) trait ModelData: Any {
    #[allow(unused_variables)]
    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {}

    fn as_any_ref(&self) -> &dyn Any;
    #[cfg(debug_assertions)]
    fn name(&self) -> Option<&'static str>;
}

impl dyn ModelData {
    pub fn downcast_ref<T: Any>(&self) -> Option<&T> {
        self.as_any_ref().downcast_ref()
    }
}

impl<T: Model> ModelData for T {
    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        <T as Model>::event(self, cx, event);
    }

    fn as_any_ref(&self) -> &dyn Any {
        self
    }

    #[cfg(debug_assertions)]
    fn name(&self) -> Option<&'static str> {
        <T as Model>::name(self)
    }
}

impl Model for () {}
