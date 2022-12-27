use crate::{
    define_property, Angle, BorderColor, BorderCornerShape, BorderRadius, BorderWidth,
    BorderWidthValue, BoxShadow, Color, CursorIcon, CustomParseError, CustomProperty, Display,
    FontSize, InsetKeyword, LayoutType, Length, LengthOrPercentage, Opacity, Overflow, Parse,
    PositionType, Rect, Scale, Transform, Transition, Translate, Units, UnparsedProperty,
    Visibility,
};
use cssparser::Parser;

define_property! {
    pub enum Property<'i> {
        // General
        "display": Display(Display),
        "visibility": Visibility(Visibility),
        "overflow": Overflow(Overflow),
        "opacity": Opacity(Opacity),
        "z-index": ZIndex(i32),

        // Positioning
        "layout-type": LayoutType(LayoutType),
        "position-type": PositionType(PositionType),

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
        "min-left": MinLeft(Units),
        "max-left": MaxLeft(Units),
        "min-width": MinWidth(Units),
        "max-width": MaxWidth(Units),
        "min-right": MinRight(Units),
        "max-right": MaxRight(Units),

        "min-top": MinTop(Units),
        "max-top": MaxTop(Units),
        "min-height": MinHeight(Units),
        "max-height": MaxHeight(Units),
        "min-bottom": MinBottom(Units),
        "max-bottom": MaxBottom(Units),

        // Child Spacing
        "child-space": ChildSpace(Units),
        "child-left": ChildLeft(Units),
        "child-right": ChildRight(Units),
        "child-top": ChildTop(Units),
        "child-bottom": ChildBottom(Units),
        "row-between": RowBetween(Units),
        "col-between": ColBetween(Units),
        // ----- Border -----

        // Border Shorthand
        // TODO: Support coloring and styling individual borders and enable this.
        // "border": Border(Border),

        // Border Color
        "border-color": BorderColor(Color),
        // TODO: Support coloring individual borders.
        // "border-top-color": BorderTopColor(Color),
        // "border-right-color": BorderRightColor(Color),
        // "border-bottom-color": BorderBottomColor(Color),
        // "border-left-color": BorderLeftColor(Color),

        // Border Corner Shape
        "border-corner-shape": BorderCornerShape(Rect<BorderCornerShape>),
        "border-top-left-shape": BorderTopLeftShape(BorderCornerShape),
        "border-top-right-shape": BorderTopRightShape(BorderCornerShape),
        "border-bottom-left-shape": BorderBottomLeftShape(BorderCornerShape),
        "border-bottom-right-shape": BorderBottomRightShape(BorderCornerShape),

        // Border Radius
        "border-radius": BorderRadius(BorderRadius),
        "border-top-left-radius": BorderTopLeftRadius(LengthOrPercentage),
        "border-top-right-radius": BorderTopRightRadius(LengthOrPercentage),
        "border-bottom-left-radius": BorderBottomLeftRadius(LengthOrPercentage),
        "border-bottom-right-radius": BorderBottomRightRadius(LengthOrPercentage),

        // Border Style
        // TODO: Support styling borders.
        // "border-style": BorderStyle(BorderStyle),
        // "border-top-style": BorderTopStyle(BorderStyleKeyword),
        // "border-right-style": BorderRightStyle(BorderStyleKeyword),
        // "border-bottom-style": BorderBottomStyle(BorderStyleKeyword),
        // "border-left-style": BorderLeftStyle(BorderStyleKeyword),

        // Border Width
        "border-width": BorderWidth(BorderWidth),
        "border-top-width": BorderTopWidth(BorderWidthValue),
        "border-right-width": BorderRightWidth(BorderWidthValue),
        "border-bottom-width": BorderBottomWidth(BorderWidthValue),
        "border-left-width": BorderLeftWidth(BorderWidthValue),


        // ----- Outline -----

        // Outline Shorthand
        // TODO: Support coloring and styling individual outlines.
        // "outline": Outline(Outline),

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
        "background-image": BackgroundImage(String),
        // // TODO
        // //BackgroundGradient(LinearGradient),

        // Font
        "font-size": FontSize(FontSize),
        "color": FontColor(Color),
        "font": Font(String),
        "selection-color": SelectionColor(Color), // TODO: Remove this once we have the pseudoselector version.
        "caret-color": CaretColor(Color),
        "text-wrap": TextWrap(bool),

        // Box Shadow
        "box-shadow": BoxShadow(Vec<BoxShadow>),
        "box-shadow-x-offset": BoxShadowXOffset(Length),
        "box-shadow-y-offset": BoxShadowYOffset(Length),
        "box-shadow-blur-radius": BoxShadowBlurRadius(Length),
        "box-shadow-spread-radius": BoxShadowSpreadRadius(Length),
        "box-shadow-color": BoxShadowColor(Color),
        "box-shadow-inset": BoxShadowInset(InsetKeyword),

        // Animations
        "transition": Transition(Vec<Transition>),

        // Transform
        "transform": Transform(Vec<Transform>),
        "translate": Translate(Translate),
        "rotate": Rotate(Angle),
        "scale": Scale(Scale),

        // Cursor
        "cursor": Cursor(CursorIcon),
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
        let parsed_property =
            Property::parse_value(CowRcStr::from("background-color"), &mut parser);

        println!("{:?}", parsed_property);
    }
}
