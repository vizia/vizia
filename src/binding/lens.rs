

pub trait Lens: Clone {
    type Source;
    type Target;

    fn view<'a>(&self, source: &'a Self::Source) -> &'a Self::Target;
    //fn view_mut(&self, source: &mut Self::Source) -> &mut Self::Target;
}