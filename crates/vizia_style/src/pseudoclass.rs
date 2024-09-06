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
            PseudoClass::Hover => dest.write_str(":hover"),
            PseudoClass::Active => dest.write_str(":active"),
            PseudoClass::Over => dest.write_str(":over"),
            PseudoClass::Focus => dest.write_str(":focus"),
            PseudoClass::FocusVisible => dest.write_str(":focus-visible"),
            PseudoClass::FocusWithin => dest.write_str(":focus-within"),
            PseudoClass::Enabled => dest.write_str(":enabled"),
            PseudoClass::Disabled => dest.write_str(":disabled"),
            PseudoClass::ReadOnly => dest.write_str(":read-only"),
            PseudoClass::ReadWrite => dest.write_str(":read-write"),
            PseudoClass::PlaceHolderShown => dest.write_str(":placeholder-shown"),
            PseudoClass::Default => dest.write_str(":default"),
            PseudoClass::Checked => dest.write_str(":checked"),
            PseudoClass::Indeterminate => dest.write_str(":indeterminate"),
            PseudoClass::Blank => dest.write_str(":blank"),
            PseudoClass::Valid => dest.write_str(":valid"),
            PseudoClass::Invalid => dest.write_str(":invalid"),
            PseudoClass::InRange => dest.write_str(":in-range"),
            PseudoClass::OutOfRange => dest.write_str(":out-of-range"),
            PseudoClass::Required => dest.write_str(":required"),
            PseudoClass::Optional => dest.write_str(":optional"),
            PseudoClass::UserValid => dest.write_str(":user-valid"),
            PseudoClass::UserInvalid => dest.write_str(":user-invalid"),
            PseudoClass::Lang(ref _lang) => dest.write_str(":lang()"),
            PseudoClass::Dir(_) => dest.write_str(":dir()"),
            PseudoClass::Custom(_) => dest.write_str(":custom"),
        }
    }
}

impl selectors::parser::NonTSPseudoClass for PseudoClass {
    type Impl = Selectors;

    fn is_active_or_hover(&self) -> bool {
        matches!(*self, PseudoClass::Active | PseudoClass::Hover)
    }

    fn is_user_action_state(&self) -> bool {
        matches!(*self, PseudoClass::Active | PseudoClass::Hover | PseudoClass::Focus)
    }
}
