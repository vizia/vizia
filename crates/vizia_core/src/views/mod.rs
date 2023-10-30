//! Built-in views provided by vizia.

mod button;
mod checkbox;
mod chip;
mod combobox;
mod datepicker;
mod dropdown;
mod element;
mod image;
mod knob;
mod label;
mod list;
mod menu;
pub mod normalized_map;
mod notification;
mod picklist;
mod popup;
mod progressbar;
mod radio;
mod rating;
mod scrollbar;
mod scrollview;
mod slider;
mod spinbox;
mod stack;
mod switch;
mod tabview;
mod textbox;
mod timepicker;
mod tooltip;
mod virtual_list;

pub use self::image::Image;
pub use crate::binding::Binding;
pub use button::Button;
pub use checkbox::Checkbox;
pub use chip::Chip;
pub use combobox::*;
pub use datepicker::Datepicker;
pub use dropdown::Dropdown;
pub use element::Element;
pub use knob::{ArcTrack, Knob, KnobMode, TickKnob, Ticks};
pub use label::{Icon, Label};
pub use list::List;
pub use menu::*;
pub use notification::Notification;
pub use picklist::PickList;
pub use popup::*;
pub use progressbar::ProgressBar;
pub use radio::RadioButton;
pub use rating::Rating;
pub use scrollbar::Scrollbar;
pub use scrollview::{ScrollData, ScrollEvent, ScrollView};
pub use slider::{NamedSlider, Slider};
pub use spinbox::{Spinbox, SpinboxEvent, SpinboxIcons, SpinboxKind};
pub use stack::{HStack, VStack, ZStack};
pub use switch::Switch;
pub use tabview::{TabPair, TabView};
pub use textbox::{TextEvent, Textbox};
pub use timepicker::{
    AMOrPM, AnalogTimepicker, AnalogTimepickerEvent, AnalogTimepickerPage, DayTime,
    DigitalTimepicker, DigitalTimepickerEvent, Timepicker,
};
pub use tooltip::Tooltip;
pub use virtual_list::*;

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
