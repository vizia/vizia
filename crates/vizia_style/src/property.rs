use crate::{
    Alignment, Angle, AspectRatio, BackgroundImage, BackgroundRepeat, BackgroundSize, BlendMode,
    Border, BorderStyle, BorderStyleKeyword, BorderWidth, BorderWidthValue, ClipPath, Color,
    CornerRadius, CornerShape, CursorIcon, CustomParseError, CustomProperty, Direction, Display,
    Filter, FontFamily, FontSize, FontSlant, FontVariation, FontWeight, FontWidth, LayoutType,
    LayoutWrap, Length, LengthOrPercentage, LetterSpacing, LineClamp, LineHeight, Opacity, Outline,
    Overflow, Parse, PointerEvents, Position, PositionType, Rect, Scale, Shadow, TextAlign,
    TextDecoration, TextDecorationLine, TextDecorationStyle, TextOverflow, TextStroke,
    TextStrokeStyle, Transform, Transition, Translate, Units, UnparsedProperty, Visibility,
    define_property,
};
use cssparser::Parser;

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

        // Corner Shape
        "corner-shape": CornerShape(Rect<CornerShape>),
        "corner-top-left-shape": CornerTopLeftShape(CornerShape),
        "corner-top-right-shape": CornerTopRightShape(CornerShape),
        "corner-bottom-left-shape": CornerBottomLeftShape(CornerShape),
        "corner-bottom-right-shape": CornerBottomRightShape(CornerShape),

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
    use cssparser::{CowRcStr, ParserInput};

    use super::*;

    #[test]
    fn parse_property() {
        let mut parser_input = ParserInput::new("red");
        let mut parser = Parser::new(&mut parser_input);
        let _parsed_property =
            Property::parse_value(CowRcStr::from("background-color"), &mut parser);
    }

    #[test]
    fn parse_background_position_property() {
        let mut parser_input = ParserInput::new("center");
        let mut parser = Parser::new(&mut parser_input);
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
        let mut parser_input = ParserInput::new("");
        let mut parser = Parser::new(&mut parser_input);
        let parsed_property =
            Property::parse_value(CowRcStr::from("background-position"), &mut parser)
                .expect("Property parsing should fall back to Unparsed");

        assert!(!matches!(parsed_property, Property::BackgroundPosition(_)));
    }

    #[test]
    fn parse_background_repeat_property() {
        let mut parser_input = ParserInput::new("repeat-x, no-repeat");
        let mut parser = Parser::new(&mut parser_input);
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
        let mut parser_input = ParserInput::new("auto 16/9");
        let mut parser = Parser::new(&mut parser_input);
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
        let mut parser_input = ParserInput::new("blur(5px)");
        let mut parser = Parser::new(&mut parser_input);
        let parsed_property = Property::parse_value(CowRcStr::from("filter"), &mut parser).unwrap();

        assert_eq!(parsed_property, Property::Filter(Filter::Blur(Length::px(5.0))));
    }
}
