//! Styling determines the appearance of a view.
//!
//! # Styling Views
//! Vizia provides two ways to style views:
//! - Inline
//! - Shared
//!
//! ## Inline Styling
//! Inline styling refers to setting the style and layout properties of a view using view [modifiers](crate::modifiers).
//! ```
//! # use vizia_core::prelude::*;
//! # let cx = &mut Context::default();
//! Element::new(cx).background_color(Color::red());
//! ```
//! Properties set inline affect only the modified view and override any shared styling for the same property.
//!
//! ## Shared Styling
//! Shared styling refers to setting the style and layout properties using css rules.
//! ```
//! # use vizia_core::prelude::*;
//! # let cx = &mut Context::default();
//! Element::new(cx).class("foo");
//! ```
//! ```css
//! .foo {
//!     background-color: red;
//! }
//! ```
//! Rules defined in css can apply to many views but are overridden by inline properties on a view.
//!
//! ### Adding Stylesheets
//! To add a css string to an application, use [`add_theme()`](crate::context::Context::add_theme()) on [`Context`].
//! This can be used with the `include_str!()` macro to embed an external stylesheet file into the application binary when compiled.
//! Alternatively a constant string literal can be used to embed the CSS in the application.
//!
//! ```
//! # use vizia_core::prelude::*;
//! # let cx = &mut Context::default();
//!
//! const STYLE: &str = r#"
//!     .foo {
//!         background-color: red;
//!     }
//! "#;
//!
//! cx.add_theme(STYLE);
//!
//! Element::new(cx).class("foo");
//! ```
//!
//! To add an external css stylesheet which is read from a file at runtime, use [`add_stylesheet()`](crate::context::Context::add_stylesheet()) on [`Context`].
//! Stylesheets added this way can be hot-reloaded by pressing the F5 key in the application window.
//!
//! ```
//! # use vizia_core::prelude::*;
//! # let cx = &mut Context::default();
//!
//! cx.add_stylesheet("path/to/stylesheet.css");
//!
//! Element::new(cx).class("foo");
//! ```

use instant::Duration;
use morphorm::{LayoutType, PositionType, Units};
use std::collections::HashSet;
use std::fmt::Debug;
use vizia_id::GenerationalId;

use crate::prelude::*;

pub use vizia_style::{
    Angle, BackgroundImage, BackgroundSize, BorderCornerShape, BoxShadow, ClipPath, Color, CssRule,
    CursorIcon, Display, Filter, FontFamily, FontSize, FontStretch, FontStyle, FontWeight,
    FontWeightKeyword, GenericFontFamily, Gradient, Length, LengthOrPercentage, LengthValue,
    LineDirection, LinearGradient, Matrix, Opacity, Overflow, Scale, Transform, Transition,
    Translate, Visibility, RGBA,
};

use vizia_style::{EasingFunction, ParserOptions, Property, SelectorList, Selectors, StyleSheet};

mod rule;
pub(crate) use rule::Rule;

mod pseudoclass;
pub(crate) use pseudoclass::*;

mod transform;
pub(crate) use transform::*;

use crate::animation::{
    Animation, AnimationProperty, AnimationState, Interpolator, Keyframe, TimingFunction,
};
use crate::storage::animatable_set::AnimatableSet;
use crate::storage::style_set::StyleSet;
use bitflags::bitflags;
use cosmic_text::FamilyOwned;
use vizia_id::IdManager;
use vizia_storage::SparseSet;

bitflags! {
    /// Describes the capabilities of a view with respect to user interaction.
    #[derive(Debug, Clone, Copy)]
    pub(crate) struct Abilities: u8 {
        // Whether a view will be included in hit tests and receive mouse input events.
        const HOVERABLE = 1 << 0;
        // Whether a view can be focused to receive keyboard events.
        const FOCUSABLE = 1 << 1;
        // Whether a view can be checked.
        const CHECKABLE = 1 << 2;
        // Whether a view can be focused via keyboard navigation.
        const NAVIGABLE = 1 << 3;
        // Whether a view can be dragged during a drag and drop.
        const DRAGGABLE = 1 << 4;
    }
}

impl Default for Abilities {
    fn default() -> Abilities {
        Abilities::HOVERABLE
    }
}

bitflags! {
    pub struct SystemFlags: u8 {
        /// Style system flag.
        const RESTYLE = 1;
        /// Layout system flag.
        const RELAYOUT = 1 << 1;
        /// Draw system flag.
        const REDRAW = 1 << 2;
        /// Text constraints system flag.
        const REFLOW = 1 << 5;
    }
}

impl Default for SystemFlags {
    fn default() -> Self {
        SystemFlags::all()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ImageOrGradient {
    Image(String),
    Gradient(Gradient),
}

/// Stores the style properties of all entities in the application.
#[derive(Default)]
pub struct Style {
    pub(crate) rule_manager: IdManager<Rule>,

    // Creates and destroys animation ids
    pub(crate) animation_manager: IdManager<Animation>,

    // List of rules
    pub(crate) rules: Vec<(Rule, SelectorList<Selectors>)>,

    pub(crate) default_font: Vec<FamilyOwned>,

    // CSS Selector Properties
    pub(crate) ids: SparseSet<String>,
    pub(crate) classes: SparseSet<HashSet<String>>,
    pub(crate) pseudo_classes: SparseSet<PseudoClassFlags>,
    pub(crate) disabled: StyleSet<bool>,
    pub(crate) abilities: SparseSet<Abilities>,

    pub(crate) accesskit_node_classes: accesskit::NodeClassSet,

    // Accessibility Properties
    pub name: StyleSet<String>,
    pub role: SparseSet<Role>,
    pub default_action_verb: SparseSet<DefaultActionVerb>,
    pub live: SparseSet<Live>,
    pub labelled_by: SparseSet<Entity>,
    pub hidden: SparseSet<bool>,
    pub text_value: SparseSet<String>,
    pub numeric_value: SparseSet<f64>,

    // Display
    pub display: AnimatableSet<Display>,

    // Visibility
    pub visibility: StyleSet<Visibility>,

    // Opacity
    pub opacity: AnimatableSet<Opacity>,

    // Z Order
    pub z_index: StyleSet<i32>,

    // Clipping
    pub clip_path: AnimatableSet<ClipPath>,

    // Filters
    pub backdrop_filter: StyleSet<Filter>,

    // Transform
    pub transform: AnimatableSet<Vec<Transform>>,
    pub transform_origin: AnimatableSet<Translate>,
    pub translate: AnimatableSet<Translate>,
    pub rotate: AnimatableSet<Angle>,
    pub scale: AnimatableSet<Scale>,

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

    // Background
    pub background_color: AnimatableSet<Color>,
    pub background_image: AnimatableSet<Vec<ImageOrGradient>>,
    pub background_size: AnimatableSet<Vec<BackgroundSize>>,
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

    // cursor Icon
    pub cursor: StyleSet<CursorIcon>,

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

    // Child Spacing
    pub child_left: AnimatableSet<Units>,
    pub child_right: AnimatableSet<Units>,
    pub child_top: AnimatableSet<Units>,
    pub child_bottom: AnimatableSet<Units>,
    pub row_between: AnimatableSet<Units>,
    pub col_between: AnimatableSet<Units>,

    // Size
    pub width: AnimatableSet<Units>,
    pub height: AnimatableSet<Units>,

    // Size Constraints
    pub max_width: AnimatableSet<Units>,
    pub max_height: AnimatableSet<Units>,
    pub min_width: AnimatableSet<Units>,
    pub min_height: AnimatableSet<Units>,

    // Spacing Constraints
    pub min_left: AnimatableSet<Units>,
    pub max_left: AnimatableSet<Units>,
    pub min_right: AnimatableSet<Units>,
    pub max_right: AnimatableSet<Units>,
    pub min_top: AnimatableSet<Units>,
    pub max_top: AnimatableSet<Units>,
    pub min_bottom: AnimatableSet<Units>,
    pub max_bottom: AnimatableSet<Units>,

    pub(crate) system_flags: SystemFlags,

    // TODO: When we can do incremental updates on a per entity basis, change this to a bitflag
    // for layout, text layout, rendering, etc. to replace the above `needs_` members.
    pub needs_text_layout: SparseSet<bool>,

    pub needs_access_update: SparseSet<bool>,

    /// This includes both the system's HiDPI scaling factor as well as `cx.user_scale_factor`.
    pub(crate) dpi_factor: f64,
}

impl Style {
    pub fn scale_factor(&self) -> f32 {
        self.dpi_factor as f32
    }

    /// Function to convert logical points to physical pixels.
    pub fn logical_to_physical(&self, logical: f32) -> f32 {
        (logical * self.dpi_factor as f32).round()
    }

    /// Function to convert physical pixels to logical points.
    pub fn physical_to_logical(&self, physical: f32) -> f32 {
        physical / self.dpi_factor as f32
    }

    pub(crate) fn remove_rules(&mut self) {
        self.rule_manager.reset();
        self.animation_manager.reset();

        self.rules.clear();
    }

    pub(crate) fn add_animation(&mut self, animation: AnimationBuilder) -> Animation {
        let animation_id = self.animation_manager.create();

        fn add_keyframe<T: 'static + Interpolator + Debug + Clone + PartialEq + Default>(
            storage: &mut AnimatableSet<T>,
            animation_id: Animation,
            time: f32,
            value: T,
        ) {
            let keyframe = Keyframe { time, value, timing_function: TimingFunction::linear() };
            println!("{:?}", keyframe);
            if let Some(anim_state) = storage.get_animation_mut(animation_id) {
                anim_state.keyframes.push(keyframe)
            } else {
                let anim_state = AnimationState::new(animation_id).with_keyframe(keyframe);
                storage.insert_animation(animation_id, anim_state);
            }
        }

        for keyframe in animation.keyframes.iter() {
            for property in keyframe.properties.iter() {
                match property {
                    // SPACE
                    AnimationProperty::Left(value) => {
                        add_keyframe(&mut self.left, animation_id, keyframe.time, value.clone());
                    }

                    AnimationProperty::Right(value) => {
                        add_keyframe(&mut self.right, animation_id, keyframe.time, value.clone());
                    }

                    AnimationProperty::Top(value) => {
                        add_keyframe(&mut self.top, animation_id, keyframe.time, value.clone());
                    }

                    AnimationProperty::Bottom(value) => {
                        add_keyframe(&mut self.bottom, animation_id, keyframe.time, value.clone());
                    }

                    // SIZE
                    AnimationProperty::Width(value) => {
                        add_keyframe(&mut self.width, animation_id, keyframe.time, value.clone());
                    }

                    AnimationProperty::Height(value) => {
                        add_keyframe(&mut self.height, animation_id, keyframe.time, value.clone());
                    }

                    // CHILD SPACE
                    AnimationProperty::ChildLeft(value) => {
                        add_keyframe(
                            &mut self.child_left,
                            animation_id,
                            keyframe.time,
                            value.clone(),
                        );
                    }

                    AnimationProperty::ChildRight(value) => {
                        add_keyframe(
                            &mut self.child_right,
                            animation_id,
                            keyframe.time,
                            value.clone(),
                        );
                    }

                    AnimationProperty::ChildTop(value) => {
                        add_keyframe(
                            &mut self.child_top,
                            animation_id,
                            keyframe.time,
                            value.clone(),
                        );
                    }

                    AnimationProperty::ChildBottom(value) => {
                        add_keyframe(
                            &mut self.child_bottom,
                            animation_id,
                            keyframe.time,
                            value.clone(),
                        );
                    }

                    AnimationProperty::ColBetween(value) => {
                        add_keyframe(
                            &mut self.col_between,
                            animation_id,
                            keyframe.time,
                            value.clone(),
                        );
                    }

                    AnimationProperty::RowBetween(value) => {
                        add_keyframe(
                            &mut self.row_between,
                            animation_id,
                            keyframe.time,
                            value.clone(),
                        );
                    }

                    // TRANSFORM
                    AnimationProperty::Translate(value) => {
                        add_keyframe(
                            &mut self.translate,
                            animation_id,
                            keyframe.time,
                            value.clone(),
                        );
                    }

                    AnimationProperty::Rotate(value) => {
                        add_keyframe(&mut self.rotate, animation_id, keyframe.time, value.clone());
                    }

                    AnimationProperty::Scale(value) => {
                        add_keyframe(&mut self.scale, animation_id, keyframe.time, value.clone());
                    }

                    AnimationProperty::Opacity(value) => {
                        add_keyframe(&mut self.opacity, animation_id, keyframe.time, value.clone());
                    }
                }
            }
        }
        animation_id
    }

    pub(crate) fn play_animation(
        &mut self,
        entity: Entity,
        animation: Animation,
        duration: Duration,
    ) {
        self.left.play_animation(entity, animation, duration);
        self.right.play_animation(entity, animation, duration);
        self.top.play_animation(entity, animation, duration);
        self.bottom.play_animation(entity, animation, duration);

        self.width.play_animation(entity, animation, duration);
        self.height.play_animation(entity, animation, duration);

        self.child_left.play_animation(entity, animation, duration);
        self.child_right.play_animation(entity, animation, duration);
        self.child_top.play_animation(entity, animation, duration);
        self.child_bottom.play_animation(entity, animation, duration);
        self.col_between.play_animation(entity, animation, duration);
        self.row_between.play_animation(entity, animation, duration);

        self.translate.play_animation(entity, animation, duration);
        self.rotate.play_animation(entity, animation, duration);
        self.scale.play_animation(entity, animation, duration);

        self.opacity.play_animation(entity, animation, duration);
    }

    pub(crate) fn parse_theme(&mut self, stylesheet: &str) {
        if let Ok(theme) = StyleSheet::parse("test.css", stylesheet, ParserOptions::default()) {
            let rules = theme.rules.0;

            for rule in rules {
                match rule {
                    CssRule::Style(style_rule) => {
                        let rule_id = self.rule_manager.create();

                        let selectors = style_rule.selectors;

                        self.rules.push((rule_id, selectors));

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
    }

    fn insert_transition(&mut self, rule_id: Rule, transition: &Transition) {
        let animation = self.animation_manager.create();
        match transition.property.as_ref() {
            "opacity" => {
                self.opacity.insert_animation(animation, self.add_transition(transition));
                self.opacity.insert_transition(rule_id, animation);
            }

            "clip-path" => {
                self.clip_path.insert_animation(animation, self.add_transition(transition));
                self.clip_path.insert_transition(rule_id, animation);
            }

            "transform" => {
                self.transform.insert_animation(animation, self.add_transition(transition));
                self.transform.insert_transition(rule_id, animation);
            }

            "transform-origin" => {
                self.transform_origin.insert_animation(animation, self.add_transition(transition));
                self.transform_origin.insert_transition(rule_id, animation);
            }

            "translate" => {
                self.translate.insert_animation(animation, self.add_transition(transition));
                self.translate.insert_transition(rule_id, animation);
            }

            "rotate" => {
                self.rotate.insert_animation(animation, self.add_transition(transition));
                self.rotate.insert_transition(rule_id, animation);
            }

            "scale" => {
                self.scale.insert_animation(animation, self.add_transition(transition));
                self.scale.insert_transition(rule_id, animation);
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

            "background-color" => {
                self.background_color.insert_animation(animation, self.add_transition(transition));
                self.background_color.insert_transition(rule_id, animation);
            }

            "background-image" => {
                self.background_image.insert_animation(animation, self.add_transition(transition));
                self.background_image.insert_transition(rule_id, animation);
            }

            "background-size" => {
                self.background_size.insert_animation(animation, self.add_transition(transition));
                self.background_size.insert_transition(rule_id, animation);
            }

            "box-shadow" => {
                self.box_shadow.insert_animation(animation, self.add_transition(transition));
                self.box_shadow.insert_transition(rule_id, animation);
            }

            "color" => {
                self.font_color.insert_animation(animation, self.add_transition(transition));
                self.font_color.insert_transition(rule_id, animation);
            }

            "font-size" => {
                self.font_size.insert_animation(animation, self.add_transition(transition));
                self.font_size.insert_transition(rule_id, animation);
            }

            "caret-color" => {
                self.caret_color.insert_animation(animation, self.add_transition(transition));
                self.caret_color.insert_transition(rule_id, animation);
            }

            // TODO: Selection Color
            "left" => {
                self.left.insert_animation(animation, self.add_transition(transition));
                self.left.insert_transition(rule_id, animation);
            }

            "right" => {
                self.right.insert_animation(animation, self.add_transition(transition));
                self.right.insert_transition(rule_id, animation);
            }

            "top" => {
                self.top.insert_animation(animation, self.add_transition(transition));
                self.top.insert_transition(rule_id, animation);
            }

            "bottom" => {
                self.bottom.insert_animation(animation, self.add_transition(transition));
                self.bottom.insert_transition(rule_id, animation);
            }

            "child-left" => {
                self.child_left.insert_animation(animation, self.add_transition(transition));
                self.child_left.insert_transition(rule_id, animation);
            }

            "child-right" => {
                self.child_right.insert_animation(animation, self.add_transition(transition));
                self.child_right.insert_transition(rule_id, animation);
            }

            "child-top" => {
                self.child_top.insert_animation(animation, self.add_transition(transition));
                self.child_top.insert_transition(rule_id, animation);
            }

            "child-bottom" => {
                self.child_bottom.insert_animation(animation, self.add_transition(transition));
                self.child_bottom.insert_transition(rule_id, animation);
            }

            "width" => {
                self.width.insert_animation(animation, self.add_transition(transition));
                self.width.insert_transition(rule_id, animation);
            }

            "height" => {
                self.height.insert_animation(animation, self.add_transition(transition));
                self.height.insert_transition(rule_id, animation);
            }

            "max-width" => {
                self.max_width.insert_animation(animation, self.add_transition(transition));
                self.max_width.insert_transition(rule_id, animation);
            }

            "max-height" => {
                self.max_height.insert_animation(animation, self.add_transition(transition));
                self.max_height.insert_transition(rule_id, animation);
            }

            "min-width" => {
                self.min_width.insert_animation(animation, self.add_transition(transition));
                self.min_width.insert_transition(rule_id, animation);
            }

            "min-height" => {
                self.min_height.insert_animation(animation, self.add_transition(transition));
                self.min_height.insert_transition(rule_id, animation);
            }

            "min-left" => {
                self.min_left.insert_animation(animation, self.add_transition(transition));
                self.min_left.insert_transition(rule_id, animation);
            }

            "max-left" => {
                self.max_left.insert_animation(animation, self.add_transition(transition));
                self.max_left.insert_transition(rule_id, animation);
            }

            "min-right" => {
                self.min_right.insert_animation(animation, self.add_transition(transition));
                self.min_right.insert_transition(rule_id, animation);
            }

            "max-right" => {
                self.max_right.insert_animation(animation, self.add_transition(transition));
                self.max_right.insert_transition(rule_id, animation);
            }

            "min-top" => {
                self.min_top.insert_animation(animation, self.add_transition(transition));
                self.min_top.insert_transition(rule_id, animation);
            }

            "max-top" => {
                self.max_top.insert_animation(animation, self.add_transition(transition));
                self.max_top.insert_transition(rule_id, animation);
            }

            "min-bottom" => {
                self.min_bottom.insert_animation(animation, self.add_transition(transition));
                self.min_bottom.insert_transition(rule_id, animation);
            }

            "max-bottom" => {
                self.max_bottom.insert_animation(animation, self.add_transition(transition));
                self.max_bottom.insert_transition(rule_id, animation);
            }

            _ => {}
        }
    }

    fn insert_property(&mut self, rule_id: Rule, property: Property) {
        match property {
            // Display
            Property::Display(display) => {
                self.display.insert_rule(rule_id, display);
            }

            // Visibility
            Property::Visibility(visibility) => {
                self.visibility.insert_rule(rule_id, visibility);
            }

            // Opacity
            Property::Opacity(opacity) => {
                self.opacity.insert_rule(rule_id, opacity);
            }

            // Clipping
            Property::ClipPath(clip) => {
                self.clip_path.insert_rule(rule_id, clip);
            }

            // Filters
            Property::BackdropFilter(filter) => {
                self.backdrop_filter.insert_rule(rule_id, filter);
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

            // Space Constraints
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

            // Size Constraints
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

            // Background Colour
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

            // Border
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

            // Font Weight
            Property::FontWeight(font_weight) => {
                self.font_weight.insert_rule(rule_id, font_weight);
            }

            // Font Style
            Property::FontStyle(font_style) => {
                self.font_style.insert_rule(rule_id, font_style);
            }

            // Font Stretch
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

            Property::TransformOrigin(transform_origin) => {
                let x = transform_origin.x.to_length_or_percentage();
                let y = transform_origin.y.to_length_or_percentage();
                self.transform_origin.insert_rule(rule_id, Translate { x, y });
            }

            Property::Translate(translate) => {
                self.translate.insert_rule(rule_id, translate);
            }

            Property::Rotate(rotate) => {
                self.rotate.insert_rule(rule_id, rotate);
            }

            Property::Scale(scale) => {
                self.scale.insert_rule(rule_id, scale);
            }

            // Overflow
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

            // Z Index
            Property::ZIndex(z_index) => self.z_index.insert_rule(rule_id, z_index),

            // Outline
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

            // Background Images & Gradients
            Property::BackgroundImage(images) => {
                let images = images
                    .into_iter()
                    .filter_map(|img| match img {
                        BackgroundImage::None => None,
                        BackgroundImage::Gradient(gradient) => {
                            Some(ImageOrGradient::Gradient(*gradient))
                        }
                        BackgroundImage::Url(url) => {
                            Some(ImageOrGradient::Image(url.url.to_string()))
                        }
                    })
                    .collect::<Vec<_>>();

                self.background_image.insert_rule(rule_id, images);
            }

            // Background Size
            Property::BackgroundSize(sizes) => {
                self.background_size.insert_rule(rule_id, sizes);
            }

            // Text Wrapping
            Property::TextWrap(text_wrap) => {
                self.text_wrap.insert_rule(rule_id, text_wrap);
            }

            // Box Shadows
            Property::BoxShadow(box_shadows) => {
                self.box_shadow.insert_rule(rule_id, box_shadows);
            }

            // Cursor Icon
            Property::Cursor(cursor) => {
                self.cursor.insert_rule(rule_id, cursor);
            }

            // Unparsed. TODO: Log the error.
            Property::Unparsed(unparsed) => {
                println!("Unparsed: {}", unparsed.name);
            }

            // TODO: Custom property support
            Property::Custom(custom) => {
                println!("Custom Property: {}", custom.name);
            }

            _ => {}
        }
    }

    // Helper function for generating AnimationState from a transition definition.
    fn add_transition<T: Default + Interpolator>(
        &self,
        transition: &Transition,
    ) -> AnimationState<T> {
        let timing_function = transition
            .timing_function
            .map(|easing| match easing {
                EasingFunction::Linear => TimingFunction::linear(),
                EasingFunction::Ease => TimingFunction::ease(),
                EasingFunction::EaseIn => TimingFunction::ease_in(),
                EasingFunction::EaseOut => TimingFunction::ease_out(),
                EasingFunction::EaseInOut => TimingFunction::ease_in_out(),
                EasingFunction::CubicBezier(x1, y1, x2, y2) => TimingFunction::new(x1, y1, x2, y2),
            })
            .unwrap_or_default();

        AnimationState::new(Animation::null())
            .with_duration(transition.duration)
            .with_delay(transition.delay)
            .with_keyframe(Keyframe { time: 0.0, value: Default::default(), timing_function })
            .with_keyframe(Keyframe { time: 1.0, value: Default::default(), timing_function })
    }

    // Add style data for the given entity.
    pub fn add(&mut self, entity: Entity) {
        self.pseudo_classes.insert(entity, PseudoClassFlags::empty());
        self.classes.insert(entity, HashSet::new());
        self.abilities.insert(entity, Abilities::default());
        self.system_flags = SystemFlags::all();
    }

    // Remove style data for the given entity.
    pub fn remove(&mut self, entity: Entity) {
        self.ids.remove(entity);
        self.classes.remove(entity);
        self.pseudo_classes.remove(entity);
        self.disabled.remove(entity);
        self.abilities.remove(entity);

        self.role.remove(entity);
        self.default_action_verb.remove(entity);
        self.live.remove(entity);
        self.labelled_by.remove(entity);
        self.hidden.remove(entity);
        self.text_value.remove(entity);
        self.numeric_value.remove(entity);

        // Display
        self.display.remove(entity);
        // Visibility
        self.visibility.remove(entity);
        // Opacity
        self.opacity.remove(entity);
        // Z Order
        self.z_index.remove(entity);
        // Clipping
        self.clip_path.remove(entity);

        // Backdrop Filter
        self.backdrop_filter.remove(entity);

        // Transform
        self.transform.remove(entity);
        self.transform_origin.remove(entity);
        self.translate.remove(entity);
        self.rotate.remove(entity);
        self.scale.remove(entity);

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

        // Outline
        self.outline_width.remove(entity);
        self.outline_color.remove(entity);
        self.outline_offset.remove(entity);

        // Background
        self.background_color.remove(entity);
        self.background_image.remove(entity);
        self.background_size.remove(entity);

        // Box Shadow
        self.box_shadow.remove(entity);

        // Layout Type
        self.layout_type.remove(entity);

        // Position Type
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

        // Child Space
        self.child_left.remove(entity);
        self.child_right.remove(entity);
        self.child_top.remove(entity);
        self.child_bottom.remove(entity);
        self.col_between.remove(entity);
        self.row_between.remove(entity);

        // Text and Font
        self.text_wrap.remove(entity);
        self.font_family.remove(entity);
        self.font_weight.remove(entity);
        self.font_style.remove(entity);
        self.font_color.remove(entity);
        self.font_size.remove(entity);
        self.selection_color.remove(entity);
        self.caret_color.remove(entity);

        // Cursor
        self.cursor.remove(entity);

        self.name.remove(entity);

        self.needs_text_layout.remove(entity);
        self.needs_access_update.remove(entity);
    }

    pub fn needs_restyle(&mut self) {
        self.system_flags.set(SystemFlags::RESTYLE, true);
    }

    pub fn needs_relayout(&mut self) {
        self.system_flags.set(SystemFlags::RELAYOUT, true);
    }

    pub fn needs_redraw(&mut self) {
        self.system_flags.set(SystemFlags::REDRAW, true);
    }

    pub fn needs_access_update(&mut self, entity: Entity) {
        self.needs_access_update.insert(entity, true);
    }

    pub fn should_redraw<F: FnOnce()>(&mut self, f: F) {
        if self.system_flags.contains(SystemFlags::REDRAW) {
            f();
            self.system_flags.set(SystemFlags::REDRAW, false);
        }
    }

    // Remove all shared style data.
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

        // Clipping
        self.clip_path.clear_rules();

        // Backdrop Filer
        self.backdrop_filter.clear_rules();

        // Transform
        self.transform.clear_rules();
        self.transform_origin.clear_rules();
        self.translate.clear_rules();
        self.rotate.clear_rules();
        self.scale.clear_rules();

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
        self.background_size.clear_rules();

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

        // Child Space
        self.child_left.clear_rules();
        self.child_right.clear_rules();
        self.child_top.clear_rules();
        self.child_bottom.clear_rules();
        self.col_between.clear_rules();
        self.row_between.clear_rules();

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
    }
}
