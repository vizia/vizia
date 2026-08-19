use vizia::{
    icons::{ICON_BOLT, ICON_HEART, ICON_STAR},
    prelude::*,
};

use crate::components::{GalleryThrobber, Skeleton, SkeletonVariant, ThrobberVariant};

const DOC_RUNTIME_TARGET_ID: &str = "animation-doc-runtime-target";
const DOC_RUNTIME_DURATION: f32 = 6.0;

#[derive(Clone, Copy, PartialEq, Eq)]
enum ExampleKind {
    Motion,
    Duration,
    Delay,
    Steps,
    Iterations,
    Direction,
    FillMode,
    Paused,
    Composition,
    DocumentTimeline,
    ScrollTimeline,
    ViewTimeline,
    PercentageKeyframes,
    Multiple,
    Opacity,
    Transform,
    TransformTranslateOnly,
    TransformRotateOnly,
    TransformCombined,
    TransformOrigin,
    Translate,
    Rotate,
    Scale,
    ClipPath,
    Filter,
    BackdropFilter,
    BackgroundColor,
    BackgroundGeometry,
    BackgroundPosition,
    BackgroundSize,
    BackgroundRepeat,
    BorderWidth,
    BorderWidthTop,
    BorderWidthRight,
    BorderWidthBottom,
    BorderWidthLeft,
    BorderWidthAll,
    BorderColor,
    BorderColorTop,
    BorderColorRight,
    BorderColorBottom,
    BorderColorLeft,
    BorderColorAll,
    CornerRadius,
    CornerRadiusTopLeft,
    CornerRadiusTopRight,
    CornerRadiusBottomLeft,
    CornerRadiusBottomRight,
    CornerRadiusAll,
    Outline,
    Shadow,
    TextColor,
    FontSize,
    LetterSpacing,
    LineHeight,
    TextPaint,
    Fill,
    Position,
    Padding,
    Gap,
    Size,
    Constraints,
    Display,
    StaticInfo,
    Runtime,
    Popover,
    Throbber,
    ThrobberDots,
    ThrobberRing,
    ThrobberBars,
    ThrobberOrbit,
    ThrobberPulse,
    ThrobberSpinner,
    ThrobberGrid,
    ThrobberBounce,
    ThrobberRipple,
    ThrobberEqualizer,
    Skeleton,
    SkeletonText,
    SkeletonCard,
    SkeletonProfile,
    SkeletonList,
    SkeletonTable,
    SkeletonCircle,
    SkeletonRectangle,
    SkeletonButton,
    SkeletonArticle,
    SkeletonMedia,
    SvgMorph,
}

struct DocEntry {
    category: &'static str,
    title: &'static str,
    css: &'static str,
    description: &'static str,
    example: ExampleKind,
}

#[derive(Clone, Copy)]
struct DocVariant {
    label: &'static str,
    css: &'static str,
    example: ExampleKind,
}

// Documentation identifiers intentionally remain in English because they are also
// used to select the matching CSS example. Only their presentation is localized.
fn doc_title_key(title: &str) -> &'static str {
    match title {
        "animation" => "animation-doc-title-animation",
        "animation-name" => "animation-doc-title-animation-name",
        "animation-duration" => "animation-doc-title-animation-duration",
        "animation-delay" => "animation-doc-title-animation-delay",
        "animation-timing-function" => "animation-doc-title-animation-timing-function",
        "steps() / step-start / step-end" => "animation-doc-title-steps",
        "animation-iteration-count" => "animation-doc-title-animation-iteration-count",
        "animation-direction" => "animation-doc-title-animation-direction",
        "animation-fill-mode" => "animation-doc-title-animation-fill-mode",
        "animation-play-state" => "animation-doc-title-animation-play-state",
        "animation-composition" => "animation-doc-title-animation-composition",
        "animation-timeline" => "animation-doc-title-animation-timeline",
        "@keyframes" => "animation-doc-title-keyframes",
        "percentage keyframes" => "animation-doc-title-percentage-keyframes",
        "multiple animations" => "animation-doc-title-multiple-animations",
        "document timeline" => "animation-doc-title-document-timeline",
        "named scroll timeline" => "animation-doc-title-named-scroll-timeline",
        "scroll()" => "animation-doc-title-scroll",
        "view()" => "animation-doc-title-view",
        "opacity" => "animation-doc-title-opacity",
        "transform" => "animation-doc-title-transform",
        "transform-origin" => "animation-doc-title-transform-origin",
        "translate" => "animation-doc-title-translate",
        "rotate" => "animation-doc-title-rotate",
        "scale" => "animation-doc-title-scale",
        "clip-path" => "animation-doc-title-clip-path",
        "filter" => "animation-doc-title-filter",
        "backdrop-filter" => "animation-doc-title-backdrop-filter",
        "background-color" => "animation-doc-title-background-color",
        "background-image / position / repeat / size" => "animation-doc-title-background-family",
        "border-top/right/bottom/left-width" => "animation-doc-title-border-width",
        "border-top/right/bottom/left-color" => "animation-doc-title-border-color",
        "corner-*-radius / smoothing" => "animation-doc-title-corner-radius",
        "outline-width / color / offset" => "animation-doc-title-outline",
        "shadow" => "animation-doc-title-shadow",
        "color" => "animation-doc-title-color",
        "font-size" => "animation-doc-title-font-size",
        "letter-spacing" => "animation-doc-title-letter-spacing",
        "line-height" => "animation-doc-title-line-height",
        "caret / selection / text-decoration color" => "animation-doc-title-text-paint",
        "fill" => "animation-doc-title-fill",
        "left / right / top / bottom" => "animation-doc-title-position",
        "padding-left/right/top/bottom" => "animation-doc-title-padding",
        "horizontal-gap / vertical-gap" => "animation-doc-title-gap",
        "width / height" => "animation-doc-title-size",
        "min/max width/height + min/max gaps" => "animation-doc-title-constraints",
        "display" => "animation-doc-title-display",
        "typed custom properties" => "animation-doc-title-custom-properties",
        "runtime controls" => "animation-doc-title-runtime-controls",
        "blur reveal popover" => "animation-doc-title-blur-popover",
        "throbber variants" => "animation-doc-title-throbber",
        "skeleton variants" => "animation-doc-title-skeleton",
        "SVG icon morph sequence" => "animation-doc-title-svg-morph",
        _ => "animation-doc-title-unknown",
    }
}

fn doc_description_key(title: &str) -> &'static str {
    match title {
        "animation" => "animation-doc-description-animation",
        "animation-name" => "animation-doc-description-animation-name",
        "animation-duration" => "animation-doc-description-animation-duration",
        "animation-delay" => "animation-doc-description-animation-delay",
        "animation-timing-function" => "animation-doc-description-timing-function",
        "steps() / step-start / step-end" => "animation-doc-description-steps",
        "animation-iteration-count" => "animation-doc-description-iteration-count",
        "animation-direction" => "animation-doc-description-direction",
        "animation-fill-mode" => "animation-doc-description-fill-mode",
        "animation-play-state" => "animation-doc-description-play-state",
        "animation-composition" => "animation-doc-description-composition",
        "animation-timeline" => "animation-doc-description-timeline",
        "@keyframes" => "animation-doc-description-keyframes",
        "percentage keyframes" => "animation-doc-description-percentage-keyframes",
        "multiple animations" => "animation-doc-description-multiple",
        "document timeline" => "animation-doc-description-document-timeline",
        "named scroll timeline" => "animation-doc-description-named-scroll",
        "scroll()" => "animation-doc-description-scroll",
        "view()" => "animation-doc-description-view",
        "opacity" => "animation-doc-description-opacity",
        "transform" => "animation-doc-description-transform",
        "transform-origin" => "animation-doc-description-transform-origin",
        "translate" => "animation-doc-description-translate",
        "rotate" => "animation-doc-description-rotate",
        "scale" => "animation-doc-description-scale",
        "clip-path" => "animation-doc-description-clip-path",
        "filter" => "animation-doc-description-filter",
        "backdrop-filter" => "animation-doc-description-backdrop-filter",
        "background-color" => "animation-doc-description-background-color",
        "background-image / position / repeat / size" => {
            "animation-doc-description-background-family"
        }
        "border-top/right/bottom/left-width" => "animation-doc-description-border-width",
        "border-top/right/bottom/left-color" => "animation-doc-description-border-color",
        "corner-*-radius / smoothing" => "animation-doc-description-corner-radius",
        "outline-width / color / offset" => "animation-doc-description-outline",
        "shadow" => "animation-doc-description-shadow",
        "color" => "animation-doc-description-color",
        "font-size" => "animation-doc-description-font-size",
        "letter-spacing" => "animation-doc-description-letter-spacing",
        "line-height" => "animation-doc-description-line-height",
        "caret / selection / text-decoration color" => "animation-doc-description-text-paint",
        "fill" => "animation-doc-description-fill",
        "left / right / top / bottom" => "animation-doc-description-position",
        "padding-left/right/top/bottom" => "animation-doc-description-padding",
        "horizontal-gap / vertical-gap" => "animation-doc-description-gap",
        "width / height" => "animation-doc-description-size",
        "min/max width/height + min/max gaps" => "animation-doc-description-constraints",
        "display" => "animation-doc-description-display",
        "typed custom properties" => "animation-doc-description-custom-properties",
        "runtime controls" => "animation-doc-description-runtime-controls",
        "blur reveal popover" => "animation-doc-description-blur-popover",
        "throbber variants" => "animation-doc-description-throbber",
        "skeleton variants" => "animation-doc-description-skeleton",
        "SVG icon morph sequence" => "animation-doc-description-svg-morph",
        _ => "animation-doc-description-unknown",
    }
}

fn animation_category_key(category: &str) -> &'static str {
    match category {
        "Animation properties" => "animation-category-properties",
        "Keyframes & timelines" => "animation-category-keyframes",
        "Animated values" => "animation-category-values",
        "Runtime & UI patterns" => "animation-category-runtime",
        _ => "animation-category-unknown",
    }
}

fn animation_variant_key(label: &str) -> &'static str {
    match label {
        "All" => "all",
        "Translate" => "animation-variant-translate",
        "Rotate" => "animation-variant-rotate",
        "Dots" => "animation-variant-dots",
        "Ring" => "animation-variant-ring",
        "Bars" => "animation-variant-bars",
        "Orbit" => "animation-variant-orbit",
        "Pulse" => "animation-variant-pulse",
        "Spinner" => "animation-variant-spinner",
        "Grid" => "animation-variant-grid",
        "Bounce" => "animation-variant-bounce",
        "Ripple" => "animation-variant-ripple",
        "Equalizer" => "animation-variant-equalizer",
        "Text" => "animation-variant-text",
        "Card" => "animation-variant-card",
        "Profile" => "animation-variant-profile",
        "List" => "animation-variant-list",
        "Table" => "animation-variant-table",
        "Circle" => "animation-variant-circle",
        "Rectangle" => "animation-variant-rectangle",
        "Button" => "animation-variant-button",
        "Article" => "animation-variant-article",
        "Media" => "animation-variant-media",
        "Position" => "animation-preview-position",
        "Size" => "animation-preview-size",
        "Repeat" => "animation-preview-repeat",
        "Top" => "animation-preview-top",
        "Right" => "animation-preview-right",
        "Bottom" => "animation-preview-bottom",
        "Left" => "animation-preview-left",
        "Top left" => "animation-preview-top-left",
        "Top right" => "animation-preview-top-right",
        "Bottom left" => "animation-preview-bottom-left",
        "Bottom right" => "animation-preview-bottom-right",
        _ => "animation-variant-unknown",
    }
}

fn preview_label_key(label: &str) -> &'static str {
    match label {
        "CONTENT" => "animation-preview-content",
        "WIDTH × HEIGHT" => "animation-preview-width-height",
        "MIN ↔ MAX" => "animation-preview-min-max",
        "The blue box alternates between flex and none" => "animation-preview-display-hint",
        "DISPLAY: FLEX" => "animation-preview-display-flex",
        "Animated underline" => "animation-preview-animated-underline",
        "Focus the field to inspect caret and selection colors" => {
            "animation-preview-text-paint-hint"
        }
        "Scroll inside this result" => "animation-preview-scroll-hint",
        "SCROLL TIMELINE" => "animation-preview-scroll-timeline",
        "VIEW TIMELINE" => "animation-preview-view-timeline",
        "approach" => "animation-preview-approach",
        "depart" => "animation-preview-depart",
        "End" => "animation-preview-end",
        "RUNTIME" => "animation-preview-runtime",
        "Pause" => "animation-preview-pause",
        "Resume" => "animation-preview-resume",
        "Reverse" => "animation-preview-reverse",
        "Seek" => "animation-preview-seek",
        "Open popover" => "animation-preview-open-popover",
        "Blur reveal" => "animation-preview-blur-reveal",
        "Native Popover + CSS @keyframes" => "animation-preview-native-popover",
        "Close" => "animation-preview-close",
        "SHARP CONTENT" => "animation-preview-sharp-content",
        "BACKDROP FILTER" => "animation-preview-backdrop-filter",
        "glass blur over moving content" => "animation-preview-glass-copy",
        "First line\nSecond line" => "animation-preview-line-height",
        "ANIMATED VARS" => "animation-preview-animated-vars",
        "CSS" => "animation-ui-css",
        "Copy CSS" => "animation-ui-copy-css",
        "Result" => "animation-ui-result",
        "Implementation note" => "animation-ui-implementation-note",
        "Animation reference" => "animation-ui-reference",
        "CSS Animations" => "animation-ui-title",
        "FORWARDS" => "animation-preview-forwards",
        "ADD" => "animation-preview-add",
        "opacity" => "animation-preview-opacity",
        "transform" => "animation-preview-transform",
        "translateX" => "animation-preview-translate-x",
        "rotate" => "animation-preview-rotate",
        "combined" => "animation-preview-combined",
        "origin" => "animation-preview-origin",
        "translate" => "animation-preview-translate",
        "scale" => "animation-preview-scale",
        "clip-path" => "animation-preview-clip-path",
        "FILTER" => "animation-preview-filter",
        "background" => "animation-preview-background",
        "All backgrounds" => "animation-preview-all-backgrounds",
        "Position" => "animation-preview-position",
        "Size" => "animation-preview-size",
        "Repeat" => "animation-preview-repeat",
        "Border width" => "animation-preview-border-width",
        "Top" => "animation-preview-top",
        "Right" => "animation-preview-right",
        "Bottom" => "animation-preview-bottom",
        "Left" => "animation-preview-left",
        "All sides" => "animation-preview-all-sides",
        "Border color" => "animation-preview-border-color",
        "Radius" => "animation-preview-radius",
        "Top left" => "animation-preview-top-left",
        "Top right" => "animation-preview-top-right",
        "Bottom left" => "animation-preview-bottom-left",
        "Bottom right" => "animation-preview-bottom-right",
        "All corners" => "animation-preview-all-corners",
        "Outline" => "animation-preview-outline",
        "Shadow" => "animation-preview-shadow",
        "Animated text" => "animation-preview-animated-text",
        "Font size" => "animation-preview-font-size",
        "Spacing" => "animation-preview-spacing",
        _ => "animation-preview-label-unknown",
    }
}

mod animated_values;
mod animation_properties;
mod keyframes_timelines;
mod runtime_ui_patterns;

const DOC_GROUPS: &[(&str, &[DocEntry])] = &[
    ("Animation properties", animation_properties::DOCS),
    ("Keyframes & timelines", keyframes_timelines::DOCS),
    ("Animated values", animated_values::DOCS),
    ("Runtime & UI patterns", runtime_ui_patterns::DOCS),
];

fn docs() -> impl Iterator<Item = &'static DocEntry> {
    DOC_GROUPS.iter().flat_map(|(_, entries)| entries.iter())
}

fn doc_at(index: usize) -> &'static DocEntry {
    docs().nth(index).expect("animation documentation index should be valid")
}

fn selected_variant<'a>(
    doc: &'a DocEntry,
    variants: &'a [DocVariant],
    index: usize,
) -> (&'a str, ExampleKind) {
    variants
        .get(index)
        .map(|variant| (variant.css, variant.example))
        .unwrap_or((doc.css, doc.example))
}

fn default_variant_index(variants: &[DocVariant]) -> usize {
    variants.iter().position(|variant| variant.label == "All").unwrap_or(0)
}

fn variants(doc: &DocEntry) -> &'static [DocVariant] {
    let variants = animated_values::variants(doc.title);
    if variants.is_empty() { runtime_ui_patterns::variants(doc.title) } else { variants }
}

fn render_variant_picker(
    cx: &mut Context,
    variants: &'static [DocVariant],
    selected: Signal<usize>,
) {
    if variants.len() < 2 {
        return;
    }

    ButtonGroup::new(cx, move |cx| {
        for (index, variant) in variants.iter().enumerate() {
            ToggleButton::new(cx, selected.map(move |current| *current == index), move |cx| {
                Label::new(cx, Localized::new(animation_variant_key(variant.label)))
                    .hoverable(false)
            })
            .on_toggle(move |_cx| selected.set(index));
        }
    })
    .class("animation-doc-variant-tabs");
}

fn runtime_animation_id(cx: &EventContext) -> Option<CssAnimationId> {
    let entity = cx.resolve_entity_identifier(DOC_RUNTIME_TARGET_ID)?;
    cx.css_animations(entity).into_iter().next().map(|snapshot| snapshot.id)
}

fn runtime_snapshot_text(cx: &EventContext) -> String {
    let Some(entity) = cx.resolve_entity_identifier(DOC_RUNTIME_TARGET_ID) else {
        return Localized::new("animation-runtime-not-mounted").get_value(cx);
    };
    let Some(snapshot) = cx.css_animations(entity).into_iter().next() else {
        return Localized::new("animation-runtime-no-occurrence").get_value(cx);
    };
    let state = Localized::new(match snapshot.state {
        CssAnimationPlaybackState::Pending => "animation-runtime-state-pending",
        CssAnimationPlaybackState::Running => "animation-runtime-state-running",
        CssAnimationPlaybackState::Paused => "animation-runtime-state-paused",
        CssAnimationPlaybackState::Finished => "animation-runtime-state-finished",
    })
    .get_value(cx);
    let progress = snapshot
        .progress
        .map(|value| format!("{:.0}%", value * 100.0))
        .unwrap_or_else(|| "—".to_string());
    format!(
        "id={}  ·  {}  ·  {:.2}s  ·  {}  ·  {:.2}×",
        snapshot.id.get(),
        state,
        snapshot.current_time,
        progress,
        snapshot.playback_rate
    )
}

fn simple_stage(cx: &mut Context, label: &'static str, class: &'static str) {
    HStack::new(cx, move |cx| {
        Label::new(cx, Localized::new(preview_label_key(label)))
            .class("animation-doc-target")
            .class(class);
    })
    .class("animation-doc-stage")
    .alignment(Alignment::Center);
}

fn surface_stage(cx: &mut Context, label: &'static str, class: &'static str) {
    HStack::new(cx, move |cx| {
        Card::new(cx, move |cx| {
            CardContent::new(cx, move |cx| {
                Label::new(cx, Localized::new(preview_label_key(label)))
                    .class("animation-doc-surface-label")
                    .hoverable(false);
            })
            .class("animation-doc-surface-content");
        })
        .class("animation-doc-surface-target")
        .class(class);
    })
    .class("animation-doc-stage")
    .alignment(Alignment::Center);
}

fn shape_stage(cx: &mut Context, label: &'static str, class: &'static str) {
    ZStack::new(cx, move |cx| {
        Element::new(cx).class("animation-doc-shape-target").class(class);
        Label::new(cx, Localized::new(preview_label_key(label)))
            .class("animation-doc-shape-label")
            .hoverable(false);
    })
    .class("animation-doc-stage")
    .alignment(Alignment::Center);
}

fn radius_stage(cx: &mut Context, label: &'static str, class: &'static str) {
    ZStack::new(cx, move |cx| {
        Element::new(cx).class("animation-doc-shape-target").class(class);
        Label::new(cx, Localized::new(preview_label_key(label)))
            .class("animation-doc-radius-label")
            .hoverable(false);
    })
    .class("animation-doc-stage")
    .alignment(Alignment::Center);
}

fn text_stage(cx: &mut Context, label: &'static str, class: &'static str) {
    HStack::new(cx, move |cx| {
        Label::new(cx, Localized::new(preview_label_key(label)))
            .class("animation-doc-text-target")
            .class(class)
            .hoverable(false);
    })
    .class("animation-doc-stage")
    .alignment(Alignment::Center);
}

fn render_padding_preview(cx: &mut Context) {
    ZStack::new(cx, |cx| {
        VStack::new(cx, |cx| {
            Label::new(cx, Localized::new("animation-preview-content"))
                .class("animation-doc-padding-child")
                .hoverable(false);
        })
        .class("animation-doc-padding-surface")
        .class("animation-doc-padding");
    })
    .class("animation-doc-padding-shell");
}

fn render_size_preview(cx: &mut Context) {
    ZStack::new(cx, |cx| {
        ZStack::new(cx, |cx| {
            Label::new(cx, Localized::new("animation-preview-width-height"))
                .class("animation-doc-size-label")
                .hoverable(false);
        })
        .class("animation-doc-size-target")
        .class("animation-doc-size");
    })
    .class("animation-doc-stage")
    .alignment(Alignment::Center);
}

fn render_constraints_preview(cx: &mut Context) {
    ZStack::new(cx, |cx| {
        ZStack::new(cx, |cx| {
            Label::new(cx, Localized::new("animation-preview-min-max"))
                .class("animation-doc-constraints-label")
                .hoverable(false);
        })
        .class("animation-doc-constraints-target")
        .class("animation-doc-constraints");
    })
    .class("animation-doc-constraints-track")
    .alignment(Alignment::Center);
}

fn render_display_preview(cx: &mut Context) {
    VStack::new(cx, |cx| {
        Label::new(cx, Localized::new("animation-preview-display-hint"))
            .class("animation-doc-display-hint")
            .hoverable(false);
        ZStack::new(cx, |cx| {
            Label::new(cx, Localized::new("animation-preview-display-flex")).hoverable(false);
        })
        .class("animation-doc-display")
        .width(Pixels(220.0))
        .height(Pixels(100.0))
        .min_width(Pixels(220.0))
        .min_height(Pixels(100.0))
        .background_color(Color::rgb(37, 99, 235))
        .color(Color::white());
    })
    .class("animation-doc-stage")
    .class("animation-doc-display-stage")
    .alignment(Alignment::Center);
}

fn render_text_paint_preview(cx: &mut Context) {
    let value = Signal::new("Select this text".to_string());
    VStack::new(cx, move |cx| {
        Label::new(cx, Localized::new("animation-preview-animated-underline"))
            .class("animation-doc-text-paint-label")
            .class("animation-doc-text-paint")
            .hoverable(false);
        Textbox::new(cx, value)
            .on_edit(move |_cx, text| value.set(text))
            .class("animation-doc-text-paint-field")
            .class("animation-doc-text-paint");
        Label::new(cx, Localized::new("animation-preview-text-paint-hint"))
            .class("animation-doc-text-paint-hint")
            .hoverable(false);
    })
    .class("animation-doc-stage")
    .class("animation-doc-text-paint-stage")
    .alignment(Alignment::Center);
}

fn motion_track(cx: &mut Context, class: &'static str) {
    ZStack::new(cx, move |cx| {
        Element::new(cx).class("animation-doc-track-line");
        Element::new(cx).class("animation-doc-dot").class(class);
    })
    .class("animation-doc-track");
}

fn render_scroll_preview(cx: &mut Context) {
    ScrollView::new(cx, |cx| {
        VStack::new(cx, |cx| {
            Label::new(cx, Localized::new("animation-preview-scroll-hint"))
                .class("animation-doc-hint");
            Element::new(cx).class("animation-doc-scroll-spacer");
            Label::new(cx, Localized::new("animation-preview-scroll-timeline"))
                .class("animation-doc-scroll-subject");
            Element::new(cx).class("animation-doc-scroll-spacer");
            Label::new(cx, Localized::new("animation-preview-end")).class("animation-doc-hint");
        })
        .class("animation-doc-scroll-content");
    })
    .timeline_name("--docs-scroll")
    .show_horizontal_scrollbar(false)
    .show_vertical_scrollbar(false)
    .class("animation-doc-scrollview");
}

fn render_view_preview(cx: &mut Context) {
    ScrollView::new(cx, |cx| {
        VStack::new(cx, |cx| {
            Label::new(cx, Localized::new("animation-preview-approach"))
                .class("animation-doc-hint");
            Element::new(cx).class("animation-doc-view-spacer");
            Label::new(cx, Localized::new("animation-preview-view-timeline"))
                .class("animation-doc-view-subject");
            Element::new(cx).class("animation-doc-view-spacer");
            Label::new(cx, Localized::new("animation-preview-depart")).class("animation-doc-hint");
        })
        .class("animation-doc-scroll-content");
    })
    .show_horizontal_scrollbar(false)
    .show_vertical_scrollbar(false)
    .class("animation-doc-scrollview");
}

fn render_runtime_preview(cx: &mut Context) {
    let readout =
        Signal::new("Use the controls, then inspect the stable occurrence snapshot.".to_string());
    let seek = Signal::new(0.0_f32);
    VStack::new(cx, move |cx| {
        HStack::new(cx, |cx| {
            Label::new(cx, Localized::new("animation-preview-runtime"))
                .id(DOC_RUNTIME_TARGET_ID)
                .class("animation-doc-target")
                .class("animation-doc-runtime-target");
        })
        .class("animation-doc-stage")
        .alignment(Alignment::Center);
        HStack::new(cx, move |cx| {
            Button::new(cx, |cx| Label::new(cx, Localized::new("animation-preview-pause")))
                .on_press(move |cx| {
                    if let Some(id) = runtime_animation_id(cx) {
                        let _ = cx.pause_css_animation(id);
                    }
                    readout.set(runtime_snapshot_text(cx));
                });
            Button::new(cx, |cx| Label::new(cx, Localized::new("animation-preview-resume")))
                .variant(ButtonVariant::Secondary)
                .on_press(move |cx| {
                    if let Some(id) = runtime_animation_id(cx) {
                        let _ = cx.resume_css_animation(id);
                    }
                    readout.set(runtime_snapshot_text(cx));
                });
            Button::new(cx, |cx| Label::new(cx, Localized::new("animation-preview-reverse")))
                .variant(ButtonVariant::Outline)
                .on_press(move |cx| {
                    if let Some(id) = runtime_animation_id(cx) {
                        let _ = cx.reverse_css_animation(id);
                    }
                    readout.set(runtime_snapshot_text(cx));
                });
            Button::new(cx, |cx| Label::new(cx, Localized::new("animation-preview-speed-2x")))
                .variant(ButtonVariant::Text)
                .on_press(move |cx| {
                    if let Some(id) = runtime_animation_id(cx) {
                        let _ = cx.set_css_animation_playback_rate(id, 2.0);
                    }
                    readout.set(runtime_snapshot_text(cx));
                });
        })
        .class("animation-doc-controls");
        HStack::new(cx, move |cx| {
            Label::new(cx, Localized::new("animation-preview-seek"))
                .class("animation-doc-control-label");
            Slider::new(cx, seek)
                .on_change(move |cx, value| {
                    seek.set(value);
                    if let Some(id) = runtime_animation_id(cx) {
                        let _ = cx.seek_css_animation(id, value * DOC_RUNTIME_DURATION);
                    }
                    readout.set(runtime_snapshot_text(cx));
                })
                .width(Stretch(1.0));
        })
        .class("animation-doc-seek-row");
        Label::new(cx, readout).class("animation-doc-runtime-readout");
    })
    .class("animation-doc-runtime");
}

fn render_popover_preview(cx: &mut Context) {
    let open = Signal::new(false);
    HStack::new(cx, move |cx| {
        Button::new(cx, |cx| Label::new(cx, Localized::new("animation-preview-open-popover")))
            .on_press(move |_cx| open.set(true));
        Binding::new(cx, open, move |cx| {
            if open.get() {
                Popover::new(cx, move |cx| {
                    VStack::new(cx, |cx| {
                        Label::new(cx, Localized::new("animation-preview-blur-reveal"))
                            .class("animation-doc-popover-title");
                        Label::new(cx, Localized::new("animation-preview-native-popover"))
                            .class("animation-doc-popover-copy");
                        Button::new(cx, |cx| {
                            Label::new(cx, Localized::new("animation-preview-close"))
                        })
                        .variant(ButtonVariant::Secondary)
                        .on_press(move |_cx| open.set(false));
                    })
                    .class("animation-doc-popover-content");
                })
                .class("animation-doc-popover")
                .on_blur(move |_cx| open.set(false))
                .placement(Placement::BottomStart)
                .show_arrow(false);
            }
        });
    })
    .class("animation-doc-stage")
    .alignment(Alignment::Center);
}

fn throbber_variant(cx: &mut Context, variant: ThrobberVariant) {
    GalleryThrobber::new(cx, variant);
}

fn render_throbber_preview(cx: &mut Context, variant: Option<ThrobberVariant>) {
    if let Some(variant) = variant {
        ZStack::new(cx, move |cx| throbber_variant(cx, variant))
            .class("animation-doc-stage")
            .alignment(Alignment::Center);
        return;
    }

    HStack::new(cx, |cx| {
        for (label, variant) in [
            ("Dots", ThrobberVariant::Dots),
            ("Ring", ThrobberVariant::Ring),
            ("Bars", ThrobberVariant::Bars),
            ("Orbit", ThrobberVariant::Orbit),
            ("Pulse", ThrobberVariant::Pulse),
            ("Spinner", ThrobberVariant::Spinner),
            ("Grid", ThrobberVariant::Grid),
            ("Bounce", ThrobberVariant::Bounce),
            ("Ripple", ThrobberVariant::Ripple),
            ("EQ", ThrobberVariant::Equalizer),
        ] {
            VStack::new(cx, move |cx| {
                GalleryThrobber::new(cx, variant);
                Label::new(cx, Localized::new(animation_variant_key(label)))
                    .class("animation-pattern-label")
                    .hoverable(false);
            })
            .class("animation-pattern-tile");
        }
    })
    .class("animation-doc-stage")
    .class("animation-pattern-gallery")
    .alignment(Alignment::Center);
}

fn render_skeleton_preview(cx: &mut Context, variant: Option<SkeletonVariant>) {
    if let Some(variant) = variant {
        ZStack::new(cx, move |cx| {
            Skeleton::new(cx, variant);
        })
        .class("animation-doc-stage")
        .alignment(Alignment::Center);
        return;
    }

    HStack::new(cx, |cx| {
        for (label, variant) in [
            ("Text", SkeletonVariant::Text),
            ("Card", SkeletonVariant::Card),
            ("Profile", SkeletonVariant::Profile),
            ("List", SkeletonVariant::List),
            ("Table", SkeletonVariant::Table),
            ("Circle", SkeletonVariant::Circle),
            ("Rect", SkeletonVariant::Rectangle),
            ("Button", SkeletonVariant::Button),
            ("Article", SkeletonVariant::Article),
            ("Media", SkeletonVariant::Media),
        ] {
            VStack::new(cx, move |cx| {
                Skeleton::new(cx, variant);
                Label::new(cx, Localized::new(animation_variant_key(label)))
                    .class("animation-pattern-label")
                    .hoverable(false);
            })
            .class("animation-pattern-tile")
            .class("animation-skeleton-tile");
        }
    })
    .class("animation-doc-stage")
    .class("animation-pattern-gallery")
    .alignment(Alignment::Center);
}

fn render_svg_morph_preview(cx: &mut Context) {
    ZStack::new(cx, |cx| {
        Svg::new(cx, ICON_HEART).class("animation-morph-icon").class("morph-heart");
        Svg::new(cx, ICON_STAR).class("animation-morph-icon").class("morph-star");
        Svg::new(cx, ICON_BOLT).class("animation-morph-icon").class("morph-bolt");
    })
    .class("animation-doc-stage")
    .class("animation-morph-stage")
    .alignment(Alignment::Center);
}

fn render_preview(cx: &mut Context, example: ExampleKind) {
    match example {
        ExampleKind::Motion => motion_track(cx, "animation-doc-motion"),
        ExampleKind::Duration => motion_track(cx, "animation-doc-duration"),
        ExampleKind::Delay => motion_track(cx, "animation-doc-delay"),
        ExampleKind::Steps => motion_track(cx, "animation-doc-steps"),
        ExampleKind::Iterations => motion_track(cx, "animation-doc-iterations"),
        ExampleKind::Direction => motion_track(cx, "animation-doc-direction"),
        ExampleKind::FillMode => simple_stage(cx, "FORWARDS", "animation-doc-fill-mode"),
        ExampleKind::Paused => motion_track(cx, "animation-doc-paused"),
        ExampleKind::Composition => simple_stage(cx, "ADD", "animation-doc-composition"),
        ExampleKind::DocumentTimeline => motion_track(cx, "animation-doc-document"),
        ExampleKind::ScrollTimeline => render_scroll_preview(cx),
        ExampleKind::ViewTimeline => render_view_preview(cx),
        ExampleKind::PercentageKeyframes => motion_track(cx, "animation-doc-percentage"),
        ExampleKind::Multiple => motion_track(cx, "animation-doc-multiple"),
        ExampleKind::Opacity => simple_stage(cx, "opacity", "animation-doc-opacity"),
        ExampleKind::Transform => surface_stage(cx, "transform", "animation-doc-transform"),
        ExampleKind::TransformTranslateOnly => {
            surface_stage(cx, "translateX", "animation-doc-transform-translate-only")
        }
        ExampleKind::TransformRotateOnly => {
            surface_stage(cx, "rotate", "animation-doc-transform-rotate-only")
        }
        ExampleKind::TransformCombined => {
            surface_stage(cx, "combined", "animation-doc-transform-combined")
        }
        ExampleKind::TransformOrigin => {
            surface_stage(cx, "origin", "animation-doc-transform-origin")
        }
        ExampleKind::Translate => surface_stage(cx, "translate", "animation-doc-translate"),
        ExampleKind::Rotate => surface_stage(cx, "rotate", "animation-doc-rotate"),
        ExampleKind::Scale => surface_stage(cx, "scale", "animation-doc-scale"),
        ExampleKind::ClipPath => surface_stage(cx, "clip-path", "animation-doc-clip"),
        ExampleKind::Filter => surface_stage(cx, "FILTER", "animation-doc-filter"),
        ExampleKind::BackdropFilter => {
            ZStack::new(cx, |cx| {
                Element::new(cx)
                    .class("animation-doc-backdrop-blob")
                    .class("animation-doc-backdrop-a");
                Element::new(cx)
                    .class("animation-doc-backdrop-blob")
                    .class("animation-doc-backdrop-b");
                Element::new(cx)
                    .class("animation-doc-backdrop-blob")
                    .class("animation-doc-backdrop-c");
                Label::new(cx, Localized::new("animation-preview-sharp-content"))
                    .class("animation-doc-backdrop-behind-text");
                VStack::new(cx, |cx| {
                    Label::new(cx, Localized::new("animation-preview-backdrop-filter"))
                        .class("animation-doc-backdrop-title");
                    Label::new(cx, Localized::new("animation-preview-glass-copy"))
                        .class("animation-doc-backdrop-copy");
                })
                .class("animation-doc-backdrop-glass");
            })
            .class("animation-doc-backdrop-stage");
        }
        ExampleKind::BackgroundColor => {
            surface_stage(cx, "background", "animation-doc-background-color")
        }
        ExampleKind::BackgroundGeometry => {
            shape_stage(cx, "All backgrounds", "animation-doc-bg-all")
        }
        ExampleKind::BackgroundPosition => shape_stage(cx, "Position", "animation-doc-bg-position"),
        ExampleKind::BackgroundSize => shape_stage(cx, "Size", "animation-doc-bg-size"),
        ExampleKind::BackgroundRepeat => shape_stage(cx, "Repeat", "animation-doc-bg-repeat"),
        ExampleKind::BorderWidth => {
            shape_stage(cx, "Border width", "animation-doc-border-width-top")
        }
        ExampleKind::BorderWidthTop => shape_stage(cx, "Top", "animation-doc-border-width-top"),
        ExampleKind::BorderWidthRight => {
            shape_stage(cx, "Right", "animation-doc-border-width-right")
        }
        ExampleKind::BorderWidthBottom => {
            shape_stage(cx, "Bottom", "animation-doc-border-width-bottom")
        }
        ExampleKind::BorderWidthLeft => shape_stage(cx, "Left", "animation-doc-border-width-left"),
        ExampleKind::BorderWidthAll => {
            shape_stage(cx, "All sides", "animation-doc-border-width-all")
        }
        ExampleKind::BorderColor => {
            shape_stage(cx, "Border color", "animation-doc-border-color-top")
        }
        ExampleKind::BorderColorTop => shape_stage(cx, "Top", "animation-doc-border-color-top"),
        ExampleKind::BorderColorRight => {
            shape_stage(cx, "Right", "animation-doc-border-color-right")
        }
        ExampleKind::BorderColorBottom => {
            shape_stage(cx, "Bottom", "animation-doc-border-color-bottom")
        }
        ExampleKind::BorderColorLeft => shape_stage(cx, "Left", "animation-doc-border-color-left"),
        ExampleKind::BorderColorAll => {
            shape_stage(cx, "All sides", "animation-doc-border-color-all")
        }
        ExampleKind::CornerRadius => radius_stage(cx, "Radius", "animation-doc-radius-all"),
        ExampleKind::CornerRadiusTopLeft => radius_stage(cx, "Top left", "animation-doc-radius-tl"),
        ExampleKind::CornerRadiusTopRight => {
            radius_stage(cx, "Top right", "animation-doc-radius-tr")
        }
        ExampleKind::CornerRadiusBottomLeft => {
            radius_stage(cx, "Bottom left", "animation-doc-radius-bl")
        }
        ExampleKind::CornerRadiusBottomRight => {
            radius_stage(cx, "Bottom right", "animation-doc-radius-br")
        }
        ExampleKind::CornerRadiusAll => radius_stage(cx, "All corners", "animation-doc-radius-all"),
        ExampleKind::Outline => shape_stage(cx, "Outline", "animation-doc-outline"),
        ExampleKind::Shadow => shape_stage(cx, "Shadow", "animation-doc-shadow"),
        ExampleKind::TextColor => text_stage(cx, "Animated text", "animation-doc-text-color"),
        ExampleKind::FontSize => text_stage(cx, "Font size", "animation-doc-font-size"),
        ExampleKind::LetterSpacing => text_stage(cx, "Spacing", "animation-doc-letter-spacing"),
        ExampleKind::LineHeight => {
            VStack::new(cx, |cx| {
                Label::new(cx, Localized::new("animation-preview-line-height"))
                    .class("animation-doc-line-height");
            })
            .class("animation-doc-stage")
            .alignment(Alignment::Center);
        }
        ExampleKind::TextPaint => render_text_paint_preview(cx),
        ExampleKind::Fill => {
            HStack::new(cx, |cx| {
                Svg::new(cx, ICON_HEART).class("animation-doc-fill-icon");
            })
            .class("animation-doc-stage")
            .alignment(Alignment::Center);
        }
        ExampleKind::Position => {
            ZStack::new(cx, |cx| {
                Element::new(cx).class("animation-doc-position");
            })
            .class("animation-doc-stage");
        }
        ExampleKind::Padding => render_padding_preview(cx),
        ExampleKind::Gap => {
            HStack::new(cx, |cx| {
                Element::new(cx).class("animation-doc-gap-item");
                Element::new(cx).class("animation-doc-gap-item");
                Element::new(cx).class("animation-doc-gap-item");
            })
            .class("animation-doc-stage")
            .class("animation-doc-gap-stage")
            .alignment(Alignment::Center);
        }
        ExampleKind::Size => render_size_preview(cx),
        ExampleKind::Constraints => render_constraints_preview(cx),
        ExampleKind::Display => render_display_preview(cx),
        ExampleKind::StaticInfo => {
            ZStack::new(cx, |cx| {
                Label::new(cx, Localized::new("animation-preview-animated-vars"))
                    .class("animation-doc-custom-label")
                    .hoverable(false);
            })
            .class("animation-doc-stage")
            .class("animation-doc-custom-target")
            .alignment(Alignment::Center);
        }
        ExampleKind::Runtime => render_runtime_preview(cx),
        ExampleKind::Popover => render_popover_preview(cx),
        ExampleKind::Throbber => render_throbber_preview(cx, None),
        ExampleKind::ThrobberDots => render_throbber_preview(cx, Some(ThrobberVariant::Dots)),
        ExampleKind::ThrobberRing => render_throbber_preview(cx, Some(ThrobberVariant::Ring)),
        ExampleKind::ThrobberBars => render_throbber_preview(cx, Some(ThrobberVariant::Bars)),
        ExampleKind::ThrobberOrbit => render_throbber_preview(cx, Some(ThrobberVariant::Orbit)),
        ExampleKind::ThrobberPulse => render_throbber_preview(cx, Some(ThrobberVariant::Pulse)),
        ExampleKind::ThrobberSpinner => render_throbber_preview(cx, Some(ThrobberVariant::Spinner)),
        ExampleKind::ThrobberGrid => render_throbber_preview(cx, Some(ThrobberVariant::Grid)),
        ExampleKind::ThrobberBounce => render_throbber_preview(cx, Some(ThrobberVariant::Bounce)),
        ExampleKind::ThrobberRipple => render_throbber_preview(cx, Some(ThrobberVariant::Ripple)),
        ExampleKind::ThrobberEqualizer => {
            render_throbber_preview(cx, Some(ThrobberVariant::Equalizer))
        }
        ExampleKind::Skeleton => render_skeleton_preview(cx, None),
        ExampleKind::SkeletonText => render_skeleton_preview(cx, Some(SkeletonVariant::Text)),
        ExampleKind::SkeletonCard => render_skeleton_preview(cx, Some(SkeletonVariant::Card)),
        ExampleKind::SkeletonProfile => render_skeleton_preview(cx, Some(SkeletonVariant::Profile)),
        ExampleKind::SkeletonList => render_skeleton_preview(cx, Some(SkeletonVariant::List)),
        ExampleKind::SkeletonTable => render_skeleton_preview(cx, Some(SkeletonVariant::Table)),
        ExampleKind::SkeletonCircle => render_skeleton_preview(cx, Some(SkeletonVariant::Circle)),
        ExampleKind::SkeletonRectangle => {
            render_skeleton_preview(cx, Some(SkeletonVariant::Rectangle))
        }
        ExampleKind::SkeletonButton => render_skeleton_preview(cx, Some(SkeletonVariant::Button)),
        ExampleKind::SkeletonArticle => render_skeleton_preview(cx, Some(SkeletonVariant::Article)),
        ExampleKind::SkeletonMedia => render_skeleton_preview(cx, Some(SkeletonVariant::Media)),
        ExampleKind::SvgMorph => render_svg_morph_preview(cx),
    }
}

fn render_doc_content(cx: &mut Context, index: usize) {
    let doc = doc_at(index);
    // Keep the original English metadata available beside the localized presentation.
    let _source_description = doc.description;
    let variants = variants(doc);
    let selected_variant_index = Signal::new(default_variant_index(variants));

    VStack::new(cx, move |cx| {
        Label::new(cx, Localized::new(doc_title_key(doc.title))).class("animation-doc-title");
        Label::new(cx, Localized::new(doc_description_key(doc.title)))
            .class("animation-doc-description");

        VStack::new(cx, move |cx| {
            VStack::new(cx, move |cx| {
                HStack::new(cx, move |cx| {
                    Label::new(cx, Localized::new("animation-ui-css"))
                        .class("animation-doc-pane-title");
                    Element::new(cx).width(Stretch(1.0));
                    render_variant_picker(cx, variants, selected_variant_index);
                })
                .class("animation-doc-code-header");

                Binding::new(cx, selected_variant_index, move |cx| {
                    let (css, _) = selected_variant(doc, variants, selected_variant_index.get());
                    VStack::new(cx, move |cx| {
                        ScrollView::new(cx, move |cx| {
                            Label::new(cx, css)
                                .class("animation-doc-code")
                                .text_wrap(true)
                                .hoverable(false);
                        })
                        .show_horizontal_scrollbar(false)
                        .class("animation-doc-code-scroll");
                        HStack::new(cx, move |cx| {
                            Element::new(cx).width(Stretch(1.0));
                            Button::new(cx, |cx| {
                                Label::new(cx, Localized::new("animation-ui-copy-css"))
                            })
                            .variant(ButtonVariant::Outline)
                            .class("animation-doc-copy-button")
                            .on_press(move |cx| {
                                let (css, _) =
                                    selected_variant(doc, variants, selected_variant_index.get());
                                let _ = cx.set_clipboard(css.to_string());
                            });
                        })
                        .class("animation-doc-copy-row");
                    })
                    .class("animation-doc-code-body")
                    .alignment(Alignment::TopLeft);
                });
            })
            .class("animation-doc-code-pane");

            VStack::new(cx, move |cx| {
                HStack::new(cx, |cx| {
                    Label::new(cx, Localized::new("animation-ui-result"))
                        .class("animation-doc-pane-title");
                })
                .class("animation-doc-result-header");

                Binding::new(cx, selected_variant_index, move |cx| {
                    let (_, example) =
                        selected_variant(doc, variants, selected_variant_index.get());
                    VStack::new(cx, move |cx| render_preview(cx, example))
                        .class("animation-doc-result-body");
                });
            })
            .class("animation-doc-result-pane");
        })
        .class("animation-doc-example");

        VStack::new(cx, |cx| {
            Label::new(cx, Localized::new("animation-ui-implementation-note"))
                .class("animation-doc-note-title");
            Label::new(cx, Localized::new("animation-ui-implementation-copy"))
                .class("animation-doc-note-copy");
        })
        .class("animation-doc-note");
    })
    .class("animation-doc-content");
}

fn build_doc_navigation(cx: &mut Context, selected: Signal<usize>, height: Signal<Units>) {
    ScrollView::new(cx, move |cx| {
        VStack::new(cx, move |cx| {
            Label::new(cx, Localized::new("animation-ui-reference"))
                .class("animation-doc-nav-title");
            let mut index = 0;
            for (category, entries) in DOC_GROUPS {
                debug_assert!(entries.iter().all(|doc| doc.category == *category));
                Label::new(cx, Localized::new(animation_category_key(category)))
                    .class("animation-doc-nav-category");
                for doc in *entries {
                    let item_index = index;
                    Button::new(cx, move |cx| {
                        Label::new(cx, Localized::new(doc_title_key(doc.title)))
                            .class("animation-doc-nav-item-label")
                            .hoverable(false)
                    })
                    .variant(ButtonVariant::Text)
                    .class("animation-doc-nav-item")
                    .toggle_class(
                        "animation-doc-nav-item-active",
                        selected.map(move |current| *current == item_index),
                    )
                    .tooltip(move |cx| {
                        Tooltip::new(cx, move |cx| {
                            Label::new(cx, Localized::new(doc_title_key(doc.title)));
                        })
                        .placement(Placement::Right)
                    })
                    .on_press(move |_cx| selected.set(item_index));
                    index += 1;
                }
            }
        })
        .class("animation-doc-nav");
    })
    .height(height)
    .min_height(height)
    .max_height(height)
    .show_horizontal_scrollbar(false)
    .class("animation-doc-nav-scroll");
}

pub fn animation(cx: &mut Context, navigation_height: Signal<Units>) {
    let selected = Signal::new(0_usize);
    VStack::new(cx, move |cx| {
        VStack::new(cx, |cx| {
            Label::new(cx, Localized::new("animation-ui-title")).class("panel-title");
            Label::new(cx, Localized::new("animation-ui-description")).class("panel-description");
        })
        .class("animation-doc-intro");
        Divider::new(cx);
        HStack::new(cx, move |cx| {
            build_doc_navigation(cx, selected, navigation_height);
            Binding::new(cx, selected, move |cx| {
                render_doc_content(cx, selected.get());
            });
        })
        .class("animation-doc-layout")
        .alignment(Alignment::TopLeft);
    })
    .class("animation-doc-page");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn animation_gallery_stylesheets_load() {
        let mut cx = Context::default();
        for stylesheet in [
            include_style!("resources/themes/animation/layout.css"),
            include_style!("resources/themes/animation/animation_properties.css"),
            include_style!("resources/themes/animation/keyframes_timelines.css"),
            include_style!("resources/themes/animation/animated_values.css"),
            include_style!("resources/themes/animation/runtime_ui_patterns.css"),
        ] {
            cx.add_stylesheet(stylesheet).expect("animation gallery stylesheet should parse");
        }
    }

    #[test]
    fn every_variant_family_defaults_to_all() {
        for doc in docs() {
            let variants = variants(doc);
            if variants.len() > 1 {
                assert_eq!(variants[default_variant_index(variants)].label, "All", "{}", doc.title);
            }
        }
    }

    #[test]
    fn animation_gallery_translations_load_and_cover_docs() {
        let mut cx = Context::default();
        for (locale, source) in [
            ("en-US", include_str!("../../resources/translations/en-US/helper.ftl")),
            ("fr", include_str!("../../resources/translations/fr/helper.ftl")),
            ("ar", include_str!("../../resources/translations/ar/helper.ftl")),
        ] {
            cx.load_translation(locale.parse().unwrap(), source)
                .unwrap_or_else(|error| panic!("invalid {locale} translation: {error}"));
        }

        for (category, entries) in DOC_GROUPS {
            assert_ne!(animation_category_key(category), "animation-category-unknown");
            for doc in *entries {
                assert_ne!(
                    doc_title_key(doc.title),
                    "animation-doc-title-unknown",
                    "{}",
                    doc.title
                );
                assert_ne!(
                    doc_description_key(doc.title),
                    "animation-doc-description-unknown",
                    "{}",
                    doc.title
                );
                for variant in variants(doc) {
                    assert_ne!(
                        animation_variant_key(variant.label),
                        "animation-variant-unknown",
                        "{} / {}",
                        doc.title,
                        variant.label
                    );
                }
            }
        }
    }
}
