use morphorm::{LayoutType, PositionType, Units};
use std::collections::{HashMap, HashSet};
use vizia_id::GenerationalId;
use vizia_style::{
    BoxShadow, Clip, CssRule, FontFamily, FontSize, GenericFontFamily, Gradient, Transition,
};

use crate::prelude::*;

pub use vizia_style::{
    Angle, BackgroundImage, BorderCornerShape, Color, CursorIcon, Display, FontStretch, FontStyle,
    FontWeight, FontWeightKeyword, Length, LengthOrPercentage, LengthValue, LineDirection,
    LinearGradient, Opacity, Overflow, ParserOptions, Property, SelectorList, Selectors,
    StyleSheet, Transform, Visibility, RGBA,
};

mod rule;
pub use rule::Rule;

mod transform;
pub use transform::*;

mod selector;
pub use selector::*;

// mod specificity;
// use specificity::*;

// mod property;
// pub use property::*;

// mod shadow;
// use shadow::*;

// mod prop;
// pub use prop::*;

use crate::animation::{AnimationState, Interpolator};
use crate::storage::animatable_set::AnimatableSet;
use crate::storage::style_set::StyleSet;
use bitflags::bitflags;
use cosmic_text::FamilyOwned;
use vizia_id::IdManager;
use vizia_storage::SparseSet;

bitflags! {
    /// Describes the capabilities of a view with respect to user interaction.
    ///
    /// This type is part of the prelude.
    pub struct Abilities: u8 {
        const HOVERABLE = 1;
        const FOCUSABLE = 1 << 1;
        const CHECKABLE = 1 << 2;
        const SELECTABLE = 1 << 3;
        /// The element should be focusable in sequential keyboard navigation -
        /// allowing the equivilant of a negative tabindex in html.
        const NAVIGABLE = 1 << 4;
    }
}

impl Default for Abilities {
    fn default() -> Abilities {
        Abilities::HOVERABLE
    }
}

/// Stores the style properties of all entities in the application.
#[derive(Default)]
pub struct Style {
    pub(crate) rule_manager: IdManager<Rule>,

    /// Creates and destroys animation ids
    pub(crate) animation_manager: IdManager<Animation>,

    // pub(crate) rules: Vec<StyleRule>,
    pub selectors: HashMap<Rule, (u32, SelectorList<Selectors>)>,
    pub transitions: HashMap<Rule, Animation>,

    pub default_font: Vec<FamilyOwned>,

    pub elements: SparseSet<String>,
    pub ids: SparseSet<String>,
    pub classes: SparseSet<HashSet<String>>,
    pub pseudo_classes: SparseSet<PseudoClassFlags>,
    pub disabled: StyleSet<bool>,
    pub abilities: SparseSet<Abilities>,

    // Display
    pub display: StyleSet<Display>,

    // Visibility
    pub visibility: StyleSet<Visibility>,

    // Opacity
    pub opacity: AnimatableSet<Opacity>,

    // Z Order
    pub z_index: StyleSet<i32>,

    // Clipping
    pub clip: AnimatableSet<Clip>,

    // Transform
    pub transform: AnimatableSet<Vec<Transform>>,
    // pub rotate: AnimatableSet<f32>,
    // pub translate: AnimatableSet<(f32, f32)>,
    // pub scale: AnimatableSet<(f32, f32)>,

    // Overflow
    pub overflowx: StyleSet<Overflow>,
    pub overflowy: StyleSet<Overflow>,

    // Border
    pub border_width: AnimatableSet<LengthOrPercentage>,
    pub border_color: AnimatableSet<Color>,

    // Border Shape
    pub border_top_left_shape: StyleSet<BorderCornerShape>,
    pub border_top_right_shape: StyleSet<BorderCornerShape>,
    pub border_bottom_left_shape: StyleSet<BorderCornerShape>,
    pub border_bottom_right_shape: StyleSet<BorderCornerShape>,

    // Border Radius
    pub border_top_left_radius: AnimatableSet<LengthOrPercentage>,
    pub border_top_right_radius: AnimatableSet<LengthOrPercentage>,
    pub border_bottom_left_radius: AnimatableSet<LengthOrPercentage>,
    pub border_bottom_right_radius: AnimatableSet<LengthOrPercentage>,

    // Outline
    pub outline_width: AnimatableSet<LengthOrPercentage>,
    pub outline_color: AnimatableSet<Color>,
    pub outline_offset: AnimatableSet<LengthOrPercentage>,

    // Focus Order
    // pub focus_order: SparseSet<FocusOrder>,

    // Background
    pub background_color: AnimatableSet<Color>,
    pub background_image: StyleSet<String>,
    pub background_gradient: AnimatableSet<Vec<Gradient>>,

    pub box_shadow: AnimatableSet<Vec<BoxShadow>>,

    // Text & Font
    pub text_wrap: StyleSet<bool>,
    pub font_family: StyleSet<Vec<FamilyOwned>>,
    pub font_color: AnimatableSet<Color>,
    pub font_size: AnimatableSet<FontSize>,
    pub font_weight: StyleSet<FontWeight>,
    pub font_style: StyleSet<FontStyle>,
    pub font_stretch: StyleSet<FontStretch>,
    pub caret_color: AnimatableSet<Color>,
    pub selection_color: AnimatableSet<Color>,

    // Image
    pub image: StyleSet<String>,

    pub tooltip: SparseSet<String>,

    // LAYOUT

    // Layout Type
    pub layout_type: StyleSet<LayoutType>,

    // Position Type
    pub position_type: StyleSet<PositionType>,

    // Spacing
    pub left: AnimatableSet<Units>,
    pub right: AnimatableSet<Units>,
    pub top: AnimatableSet<Units>,
    pub bottom: AnimatableSet<Units>,

    // Size
    pub width: AnimatableSet<Units>,
    pub height: AnimatableSet<Units>,

    // Size Constraints
    pub max_width: AnimatableSet<Units>,
    pub max_height: AnimatableSet<Units>,
    pub min_width: AnimatableSet<Units>,
    pub min_height: AnimatableSet<Units>,
    pub content_width: StyleSet<f32>,
    pub content_height: StyleSet<f32>,

    // Spacing Constraints
    pub min_left: AnimatableSet<Units>,
    pub max_left: AnimatableSet<Units>,
    pub min_right: AnimatableSet<Units>,
    pub max_right: AnimatableSet<Units>,
    pub min_top: AnimatableSet<Units>,
    pub max_top: AnimatableSet<Units>,
    pub min_bottom: AnimatableSet<Units>,
    pub max_bottom: AnimatableSet<Units>,

    // Grid
    pub grid_rows: StyleSet<Vec<Units>>,
    pub row_between: AnimatableSet<Units>,
    pub grid_cols: StyleSet<Vec<Units>>,
    pub col_between: AnimatableSet<Units>,

    pub row_index: StyleSet<usize>,
    pub col_index: StyleSet<usize>,
    pub row_span: StyleSet<usize>,
    pub col_span: StyleSet<usize>,

    // Child Spacing
    pub child_left: AnimatableSet<Units>,
    pub child_right: AnimatableSet<Units>,
    pub child_top: AnimatableSet<Units>,
    pub child_bottom: AnimatableSet<Units>,

    pub name: StyleSet<String>,

    pub cursor: StyleSet<CursorIcon>,

    pub needs_restyle: bool,
    pub needs_relayout: bool,
    pub needs_redraw: bool,
    // TODO: When we can do incremental updates on a per entity basis, change this to a bitflag
    // for layout, text layout, rendering, etc. to replace the above `needs_` members.
    pub needs_text_layout: SparseSet<bool>,

    /// This includes both the system's HiDPI scaling factor as well as `cx.user_scale_factor`.
    pub dpi_factor: f64,
}

impl Style {
    // pub(crate) fn add_rule(&mut self, style_rule: StyleRule) {
    //     if !self.rules.contains(&style_rule) {
    //         self.rules.push(style_rule);
    //         self.rules.sort_by_key(|rule| rule.specificity());
    //         self.rules.reverse();
    //     }

    //     self.set_style_properties();
    // }

    pub fn remove_rules(&mut self) {
        // for rule in self.rules.iter() {
        //     self.rule_manager.destroy(rule.id);
        // }

        // for (_, animation) in self.transitions.iter() {
        //     self.animation_manager.destroy(*animation);
        // }
    }

    pub fn parse_theme(&mut self, stylesheet: &str) {
        if let Ok(theme) = StyleSheet::parse("test.css", stylesheet, ParserOptions::default()) {
            let rules = theme.rules.0;

            for rule in rules {
                match rule {
                    CssRule::Style(style_rule) => {
                        let rule_id = self.rule_manager.create();

                        //TODO: Store map of selectors
                        let selectors = style_rule.selectors;

                        let specificity = selectors.0.first().unwrap().specificity();

                        self.selectors.insert(rule_id, (specificity, selectors));

                        for property in style_rule.declarations.declarations {
                            match property {
                                Property::Transition(transitions) => {
                                    for transition in transitions.iter() {
                                        self.insert_transition(rule_id, transition);
                                    }
                                }

                                _ => {
                                    self.insert_property(rule_id, property);
                                }
                            }
                        }
                    }

                    _ => {}
                }
            }
        }

        // let mut input = ParserInput::new(stylesheet);
        // let mut parser = Parser::new(&mut input);
        // let rule_parser = parser::RuleParser::new();

        // let rules = {
        //     let rule_list_parser =
        //         cssparser::RuleListParser::new_for_stylesheet(&mut parser, rule_parser);
        //     rule_list_parser.collect::<Vec<_>>()
        // };

        // let mut rule_list: Vec<StyleRule> = rules
        //     .into_iter()
        //     .filter_map(|rule| {
        //         match rule {
        //             Ok(mut style_rule) => {
        //                 style_rule.id = self.rule_manager.create();
        //                 Some(style_rule)
        //             }
        //             Err(parse_error) => {
        //                 let style_parse_error = StyleParseError(parse_error.0);
        //                 println!("{}", style_parse_error);
        //                 None
        //             }
        //         }
        //         //rule.ok()
        //     })
        //     .collect();

        // self.rules.append(&mut rule_list);

        // self.rules.sort_by_key(|rule| rule.specificity());
        // self.rules.reverse();

        // // for rule in self.rules.iter() {
        // //     print!("{}", rule);
        // // }

        // self.clear_style_rules();
        // self.set_style_properties();
    }

    fn insert_transition(&mut self, rule_id: Rule, transition: &Transition) {
        let animation = self.animation_manager.create();
        match transition.property.as_ref() {
            "opacity" => {
                self.opacity.insert_animation(animation, self.add_transition(transition));
                self.opacity.insert_transition(rule_id, animation);
            }

            "background-color" => {
                self.background_color.insert_animation(animation, self.add_transition(transition));
                self.background_color.insert_transition(rule_id, animation);
            }

            "border" => {
                self.border_width.insert_animation(animation, self.add_transition(transition));
                self.border_width.insert_transition(rule_id, animation);
                self.border_color.insert_animation(animation, self.add_transition(transition));
                self.border_color.insert_transition(rule_id, animation);
            }

            "border-width" => {
                self.border_width.insert_animation(animation, self.add_transition(transition));
                self.border_width.insert_transition(rule_id, animation);
            }

            "border-color" => {
                self.border_color.insert_animation(animation, self.add_transition(transition));
                self.border_color.insert_transition(rule_id, animation);
            }

            "outline" => {
                self.outline_width.insert_animation(animation, self.add_transition(transition));
                self.outline_width.insert_transition(rule_id, animation);
                self.outline_color.insert_animation(animation, self.add_transition(transition));
                self.outline_color.insert_transition(rule_id, animation);
            }

            "outline-width" => {
                self.outline_width.insert_animation(animation, self.add_transition(transition));
                self.outline_width.insert_transition(rule_id, animation);
            }

            "outline-color" => {
                self.outline_color.insert_animation(animation, self.add_transition(transition));
                self.outline_color.insert_transition(rule_id, animation);
            }

            "outline-offset" => {
                self.outline_offset.insert_animation(animation, self.add_transition(transition));
                self.outline_offset.insert_transition(rule_id, animation);
            }

            "transform" => {
                self.transform.insert_animation(animation, self.add_transition(transition));
                self.transform.insert_transition(rule_id, animation);
            }

            "background-image" => {
                self.background_gradient
                    .insert_animation(animation, self.add_transition(transition));
                self.background_gradient.insert_transition(rule_id, animation);
            }

            "border-radius" => {
                self.border_bottom_left_radius
                    .insert_animation(animation, self.add_transition(transition));
                self.border_bottom_left_radius.insert_transition(rule_id, animation);
                self.border_bottom_right_radius
                    .insert_animation(animation, self.add_transition(transition));
                self.border_bottom_right_radius.insert_transition(rule_id, animation);
                self.border_top_left_radius
                    .insert_animation(animation, self.add_transition(transition));
                self.border_top_left_radius.insert_transition(rule_id, animation);
                self.border_top_right_radius
                    .insert_animation(animation, self.add_transition(transition));
                self.border_top_right_radius.insert_transition(rule_id, animation);
            }

            "border-bottom-left-radius" => {
                self.border_bottom_left_radius
                    .insert_animation(animation, self.add_transition(transition));
                self.border_bottom_left_radius.insert_transition(rule_id, animation);
            }

            "border-bottom-right-radius" => {
                self.border_bottom_right_radius
                    .insert_animation(animation, self.add_transition(transition));
                self.border_bottom_right_radius.insert_transition(rule_id, animation);
            }

            "border-top-left-radius" => {
                self.border_top_left_radius
                    .insert_animation(animation, self.add_transition(transition));
                self.border_top_left_radius.insert_transition(rule_id, animation);
            }

            "border-top-right-radius" => {
                self.border_top_right_radius
                    .insert_animation(animation, self.add_transition(transition));
                self.border_top_right_radius.insert_transition(rule_id, animation);
            }

            "width" => {
                self.width.insert_animation(animation, self.add_transition(transition));
                self.width.insert_transition(rule_id, animation);
            }

            "box-shadow" => {
                self.box_shadow.insert_animation(animation, self.add_transition(transition));
                self.box_shadow.insert_transition(rule_id, animation);
            }

            _ => return,
        }

        self.transitions.insert(rule_id, animation);
    }

    fn insert_property(&mut self, rule_id: Rule, property: Property) {
        match property {
            // Display
            Property::Display(display) => {
                self.display.insert_rule(rule_id, display);
            }

            Property::Visibility(visibility) => {
                self.visibility.insert_rule(rule_id, visibility);
            }

            Property::Opacity(opacity) => {
                self.opacity.insert_rule(rule_id, opacity);
            }

            Property::Clip(clip) => {
                self.clip.insert_rule(rule_id, clip);
            }

            // Layout Type
            Property::LayoutType(layout_type) => {
                self.layout_type.insert_rule(rule_id, layout_type);
            }

            // Position Type
            Property::PositionType(position_type) => {
                self.position_type.insert_rule(rule_id, position_type);
            }

            // Space
            Property::Space(space) => {
                self.left.insert_rule(rule_id, space);
                self.right.insert_rule(rule_id, space);
                self.top.insert_rule(rule_id, space);
                self.bottom.insert_rule(rule_id, space);
            }

            Property::Left(left) => {
                self.left.insert_rule(rule_id, left);
            }

            Property::Right(right) => {
                self.right.insert_rule(rule_id, right);
            }

            Property::Top(top) => {
                self.top.insert_rule(rule_id, top);
            }

            Property::Bottom(bottom) => {
                self.bottom.insert_rule(rule_id, bottom);
            }

            // Size
            Property::Size(size) => {
                self.width.insert_rule(rule_id, size);
                self.height.insert_rule(rule_id, size);
            }

            Property::Width(width) => {
                self.width.insert_rule(rule_id, width);
            }

            Property::Height(height) => {
                self.height.insert_rule(rule_id, height);
            }

            // Child Space
            Property::ChildSpace(child_space) => {
                self.child_left.insert_rule(rule_id, child_space);
                self.child_right.insert_rule(rule_id, child_space);
                self.child_top.insert_rule(rule_id, child_space);
                self.child_bottom.insert_rule(rule_id, child_space);
            }

            Property::ChildLeft(child_left) => {
                self.child_left.insert_rule(rule_id, child_left);
            }

            Property::ChildRight(child_right) => {
                self.child_right.insert_rule(rule_id, child_right);
            }

            Property::ChildTop(child_top) => {
                self.child_top.insert_rule(rule_id, child_top);
            }

            Property::ChildBottom(child_bottom) => {
                self.child_bottom.insert_rule(rule_id, child_bottom);
            }

            Property::RowBetween(row_between) => {
                self.row_between.insert_rule(rule_id, row_between);
            }

            Property::ColBetween(col_between) => {
                self.col_between.insert_rule(rule_id, col_between);
            }

            Property::MinSpace(min_space) => {
                self.min_left.insert_rule(rule_id, min_space);
                self.min_right.insert_rule(rule_id, min_space);
                self.min_top.insert_rule(rule_id, min_space);
                self.min_bottom.insert_rule(rule_id, min_space);
            }

            Property::MinLeft(min_left) => {
                self.min_left.insert_rule(rule_id, min_left);
            }

            Property::MinRight(min_right) => {
                self.min_right.insert_rule(rule_id, min_right);
            }

            Property::MinTop(min_top) => {
                self.min_top.insert_rule(rule_id, min_top);
            }

            Property::MinBottom(min_bottom) => {
                self.min_bottom.insert_rule(rule_id, min_bottom);
            }

            Property::MaxSpace(max_space) => {
                self.max_left.insert_rule(rule_id, max_space);
                self.max_right.insert_rule(rule_id, max_space);
                self.max_top.insert_rule(rule_id, max_space);
                self.max_bottom.insert_rule(rule_id, max_space);
            }

            Property::MaxLeft(max_left) => {
                self.max_left.insert_rule(rule_id, max_left);
            }

            Property::MaxRight(max_right) => {
                self.max_right.insert_rule(rule_id, max_right);
            }

            Property::MaxTop(max_top) => {
                self.max_top.insert_rule(rule_id, max_top);
            }

            Property::MaxBottom(max_bottom) => {
                self.max_bottom.insert_rule(rule_id, max_bottom);
            }

            Property::MinSize(min_size) => {
                self.min_width.insert_rule(rule_id, min_size);
                self.min_height.insert_rule(rule_id, min_size);
            }

            Property::MinWidth(min_width) => {
                self.min_width.insert_rule(rule_id, min_width);
            }

            Property::MinHeight(min_height) => {
                self.min_height.insert_rule(rule_id, min_height);
            }

            Property::MaxSize(max_size) => {
                self.max_width.insert_rule(rule_id, max_size);
                self.max_height.insert_rule(rule_id, max_size);
            }

            Property::MaxWidth(max_width) => {
                self.max_width.insert_rule(rule_id, max_width);
            }

            Property::MaxHeight(max_height) => {
                self.max_height.insert_rule(rule_id, max_height);
            }

            // Background
            Property::BackgroundColor(color) => {
                self.background_color.insert_rule(rule_id, color);
            }

            // Border
            Property::Border(border) => {
                if let Some(border_color) = border.color {
                    self.border_color.insert_rule(rule_id, border_color);
                }

                if let Some(border_width) = border.width {
                    self.border_width.insert_rule(rule_id, border_width.into());
                }
            }

            Property::BorderWidth(border_width) => {
                self.border_width.insert_rule(rule_id, border_width.top.0);
            }
            Property::BorderColor(color) => {
                self.border_color.insert_rule(rule_id, color);
            }

            // Border Radius
            Property::BorderRadius(border_radius) => {
                self.border_bottom_left_radius.insert_rule(rule_id, border_radius.bottom_left);
                self.border_bottom_right_radius.insert_rule(rule_id, border_radius.bottom_right);
                self.border_top_left_radius.insert_rule(rule_id, border_radius.top_left);
                self.border_top_right_radius.insert_rule(rule_id, border_radius.top_right);
            }

            Property::BorderBottomLeftRadius(border_radius) => {
                self.border_bottom_left_radius.insert_rule(rule_id, border_radius);
            }

            Property::BorderTopLeftRadius(border_radius) => {
                self.border_top_left_radius.insert_rule(rule_id, border_radius);
            }

            Property::BorderBottomRightRadius(border_radius) => {
                self.border_bottom_right_radius.insert_rule(rule_id, border_radius);
            }

            Property::BorderTopRightRadius(border_radius) => {
                self.border_top_right_radius.insert_rule(rule_id, border_radius);
            }

            // Border Corner Shape
            Property::BorderCornerShape(border_corner_shape) => {
                self.border_top_left_shape.insert_rule(rule_id, border_corner_shape.0);
                self.border_top_right_shape.insert_rule(rule_id, border_corner_shape.1);
                self.border_bottom_right_shape.insert_rule(rule_id, border_corner_shape.2);
                self.border_bottom_left_shape.insert_rule(rule_id, border_corner_shape.3);
            }

            Property::BorderTopLeftShape(border_corner_shape) => {
                self.border_top_left_shape.insert_rule(rule_id, border_corner_shape);
            }

            Property::BorderTopRightShape(border_corner_shape) => {
                self.border_top_right_shape.insert_rule(rule_id, border_corner_shape);
            }

            Property::BorderBottomLeftShape(border_corner_shape) => {
                self.border_bottom_left_shape.insert_rule(rule_id, border_corner_shape);
            }

            Property::BorderBottomRightShape(border_corner_shape) => {
                self.border_bottom_right_shape.insert_rule(rule_id, border_corner_shape);
            }

            // Font Family
            Property::FontFamily(font_family) => {
                self.font_family.insert_rule(
                    rule_id,
                    font_family
                        .iter()
                        .map(|family| match family {
                            FontFamily::Named(name) => FamilyOwned::Name(name.to_string()),
                            FontFamily::Generic(generic) => match generic {
                                GenericFontFamily::Serif => FamilyOwned::Serif,
                                GenericFontFamily::SansSerif => FamilyOwned::SansSerif,
                                GenericFontFamily::Cursive => FamilyOwned::Cursive,
                                GenericFontFamily::Fantasy => FamilyOwned::Fantasy,
                                GenericFontFamily::Monospace => FamilyOwned::Monospace,
                            },
                        })
                        .collect::<Vec<_>>(),
                );
            }

            // Font Color
            Property::FontColor(font_color) => {
                self.font_color.insert_rule(rule_id, font_color);
            }

            // Font Size
            Property::FontSize(font_size) => {
                self.font_size.insert_rule(rule_id, font_size);
            }

            Property::FontWeight(font_weight) => {
                self.font_weight.insert_rule(rule_id, font_weight);
            }

            Property::FontStyle(font_style) => {
                self.font_style.insert_rule(rule_id, font_style);
            }

            Property::FontStretch(font_stretch) => {
                self.font_stretch.insert_rule(rule_id, font_stretch);
            }

            // Caret Color
            Property::CaretColor(caret_color) => {
                self.caret_color.insert_rule(rule_id, caret_color);
            }

            // Selection Color
            Property::SelectionColor(selection_color) => {
                self.selection_color.insert_rule(rule_id, selection_color);
            }

            // Transform
            Property::Transform(transforms) => {
                self.transform.insert_rule(rule_id, transforms);
            }

            Property::Overflow(overflow) => {
                self.overflowx.insert_rule(rule_id, overflow);
                self.overflowy.insert_rule(rule_id, overflow);
            }

            Property::OverflowX(overflow) => {
                self.overflowx.insert_rule(rule_id, overflow);
            }

            Property::OverflowY(overflow) => {
                self.overflowy.insert_rule(rule_id, overflow);
            }

            Property::ZIndex(z_index) => self.z_index.insert_rule(rule_id, z_index),

            Property::Outline(outline) => {
                if let Some(outline_color) = outline.color {
                    self.outline_color.insert_rule(rule_id, outline_color);
                }

                if let Some(outline_width) = outline.width {
                    self.outline_width.insert_rule(rule_id, outline_width.into());
                }
            }

            Property::OutlineColor(outline_color) => {
                self.outline_color.insert_rule(rule_id, outline_color);
            }

            Property::OutlineWidth(outline_width) => {
                self.outline_width.insert_rule(rule_id, outline_width.left.0);
            }

            Property::OutlineOffset(outline_offset) => {
                self.outline_offset.insert_rule(rule_id, outline_offset);
            }

            Property::BackgroundImage(images) => {
                let gradients = images
                    .into_iter()
                    .filter_map(|img| match img {
                        BackgroundImage::Gradient(gradient) => Some(*gradient),
                        _ => None,
                    })
                    .collect::<Vec<_>>();
                self.background_gradient.insert_rule(rule_id, gradients);
                // BackgroundImage::Name(_) => {}
                // BackgroundImage::Gradient(gradient) => {
                //     self.background_gradient.insert_rule(rule_id, *gradient);
                // }
            }
            // Property::TextWrap(_) => todo!(),
            Property::BoxShadow(box_shadows) => {
                self.box_shadow.insert_rule(rule_id, box_shadows);
            }

            Property::Translate(_) => todo!(),
            Property::Rotate(_) => todo!(),
            Property::Scale(_) => todo!(),

            Property::Cursor(cursor) => {
                self.cursor.insert_rule(rule_id, cursor);
            }
            Property::Unparsed(unparsed) => {
                println!("Unparsed: {}", unparsed.name);
            }
            Property::Custom(custom) => {
                println!("Custom Property: {}", custom.name);
            }

            _ => {}
        }
    }

    fn add_transition<T: Default + Interpolator>(
        &self,
        transition: &Transition,
    ) -> AnimationState<T> {
        AnimationState::new(Animation::null())
            .with_duration(transition.duration)
            .with_delay(transition.delay)
            .with_keyframe((0.0, Default::default()))
            .with_keyframe((1.0, Default::default()))
    }

    // Add style data to an entity
    pub fn add(&mut self, entity: Entity) {
        self.pseudo_classes
            .insert(entity, PseudoClassFlags::default())
            .expect("Failed to add pseudoclasses");
        self.classes.insert(entity, HashSet::new()).expect("Failed to add class list");
        self.abilities.insert(entity, Abilities::default()).expect("Failed to add abilities");
        self.needs_restyle = true;
        self.needs_relayout = true;
        self.needs_redraw = true;
    }

    pub fn remove(&mut self, entity: Entity) {
        self.elements.remove(entity);
        self.ids.remove(entity);
        self.classes.remove(entity);
        self.pseudo_classes.remove(entity);
        self.disabled.remove(entity);
        self.abilities.remove(entity);
        // Display
        self.display.remove(entity);
        // Visibility
        self.visibility.remove(entity);
        // Opacity
        self.opacity.remove(entity);
        // Z Order
        self.z_index.remove(entity);
        // Clipping
        self.clip.remove(entity);

        // Transform
        self.transform.remove(entity);
        // self.translate.remove(entity);
        // self.rotate.remove(entity);
        // self.scale.remove(entity);

        self.overflowx.remove(entity);
        self.overflowy.remove(entity);

        // Border
        self.border_width.remove(entity);
        self.border_color.remove(entity);

        // Border Shape
        self.border_bottom_left_shape.remove(entity);
        self.border_bottom_right_shape.remove(entity);
        self.border_top_left_shape.remove(entity);
        self.border_top_right_shape.remove(entity);

        // Border Radius
        self.border_bottom_left_radius.remove(entity);
        self.border_bottom_right_radius.remove(entity);
        self.border_top_left_radius.remove(entity);
        self.border_top_right_radius.remove(entity);

        self.outline_width.remove(entity);
        self.outline_color.remove(entity);
        self.outline_offset.remove(entity);

        //self.focus_order.remove(entity);

        // Background
        self.background_color.remove(entity);
        self.background_image.remove(entity);
        self.background_gradient.remove(entity);

        self.box_shadow.remove(entity);

        self.layout_type.remove(entity);
        self.position_type.remove(entity);

        // Space
        self.left.remove(entity);
        self.right.remove(entity);
        self.top.remove(entity);
        self.bottom.remove(entity);

        // Size
        self.width.remove(entity);
        self.height.remove(entity);

        // Space Constraints
        self.min_left.remove(entity);
        self.max_left.remove(entity);
        self.min_right.remove(entity);
        self.max_right.remove(entity);
        self.min_top.remove(entity);
        self.max_top.remove(entity);
        self.min_bottom.remove(entity);
        self.max_bottom.remove(entity);

        // Size Constraints
        self.min_width.remove(entity);
        self.max_width.remove(entity);
        self.min_height.remove(entity);
        self.max_height.remove(entity);
        self.content_width.remove(entity);
        self.content_height.remove(entity);

        // Child Space
        self.child_left.remove(entity);
        self.child_right.remove(entity);
        self.child_top.remove(entity);
        self.child_bottom.remove(entity);
        self.col_between.remove(entity);
        self.row_between.remove(entity);

        // Grid
        self.grid_cols.remove(entity);
        self.grid_rows.remove(entity);
        self.col_index.remove(entity);
        self.col_span.remove(entity);
        self.row_index.remove(entity);
        self.row_span.remove(entity);

        // Text and Font
        self.text_wrap.remove(entity);
        self.font_family.remove(entity);
        self.font_weight.remove(entity);
        self.font_style.remove(entity);
        self.font_color.remove(entity);
        self.font_size.remove(entity);
        self.selection_color.remove(entity);
        self.caret_color.remove(entity);

        self.cursor.remove(entity);

        self.name.remove(entity);

        self.image.remove(entity);

        self.needs_text_layout.remove(entity);
    }

    pub fn clear_style_rules(&mut self) {
        self.disabled.clear_rules();
        // Display
        self.display.clear_rules();
        // Visibility
        self.visibility.clear_rules();
        // Opacity
        self.opacity.clear_rules();
        // Z Order
        self.z_index.clear_rules();

        self.clip.clear_rules();

        // Transform
        self.transform.clear_rules();
        // self.translate.clear_rules();
        // self.rotate.clear_rules();
        // self.scale.clear_rules();

        self.overflowx.clear_rules();
        self.overflowy.clear_rules();

        // Border
        self.border_width.clear_rules();
        self.border_color.clear_rules();

        // Border Shape
        self.border_bottom_left_shape.clear_rules();
        self.border_bottom_right_shape.clear_rules();
        self.border_top_left_shape.clear_rules();
        self.border_top_right_shape.clear_rules();

        // Border Radius
        self.border_bottom_left_radius.clear_rules();
        self.border_bottom_right_radius.clear_rules();
        self.border_top_left_radius.clear_rules();
        self.border_top_right_radius.clear_rules();

        // Outline
        self.outline_width.clear_rules();
        self.outline_color.clear_rules();
        self.outline_offset.clear_rules();

        // Background
        self.background_color.clear_rules();
        self.background_image.clear_rules();
        self.background_gradient.clear_rules();

        self.box_shadow.clear_rules();

        self.layout_type.clear_rules();
        self.position_type.clear_rules();

        // Space
        self.left.clear_rules();
        self.right.clear_rules();
        self.top.clear_rules();
        self.bottom.clear_rules();

        // Size
        self.width.clear_rules();
        self.height.clear_rules();

        // Space Constraints
        self.min_left.clear_rules();
        self.max_left.clear_rules();
        self.min_right.clear_rules();
        self.max_right.clear_rules();
        self.min_top.clear_rules();
        self.max_top.clear_rules();
        self.min_bottom.clear_rules();
        self.max_bottom.clear_rules();

        // Size Constraints
        self.min_width.clear_rules();
        self.max_width.clear_rules();
        self.min_height.clear_rules();
        self.max_height.clear_rules();
        self.content_width.clear_rules();
        self.content_height.clear_rules();

        // Child Space
        self.child_left.clear_rules();
        self.child_right.clear_rules();
        self.child_top.clear_rules();
        self.child_bottom.clear_rules();
        self.col_between.clear_rules();
        self.row_between.clear_rules();

        // Grid
        self.grid_cols.clear_rules();
        self.grid_rows.clear_rules();
        self.col_index.clear_rules();
        self.col_span.clear_rules();
        self.row_index.clear_rules();
        self.row_span.clear_rules();

        // Text and Font
        self.text_wrap.clear_rules();
        self.font_family.clear_rules();
        self.font_weight.clear_rules();
        self.font_style.clear_rules();
        self.font_color.clear_rules();
        self.font_size.clear_rules();
        self.selection_color.clear_rules();
        self.caret_color.clear_rules();

        self.cursor.clear_rules();

        self.name.clear_rules();

        self.image.clear_rules();
    }
}
