use crate::{
    CustomParseError, Gap, Padding, Parse, ParserOptions, Property, Rect, UnparsedProperty,
};

use cssparser::*;

#[derive(Debug, PartialEq, Clone)]
pub struct DeclarationBlock<'i> {
    pub declarations: Vec<Property<'i>>,
    pub important_declarations: Vec<Property<'i>>,
}

impl<'i> DeclarationBlock<'i> {
    pub fn parse<'a>(
        input: &mut Parser<'i>,
        options: &'a ParserOptions<'i>,
    ) -> Result<Self, ParseError<CustomParseError<'i>>> {
        let mut important_declarations = DeclarationList::new();
        let mut declarations = DeclarationList::new();
        let mut decl_parser = PropertyDeclarationParser {
            important_declarations: &mut important_declarations,
            declarations: &mut declarations,
            options,
        };
        let parser = RuleBodyParser::new(input, &mut decl_parser);
        for res in parser {
            if let Err((err, _, location)) = res {
                if options.error_recovery {
                    options.warn(err, location);
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
        input: &mut Parser<'i>,
        _state: &ParserState,
    ) -> Result<Self::Declaration, ParseError<Self::Error>> {
        parse_declaration(name, input, self.declarations, self.important_declarations, self.options)
    }
}

pub(crate) fn parse_declaration<'i>(
    name: CowRcStr<'i>,
    input: &mut cssparser::Parser<'i>,
    declarations: &mut DeclarationList<'i>,
    important_declarations: &mut DeclarationList<'i>,
    _options: &ParserOptions,
) -> Result<(), ParseError<CustomParseError<'i>>> {
    enum ParsedDeclaration<'i> {
        Single(Property<'i>),
        Padding([crate::Units; 4]),
        Gap([crate::Units; 2]),
    }

    let declaration = input.parse_until_before(Delimiter::Bang, |input| match name.as_ref() {
        "padding" => match input.try_parse(Padding::parse) {
            Ok(Padding(Rect(top, right, bottom, left))) => {
                if top == right && top == bottom && top == left {
                    Ok(ParsedDeclaration::Single(Property::Padding(top)))
                } else {
                    Ok(ParsedDeclaration::Padding([top, right, bottom, left]))
                }
            }
            Err(_) => Ok(ParsedDeclaration::Single(Property::Unparsed(UnparsedProperty::parse(
                name, input,
            )?))),
        },
        "gap" => match input.try_parse(Gap::parse) {
            Ok(Gap { row, column }) => {
                if row == column {
                    Ok(ParsedDeclaration::Single(Property::Gap(row)))
                } else {
                    Ok(ParsedDeclaration::Gap([row, column]))
                }
            }
            Err(_) => Ok(ParsedDeclaration::Single(Property::Unparsed(UnparsedProperty::parse(
                name, input,
            )?))),
        },
        _ => Ok(ParsedDeclaration::Single(Property::parse_value(name, input)?)),
    })?;

    let important = input
        .try_parse(|input| {
            input.expect_delim('!')?;
            input.expect_ident_matching("important")
        })
        .is_ok();

    let declarations = if important { important_declarations } else { declarations };
    match declaration {
        ParsedDeclaration::Single(property) => declarations.push(property),
        ParsedDeclaration::Padding([top, right, bottom, left]) => declarations.extend([
            Property::PaddingTop(top),
            Property::PaddingRight(right),
            Property::PaddingBottom(bottom),
            Property::PaddingLeft(left),
        ]),
        ParsedDeclaration::Gap([row, column]) => {
            declarations.extend([Property::VerticalGap(row), Property::HorizontalGap(column)]);
        }
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Units;

    #[test]
    fn spacing_shorthands_expand_into_existing_properties() {
        let mut parser = Parser::new(
            "padding: 1px 2px 3px; gap: 4px 5px; padding: 6px; gap: 7px;
             padding: 8px 9px !important;",
        );
        let block = DeclarationBlock::parse(&mut parser, &ParserOptions::new()).unwrap();
        let px = Units::Pixels;
        assert_eq!(
            block.declarations,
            vec![
                Property::PaddingTop(px(1.0)),
                Property::PaddingRight(px(2.0)),
                Property::PaddingBottom(px(3.0)),
                Property::PaddingLeft(px(2.0)),
                Property::VerticalGap(px(4.0)),
                Property::HorizontalGap(px(5.0)),
                Property::Padding(px(6.0)),
                Property::Gap(px(7.0)),
            ]
        );
        assert_eq!(
            block.important_declarations,
            vec![
                Property::PaddingTop(px(8.0)),
                Property::PaddingRight(px(9.0)),
                Property::PaddingBottom(px(8.0)),
                Property::PaddingLeft(px(9.0)),
            ]
        );
    }
}
