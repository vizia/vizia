use crate::Res;



pub trait WindowModifiers {
    fn title<T: ToString>(self, title: impl Res<T>) -> Self;
    fn inner_size(self, width: u32, height: u32) -> Self;
}