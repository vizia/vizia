use cssparser::*;

use crate::{CustomParseError, DeclarationBlock, Ident, Parse, ParserOptions, Percentage};

#[derive(Debug, PartialEq, Clone)]
pub enum KeyframesName<'i> {
    Ident(Ident<'i>),
    Custom(CowRcStr<'i>),
}

impl<'i> KeyframesName<'i> {
    pub fn as_string(&self) -> String {
        match self {
            KeyframesName::Ident(name) => name.0.to_owned().to_string(),
            KeyframesName::Custom(custom) => custom.to_owned().to_string(),
        }
    }
}

impl<'i> Parse<'i> for KeyframesName<'i> {
    fn parse<'t>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i, CustomParseError<'i>>> {
        match input.next()?.clone() {
            Token::Ident(ref s) => {
                // CSS-wide keywords without quotes throws an error.
                match_ignore_ascii_case! { s,
                    "none" | "initial" | "inherit" | "unset" | "default" | "revert" | "revert-layer" => {
                        Err(input.new_unexpected_token_error(Token::Ident(s.clone())))
                    },
                    _ => {
                        Ok(KeyframesName::Ident(Ident(s.clone())))
                    }
                }
            }

            Token::QuotedString(ref s) => Ok(KeyframesName::Custom(s.clone())),
            t => return Err(input.new_unexpected_token_error(t.clone())),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct KeyframesRule<'i> {
    pub name: KeyframesName<'i>,
    pub keyframes: Vec<Keyframe<'i>>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum KeyframeSelector {
    Percentage(Percentage),
    From,
    To,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Keyframe<'i> {
    pub selectors: Vec<KeyframeSelector>,
    pub declarations: DeclarationBlock<'i>,
}

impl<'i> Parse<'i> for KeyframeSelector {
    fn parse<'t>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i, CustomParseError<'i>>> {
        if let Ok(val) = input.try_parse(Percentage::parse) {
            return Ok(KeyframeSelector::Percentage(val));
        }

        let location = input.current_source_location();
        let ident = input.expect_ident()?;
        match_ignore_ascii_case! { ident,
            "from" => Ok(KeyframeSelector::From),
            "to" => Ok(KeyframeSelector::To),
            _ => Err(location.new_unexpected_token_error(
                cssparser::Token::Ident(ident.clone())
            ))
        }
    }
}

pub struct KeyframeListParser;

impl<'i> AtRuleParser<'i> for KeyframeListParser {
    type Prelude = ();
    type AtRule = Keyframe<'i>;
    type Error = CustomParseError<'i>;
}

impl<'i> QualifiedRuleParser<'i> for KeyframeListParser {
    type Prelude = Vec<KeyframeSelector>;
    type QualifiedRule = Keyframe<'i>;
    type Error = CustomParseError<'i>;

    fn parse_prelude<'t>(
        &mut self,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self::Prelude, ParseError<'i, CustomParseError<'i>>> {
        input.parse_comma_separated(KeyframeSelector::parse)
    }

    fn parse_block<'t>(
        &mut self,
        selectors: Self::Prelude,
        _: &ParserState,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self::QualifiedRule, ParseError<'i, CustomParseError<'i>>> {
        // For now there are no options that apply within @keyframes
        let options = ParserOptions::default();
        Ok(Keyframe { selectors, declarations: DeclarationBlock::parse(input, &options)? })
    }
}
