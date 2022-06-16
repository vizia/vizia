use crate::context::Context;

#[derive(Clone)]
pub struct KeymapEntry<T>
where
    T: 'static + Send + Sync + Copy + Clone,
{
    action: T,
    callback: Box<dyn Fn(&mut Context)>,
}

impl<T> std::fmt::Debug for KeymapEntry<T>
where
    T: 'static + std::fmt::Debug + Send + Sync + Copy + Clone,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.action)
    }
}

unsafe impl<T> Send for KeymapEntry<T> where T: 'static + Send + Sync + Copy + Clone {}
unsafe impl<T> Sync for KeymapEntry<T> where T: 'static + Send + Sync + Copy + Clone {}

impl<T> PartialEq for KeymapEntry<T>
where
    T: 'static + Send + Sync + Copy + Clone + PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.action == other.action
    }
}

impl<T> KeymapEntry<T>
where
    T: 'static + Send + Sync + Copy + Clone + PartialEq,
{
    pub fn new<F>(action: T, callback: F) -> Self
    where
        F: 'static + Fn(&mut Context),
    {
        Self { action, callback: Box::new(callback) }
    }

    pub fn callback(&self) -> &Box<dyn Fn(&mut Context)> {
        &self.callback
    }
}

impl<T> PartialEq<T> for KeymapEntry<T>
where
    T: 'static + Send + Sync + Copy + Clone + PartialEq,
{
    fn eq(&self, other: &T) -> bool {
        self.action == *other
    }
}
