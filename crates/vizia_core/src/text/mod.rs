mod movement;
pub use movement::*;

mod direction;
pub(crate) use direction::*;

pub(crate) mod scrolling;
pub(crate) use scrolling::*;

pub(crate) mod text_context;
pub(crate) use text_context::*;

pub mod editable_text;
pub use editable_text::*;

pub mod selection;
pub use selection::*;

pub mod backspace;
pub use backspace::*;

pub mod preedit_backup;
pub use preedit_backup::*;

#[allow(unused_imports)]
pub mod shaped_text;
#[allow(unused_imports)]
pub use shaped_text::{
    GlyphClusterInfo, GlyphPosition, LineMetrics, ShapedText, TextBox,
};

pub(crate) mod shaper;
pub(crate) use shaper::build_pre_shaped_text;
