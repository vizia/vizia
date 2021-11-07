pub trait Lens: 'static + Clone + std::fmt::Debug {
    type Source;
    type Target;

    fn view<'a>(&self, source: &'a Self::Source) -> &'a Self::Target;
}