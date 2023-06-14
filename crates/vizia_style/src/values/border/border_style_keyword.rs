use crate::{define_enum, Parse};

define_enum! {
    /// The keyword of a single border style.
    pub enum BorderStyleKeyword {
        /// Specifies no border.
        "none": None,
        /// Specifies no border, except in border conflict resolution for table elements.
        "hidden": Hidden,
        /// Specifies a dotted border.
        "dotted": Dotted,
        /// Specifies a dashed border.
        "dashed": Dashed,
        /// Specifies a solid border.
        "solid": Solid,
        /// Specifies a double border.
        "double": Double,
        /// Specifies a 3D grooved border.
        "groove": Groove,
        /// Specifies a 3D ridged border.
        "ridge": Ridge,
        /// Specifies a 3D inset border.
        "inset": Inset,
        /// Specifies a 3D outset border.
        "outset": Outset,
    }
}

impl Default for BorderStyleKeyword {
    fn default() -> Self {
        Self::Solid
    }
}
