pub trait Data: 'static + Clone + Eq {}

impl<D: 'static + Clone + Eq> Data for D {}
