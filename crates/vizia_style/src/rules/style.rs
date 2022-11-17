use selectors::SelectorList;

use crate::{CssRuleList, DeclarationBlock, Location, Selectors};

#[derive(Debug, Clone, PartialEq)]
pub struct StyleRule<'i> {
    pub selectors: SelectorList<Selectors>,
    pub declarations: DeclarationBlock<'i>,
    pub rules: CssRuleList<'i>,
    pub loc: Location,
}
