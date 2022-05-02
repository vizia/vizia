//! This is the module where all types which implement View live.
//!
//! Every member of this module is part of the prelude.

mod label;
mod menu;
mod stack;
mod button;
mod list;
mod table;
mod textbox;
mod checkbox;
mod dropdown;
mod element;
mod slider;
mod knob;
pub mod normalized_map;
mod popup;
mod scrollview;
mod scrollbar;
mod radio_buttons;
mod image;

pub use label::Label;
pub use stack::{HStack, VStack, ZStack};
pub use button::Button;
pub use list::List;
pub use table::{Table, TableColumn};
pub use textbox::Textbox;
pub use checkbox::Checkbox;
pub use dropdown::Dropdown;
pub use element::Element;
pub use slider::Slider;
pub use knob::{ArcTrack, Knob, KnobMode, TickKnob, Ticks};
pub use popup::{PopupEvent, Popup, PopupData};
pub use scrollview::{ScrollView, ScrollEvent, ScrollData};
pub use scrollbar::Scrollbar;
pub use radio_buttons::RadioButton;
pub use self::image::Image;
pub use menu::{MenuController, Menu, MenuEvent, MenuButton, MenuStack, setup_menu_entry};

use crate::prelude::*;

/// The orientation of a widget, such as a slider or scrollbar
#[derive(Debug, Clone, Copy, PartialEq, Eq, Data)]
pub enum Orientation {
    Horizontal,
    Vertical,
}

impl Default for Orientation {
    fn default() -> Self {
        Orientation::Horizontal
    }
}
