//! This is the module where all types which implement View live.
//!
//! Every member of this module is part of the prelude.

mod button;
mod checkbox;
mod chip;
mod colorpicker;
mod datepicker;
mod datetimepicker;
mod dropdown;
mod element;
mod image;
mod knob;
mod label;
mod list;
// mod menu;
mod menu2;
pub mod normalized_map;
mod notification;
mod picklist;
mod popup;
mod radio_buttons;
mod scrollbar;
mod scrollview;
mod slider;
mod spinbox;
mod stack;
mod switch;
mod tab;
mod table;
mod textbox;
mod timepicker;
mod tooltip;
mod xypad;

pub use self::image::Image;
pub use button::Button;
pub use checkbox::Checkbox;
pub use chip::Chip;
pub use colorpicker::ColorPicker;
pub use datepicker::Datepicker;
pub use datetimepicker::DatetimePicker;
pub use dropdown::Dropdown;
pub use element::Element;
pub use knob::{ArcTrack, Knob, KnobMode, TickKnob, Ticks};
pub use label::Label;
pub use list::List;
// pub use menu::{setup_menu_entry, Menu, MenuButton, MenuController, MenuEvent, MenuStack};
pub use menu2::*;
pub use notification::Notification;
pub use picklist::PickList;
pub use popup::{Popup, PopupData, PopupEvent};
pub use radio_buttons::RadioButton;
pub use scrollbar::Scrollbar;
pub use scrollview::{ScrollData, ScrollEvent, ScrollView};
pub use slider::{NamedSlider, Slider};
pub use spinbox::{Spinbox, SpinboxIcons, SpinboxKind};
pub use stack::{HStack, VStack, ZStack};
pub use switch::Switch;
pub use tab::{TabPair, TabView};
pub use table::{Table, TableColumn};
pub use textbox::{TextEvent, Textbox};
pub use timepicker::{
    AMOrPM, AnalogTimepicker, AnalogTimepickerEvent, AnalogTimepickerPage, DayTime,
    DigitalTimepicker, DigitalTimepickerEvent, Timepicker,
};
pub use tooltip::{Tooltip, TooltipEvent, TooltipSeq};
pub use xypad::XYPad;

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
