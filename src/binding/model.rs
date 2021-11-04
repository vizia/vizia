use std::any::{Any, TypeId};

use crate::{Context, Entity, Event, Store, sparse_set::SparseSet};



pub trait Model: 'static + Sized {
    fn build(self, cx: &mut Context) {
        if let Some(data_list) = cx.data.model_data.get_mut(cx.current) {
            data_list.push(Box::new(Store::new(self)));
        } else {
            let mut data_list: Vec<Box<dyn ModelData>> = Vec::new();
            data_list.push(Box::new(Store::new(self)));
            cx.data.model_data.insert(cx.current, data_list);
        }
        
    }

    fn event(&mut self, cx: &mut Context, event: &mut Event) -> bool {
        false
    }

    fn update(&mut self, cx: &mut Context) {

    }
}

pub trait ModelData: Any {
    fn event(&mut self, cx: &mut Context, event: &mut Event) {
        println!("Default");
    }

    fn update(&self) -> Vec<Entity> {
        println!("Default");
        Vec::new()
    }

    fn is_dirty(&self) -> bool {
        false
    }

    fn reset(&mut self) {
        
    }
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
    fn event(&mut self, cx: &mut Context, event: &mut Event) {
        if <T as Model>::event(&mut self.data, cx, event) {
            self.dirty = true;
        }
    }

    fn update(&self) -> Vec<Entity> {
        self.observers.clone()
    }

    fn is_dirty(&self) -> bool {
        self.dirty
    }

    fn reset(&mut self) {
        self.dirty = false;
    }
}


pub struct DataId {
    entity: Entity,
    type_id: TypeId,
}

pub struct Data {
    // pub model_data: HashMap<TypeId, Box<dyn ModelData>>,
    pub model_data: SparseSet<Vec<Box<dyn ModelData>>>,
}

impl Data {
    pub fn new() -> Self {
        Self {
            model_data: SparseSet::default(),
        }
    }

    // pub fn data<T: 'static>(&self) -> Option<&T> {
    //     self.model_data
    //         .get(cx.current)
    //         .get(&TypeId::of::<T>())
    //         .and_then(|model| model.downcast_ref::<Store<T>>())
    //         .map(|store| &store.data)
    // }
}

