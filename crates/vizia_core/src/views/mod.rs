//! Built-in views provided by vizia.

mod avatar;
mod badge;
mod button;
mod checkbox;
mod chip;
mod collapsible;
mod combobox;
mod datepicker;
mod divider;
mod dropdown;
mod element;
mod grid;
mod image;
mod knob;
mod label;
mod list;
mod markdown;
mod menu;
mod picklist;
mod popup;
mod progressbar;
mod radio;
mod rating;
mod resizable_stack;
mod scrollbar;
mod scrollview;
mod slider;
mod spinbox;
mod stack;
mod switch;
mod tabview;
mod textbox;
mod toggle_button;
mod tooltip;
mod virtual_list;
mod virtual_table;
mod xypad;

pub use crate::binding::Binding;
pub use avatar::*;
pub use badge::*;
pub use button::*;
pub use checkbox::*;
pub use chip::*;
pub use collapsible::*;
pub use combobox::*;
pub use datepicker::*;
pub use divider::*;
pub use dropdown::*;
pub use element::*;
pub use grid::*;
pub use image::*;
pub use knob::*;
pub use label::*;
pub use list::*;
#[cfg(feature = "markdown")]
pub use markdown::*;
pub use menu::*;
pub use picklist::*;
pub use popup::*;
pub use progressbar::*;
pub use radio::*;
pub use rating::*;
pub use resizable_stack::*;
pub use scrollbar::*;
pub use scrollview::*;
pub use slider::*;
pub use spinbox::*;
pub use stack::*;
pub use switch::*;
pub use tabview::*;
pub use textbox::*;
pub use toggle_button::*;
pub use tooltip::*;
pub use virtual_list::*;
pub use virtual_table::*;
pub use xypad::*;

use crate::prelude::*;

/// The orientation of a widget, such as a slider or scrollbar
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Data)]
pub enum Orientation {
    #[default]
    /// A horizontal orientation.
    Horizontal,
    /// A vertical orientation.
    Vertical,
}

impl_res_simple!(Orientation);

/// Describes the placement of a view relative to its parent element.
#[derive(Debug, Clone, Copy, Data, PartialEq, Eq)]
pub enum Placement {
    /// The view should be placed above its parent with its left edge aligned with the left edge of its parent.
    TopStart,
    /// The view should be placed above its parent with its center aligned with the center of its parent.
    Top,
    /// The view should be placed above its parent with its right edge aligned with the right edge of its parent.
    TopEnd,
    /// The view should be placed below its parent with its left edge aligned with the left edge of its parent.
    BottomStart,
    /// The view should be placed below its parent with its center aligned with the center of its parent.
    Bottom,
    /// The view should be placed below its parent with its right edge aligned with the right edge of its parent.
    BottomEnd,
    /// The view should be placed to the right of its parent with its top edge aligned with the top edge of its parent.
    RightStart,
    /// The view should be placed to the right of its parent with its center aligned with the center of its parent.
    Right,
    /// The view should be placed to the right of its parent with its bottom edge aligned with the bottom edge of its parent.
    RightEnd,
    /// The view should be placed to the left of its parent with its top edge aligned with the top edge of its parent.
    LeftStart,
    /// The view should be placed to the left of its parent with its center aligned with the center of its parent.
    Left,
    /// The view should be placed to the left of its parent with its bottom edge aligned with the bottom edge of its parent.
    LeftEnd,
    /// The view should be placed over its parent.
    Over,
    /// The view should follow the cursor.
    Cursor,
}

impl_res_simple!(Placement);
