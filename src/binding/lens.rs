
pub trait Lens: Clone {
    type Source;
    type Target;

    fn view<'a>(&self, source: &'a Self::Source) -> &'a Self::Target;
}