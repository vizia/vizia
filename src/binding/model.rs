use std::{any::{Any, TypeId}, collections::HashMap};

use crate::{Context, Store};



pub trait Model: 'static + Sized {
    fn build(self, cx: &mut Context) {
        cx.data.model_data.insert(
            TypeId::of::<Self>(), 
            Box::new(Store::new(self)));
    }

    fn update(self, cx: &mut Context) {
        
    }
}

pub trait ModelData: Any {

}

impl dyn ModelData {
    // Check if a message is a certain type
    pub fn is<T: Any + 'static>(&self) -> bool {
        // Get TypeId of the type this function is instantiated with
        let t = TypeId::of::<T>();

        // Get TypeId of the type in the trait object
        let concrete = self.type_id();

        // Compare both TypeIds on equality
        t == concrete
    }

    // Casts a message to the specified type if the message is of that type
    pub fn downcast<T>(&mut self) -> Option<&mut T>
    where
        T: ModelData + 'static,
    {
        if self.is::<T>() {
            unsafe { Some(&mut *(self as *mut dyn ModelData as *mut T)) }
        } else {
            None
        }
    }

    pub fn downcast_ref<T>(&self) -> Option<&T>
    where
        T: Any + 'static,
    {
        if self.is::<T>() {
            unsafe { Some(&*(self as *const dyn ModelData as *const T)) }
        } else {
            None
        }
    }
}

trait Downcast {
    fn as_any (self: &'_ Self)
      -> &'_ dyn Any
    where
        Self : 'static,
    ;
}

impl<T: ModelData> Downcast for T {
    fn as_any (self: &'_ Self)
      -> &'_ dyn Any
    where
        Self : 'static,
    {
        self
    }
}

impl<T: Model> ModelData for Store<T> {
    
}


pub struct Data {
    pub model_data: HashMap<TypeId, Box<dyn ModelData>>,
}

impl Data {
    pub fn new() -> Self {
        Self {
            model_data: HashMap::default(),
        }
    }

    pub fn data<T: 'static>(&self) -> Option<&T> {
        self.model_data
            .get(&TypeId::of::<T>())
            .and_then(|model| model.downcast_ref::<Store<T>>())
            .map(|store| &store.data)
    }
}

