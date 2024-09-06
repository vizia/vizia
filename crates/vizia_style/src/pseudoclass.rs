use cssparser::*;

use crate::{Direction, Selectors};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PseudoClass {
    Hover,
    Active,
    Over,
    Focus,
    FocusVisible,
    FocusWithin,

    Enabled,
    Disabled,
    ReadOnly,
    ReadWrite,
    PlaceHolderShown,
    Default,
    Checked,
    Indeterminate,
    Blank,
    Valid,
    Invalid,
    InRange,
    OutOfRange,
    Required,
    Optional,
    UserValid,
    UserInvalid,

    Lang(Vec<String>),
    Dir(Direction),
    Custom(String),
}

impl ToCss for PseudoClass {
    fn to_css<W>(&self, dest: &mut W) -> std::fmt::Result
    where
        W: std::fmt::Write,
    {
        match *self {
            Self::Hover => dest.write_str(":hover"),
            Self::Active => dest.write_str(":active"),
            Self::Over => dest.write_str(":over"),
            Self::Focus => dest.write_str(":focus"),
            Self::FocusVisible => dest.write_str(":focus-visible"),
            Self::FocusWithin => dest.write_str(":focus-within"),
            Self::Enabled => dest.write_str(":enabled"),
            Self::Disabled => dest.write_str(":disabled"),
            Self::ReadOnly => dest.write_str(":read-only"),
            Self::ReadWrite => dest.write_str(":read-write"),
            Self::PlaceHolderShown => dest.write_str(":placeholder-shown"),
            Self::Default => dest.write_str(":default"),
            Self::Checked => dest.write_str(":checked"),
            Self::Indeterminate => dest.write_str(":indeterminate"),
            Self::Blank => dest.write_str(":blank"),
            Self::Valid => dest.write_str(":valid"),
            Self::Invalid => dest.write_str(":invalid"),
            Self::InRange => dest.write_str(":in-range"),
            Self::OutOfRange => dest.write_str(":out-of-range"),
            Self::Required => dest.write_str(":required"),
            Self::Optional => dest.write_str(":optional"),
            Self::UserValid => dest.write_str(":user-valid"),
            Self::UserInvalid => dest.write_str(":user-invalid"),
            Self::Lang(ref _lang) => dest.write_str(":lang()"),
            Self::Dir(_) => dest.write_str(":dir()"),
            Self::Custom(_) => dest.write_str(":custom"),
        }
    }
}

impl selectors::parser::NonTSPseudoClass for PseudoClass {
    type Impl = Selectors;

    fn is_active_or_hover(&self) -> bool {
        matches!(*self, Self::Active | Self::Hover)
    }

    fn is_user_action_state(&self) -> bool {
        matches!(*self, Self::Active | Self::Hover | Self::Focus)
    }
}
