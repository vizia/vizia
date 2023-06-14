pub trait Setter {
    type Source;

    fn apply(self, source: &mut Self::Source);
    fn swap(&mut self, source: &mut Self::Source);
}
