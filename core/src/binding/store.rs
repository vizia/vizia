use std::collections::HashSet;

use crate::{Context, Entity};



pub struct Store<T> {
    pub data: T,
    pub observers: HashSet<Entity>,
    pub dirty: bool,
}

impl<T> Store<T> {

    pub fn new(data: T) -> Self {
        Self {
            data,
            observers: HashSet::new(),
            dirty: false,
        }
    }

    pub fn insert_observer(&mut self, entity: Entity) {
        self.observers.insert(entity);
        
    }

    pub fn remove_observer(&mut self, entity: Entity) {
        self.observers.remove(&entity);
    }

    // pub fn needs_update(&mut self) {
    //     self.dirty = true;
    // }

    pub fn update_observers(&mut self, cx: &mut Context) {
        if self.dirty {

            for observer in self.observers.iter() {
                if let Some(mut view) = cx.views.remove(observer) {
    
                    let prev = cx.current;
                    cx.current = *observer;
                    view.body(cx);
                    cx.current = prev;
        
    
                    cx.views.insert(*observer, view);
                }
            }
    
            self.dirty = false;
        }
    }
}

// impl<T: 'static> View for Store<T> {
//     fn event(&mut self, cx: &mut Context, event: &mut Event) {
//         self.update(cx, event);
//     }
// }

