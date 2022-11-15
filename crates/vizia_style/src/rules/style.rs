use parcel_selectors::SelectorList;

use crate::{CssRuleList, DeclarationBlock, Location, Selectors};

#[derive(Debug, Clone, PartialEq)]
pub struct StyleRule<'i> {
    pub selectors: SelectorList<'i, Selectors>,
    pub declarations: DeclarationBlock<'i>,
    pub rules: CssRuleList<'i>,
    pub loc: Location,
}
