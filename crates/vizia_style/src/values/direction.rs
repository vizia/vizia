use crate::{macros::define_enum, Parse};

define_enum! {
    #[derive(Default)]
    pub enum Direction {
        /// The entity will be rendered and acted on by the layout system.
        #[default]
        "ltr": Ltr,
        /// The entity will not be rendered and acted on by the layout system.
        "rtl": Rtl,
    }
}

impl From<bool> for Direction {
    fn from(boolean: bool) -> Self {
        if boolean {
            Self::Ltr
        } else {
            Self::Rtl
        }
    }
}
