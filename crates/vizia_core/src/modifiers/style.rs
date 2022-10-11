use super::internal;
use crate::prelude::*;

pub trait StyleModifiers: internal::Modifiable {
    // Background Properties

    modifier!(background_color, Color);

    // Border Properties
    modifier!(border_width, Units);
    modifier!(border_color, Color);

    modifier!(border_radius_bottom_left, Units);
    modifier!(border_radius_bottom_right, Units);
    modifier!(border_radius_bottom_top, Units);
    modifier!(border_radius_bottom_bottom, Units);
}
