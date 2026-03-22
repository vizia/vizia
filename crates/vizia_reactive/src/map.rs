use crate::{MappedSignal, SignalGet, SignalWith};

pub trait SignalMapExt<T>: SignalGet<T> + SignalWith<T> + Copy + 'static
where
    T: Clone + 'static,
{
    fn map<U, F>(self, map: F) -> MappedSignal<T, U, Self, F>
    where
        U: Clone + 'static,
        F: 'static + Clone + Fn(&T) -> U,
    {
        MappedSignal::new(self, map)
    }
}

impl<T, S> SignalMapExt<T> for S
where
    S: SignalGet<T> + SignalWith<T> + Copy + 'static,
    T: Clone + 'static,
{
}
