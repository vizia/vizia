use crate::Model;

pub trait Lens: 'static + Clone + Copy + std::fmt::Debug {
    type Source: Model;
    type Target;

    fn view<'a>(&self, source: &'a Self::Source) -> &'a Self::Target;
    fn view_mut<'a>(&self, source: &'a mut Self::Source) -> &'a mut Self::Target;
}