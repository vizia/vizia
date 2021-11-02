use crate::{Context, Event, View};



pub struct Store<T> {
    pub data: T,
}

impl<T> Store<T> {

    pub fn new(data: T) -> Self {
        Self {
            data,
        }
    }

    fn update(&self, cx: &mut Context, event: &mut Event) {
        println!("Update observers");
    }
}

impl<T: 'static> View for Store<T> {
    fn event(&mut self, cx: &mut Context, event: &mut Event) {
        self.update(cx, event);
    }
}

