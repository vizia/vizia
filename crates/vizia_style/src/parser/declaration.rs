use crate::{CustomParseError, Property};
use cssparser::{CowRcStr, ParseError, Parser};

#[derive(Debug)]
pub struct DeclarationParser;

impl<'i> cssparser::DeclarationParser<'i> for DeclarationParser {
    type Declaration = Property<'i>;
    type Error = CustomParseError<'i>;

    fn parse_value<'t>(
        &mut self,
        name: CowRcStr<'i>,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self::Declaration, ParseError<'i, Self::Error>> {
        Property::parse_value(name, input)
    }
}

impl<'i> cssparser::AtRuleParser<'i> for DeclarationParser {
    type Prelude = ();
    type AtRule = Property<'i>;
    type Error = CustomParseError<'i>;
}
