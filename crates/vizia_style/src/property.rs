use crate::{
    Alignment, Angle, AspectRatio, BackgroundImage, BackgroundRepeat, BackgroundSize, BlendMode,
    Border, BorderStyle, BorderStyleKeyword, BorderWidth, BorderWidthValue, ClipPath, Color,
    CornerRadius, CursorIcon, CustomParseError, CustomProperty, Direction, Display, Filter,
    FontFamily, FontSize, FontSlant, FontVariation, FontWeight, FontWidth, LayoutType, LayoutWrap,
    Length, LengthOrPercentage, LetterSpacing, LineClamp, LineHeight, Opacity, Outline, Overflow,
    Parse, PointerEvents, Position, PositionType, Rect, Scale, Shadow, TextAlign, TextDecoration,
    TextDecorationLine, TextDecorationStyle, TextOverflow, TextStroke, TextStrokeStyle, Transform,
    Transition, Translate, Units, UnparsedProperty, Visibility, define_property,
};
use cssparser::Parser;

/// Returns the Vizia property name for a supported CSS property alias.
pub fn canonical_property_name(name: &str) -> &str {
    match name {
        "position" => "position-type",
        "flex-direction" => "layout-type",
        "flex-wrap" => "wrap",
        "grid-template-columns" => "grid-columns",
        "grid-template-rows" => "grid-rows",
        "row-gap" => "vertical-gap",
        "column-gap" => "horizontal-gap",
        "border-radius" => "corner-radius",
        "border-top-left-radius" => "corner-top-left-radius",
        "border-top-right-radius" => "corner-top-right-radius",
        "border-bottom-left-radius" => "corner-bottom-left-radius",
        "border-bottom-right-radius" => "corner-bottom-right-radius",
        "box-shadow" => "shadow",
        "mix-blend-mode" => "blend-mode",
        "font-style" => "font-slant",
        "font-stretch" => "font-width",
        _ => name,
    }
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Padding(pub Rect<Units>);

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Gap {
    pub row: Units,
    pub column: Units,
}

impl<'i> Parse<'i> for Padding {
    fn parse<'t>(
        input: &mut Parser<'i>,
    ) -> Result<Self, cssparser::ParseError<CustomParseError<'i>>> {
        let Rect(top, right, bottom, left) = Rect::<Units>::parse(input)?;
        input.expect_exhausted()?;
        Ok(Self(Rect(top, right, bottom, left)))
    }
}

impl<'i> Parse<'i> for Gap {
    fn parse<'t>(
        input: &mut Parser<'i>,
    ) -> Result<Self, cssparser::ParseError<CustomParseError<'i>>> {
        let row = Units::parse(input)?;
        let column = input.try_parse(Units::parse).unwrap_or(row);
        input.expect_exhausted()?;
        Ok(Self { row, column })
    }
}

define_property! {
    pub enum Property<'i> {
        // General
        "display": Display(Display),
        "visibility": Visibility(Visibility),
        "overflow": Overflow(Overflow),
        "overflow-x": OverflowX(Overflow),
        "overflow-y": OverflowY(Overflow),
        "clip-path": ClipPath(ClipPath),
        "opacity": Opacity(Opacity),
        "z-index": ZIndex(i32),
        "blend-mode": BlendMode(BlendMode),

        // Positioning
        "layout-type": LayoutType(LayoutType),
        "position-type": PositionType(PositionType),

        "alignment": Alignment(Alignment),
        "direction": Direction(Direction),
        "wrap": Wrap(LayoutWrap),

        // Grid
        "grid-columns": GridColumns(Vec<Units>),
        "grid-rows": GridRows(Vec<Units>),
        "column-start": ColumnStart(usize),
        "column-span": ColumnSpan(usize),
        "row-start": RowStart(usize),
        "row-span": RowSpan(usize),

        // Position and Size
        "space": Space(Units),
        "left": Left(Units),
        "width": Width(Units),
        "right": Right(Units),
        "top": Top(Units),
        "size": Size(Units),
        "height": Height(Units),
        "aspect-ratio": AspectRatio(AspectRatio),
        "bottom": Bottom(Units),

        // Constraints
        "min-size": MinSize(Units),
        "min-width": MinWidth(Units),
        "min-height": MinHeight(Units),

        "max-size": MaxSize(Units),
        "max-width": MaxWidth(Units),
        "max-height": MaxHeight(Units),

        "min-gap": MinGap(Units),
        "min-horizontal-gap": MinHorizontalGap(Units),
        "max-horizontal-gap": MaxHorizontalGap(Units),

        "max-gap": MaxGap(Units),
        "min-vertical-gap": MinVerticalGap(Units),
        "max-vertical-gap": MaxVerticalGap(Units),

        // Padding
        "padding": Padding(Units),
        "padding-left": PaddingLeft(Units),
        "padding-right": PaddingRight(Units),
        "padding-top": PaddingTop(Units),
        "padding-bottom": PaddingBottom(Units),
        "vertical-gap": VerticalGap(Units),
        "horizontal-gap": HorizontalGap(Units),
        "gap": Gap(Units),
        // ----- Border -----

        // Border Shorthand
        "border": Border(Border),

        // Border Side Shorthands
        "border-top": BorderTop(Border),
        "border-right": BorderRight(Border),
        "border-bottom": BorderBottom(Border),
        "border-left": BorderLeft(Border),

        // Border Color
        "border-color": BorderColor(Color),
        "border-top-color": BorderTopColor(Color),
        "border-right-color": BorderRightColor(Color),
        "border-bottom-color": BorderBottomColor(Color),
        "border-left-color": BorderLeftColor(Color),

        // Corner Radius
        "corner-radius": CornerRadius(CornerRadius),
        "corner-top-left-radius": CornerTopLeftRadius(LengthOrPercentage),
        "corner-top-right-radius": CornerTopRightRadius(LengthOrPercentage),
        "corner-bottom-left-radius": CornerBottomLeftRadius(LengthOrPercentage),
        "corner-bottom-right-radius": CornerBottomRightRadius(LengthOrPercentage),

        // Border Style
        "border-style": BorderStyle(BorderStyle),
        "border-top-style": BorderTopStyle(BorderStyleKeyword),
        "border-right-style": BorderRightStyle(BorderStyleKeyword),
        "border-bottom-style": BorderBottomStyle(BorderStyleKeyword),
        "border-left-style": BorderLeftStyle(BorderStyleKeyword),

        // Border Width
        "border-width": BorderWidth(BorderWidth),
        "border-top-width": BorderTopWidth(BorderWidthValue),
        "border-right-width": BorderRightWidth(BorderWidthValue),
        "border-bottom-width": BorderBottomWidth(BorderWidthValue),
        "border-left-width": BorderLeftWidth(BorderWidthValue),


        // ----- Outline -----

        // Outline Shorthand
        "outline": Outline(Outline),

        // Outline Color
        "outline-color": OutlineColor(Color),
        // TODO: Support coloring individual outlines.
        // "outline-top-color": OutlineTopColor(Color),
        // "outline-right-color": OutlineRightColor(Color),
        // "outline-bottom-color": OutlineBottomColor(Color),
        // "outline-left-color": OutlineLeftColor(Color),

        // Outline Style
        // TODO: Support styling outlines.
        // "outline-style": OutlineStyle(BorderStyle),
        // "outline-top-style": OutlineTopStyle(BorderStyleKeyword),
        // "outline-right-style": OutlineRightStyle(BorderStyleKeyword),
        // "outline-bottom-style": OutlineBottomStyle(BorderStyleKeyword),
        // "outline-left-style": OutlineLeftStyle(BorderStyleKeyword),

        // Outline Width
        "outline-width": OutlineWidth(BorderWidth),
        // "outline-top-width": OutlineTopWidth(BorderWidthValue),
        // "outline-right-width": OutlineRightWidth(BorderWidthValue),
        // "outline-bottom-width": OutlineBottomWidth(BorderWidthValue),
        // "outline-left-width": OutlineLeftWidth(BorderWidthValue),
        "outline-offset": OutlineOffset(LengthOrPercentage),

        // Background
        "background-color": BackgroundColor(Color),
        "background-image": BackgroundImage(Vec<BackgroundImage<'i>>),
        "background-position": BackgroundPosition(Vec<Position>),
        "background-repeat": BackgroundRepeat(Vec<BackgroundRepeat>),
        "background-size": BackgroundSize(Vec<BackgroundSize>),

        "fill": Fill(Color),

        // Text
        "font-size": FontSize(FontSize),
        "color": FontColor(Color),
        "font-family": FontFamily(Vec<FontFamily<'i>>),
        "font-variation-settings": FontVariationSettings(Vec<FontVariation>),
        "font-weight": FontWeight(FontWeight),
        "font-slant": FontSlant(FontSlant),
        "font-width": FontWidth(FontWidth),
        "selection-color": SelectionColor(Color), // TODO: Remove this once we have the pseudoselector version.
        "caret-color": CaretColor(Color),
        "text-wrap": TextWrap(bool),
        "text-align": TextAlign(TextAlign),
        "text-overflow": TextOverflow(TextOverflow),
        "letter-spacing": LetterSpacing(LetterSpacing),
        "line-height": LineHeight(LineHeight),
        "line-clamp": LineClamp(LineClamp),
        "text-decoration": TextDecoration(TextDecoration),
        "text-decoration-line": TextDecorationLine(TextDecorationLine),
        "text-decoration-color": TextDecorationColor(Color),
        "text-decoration-style": TextDecorationStyle(TextDecorationStyle),
        "text-stroke": TextStroke(TextStroke),
        "text-stroke-width": TextStrokeWidth(Length),
        "text-stroke-style": TextStrokeStyle(TextStrokeStyle),
        "underline-thickness": UnderlineThickness(LengthOrPercentage),
        "overline-thickness": OverlineThickness(LengthOrPercentage),
        "strikethrough-thickness": StrikethroughThickness(LengthOrPercentage),

        // Shadow
        "shadow": Shadow(Vec<Shadow>),

        // Backdrop Filter
        "filter": Filter(Filter),
        "backdrop-filter": BackdropFilter(Filter),

        // Animations
        "transition": Transition(Vec<Transition>),

        // Transform
        "transform": Transform(Vec<Transform>),
        "transform-origin": TransformOrigin(Position),
        "translate": Translate(Translate),
        "rotate": Rotate(Angle),
        "scale": Scale(Scale),

        // Cursor
        "cursor": Cursor(CursorIcon),
        "pointer-events": PointerEvents(PointerEvents),
    }
}

#[cfg(test)]
mod tests {
    use cssparser::CowRcStr;

    use super::*;

    #[test]
    fn spacing_shorthands_expand_in_css_order() {
        let px = Units::Pixels;
        let padding_cases = [
            ("1px", Rect(px(1.0), px(1.0), px(1.0), px(1.0))),
            ("1px 2px", Rect(px(1.0), px(2.0), px(1.0), px(2.0))),
            ("1px 2px 3px", Rect(px(1.0), px(2.0), px(3.0), px(2.0))),
            ("0 2px 3% 4s", Rect(px(0.0), px(2.0), Units::Percentage(3.0), Units::Stretch(4.0))),
        ];
        for (value, expected) in padding_cases {
            let mut parser = Parser::new(value);
            assert_eq!(Padding::parse(&mut parser).unwrap(), Padding(expected), "{value}");
            assert!(parser.is_exhausted());
        }

        for (value, row, column) in [("5px", px(5.0), px(5.0)), ("0 2px", px(0.0), px(2.0))] {
            let mut parser = Parser::new(value);
            assert_eq!(Gap::parse(&mut parser).unwrap(), Gap { row, column }, "{value}");
            assert!(parser.is_exhausted());
        }
    }

    #[test]
    fn spacing_shorthands_do_not_parse_partial_values() {
        for (name, value) in [
            ("padding", "1px 2px 3px 4px 5px"),
            ("padding", "1px invalid"),
            ("padding", "1px var(--spacing)"),
            ("gap", "1px 2px 3px"),
            ("gap", "1px invalid"),
            ("gap", "var(--spacing) 2px"),
        ] {
            let mut parser = Parser::new(value);
            let parsed = match name {
                "padding" => Padding::parse(&mut parser).map(|_| ()),
                "gap" => Gap::parse(&mut parser).map(|_| ()),
                _ => unreachable!(),
            };
            assert!(parsed.is_err(), "{name}: {value}");
        }
    }

    #[test]
    fn standard_property_aliases_parse_like_vizia_names() {
        let aliases = [
            ("position", "position-type", "absolute"),
            ("flex-direction", "layout-type", "column"),
            ("flex-wrap", "wrap", "wrap"),
            ("flex-wrap", "wrap", "nowrap"),
            ("grid-template-columns", "grid-columns", "20px 1s"),
            ("grid-template-rows", "grid-rows", "20px 1s"),
            ("row-gap", "vertical-gap", "5px"),
            ("column-gap", "horizontal-gap", "6px"),
            ("border-radius", "corner-radius", "4px"),
            ("border-top-left-radius", "corner-top-left-radius", "5px"),
            ("border-top-right-radius", "corner-top-right-radius", "6px"),
            ("border-bottom-left-radius", "corner-bottom-left-radius", "7px"),
            ("border-bottom-right-radius", "corner-bottom-right-radius", "8px"),
            ("box-shadow", "shadow", "1px 2px 3px red"),
            ("mix-blend-mode", "blend-mode", "multiply"),
            ("font-style", "font-slant", "italic"),
            ("font-stretch", "font-width", "condensed"),
        ];

        for (alias, original, value) in aliases {
            let parsed = |name| {
                let mut parser = Parser::new(value);
                let property = Property::parse_value(CowRcStr::from(name), &mut parser).unwrap();
                assert!(parser.is_exhausted(), "{name}: {value}");
                assert!(!matches!(property, Property::Custom(_) | Property::Unparsed(_)));
                property
            };
            assert_eq!(parsed(alias), parsed(original), "{alias}");
        }
    }

    #[test]
    fn standard_property_aliases_keep_canonical_names_for_variable_resolution() {
        for (alias, original) in [
            ("border-radius", "corner-radius"),
            ("box-shadow", "shadow"),
            ("row-gap", "vertical-gap"),
        ] {
            let mut parser = Parser::new("var(--value, 4px)");
            let property = Property::parse_value(CowRcStr::from(alias), &mut parser).unwrap();
            assert!(matches!(property, Property::Unparsed(ref value) if value.name == original));
        }
    }

    #[test]
    fn parse_property() {
        let mut parser = Parser::new("red");
        let _parsed_property =
            Property::parse_value(CowRcStr::from("background-color"), &mut parser);
    }

    #[test]
    fn parse_background_position_property() {
        let mut parser = Parser::new("center");
        let parsed_property =
            Property::parse_value(CowRcStr::from("background-position"), &mut parser)
                .expect("Failed to parse background-position");

        match parsed_property {
            Property::BackgroundPosition(positions) => {
                assert_eq!(positions.len(), 1);
                assert!(positions[0].is_center());
            }

            _ => panic!("background-position parsed to wrong property"),
        }
    }

    #[test]
    fn parse_background_position_property_rejects_empty_value() {
        let mut parser = Parser::new("");
        let parsed_property =
            Property::parse_value(CowRcStr::from("background-position"), &mut parser)
                .expect("Property parsing should fall back to Unparsed");

        assert!(!matches!(parsed_property, Property::BackgroundPosition(_)));
    }

    #[test]
    fn parse_background_repeat_property() {
        let mut parser = Parser::new("repeat-x, no-repeat");
        let parsed_property =
            Property::parse_value(CowRcStr::from("background-repeat"), &mut parser)
                .expect("Failed to parse background-repeat");

        match parsed_property {
            Property::BackgroundRepeat(repeats) => {
                assert_eq!(repeats.len(), 2);
                assert_eq!(repeats[0], BackgroundRepeat::RepeatX);
                assert_eq!(repeats[1], BackgroundRepeat::NoRepeat);
            }

            _ => panic!("background-repeat parsed to wrong property"),
        }
    }

    #[test]
    fn parse_aspect_ratio_property() {
        let mut parser = Parser::new("auto 16/9");
        let parsed_property = Property::parse_value(CowRcStr::from("aspect-ratio"), &mut parser)
            .expect("Failed to parse aspect-ratio");

        match parsed_property {
            Property::AspectRatio(AspectRatio::AutoRatio(ratio)) => {
                assert_eq!(ratio, 16.0 / 9.0);
            }

            _ => panic!("aspect-ratio parsed to wrong property"),
        }
    }

    #[test]
    fn parse_filter_property() {
        let mut parser = Parser::new("blur(5px)");
        let parsed_property = Property::parse_value(CowRcStr::from("filter"), &mut parser).unwrap();

        assert_eq!(parsed_property, Property::Filter(Filter::Blur(Length::px(5.0))));
    }
}
