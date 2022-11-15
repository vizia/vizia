pub mod style;
pub use style::*;
pub mod property;
pub use property::*;

use cssparser::*;

use crate::{CustomParseError, ParserOptions, Property};

#[derive(Debug, PartialEq, Clone)]
pub struct CssRuleList<'i>(pub Vec<CssRule<'i>>);

#[derive(Debug, PartialEq, Clone)]
pub enum CssRule<'i> {
    Style(StyleRule<'i>),
    Property(PropertyRule<'i>),
    Ignored,
    //Keyframes(KeyframesRule<'i>),
}

impl<'i> CssRule<'i> {
    // Parse a single rule
    // pub fn parse<'t>(
    //     input: &mut Parser<'i, 't>,
    //     options: &ParserOptions<'i>,
    // ) -> Result<Self, ParseError<'i, CustomParseError<'i>>>{

    // }
}

mod tests {
    use super::*;

    const CSS_EXAMPLE: &str = r#"
        button {
            background-color: blue;
        }
    "#;

    #[test]
    fn parse_rule() {
        //let input = ParserInput::new(CSS_EXAMPLE);
        //let mut parser = Parser::new(&mut input);
        //let rule_parser = RuleParser::new();
    }
}
