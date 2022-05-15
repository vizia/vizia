use crate::events::ViewHandler;
use std::{
    any::{Any, TypeId},
    collections::HashMap,
};

use crate::prelude::*;

/// A trait implemented by application data in order to mutate in response to events.
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
///     fn event(&mut self, cx: &mut Context, event: &mut Event) {
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
///
/// This trait is part of the prelude.
pub trait Model: 'static + Sized + Any {
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
        if let Some(data_list) = cx.data.get_mut(cx.current()) {
            data_list.data.insert(TypeId::of::<Self>(), Box::new(self));
        } else {
            let mut data_list: HashMap<TypeId, Box<dyn ModelData>> = HashMap::new();
            data_list.insert(TypeId::of::<Self>(), Box::new(self));
            cx.data
                .insert(
                    cx.current(),
                    ModelDataStore {
                        data: data_list,
                        lenses_dedup: HashMap::default(),
                        lenses_dup: vec![],
                    },
                )
                .expect("Failed to add data");
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
    ///     fn event(&mut self, cx: &mut Context, event: &mut Event) {
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
    fn event(&mut self, cx: &mut Context, event: &mut Event) {}
}

pub trait ModelData: Any {
    #[allow(unused_variables)]
    fn event(&mut self, cx: &mut Context, event: &mut Event) {}

    fn as_any_ref(&self) -> &dyn Any;
}

impl dyn ModelData {
    pub fn downcast_ref<T: Any>(&self) -> Option<&T> {
        self.as_any_ref().downcast_ref()
    }
}

impl<T: Model> ModelData for T {
    fn event(&mut self, cx: &mut Context, event: &mut Event) {
        <T as Model>::event(self, cx, event);
    }

    fn as_any_ref(&self) -> &dyn Any {
        self
    }
}

#[derive(Default)]
pub(crate) struct ModelDataStore {
    pub data: HashMap<TypeId, Box<dyn ModelData>>,
    pub lenses_dedup: HashMap<TypeId, Box<dyn StoreHandler>>,
    pub lenses_dup: Vec<Box<dyn StoreHandler>>,
}

#[derive(Copy, Clone)]
pub enum ModelOrView<'a> {
    Model(&'a dyn ModelData),
    View(&'a dyn ViewHandler),
}

impl<'a> ModelOrView<'a> {
    pub fn downcast_ref<T: 'static>(self) -> Option<&'a T> {
        match self {
            ModelOrView::Model(m) => m.downcast_ref(),
            ModelOrView::View(v) => v.downcast_ref(),
        }
    }
}
