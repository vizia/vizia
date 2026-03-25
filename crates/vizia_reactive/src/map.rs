use crate::{Memo, SignalWith};

pub trait SignalMap<T: 'static>: SignalWith<T> + Copy + 'static {
    fn map<U, F>(self, map: F) -> Memo<U>
    where
        U: Clone + PartialEq + 'static,
        F: 'static + Fn(&T) -> U,
    {
        Memo::new(move |_| self.with(|value| map(value)))
    }
}

impl<T: 'static, S> SignalMap<T> for S where S: SignalWith<T> + Copy + 'static {}
