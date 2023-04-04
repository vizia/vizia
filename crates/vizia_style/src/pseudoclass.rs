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
            PseudoClass::ReadWrite => todo!(),
            PseudoClass::PlaceHolderShown => todo!(),
            PseudoClass::Default => todo!(),
            PseudoClass::Checked => todo!(),
            PseudoClass::Indeterminate => todo!(),
            PseudoClass::Blank => todo!(),
            PseudoClass::Valid => todo!(),
            PseudoClass::Invalid => todo!(),
            PseudoClass::InRange => todo!(),
            PseudoClass::OutOfRange => todo!(),
            PseudoClass::Required => todo!(),
            PseudoClass::Optional => todo!(),
            PseudoClass::UserValid => todo!(),
            PseudoClass::UserInvalid => todo!(),
            PseudoClass::Lang(ref _lang) => todo!(),
            PseudoClass::Dir(_) => todo!(),
            PseudoClass::Custom(_) => todo!(),
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
