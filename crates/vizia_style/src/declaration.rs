use crate::{CustomParseError, ParserOptions, Property};

use cssparser::*;

#[derive(Debug, PartialEq, Clone)]
pub struct DeclarationBlock<'i> {
    pub declarations: Vec<Property<'i>>,
    pub important_declarations: Vec<Property<'i>>,
}

impl<'i> DeclarationBlock<'i> {
    pub fn parse<'a, 'o, 't>(
        input: &mut Parser<'i, 't>,
        options: &'a ParserOptions<'o>,
    ) -> Result<Self, ParseError<'i, CustomParseError<'i>>> {
        let mut important_declarations = DeclarationList::new();
        let mut declarations = DeclarationList::new();
        let mut parser = DeclarationListParser::new(
            input,
            PropertyDeclarationParser {
                important_declarations: &mut important_declarations,
                declarations: &mut declarations,
                options,
            },
        );
        while let Some(res) = parser.next() {
            if let Err((err, _)) = res {
                return Err(err);
            }
        }

        Ok(DeclarationBlock {
            important_declarations,
            declarations,
        })
    }
}

struct PropertyDeclarationParser<'a, 'o, 'i> {
    declarations: &'a mut Vec<Property<'i>>,
    important_declarations: &'a mut Vec<Property<'i>>,
    options: &'a ParserOptions<'o>,
}

impl<'a, 'o, 'i> DeclarationParser<'i> for PropertyDeclarationParser<'a, 'o, 'i> {
    type Declaration = ();
    type Error = CustomParseError<'i>;

    fn parse_value<'t>(
        &mut self,
        name: CowRcStr<'i>,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self::Declaration, ParseError<'i, Self::Error>> {
        parse_declaration(
            name,
            input,
            &mut self.declarations,
            &mut self.important_declarations,
            &mut self.options,
        )
    }
}

pub(crate) fn parse_declaration<'i, 't>(
    name: CowRcStr<'i>,
    input: &mut cssparser::Parser<'i, 't>,
    declarations: &mut DeclarationList<'i>,
    important_declarations: &mut DeclarationList<'i>,
    options: &ParserOptions,
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

pub(crate) type DeclarationList<'i> = Vec<Property<'i>>;

/// Default methods reject all at rules.
impl<'a, 'o, 'i> AtRuleParser<'i> for PropertyDeclarationParser<'a, 'o, 'i> {
    type Prelude = ();
    type AtRule = ();
    type Error = CustomParseError<'i>;
}
