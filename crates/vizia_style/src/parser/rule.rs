use std::marker::PhantomData;

use crate::{
    parse_declaration, parser::declaration::DeclarationParser, CssRule, CssRuleList,
    CustomParseError, DashedIdent, DeclarationBlock, DeclarationList, Location, Parse,
    ParserOptions, Property, PropertyRule, SelectorParser, Selectors, StyleRule,
};
use cssparser::*;
use parcel_selectors::{
    parser::{NestingRequirement, Selector},
    SelectorList,
};

#[derive(PartialEq, PartialOrd)]
enum State {
    Start = 1,
    Layers = 2,
    Imports = 3,
    Namespaces = 4,
    Body = 5,
}

// Parser for top-level rules in a stylesheet
pub struct TopLevelRuleParser<'a, 'o, 'i> {
    default_namespace: Option<CowRcStr<'i>>,
    options: &'a ParserOptions<'o>,
    state: State,
}

impl<'a, 'o, 'b, 'i> TopLevelRuleParser<'a, 'o, 'i> {
    pub fn new(options: &'a ParserOptions<'o>) -> Self {
        TopLevelRuleParser {
            default_namespace: None,
            options,
            state: State::Start,
        }
    }

    fn nested<'x: 'b>(&'x mut self) -> NestedRuleParser<'_, 'o, 'i> {
        NestedRuleParser {
            default_namespace: &mut self.default_namespace,
            options: &self.options,
        }
    }
}

pub enum AtRulePrelude<'i> {
    Property(DashedIdent<'i>),
}

impl<'a, 'o, 'i> AtRuleParser<'i> for TopLevelRuleParser<'a, 'o, 'i> {
    type Prelude = AtRulePrelude<'i>;
    type AtRule = (SourcePosition, CssRule<'i>);
    type Error = CustomParseError<'i>;

    fn parse_prelude<'t>(
        &mut self,
        name: cssparser::CowRcStr<'i>,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self::Prelude, ParseError<'i, Self::Error>> {
        match_ignore_ascii_case! {
            &*name,
            // "property" => {
            //     let name = DashedIdent::parse(input)?;
            //     return Ok(AtRulePrelude::Property(name));
            // },
            _=> {}
        }

        AtRuleParser::parse_prelude(&mut self.nested(), name, input)
    }

    fn parse_block<'t>(
        &mut self,
        prelude: Self::Prelude,
        start: &ParserState,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self::AtRule, ParseError<'i, Self::Error>> {
        self.state = State::Body;
        let rule = AtRuleParser::parse_block(&mut self.nested(), prelude, start, input)?;
        Ok((start.position(), rule))
    }
}

impl<'a, 'o, 'i> QualifiedRuleParser<'i> for TopLevelRuleParser<'a, 'o, 'i> {
    type Prelude = SelectorList<'i, Selectors>;
    type QualifiedRule = (SourcePosition, CssRule<'i>);
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
        let rule = QualifiedRuleParser::parse_block(&mut self.nested(), prelude, start, input)?;
        Ok((start.position(), rule))
    }
}

struct NestedRuleParser<'a, 'o, 'i> {
    default_namespace: &'a Option<CowRcStr<'i>>,
    options: &'a ParserOptions<'o>,
}

impl<'a, 'o, 'b, 'i> NestedRuleParser<'a, 'o, 'i> {
    fn parse_nested_rules<'t>(&mut self, input: &mut Parser<'i, 't>) -> CssRuleList<'i> {
        let nested_parser = NestedRuleParser {
            default_namespace: self.default_namespace,
            options: self.options,
        };

        let mut iter = RuleListParser::new_for_nested_rule(input, nested_parser);
        let mut rules = Vec::new();
        while let Some(result) = iter.next() {
            match result {
                Ok(CssRule::Ignored) => {}
                Ok(rule) => rules.push(rule),
                Err(_) => {
                    // TODO
                }
            }
        }

        CssRuleList(rules)
    }

    fn loc(&self, start: &ParserState) -> Location {
        let loc = start.source_location();
        Location {
            line: loc.line,
            column: loc.column,
        }
    }
}

impl<'a, 'o, 'b, 'i> AtRuleParser<'i> for NestedRuleParser<'a, 'o, 'i> {
    type Prelude = AtRulePrelude<'i>;
    type AtRule = CssRule<'i>;
    type Error = CustomParseError<'i>;

    fn parse_prelude<'t>(
        &mut self,
        name: CowRcStr<'i>,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self::Prelude, ParseError<'i, Self::Error>> {
        match_ignore_ascii_case! { &*name,
            _=> Err(input.new_error(BasicParseErrorKind::AtRuleInvalid(name)))
        }
    }

    // fn parse_block<'t>(&mut self, prelude: Self::Prelude, start: &ParserState, input: &mut Parser<'i, 't>) -> Result<Self::AtRule, ParseError<'i, Self::Error>> {
    //     let loc = self.loc(start);

    //     match prelude {
    //         AtRulePrelude::Property(name) => Ok(CssRule::Property(PropertyRule::parse(name, input, loc)?)),
    //     }
    // }
}

impl<'a, 'o, 'b, 'i> QualifiedRuleParser<'i> for NestedRuleParser<'a, 'o, 'i> {
    type Prelude = SelectorList<'i, Selectors>;
    type QualifiedRule = CssRule<'i>;
    type Error = CustomParseError<'i>;

    fn parse_prelude<'t>(
        &mut self,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self::Prelude, ParseError<'i, Self::Error>> {
        let selector_parser = SelectorParser {
            default_namespace: self.default_namespace,
            is_nesting_allowed: false,
        };

        SelectorList::parse(&selector_parser, input, NestingRequirement::None)
    }

    fn parse_block<'t>(
        &mut self,
        selectors: Self::Prelude,
        start: &ParserState,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self::QualifiedRule, ParseError<'i, Self::Error>> {
        let loc = self.loc(start);
        let (declarations, rules) = if self.options.nesting {
            parse_declarations_and_nested_rules(input, self.default_namespace, self.options)?
        } else {
            (
                DeclarationBlock::parse(input, self.options)?,
                CssRuleList(vec![]),
            )
        };

        Ok(CssRule::Style(StyleRule {
            selectors,
            declarations,
            rules,
            loc,
        }))
    }
}

fn parse_declarations_and_nested_rules<'a, 'o, 'i, 't>(
    input: &mut Parser<'i, 't>,
    default_namespace: &'a Option<CowRcStr<'i>>,
    options: &'a ParserOptions<'o>,
) -> Result<(DeclarationBlock<'i>, CssRuleList<'i>), ParseError<'i, CustomParseError<'i>>> {
    let mut important_declarations = DeclarationList::new();
    let mut declarations = DeclarationList::new();
    let mut rules = CssRuleList(vec![]);
    let parser = StyleRuleParser {
        default_namespace,
        options,
        declarations: &mut declarations,
        important_declarations: &mut important_declarations,
        rules: &mut rules,
    };

    let mut declaration_parser = DeclarationListParser::new(input, parser);
    let mut last = declaration_parser.input.state();
    while let Some(decl) = declaration_parser.next() {
        match decl {
            Ok(_) => {}
            _ => {
                declaration_parser.input.reset(&last);
                break;
            }
        }

        last = declaration_parser.input.state();
    }

    let mut iter =
        RuleListParser::new_for_nested_rule(declaration_parser.input, declaration_parser.parser);
    while let Some(result) = iter.next() {
        if let Err((err, _)) = result {
            return Err(err);
        }
    }

    Ok((
        DeclarationBlock {
            declarations,
            important_declarations,
        },
        rules,
    ))
}

// Style Rule
pub struct StyleRuleParser<'a, 'o, 'i> {
    default_namespace: &'a Option<CowRcStr<'i>>,
    options: &'a ParserOptions<'o>,
    declarations: &'a mut DeclarationList<'i>,
    important_declarations: &'a mut DeclarationList<'i>,
    rules: &'a mut CssRuleList<'i>,
}

impl<'a, 'o, 'i> cssparser::DeclarationParser<'i> for StyleRuleParser<'a, 'o, 'i> {
    type Declaration = ();
    type Error = CustomParseError<'i>;

    fn parse_value<'t>(
        &mut self,
        name: CowRcStr<'i>,
        input: &mut cssparser::Parser<'i, 't>,
    ) -> Result<Self::Declaration, cssparser::ParseError<'i, Self::Error>> {
        if !self.rules.0.is_empty() {
            // Declarations cannot come after nested rules.
            return Err(input.new_custom_error(CustomParseError::InvalidNesting));
        }
        parse_declaration(
            name,
            input,
            &mut self.declarations,
            &mut self.important_declarations,
            &self.options,
        )
    }
}

impl<'a, 'o, 'i> AtRuleParser<'i> for StyleRuleParser<'a, 'o, 'i> {
    type Prelude = AtRulePrelude<'i>;
    type AtRule = ();
    type Error = CustomParseError<'i>;

    fn parse_prelude<'t>(
        &mut self,
        name: CowRcStr<'i>,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self::Prelude, ParseError<'i, Self::Error>> {
        match_ignore_ascii_case! { &*name,
            _ => Err(input.new_error(BasicParseErrorKind::AtRuleInvalid(name)))
        }
    }

    fn parse_block<'t>(
        &mut self,
        prelude: Self::Prelude,
        start: &ParserState,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self::AtRule, ParseError<'i, Self::Error>> {
        Err(input.new_error(BasicParseErrorKind::AtRuleBodyInvalid))
    }
}

fn parse_nested_at_rule<'a, 'o, 'i, 't>(
    input: &mut Parser<'i, 't>,
    default_namespace: &'a Option<CowRcStr<'i>>,
    options: &'a ParserOptions<'i>,
) -> Result<CssRuleList<'i>, ParseError<'i, CustomParseError<'i>>> {
    let loc = input.current_source_location();
    let loc = Location {
        line: loc.line,
        column: loc.column,
    };

    // Declarations can be immediately within @media and @supports blocks that are nested within a parent style rule.
    // These act the same way as if they were nested within a `& { ... }` block.
    let (declarations, mut rules) =
        parse_declarations_and_nested_rules(input, default_namespace, options)?;

    if declarations.declarations.len() > 0 {
        rules.0.insert(
            0,
            CssRule::Style(StyleRule {
                selectors: SelectorList(smallvec::smallvec![
                    parcel_selectors::parser::Selector::from_vec2(vec![
                        parcel_selectors::parser::Component::Nesting
                    ])
                ]),
                declarations,
                rules: CssRuleList(vec![]),
                loc,
            }),
        )
    }

    Ok(rules)
}

impl<'a, 'o, 'b, 'i> QualifiedRuleParser<'i> for StyleRuleParser<'a, 'o, 'i> {
    type Prelude = SelectorList<'i, Selectors>;
    type QualifiedRule = ();
    type Error = CustomParseError<'i>;

    fn parse_prelude<'t>(
        &mut self,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self::Prelude, ParseError<'i, Self::Error>> {
        let selector_parser = SelectorParser {
            default_namespace: self.default_namespace,
            is_nesting_allowed: true,
        };
        SelectorList::parse(&selector_parser, input, NestingRequirement::Prefixed)
    }

    fn parse_block<'t>(
        &mut self,
        selectors: Self::Prelude,
        start: &ParserState,
        input: &mut Parser<'i, 't>,
    ) -> Result<(), ParseError<'i, Self::Error>> {
        let loc = start.source_location();
        let (declarations, rules) =
            parse_declarations_and_nested_rules(input, self.default_namespace, self.options)?;
        self.rules.0.push(CssRule::Style(StyleRule {
            selectors,
            declarations,
            rules,
            loc: Location {
                line: loc.line,
                column: loc.column,
            },
        }));
        Ok(())
    }
}

// #[derive(Debug)]
// pub struct RuleParser;

// impl RuleParser {
//     pub fn new() -> Self {
//         RuleParser
//     }
// }

// impl<'i> QualifiedRuleParser<'i> for RuleParser {
//     type Prelude = Vec<Selector>;
//     type QualifiedRule = StyleRule;
//     type Error = CustomParseError<'i>;

//     fn parse_prelude<'t>(
//         &mut self,
//         input: &mut Parser<'i, 't>,
//     ) -> Result<Self::Prelude, ParseError<'i, Self::Error>> {
//         let mut selectors: Vec<Selector> = Vec::new();

//         while let Ok(token) = input.next() {
//             match token {
//                 Token::Ident(ident) => {
//                     selectors.push(Selector::new(ident.to_string()));
//                 }
//                 _ => {}
//             }
//         }

//         Ok(selectors)
//     }

//     fn parse_block<'t>(
//         &mut self,
//         prelude: Self::Prelude,
//         _start: &ParserState,
//         input: &mut Parser<'i, 't>,
//     ) -> Result<Self::QualifiedRule, ParseError<'i, Self::Error>> {
//         let decl_parser = DeclarationParser;
//         let parsed_properties = DeclarationListParser::new(input, decl_parser);
//         let mut properties: Vec<Property> = Vec::new();
//         let mut errors: Vec<(ParseError<CustomParseError>, &str)> = Vec::new();

//         for property in parsed_properties {
//             match property {
//                 Ok(prop) => properties.push(prop),
//                 Err(error) => errors.push(error),
//             }
//         }

//         if errors.len() > 0 {
//             for error in errors {
//                 eprintln!(
//                     "ERROR: Error while parsing `{}` at {}:{}:\n{:?}\n",
//                     error.1, error.0.location.line, error.0.location.column, error.0.kind
//                 );
//             }
//         }

//         Ok(StyleRule {
//             id: Rule(0),
//             selectors: prelude,
//             properties,
//         })
//     }
// }

// impl<'i> AtRuleParser<'i> for RuleParser {
//     type Prelude = ();
//     type AtRule = StyleRule;
//     type Error = CustomParseError<'i>;
// }

// #[derive(Debug)]
// pub struct StyleRule {
//     pub id: Rule,
//     pub selectors: Vec<Selector>,
//     pub properties: Vec<Property>,
// }

// #[derive(Debug)]
// pub struct Selector {
//     pub name: String,
// }

// impl Selector {
//     pub fn new(name: String) -> Self {
//         Self { name }
//     }
// }

// #[derive(Debug)]
// pub struct Rule(u32);

// pub struct StyleRuleParser<'a, 'o, 'i> {
//     rules: CssRuleList
// }
