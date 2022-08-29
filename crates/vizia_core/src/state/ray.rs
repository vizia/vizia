pub trait Ray {
    type Source;

    fn strike(&mut self, source: &mut Self::Source);
}
