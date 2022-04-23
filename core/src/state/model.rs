use std::{
    any::{Any, TypeId},
    collections::HashMap,
};

use crate::{Context, Event, LensWrap};

/// A trait implemented by application data in order to mutate in response to events.
///
/// # Examples
///
/// ```ignore
/// pub struct AppData {
///     some_data: bool,
/// }
///
/// enum AppEvent {
///     SetTrue,
///     SetFalse,
/// }
///
/// impl Model for AppData {
///     fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
///         event.map(|app_event, _| match app_event {
///             AppEvent::SetTrue => {
///                     self.some_data = true;
///             }
///
///             AppEvent::SetFalse => {
///                 self.some_data = false;
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
    /// ```ignore
    /// fn main() {
    ///     Application::new(WindowDescription::new(), |cx|{
    ///         AppData::default().build(cx);
    ///     }).run();  
    /// }
    /// ```
    fn build(self, cx: &mut Context) {
        if let Some(data_list) = cx.data.get_mut(cx.current) {
            data_list.data.insert(TypeId::of::<Self>(), Box::new(self));
        } else {
            let mut data_list: HashMap<TypeId, Box<dyn ModelData>> = HashMap::new();
            data_list.insert(TypeId::of::<Self>(), Box::new(self));
            cx.data
                .insert(
                    cx.current,
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
    /// Example
    /// ```ignore
    /// impl Model for AppData {
    ///     fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
    ///         event.map(|app_event, _| match app_event {
    ///             AppEvent::SetTrue => {
    ///                 self.some_data = true;
    ///             }
    ///
    ///             AppEvent::SetFalse => {
    ///                 self.some_data = false;
    ///             }
    ///         });
    ///     }
    /// }
    /// ```
    #[allow(unused_variables)]
    fn event(&mut self, cx: &mut Context, event: &mut Event) {}
}

pub(crate) trait ModelData: Any {
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
    pub lenses_dedup: HashMap<TypeId, Box<dyn LensWrap>>,
    pub lenses_dup: Vec<Box<dyn LensWrap>>,
}

impl Model for () {}
