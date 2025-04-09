use std::time::Duration;

use crate::{
    define_property, Alignment, Angle, Animation, AnimationDirection, AnimationFillMode,
    AnimationIterationCount, AnimationName, BackgroundImage, BackgroundSize, BlendMode, Border,
    BorderStyle, BorderWidth, ClipPath, Color, CornerRadius, CornerShape, CursorIcon,
    CustomParseError, CustomProperty, Display, EasingFunction, Filter, FontFamily, FontSize,
    FontSlant, FontVariation, FontWeight, FontWidth, LayoutType, Length, LengthOrPercentage,
    LineClamp, Opacity, Outline, Overflow, Parse, PointerEvents, Position, PositionType, Rect,
    Scale, Shadow, TextAlign, TextDecoration, TextDecorationLine, TextDecorationStyle,
    TextOverflow, TextStroke, TextStrokeStyle, Transform, Transition, Translate, Units,
    UnparsedProperty, Visibility,
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

        // Border Color
        "border-color": BorderColor(Color),
        // TODO: Support coloring individual borders.
        // "border-top-color": BorderTopColor(Color),
        // "border-right-color": BorderRightColor(Color),
        // "border-bottom-color": BorderBottomColor(Color),
        // "border-left-color": BorderLeftColor(Color),

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
        // TODO: Support styling borders.
        "border-style": BorderStyle(BorderStyle),
        // "border-top-style": BorderTopStyle(BorderStyleKeyword),
        // "border-right-style": BorderRightStyle(BorderStyleKeyword),
        // "border-bottom-style": BorderBottomStyle(BorderStyleKeyword),
        // "border-left-style": BorderLeftStyle(BorderStyleKeyword),

        // Border Width
        "border-width": BorderWidth(BorderWidth),
        // "border-top-width": BorderTopWidth(BorderWidthValue),
        // "border-right-width": BorderRightWidth(BorderWidthValue),
        // "border-bottom-width": BorderBottomWidth(BorderWidthValue),
        // "border-left-width": BorderLeftWidth(BorderWidthValue),


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
        "line-clamp": LineClamp(LineClamp),
        "text-decoration": TextDecoration(TextDecoration),
        "text-decoration-line": TextDecorationLine(TextDecorationLine),
        "text-stroke": TextStroke(TextStroke),
        "text-stroke-width": TextStrokeWidth(Length),
        "text-stroke-style": TextStrokeStyle(TextStrokeStyle),
        "underline-style": UnderlineStyle(TextDecorationStyle),
        "underline-thickness": UnderlineThickness(LengthOrPercentage),
        "underline-color": UnderlineColor(Color),
        "overline-style": OverlineStyle(TextDecorationStyle),
        "overline-thickness": OverlineThickness(LengthOrPercentage),
        "overline-color": OverlineColor(Color),
        "strikethrough-style": StrikethroughStyle(TextDecorationStyle),
        "strikethrough-thickness": StrikethroughThickness(LengthOrPercentage),
        "strikethrough-color": StrikethroughColor(Color),

        // Shadow
        "shadow": Shadow(Vec<Shadow>),

        // Backdrop Filter
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

        // Animation
        "animation-name": AnimationName(AnimationName<'i>),
        "animation-duration": AnimationDuration(Duration),
        "animation-delay": AnimationDelay(Duration),
        "animation-fill-mode": AnimationFillMode(AnimationFillMode),
        "animation-direction": AnimationDirection(AnimationDirection),
        "animation-iteration-count": AnimationIterationCount(AnimationIterationCount),
        "animation-timing-function": AnimationTimingFunction(EasingFunction),
        "animation": Animation(Animation<'i>),

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
}
