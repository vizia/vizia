use cssparser::*;

use crate::Selectors;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PseudoElement<'i> {
    After,
    Before,
    Selection,
    Custom(CowRcStr<'i>),
}

impl<'i> parcel_selectors::parser::PseudoElement<'i> for PseudoElement<'i> {
    type Impl = Selectors;

    fn accepts_state_pseudo_classes(&self) -> bool {
        true
    }

    // TODO - Remove this
    fn valid_after_slotted(&self) -> bool {
        matches!(*self, PseudoElement::Before | PseudoElement::After)
    }
}
