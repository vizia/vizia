use crate::{
    BackgroundImage, BackgroundRepeat, BackgroundSize, Color, CustomParseError, Gap,
    HorizontalPosition, Padding, Parse, ParserOptions, Position, Property, Rect, UnparsedProperty,
    VerticalPosition,
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
        Background(Vec<Property<'i>>),
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
        "background" => match input.try_parse(parse_background) {
            Ok(properties) => Ok(ParsedDeclaration::Background(properties)),
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
        ParsedDeclaration::Background(properties) => declarations.extend(properties),
    }
    Ok(())
}

fn parse_background<'i>(
    input: &mut Parser<'i>,
) -> Result<Vec<Property<'i>>, ParseError<CustomParseError<'i>>> {
    let layers = input.parse_comma_separated(parse_background_layer)?;
    let Some((last, previous)) = layers.split_last() else {
        return Err(input.new_error_for_next_token());
    };
    if previous.iter().any(|layer| layer.color.is_some()) {
        return Err(input.new_error_for_next_token());
    }
    let color = last.color.unwrap_or(Color::rgba(0, 0, 0, 0));

    let mut images = Vec::with_capacity(layers.len());
    let mut positions = Vec::with_capacity(layers.len());
    let mut sizes = Vec::with_capacity(layers.len());
    let mut repeats = Vec::with_capacity(layers.len());
    for layer in layers {
        images.push(layer.image);
        positions.push(layer.position);
        sizes.push(layer.size);
        repeats.push(layer.repeat);
    }

    Ok(vec![
        Property::BackgroundColor(color),
        Property::BackgroundImage(images),
        Property::BackgroundPosition(positions),
        Property::BackgroundSize(sizes),
        Property::BackgroundRepeat(repeats),
    ])
}

struct BackgroundLayer<'i> {
    color: Option<Color>,
    image: BackgroundImage<'i>,
    position: Position,
    size: BackgroundSize,
    repeat: BackgroundRepeat,
}

fn parse_background_layer<'i>(
    input: &mut Parser<'i>,
) -> Result<BackgroundLayer<'i>, ParseError<CustomParseError<'i>>> {
    let mut color = None;
    let mut image = None;
    let mut position = None;
    let mut size = None;
    let mut repeat = None;
    let mut any = false;

    while !input.is_exhausted() {
        input.skip_whitespace();
        if color.is_none()
            && let Ok(value) = input.try_parse(Color::parse)
        {
            color = Some(value);
        } else if image.is_none()
            && let Ok(value) = input.try_parse(|input| {
                let state = input.state();
                let valid = match input.next()? {
                    Token::UnquotedUrl(_) | Token::Function(_) => true,
                    Token::Ident(name) => name.eq_ignore_ascii_case("none"),
                    _ => false,
                };
                input.reset(&state);
                if valid {
                    BackgroundImage::parse(input)
                } else {
                    Err(input.new_error_for_next_token())
                }
            })
        {
            image = Some(value);
        } else if repeat.is_none()
            && let Ok(value) = input.try_parse(BackgroundRepeat::parse)
        {
            repeat = Some(value);
        } else if position.is_none()
            && let Ok(value) = input.try_parse(|input| {
                let state = input.state();
                let first = input.try_parse(HorizontalPosition::parse);
                let starts_position = if let Ok(first) = first {
                    if input.try_parse(VerticalPosition::parse).is_err()
                        && !matches!(first, HorizontalPosition::Center)
                        && input.try_parse(HorizontalPosition::parse).is_ok()
                    {
                        return Err(input.new_error_for_next_token());
                    }
                    true
                } else {
                    input.try_parse(VerticalPosition::parse).is_ok()
                };
                input.reset(&state);
                if !starts_position {
                    return Err(input.new_error_for_next_token());
                }
                Position::parse(input)
            })
        {
            position = Some(value);
            if input.try_parse(|input| input.expect_delim('/')).is_ok() {
                size = Some(BackgroundSize::parse(input)?);
            }
        } else {
            return Err(input.new_error_for_next_token());
        }
        any = true;
    }

    if !any {
        return Err(input.new_error_for_next_token());
    }

    Ok(BackgroundLayer {
        color,
        image: image.unwrap_or(BackgroundImage::None),
        position: position.unwrap_or_default(),
        size: size.unwrap_or_default(),
        repeat: repeat.unwrap_or_default(),
    })
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

    #[test]
    fn background_shorthand_expands_and_resets_omitted_values() {
        let mut parser = Parser::new(
            "background: url(\"image.png\") center / cover no-repeat red;
             background: blue !important;",
        );

        let block = DeclarationBlock::parse(&mut parser, &ParserOptions::new()).unwrap();
        assert_eq!(
            block.declarations,
            vec![
                Property::BackgroundColor(Color::rgb(255, 0, 0)),
                Property::BackgroundImage(vec![BackgroundImage::from("url(\"image.png\")")]),
                Property::BackgroundPosition(vec![Position::center()]),
                Property::BackgroundSize(vec![BackgroundSize::Cover]),
                Property::BackgroundRepeat(vec![BackgroundRepeat::NoRepeat]),
            ]
        );
        assert_eq!(
            block.important_declarations,
            vec![
                Property::BackgroundColor(Color::rgb(0, 0, 255)),
                Property::BackgroundImage(vec![BackgroundImage::None]),
                Property::BackgroundPosition(vec![Position::default()]),
                Property::BackgroundSize(vec![BackgroundSize::default()]),
                Property::BackgroundRepeat(vec![BackgroundRepeat::Repeat]),
            ]
        );

        let mut parser = Parser::new(
            "background: no-repeat repeat bottom right / contain linear-gradient(red, blue);",
        );
        let block = DeclarationBlock::parse(&mut parser, &ParserOptions::new()).unwrap();
        assert!(matches!(
            block.declarations.as_slice(),
            [
                Property::BackgroundColor(color),
                Property::BackgroundImage(images),
                Property::BackgroundPosition(positions),
                Property::BackgroundSize(sizes),
                Property::BackgroundRepeat(repeats),
            ] if *color == Color::rgba(0, 0, 0, 0)
                && matches!(images.as_slice(), [BackgroundImage::Gradient(_)])
                && positions[0].x.to_length_or_percentage() == crate::LengthOrPercentage::Percentage(100.0)
                && positions[0].y.to_length_or_percentage() == crate::LengthOrPercentage::Percentage(100.0)
                && sizes == &[BackgroundSize::Contain]
                && repeats == &[BackgroundRepeat::RepeatY]
        ));
    }

    #[test]
    fn layered_background_shorthand_expands_each_layer() {
        let mut parser = Parser::new(
            "background: url(\"front,a.png\") right bottom / 8px 6px no-repeat,
                         none center / auto repeat-x,
                         linear-gradient(red, blue) left top / cover repeat-y rgba(0, 255, 0, 1);
             background: none, url(back.png) blue !important;",
        );
        let block = DeclarationBlock::parse(&mut parser, &ParserOptions::new()).unwrap();
        assert!(matches!(
            block.declarations.as_slice(),
            [
                Property::BackgroundColor(color),
                Property::BackgroundImage(images),
                Property::BackgroundPosition(positions),
                Property::BackgroundSize(sizes),
                Property::BackgroundRepeat(repeats),
            ] if *color == Color::rgb(0, 255, 0)
                && images.len() == 3
                && images[0] == BackgroundImage::from("url(\"front,a.png\")")
                && images[1] == BackgroundImage::None
                && matches!(images[2], BackgroundImage::Gradient(_))
                && positions[0].x.to_length_or_percentage() == crate::LengthOrPercentage::Percentage(100.0)
                && positions[0].y.to_length_or_percentage() == crate::LengthOrPercentage::Percentage(100.0)
                && positions[1] == Position::center()
                && positions[2] == Position::new(
                    crate::HorizontalPositionKeyword::Left,
                    crate::VerticalPositionKeyword::Top,
                )
                && sizes == &[
                    BackgroundSize::Explicit {
                        width: crate::LengthPercentageOrAuto::px(8.0),
                        height: crate::LengthPercentageOrAuto::px(6.0),
                    },
                    BackgroundSize::default(),
                    BackgroundSize::Cover,
                ]
                && repeats == &[BackgroundRepeat::NoRepeat, BackgroundRepeat::RepeatX, BackgroundRepeat::RepeatY]
        ));
        assert!(matches!(
            block.important_declarations.as_slice(),
            [
                Property::BackgroundColor(color),
                Property::BackgroundImage(images),
                Property::BackgroundPosition(positions),
                Property::BackgroundSize(sizes),
                Property::BackgroundRepeat(repeats),
            ] if *color == Color::rgb(0, 0, 255)
                && images == &[BackgroundImage::None, BackgroundImage::from("url(back.png)")]
                && positions == &[Position::default(), Position::default()]
                && sizes == &[BackgroundSize::default(), BackgroundSize::default()]
                && repeats == &[BackgroundRepeat::Repeat, BackgroundRepeat::Repeat]
        ));
    }

    #[test]
    fn background_shorthand_rejects_unsupported_or_invalid_values_atomically() {
        for value in [
            "",
            "red blue",
            "none none",
            "center /",
            "/ cover",
            "red, blue",
            "red fixed",
            "url(a.png) / cover",
            "\"image.png\"",
            "var(--background)",
            "left right",
            "10px left",
            "red, url(image.png)",
            "url(a.png) red, url(b.png) blue",
            "url(a.png),",
            ", url(a.png)",
            "url(a.png), , url(b.png)",
            "url(a.png), center /",
            "url(a.png), var(--background)",
        ] {
            let css = format!("background: {value};");
            let mut parser = Parser::new(&css);
            let block = DeclarationBlock::parse(&mut parser, &ParserOptions::new()).unwrap();
            assert!(
                block.declarations.is_empty()
                    || matches!(block.declarations.as_slice(), [Property::Unparsed(_)]),
                "{value}: {:?}",
                block.declarations
            );
        }
    }
}
