use crate::{CustomParseError, ParserOptions, Property};

use cssparser::*;

#[derive(Debug, PartialEq, Clone)]
pub struct DeclarationBlock<'i> {
    pub declarations: Vec<Property<'i>>,
    pub important_declarations: Vec<Property<'i>>,
}

impl<'i> DeclarationBlock<'i> {
    pub fn parse<'a, 't>(
        input: &mut Parser<'i, 't>,
        options: &'a ParserOptions<'i>,
    ) -> Result<Self, ParseError<'i, CustomParseError<'i>>> {
        let mut important_declarations = DeclarationList::new();
        let mut declarations = DeclarationList::new();
        let mut decl_parser = PropertyDeclarationParser {
            important_declarations: &mut important_declarations,
            declarations: &mut declarations,
            options,
        };
        let parser = RuleBodyParser::new(input, &mut decl_parser);
        for res in parser {
            if let Err((err, _)) = res {
                if options.error_recovery {
                    options.warn(err);
                    continue;
                }
                return Err(err);
            }
        }

        Ok(DeclarationBlock { important_declarations, declarations })
    }
}

struct PropertyDeclarationParser<'a, 'i> {
    declarations: &'a mut Vec<Property<'i>>,
    important_declarations: &'a mut Vec<Property<'i>>,
    options: &'a ParserOptions<'i>,
}

impl<'i> DeclarationParser<'i> for PropertyDeclarationParser<'_, 'i> {
    type Declaration = ();
    type Error = CustomParseError<'i>;

    fn parse_value<'t>(
        &mut self,
        name: CowRcStr<'i>,
        input: &mut Parser<'i, 't>,
        _state: &ParserState,
    ) -> Result<Self::Declaration, ParseError<'i, Self::Error>> {
        parse_declaration(name, input, self.declarations, self.important_declarations, self.options)
    }
}

pub(crate) fn parse_declaration<'i>(
    name: CowRcStr<'i>,
    input: &mut cssparser::Parser<'i, '_>,
    declarations: &mut DeclarationList<'i>,
    important_declarations: &mut DeclarationList<'i>,
    _options: &ParserOptions,
) -> Result<(), ParseError<'i, CustomParseError<'i>>> {
    let property =
        input.parse_until_before(Delimiter::Bang, |input| Property::parse_value(name, input))?;

    let important = input
        .try_parse(|input| {
            input.expect_delim('!')?;
            input.expect_ident_matching("important")
        })
        .is_ok();

    if important {
        important_declarations.push(property);
    } else {
        declarations.push(property);
    }
    Ok(())
}

impl<'i> AtRuleParser<'i> for PropertyDeclarationParser<'_, 'i> {
    type Prelude = ();
    type AtRule = ();
    type Error = CustomParseError<'i>;
}

impl<'i> QualifiedRuleParser<'i> for PropertyDeclarationParser<'_, 'i> {
    type Prelude = ();
    type QualifiedRule = ();
    type Error = CustomParseError<'i>;
}

impl<'i> RuleBodyItemParser<'i, (), CustomParseError<'i>> for PropertyDeclarationParser<'_, 'i> {
    fn parse_qualified(&self) -> bool {
        false
    }

    fn parse_declarations(&self) -> bool {
        true
    }
}

pub(crate) type DeclarationList<'i> = Vec<Property<'i>>;
