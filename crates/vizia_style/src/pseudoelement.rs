use cssparser::*;

use crate::Selectors;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PseudoElement {
    After,
    Before,
    Selection,
    Custom(String),
}

impl ToCss for PseudoElement {
    fn to_css<W>(&self, _dest: &mut W) -> std::fmt::Result
    where
        W: std::fmt::Write,
    {
        match *self {
            PseudoElement::After => todo!(),
            PseudoElement::Before => todo!(),
            PseudoElement::Selection => todo!(),
            PseudoElement::Custom(_) => todo!(),
        }
    }
}

impl selectors::parser::PseudoElement for PseudoElement {
    type Impl = Selectors;

    fn accepts_state_pseudo_classes(&self) -> bool {
        true
    }

    // TODO - Remove this
    fn valid_after_slotted(&self) -> bool {
        matches!(*self, PseudoElement::Before | PseudoElement::After)
    }
}
