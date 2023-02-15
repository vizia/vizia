use crate::prelude::*;
use cosmic_text::{FamilyOwned, Style, Weight};

use crate::animation::Transition;
use crate::style::shadow::BoxShadow;
use morphorm::{LayoutType, PositionType, Units};

#[derive(Debug, Clone, PartialEq)]
pub enum PropType {
    Units(Units),
    String(String),
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum Property {
    Unknown(String, PropType),

    // General
    Display(Display),
    Visibility(Visibility),
    Overflow(Overflow),
    Opacity(f32),

    // Positioning
    LayoutType(LayoutType),
    PositionType(PositionType),

    // Position and Size
    Space(Units),
    Left(Units),
    Width(Units),
    Right(Units),
    Top(Units),
    Height(Units),
    Bottom(Units),

    // Constraints
    MinLeft(Units),
    MaxLeft(Units),
    MinWidth(Units),
    MaxWidth(Units),
    MinRight(Units),
    MaxRight(Units),

    MinTop(Units),
    MaxTop(Units),
    MinHeight(Units),
    MaxHeight(Units),
    MinBottom(Units),
    MaxBottom(Units),

    // Child Spacing
    ChildSpace(Units),
    ChildLeft(Units),
    ChildRight(Units),
    ChildTop(Units),
    ChildBottom(Units),
    RowBetween(Units),
    ColBetween(Units),

    // Border Radius
    BorderRadius(Units),
    BorderTopLeftRadius(Units),
    BorderTopRightRadius(Units),
    BorderBottomLeftRadius(Units),
    BorderBottomRightRadius(Units),

    // Border Width
    BorderWidth(Units),
    BorderLeftWidth(Units),
    BorderRightWidth(Units),
    BorderTopWidth(Units),
    BorderBottomWidth(Units),

    // Border Color
    BorderColor(Color),

    // Border Shape
    BorderCornerShape(BorderCornerShape),
    BorderTopLeftShape(BorderCornerShape),
    BorderTopRightShape(BorderCornerShape),
    BorderBottomLeftShape(BorderCornerShape),
    BorderBottomRightShape(BorderCornerShape),

    // Outline
    OutlineWidth(Units),
    OutlineColor(Color),
    OutlineOffset(Units),

    // Background
    BackgroundColor(Color),
    BackgroundImage(String),
    // TODO
    //BackgroundGradient(LinearGradient),

    // Font
    FontSize(f32),
    FontColor(Color),
    FontFamily(Vec<FamilyOwned>),
    FontWeight(Weight),
    FontStyle(FontStyle),
    SelectionColor(Color),
    CaretColor(Color),
    TextWrap(bool),

    // Shadow
    OuterShadow(BoxShadow),
    OuterShadowHOffset(Units),
    OuterShadowVOffset(Units),
    OuterShadowBlur(Units),
    OuterShadowColor(Color),

    InnerShadow(BoxShadow),
    InnerShadowHOffset(Units),
    InnerShadowVOffset(Units),
    InnerShadowBlur(Units),
    InnerShadowColor(Color),

    Transition(Vec<Transition>),

    ZIndex(i32),

    // TODO
    // Translate((f32, f32)),
    // Rotate(f32),
    // Scale((f32, f32)),
    Cursor(CursorIcon),
}

pub(crate) fn fmt_units(val: &Units) -> String {
    match val {
        Pixels(px) => format!("{}px", px),
        Percentage(p) => format!("{}%", p),
        Stretch(s) => format!("{}s", s),
        Auto => format!("auto"),
    }
}

fn fmt_layout_type(val: &LayoutType) -> String {
    match val {
        LayoutType::Row => "row",
        LayoutType::Column => "column",
        LayoutType::Grid => "grid",
    }
    .to_owned()
}

fn fmt_position_type(val: &PositionType) -> String {
    match val {
        PositionType::SelfDirected => "self-directed",
        PositionType::ParentDirected => "parent-directed",
    }
    .to_owned()
}

impl std::fmt::Display for Property {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Property::Unknown(ident, prop) => {
                write!(
                    f,
                    "/* unknown: {}: {}; /*",
                    ident,
                    match prop {
                        PropType::Units(val) => fmt_units(val),

                        PropType::String(string) => {
                            string.clone()
                        }
                    }
                )
            }
            // General
            Property::Display(val) => write!(f, "display: {};", val),
            Property::Visibility(val) => write!(f, "visibility: {};", val),
            Property::Overflow(val) => write!(f, "overflow: {};", val),
            Property::Opacity(val) => write!(f, "opacity: {};", val),

            // Positioning
            Property::LayoutType(val) => write!(f, "layout-type: {};", fmt_layout_type(val)),
            Property::PositionType(val) => write!(f, "position-type: {};", fmt_position_type(val)),

            // Position and Size
            Property::Space(val) => write!(f, "space: {};", fmt_units(val)),
            Property::Left(val) => write!(f, "left: {};", fmt_units(val)),
            Property::Width(val) => write!(f, "width: {};", fmt_units(val)),
            Property::Right(val) => write!(f, "right: {};", fmt_units(val)),
            Property::Top(val) => write!(f, "top: {};", fmt_units(val)),
            Property::Height(val) => write!(f, "height: {};", fmt_units(val)),
            Property::Bottom(val) => write!(f, "bottom: {};", fmt_units(val)),

            // Constraints
            Property::MinLeft(val) => write!(f, "min-left: {};", fmt_units(val)),
            Property::MaxLeft(val) => write!(f, "max-left: {};", fmt_units(val)),
            Property::MinWidth(val) => write!(f, "min-width: {};", fmt_units(val)),
            Property::MaxWidth(val) => write!(f, "max-width: {};", fmt_units(val)),
            Property::MinRight(val) => write!(f, "min-right: {};", fmt_units(val)),
            Property::MaxRight(val) => write!(f, "max-right: {};", fmt_units(val)),

            Property::MinTop(val) => write!(f, "min-top: {};", fmt_units(val)),
            Property::MaxTop(val) => write!(f, "max-top: {};", fmt_units(val)),
            Property::MinHeight(val) => write!(f, "min-height: {};", fmt_units(val)),
            Property::MaxHeight(val) => write!(f, "max-height: {};", fmt_units(val)),
            Property::MinBottom(val) => write!(f, "min-bottom: {};", fmt_units(val)),
            Property::MaxBottom(val) => write!(f, "max-bottom: {};", fmt_units(val)),

            // Child Spacing
            Property::ChildSpace(val) => write!(f, "child-space: {};", fmt_units(val)),
            Property::ChildLeft(val) => write!(f, "child-left: {};", fmt_units(val)),
            Property::ChildRight(val) => write!(f, "child-right: {};", fmt_units(val)),
            Property::ChildTop(val) => write!(f, "child-top: {};", fmt_units(val)),
            Property::ChildBottom(val) => write!(f, "child-bottom: {};", fmt_units(val)),
            Property::RowBetween(val) => write!(f, "row-between: {};", fmt_units(val)),
            Property::ColBetween(val) => write!(f, "col-between: {};", fmt_units(val)),

            // Border
            Property::BorderRadius(val) => write!(f, "border-radius: {};", fmt_units(val)),
            Property::BorderTopLeftRadius(val) => {
                write!(f, "border-top-left-radius: {};", fmt_units(val))
            }
            Property::BorderTopRightRadius(val) => {
                write!(f, "border-top-right-radius: {};", fmt_units(val))
            }
            Property::BorderBottomLeftRadius(val) => {
                write!(f, "border-bottom-left-radius: {};", fmt_units(val))
            }
            Property::BorderBottomRightRadius(val) => {
                write!(f, "border-bottom-right-radius: {};", fmt_units(val))
            }
            Property::BorderWidth(val) => write!(f, "border-width: {};", fmt_units(val)),
            Property::BorderLeftWidth(val) => {
                write!(f, "border-left-width: {};", fmt_units(val))
            }
            Property::BorderRightWidth(val) => {
                write!(f, "border-right-width: {};", fmt_units(val))
            }
            Property::BorderTopWidth(val) => {
                write!(f, "border-top-width: {};", fmt_units(val))
            }
            Property::BorderBottomWidth(val) => {
                write!(f, "border-bottom-width: {};", fmt_units(val))
            }
            Property::BorderColor(val) => write!(f, "border-color: {};", val),
            Property::BorderCornerShape(val) => write!(f, "border-corner-shape: {};", val),
            Property::BorderTopLeftShape(val) => write!(f, "border-top-left-shape: {};", val),
            Property::BorderTopRightShape(val) => write!(f, "border-top-right-shape: {};", val),
            Property::BorderBottomLeftShape(val) => write!(f, "border-bottom-left-shape: {};", val),
            Property::BorderBottomRightShape(val) => {
                write!(f, "border-bottom-right-shape: {};", val)
            }

            // Background
            Property::BackgroundColor(val) => write!(f, "background-color: {};", val),
            Property::BackgroundImage(val) => write!(f, "background-image: {};", val),

            // Outline
            Property::OutlineWidth(val) => write!(f, "outline-width: {}", fmt_units(val)),
            Property::OutlineColor(val) => write!(f, "outline-color: {}", val),
            Property::OutlineOffset(val) => write!(f, "outline-offset: {}", fmt_units(val)),

            // Text
            Property::FontSize(val) => write!(f, "font-size: {};", val),
            Property::FontColor(val) => write!(f, "color: {};", val),
            Property::FontFamily(val) => write!(
                f,
                "font-family: {};",
                val.iter().map(fmt_font_family).collect::<Vec<_>>().join(", ")
            ),
            Property::FontWeight(val) => write!(f, "font-weight: {}", val.0),
            Property::FontStyle(val) => write!(f, "font-style: {}", fmt_font_style(val)),
            Property::SelectionColor(val) => write!(f, "selection-color: {}", val),
            Property::CaretColor(val) => write!(f, "caret-color: {}", val),
            Property::TextWrap(val) => write!(f, "text-wrap: {}", val),

            // Shadow
            Property::OuterShadow(val) => write!(f, "outer-shadow: {};", val),
            Property::InnerShadow(val) => write!(f, "inner-shadow: {};", val),
            Property::OuterShadowHOffset(val) => {
                write!(f, "outer-shadow-h-offset: {}", fmt_units(val))
            }
            Property::OuterShadowVOffset(val) => {
                write!(f, "outer-shadow-v-offset: {}", fmt_units(val))
            }
            Property::OuterShadowBlur(val) => write!(f, "inner-shadow-blur: {}", fmt_units(val)),
            Property::OuterShadowColor(val) => write!(f, "inner-shadow-color: {}", val),
            Property::InnerShadowHOffset(val) => {
                write!(f, "inner-shadow-h-offset: {}", fmt_units(val))
            }
            Property::InnerShadowVOffset(val) => {
                write!(f, "inner-shadow-v-offset: {}", fmt_units(val))
            }
            Property::InnerShadowBlur(val) => write!(f, "inner-shadow-blur: {}", fmt_units(val)),
            Property::InnerShadowColor(val) => write!(f, "inner-shadow-color: {}", val),

            Property::Transition(val) => write!(f, "transition: {:?};", val),

            Property::ZIndex(val) => write!(f, "z-index: {};", val),

            Property::Cursor(val) => write!(f, "cursor: {};", val),
        }
    }
}

fn fmt_font_style(val: &Style) -> String {
    match val {
        Style::Normal => "normal",
        Style::Italic => "italic",
        Style::Oblique => "oblique",
    }
    .to_owned()
}

fn fmt_font_family(val: &FamilyOwned) -> String {
    match val {
        FamilyOwned::Name(name) => format!("\"{}\"", name),
        FamilyOwned::Serif => "serif".to_owned(),
        FamilyOwned::SansSerif => "sans-serif".to_owned(),
        FamilyOwned::Cursive => "cursive".to_owned(),
        FamilyOwned::Fantasy => "fantasy".to_owned(),
        FamilyOwned::Monospace => "monospace".to_owned(),
    }
}
