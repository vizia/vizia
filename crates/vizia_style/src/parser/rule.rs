use crate::{
    parse_declaration, CssRule, CssRuleList, CustomParseError, DeclarationBlock, DeclarationList,
    KeyframeListParser, KeyframesName, KeyframesRule, Location, Parse, ParserOptions,
    SelectorParser, Selectors, StyleRule,
};
use cssparser::*;
use selectors::{parser::ParseRelative, SelectorList};

#[derive(PartialEq, PartialOrd)]
enum State {
    Start = 1,
    // Layers = 2,
    // Imports = 3,
    // Namespaces = 4,
    Body = 5,
}

// Parser for top-level rules in a stylesheet
pub struct TopLevelRuleParser<'a, 'i> {
    pub options: &'a ParserOptions<'i>,
    state: State,
    rules: &'a mut CssRuleList<'i>,
}

impl<'a, 'b, 'i> TopLevelRuleParser<'a, 'i> {
    pub fn new(options: &'a ParserOptions<'i>, rules: &'a mut CssRuleList<'i>) -> Self {
        TopLevelRuleParser { options, state: State::Start, rules }
    }

    pub fn nested<'x: 'b>(&'x mut self) -> NestedRuleParser<'_, 'i> {
        NestedRuleParser {
            options: self.options,
            declarations: DeclarationList::new(),
            important_declarations: DeclarationList::new(),
            rules: self.rules,
            is_in_style_rule: false,
            allow_declarations: false,
        }
    }
}

pub enum AtRulePrelude<'i> {
    // Property(DashedIdent<'i>),
    Keyframes(KeyframesName<'i>),
}

impl<'a, 'i> AtRuleParser<'i> for TopLevelRuleParser<'a, 'i> {
    type Prelude = AtRulePrelude<'i>;
    type AtRule = ();
    type Error = CustomParseError<'i>;

    fn parse_prelude<'t>(
        &mut self,
        name: cssparser::CowRcStr<'i>,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self::Prelude, ParseError<'i, Self::Error>> {
        AtRuleParser::parse_prelude(&mut self.nested(), name, input)
    }

    fn parse_block<'t>(
        &mut self,
        prelude: Self::Prelude,
        start: &ParserState,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self::AtRule, ParseError<'i, Self::Error>> {
        self.state = State::Body;
        AtRuleParser::parse_block(&mut self.nested(), prelude, start, input)
    }
}

impl<'a, 'i> QualifiedRuleParser<'i> for TopLevelRuleParser<'a, 'i> {
    type Prelude = SelectorList<Selectors>;
    type QualifiedRule = ();
    type Error = CustomParseError<'i>;

    fn parse_prelude<'t>(
        &mut self,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self::Prelude, ParseError<'i, Self::Error>> {
        self.state = State::Body;
        QualifiedRuleParser::parse_prelude(&mut self.nested(), input)
    }

    fn parse_block<'t>(
        &mut self,
        prelude: Self::Prelude,
        start: &ParserState,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self::QualifiedRule, ParseError<'i, Self::Error>> {
        QualifiedRuleParser::parse_block(&mut self.nested(), prelude, start, input)
    }
}

pub struct NestedRuleParser<'a, 'i> {
    pub options: &'a ParserOptions<'i>,
    declarations: DeclarationList<'i>,
    important_declarations: DeclarationList<'i>,
    rules: &'a mut CssRuleList<'i>,
    is_in_style_rule: bool,
    allow_declarations: bool,
}

impl<'a, 'i> NestedRuleParser<'a, 'i> {
    pub fn parse_nested<'t>(
        &mut self,
        input: &mut Parser<'i, 't>,
        is_style_rule: bool,
    ) -> Result<(DeclarationBlock<'i>, CssRuleList<'i>), ParseError<'i, CustomParseError<'i>>> {
        let mut rules = CssRuleList(vec![]);
        let mut nested_parser = NestedRuleParser {
            options: self.options,
            declarations: DeclarationList::new(),
            important_declarations: DeclarationList::new(),
            rules: &mut rules,
            is_in_style_rule: self.is_in_style_rule || is_style_rule,
            allow_declarations: self.allow_declarations || self.is_in_style_rule || is_style_rule,
        };

        let parse_declarations = nested_parser.parse_declarations();
        let mut errors = Vec::new();
        let mut iter = RuleBodyParser::new(input, &mut nested_parser);
        while let Some(result) = iter.next() {
            match result {
                Ok(()) => {}
                Err((e, _)) => {
                    if parse_declarations {
                        iter.parser.declarations.clear();
                        iter.parser.important_declarations.clear();
                        errors.push(e);
                    } else {
                        if iter.parser.options.error_recovery {
                            iter.parser.options.warn(e);
                            continue;
                        }
                        return Err(e);
                    }
                }
            }
        }

        if parse_declarations && !errors.is_empty() {
            if self.options.error_recovery {
                for err in errors {
                    self.options.warn(err);
                }
            } else {
                return Err(errors.remove(0));
            }
        }

        Ok((
            DeclarationBlock {
                declarations: nested_parser.declarations,
                important_declarations: nested_parser.important_declarations,
            },
            rules,
        ))
    }

    fn loc(&self, start: &ParserState) -> Location {
        let loc = start.source_location();
        Location { line: loc.line, column: loc.column }
    }
}

impl<'a, 'i> AtRuleParser<'i> for NestedRuleParser<'a, 'i> {
    type Prelude = AtRulePrelude<'i>;
    type AtRule = ();
    type Error = CustomParseError<'i>;

    fn parse_prelude<'t>(
        &mut self,
        name: CowRcStr<'i>,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self::Prelude, ParseError<'i, Self::Error>> {
        match_ignore_ascii_case! { &*name,
        "keyframes" => {
                let name = input.try_parse(KeyframesName::parse)?;
                Ok(AtRulePrelude::Keyframes(name))
            },
            _ => Err(input.new_error(BasicParseErrorKind::AtRuleInvalid(name)))
        }
    }

    fn parse_block<'t>(
        &mut self,
        prelude: Self::Prelude,
        start: &ParserState,
        input: &mut Parser<'i, 't>,
    ) -> Result<(), ParseError<'i, Self::Error>> {
        let loc = self.loc(start);
        match prelude {
            AtRulePrelude::Keyframes(name) => {
                let mut parser = KeyframeListParser;
                let iter = RuleBodyParser::new(input, &mut parser);
                self.rules.0.push(CssRule::Keyframes(KeyframesRule {
                    name,
                    keyframes: iter.filter_map(Result::ok).collect(),
                    loc,
                }));
                Ok(())
            }
        }
    }
}

impl<'a, 'i> QualifiedRuleParser<'i> for NestedRuleParser<'a, 'i> {
    type Prelude = SelectorList<Selectors>;
    type QualifiedRule = ();
    type Error = CustomParseError<'i>;

    fn parse_prelude<'t>(
        &mut self,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self::Prelude, ParseError<'i, Self::Error>> {
        let selector_parser = SelectorParser { options: self.options };

        SelectorList::parse(&selector_parser, input, ParseRelative::No)
    }

    fn parse_block<'t>(
        &mut self,
        selectors: Self::Prelude,
        start: &ParserState,
        input: &mut Parser<'i, 't>,
    ) -> Result<(), ParseError<'i, Self::Error>> {
        let loc = self.loc(start);
        let (declarations, rules) = self.parse_nested(input, true)?;
        self.rules.0.push(CssRule::Style(StyleRule { selectors, declarations, rules, loc }));
        Ok(())
    }
}

/// Parse a declaration within {} block: `color: blue`
impl<'a, 'i> cssparser::DeclarationParser<'i> for NestedRuleParser<'a, 'i> {
    type Declaration = ();
    type Error = CustomParseError<'i>;

    fn parse_value<'t>(
        &mut self,
        name: CowRcStr<'i>,
        input: &mut cssparser::Parser<'i, 't>,
    ) -> Result<Self::Declaration, cssparser::ParseError<'i, Self::Error>> {
        parse_declaration(
            name,
            input,
            &mut self.declarations,
            &mut self.important_declarations,
            self.options,
        )
    }
}

impl<'a, 'i> RuleBodyItemParser<'i, (), CustomParseError<'i>> for NestedRuleParser<'a, 'i> {
    fn parse_qualified(&self) -> bool {
        true
    }

    fn parse_declarations(&self) -> bool {
        self.allow_declarations
    }
}
