use crate::prelude::*;
use crate::window::Position;

/// Methods for building a window.
///
/// This trait is part of the prelude.
pub trait WindowModifiers {
    fn title<T: ToString>(self, title: impl Res<T>) -> Self;
    fn inner_size<S: Into<WindowSize>>(self, size: impl Res<S>) -> Self;
    fn min_inner_size<S: Into<WindowSize>>(self, size: impl Res<Option<S>>) -> Self;
    fn max_inner_size<S: Into<WindowSize>>(self, size: impl Res<Option<S>>) -> Self;
    fn position<P: Into<Position>>(self, position: impl Res<P>) -> Self;
    fn resizable(self, flag: impl Res<bool>) -> Self;
    fn minimized(self, flag: impl Res<bool>) -> Self;
    fn maximized(self, flag: impl Res<bool>) -> Self;
    fn visible(self, flag: impl Res<bool>) -> Self;
    fn transparent(self, flag: bool) -> Self;
    fn decorations(self, flag: impl Res<bool>) -> Self;
    fn always_on_top(self, flag: impl Res<bool>) -> Self;
    fn vsync(self, flag: bool) -> Self;
    fn icon(self, image: Vec<u8>, width: u32, height: u32) -> Self;
    #[cfg(target_arch = "wasm32")]
    fn canvas(self, canvas: &str) -> Self;
}
