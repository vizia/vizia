use crate::{macros::define_enum, Parse};

define_enum! {
    /// A font weight keyword corresponding to a specific font weight.
    pub enum FontWeightKeyword {
        /// 100
        "thin": Thin,
        "hairline": Hairline,
        /// 200
        "extra-light": ExtraLight,
        "ultra-light": UltraLight,
        /// 300
        "light": Light,
        /// 400.
        "normal": Normal,
        "regular": Regular,
        /// 500
        "medium": Medium,
        /// 600
        "semi-bold": SemiBold,
        "demi-bold": DemiBold,
        /// 700,
        "bold": Bold,
        /// 800
        "extra-bold": ExtraBold,
        "ultra-bold": UltraBold,
        /// 900
        "black": Black,
        "heavy": Heavy,
        /// 950
        "extra-black": ExtraBlack,
        "ultra-black": UltraBlack,
        // TODO
        // /// One relative weight lighter than the parent.
        // "lighter": Lighter,
        // /// One relative weight bolder than the parent.
        // "bolder": Bolder,
    }
}
