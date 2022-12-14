pub trait Data: 'static + Clone + PartialEq {}

impl<D: 'static + Clone + PartialEq> Data for D {}
