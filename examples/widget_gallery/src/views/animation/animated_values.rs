use super::{DocEntry, DocVariant, ExampleKind};

pub(super) const DOCS: &[DocEntry] = &[
    DocEntry {
        category: "Animated values",
        title: "opacity",
        css: r#"@keyframes demo {
  from { opacity: .15; }
  to   { opacity: 1; }
}

.target { animation: demo 1.4s ease-in-out infinite alternate; }"#,
        description: "Paint-only interpolation. It does not require layout or text reconstruction.",
        example: ExampleKind::Opacity,
    },
    DocEntry {
        category: "Animated values",
        title: "transform",
        css: r#"@keyframes demo {
  from { transform: translateX(-90px) rotate(-25deg); }
  to   { transform: translateX(90px) rotate(335deg); }
}

.target { animation: demo 1.8s ease-in-out infinite alternate; }"#,
        description: "Transform lists are sampled through the retransform path rather than layout. Compatible functions interpolate component-by-component.",
        example: ExampleKind::Transform,
    },
    DocEntry {
        category: "Animated values",
        title: "transform-origin",
        css: r#"@keyframes demo {
  from { transform-origin: 0% 50%; rotate: -35deg; }
  to   { transform-origin: 100% 50%; rotate: 35deg; }
}"#,
        description: "Transform origin is store-backed and participates in the same animation clock as transforms.",
        example: ExampleKind::TransformOrigin,
    },
    DocEntry {
        category: "Animated values",
        title: "translate",
        css: r#"@keyframes demo {
  from { translate: -110px 0; }
  to   { translate: 110px 0; }
}"#,
        description: "Individual translate is independently animatable and supports additive effect composition.",
        example: ExampleKind::Translate,
    },
    DocEntry {
        category: "Animated values",
        title: "rotate",
        css: r#"@keyframes demo {
  from { rotate: -30deg; }
  to   { rotate: 330deg; }
}"#,
        description: "Individual rotate is independently animatable and supports add/accumulate composition.",
        example: ExampleKind::Rotate,
    },
    DocEntry {
        category: "Animated values",
        title: "scale",
        css: r#"@keyframes demo {
  from { scale: .65; }
  to   { scale: 1.25; }
}"#,
        description: "Individual scale is independently animatable and participates in transform invalidation.",
        example: ExampleKind::Scale,
    },
    DocEntry {
        category: "Animated values",
        title: "clip-path",
        css: r#"@keyframes demo {
  from { clip-path: inset(18px); }
  to   { clip-path: inset(0px); }
}"#,
        description: "Clip path changes use the reclip path. Incompatible shapes fall back deterministically.",
        example: ExampleKind::ClipPath,
    },
    DocEntry {
        category: "Animated values",
        title: "filter",
        css: r#"@keyframes demo {
  from { filter: blur(0px); }
  50%  { filter: blur(10px); }
  to   { filter: blur(0px); }
}"#,
        description: "Compatible filter lists interpolate; incompatible lists use discrete fallback.",
        example: ExampleKind::Filter,
    },
    DocEntry {
        category: "Animated values",
        title: "backdrop-filter",
        css: r#"@keyframes glass {
  0%, 100% { backdrop-filter: blur(0px); }
  50%      { backdrop-filter: blur(16px); }
}

.glass { animation: glass 1.8s ease-in-out infinite; }"#,
        description: "The glass surface changes blur while colored content keeps moving underneath, so the backdrop effect is visually distinct from a normal filter.",
        example: ExampleKind::BackdropFilter,
    },
    DocEntry {
        category: "Animated values",
        title: "background-color",
        css: r#"@keyframes demo {
  from { background-color: #3b82f6; }
  to   { background-color: #8b5cf6; }
}"#,
        description: "Background colors interpolate as paint-only values.",
        example: ExampleKind::BackgroundColor,
    },
    DocEntry {
        category: "Animated values",
        title: "background-image / position / repeat / size",
        css: r#"@keyframes demo {
  from { background-position: 0% 50%; }
  to   { background-position: 100% 50%; }
}"#,
        description: "The All tab combines the background stores; the remaining tabs isolate position, size and discrete repeat behavior.",
        example: ExampleKind::BackgroundGeometry,
    },
    DocEntry {
        category: "Animated values",
        title: "border-top/right/bottom/left-width",
        css: r#"@keyframes demo {
  from { border-top-width: 2px; }
  to   { border-top-width: 24px; }
}"#,
        description: "Choose a side (or all four) to see exactly which border-width store is being animated.",
        example: ExampleKind::BorderWidth,
    },
    DocEntry {
        category: "Animated values",
        title: "border-top/right/bottom/left-color",
        css: r#"@keyframes demo {
  from { border-top-color: #3b82f6; }
  to   { border-top-color: #f97316; }
}"#,
        description: "Choose a side (or all four) to isolate border-color interpolation.",
        example: ExampleKind::BorderColor,
    },
    DocEntry {
        category: "Animated values",
        title: "corner-*-radius / smoothing",
        css: r#"@keyframes demo {
  from { corner-top-left-radius: 4px; }
  to   { corner-top-left-radius: 90px; }
}"#,
        description: "Choose a corner or all corners. The preview uses a large stable surface so the geometry change is obvious.",
        example: ExampleKind::CornerRadius,
    },
    DocEntry {
        category: "Animated values",
        title: "outline-width / color / offset",
        css: r#"@keyframes demo {
  from {
    outline-width: 1px;
    outline-color: #3b82f6;
    outline-offset: 1px;
  }
  to {
    outline-width: 6px;
    outline-color: #a855f7;
    outline-offset: 8px;
  }
}"#,
        description: "Outline width, color and offset are all backed by animatable stores.",
        example: ExampleKind::Outline,
    },
    DocEntry {
        category: "Animated values",
        title: "shadow",
        css: r#"@keyframes demo {
  from { shadow: 0px 4px 10px #553b82f6; }
  to   { shadow: 0px 22px 46px #998b5cf6; }
}"#,
        description: "Compatible shadow lists interpolate and redraw without relayout.",
        example: ExampleKind::Shadow,
    },
    DocEntry {
        category: "Animated values",
        title: "color",
        css: r#"@keyframes demo {
  from { color: #3b82f6; }
  to   { color: #f43f5e; }
}"#,
        description: "Font color is a paint-only text property and does not reconstruct text layout.",
        example: ExampleKind::TextColor,
    },
    DocEntry {
        category: "Animated values",
        title: "font-size",
        css: r#"@keyframes demo {
  from { font-size: 20px; }
  to   { font-size: 42px; }
}"#,
        description: "Font size animation requests text reconstruction because glyph metrics change.",
        example: ExampleKind::FontSize,
    },
    DocEntry {
        category: "Animated values",
        title: "letter-spacing",
        css: r#"@keyframes demo {
  from { letter-spacing: 0px; }
  to   { letter-spacing: 8px; }
}"#,
        description: "Letter spacing is reflow-affecting and uses the text construction path.",
        example: ExampleKind::LetterSpacing,
    },
    DocEntry {
        category: "Animated values",
        title: "line-height",
        css: r#"@keyframes demo {
  from { line-height: 1; }
  to   { line-height: 1.8; }
}"#,
        description: "Line height is reflow-affecting and shares the animation clock with other text properties.",
        example: ExampleKind::LineHeight,
    },
    DocEntry {
        category: "Animated values",
        title: "caret / selection / text-decoration color",
        css: r#"@keyframes demo {
  from {
    caret-color: #3b82f6;
    selection-color: #3b82f666;
    text-decoration-color: #3b82f6;
  }
  to {
    caret-color: #f43f5e;
    selection-color: #f43f5e66;
    text-decoration-color: #f43f5e;
  }
}"#,
        description: "These text paint stores are animatable without text reconstruction.",
        example: ExampleKind::TextPaint,
    },
    DocEntry {
        category: "Animated values",
        title: "fill",
        css: r#"@keyframes demo {
  from { fill: #3b82f6; }
  to   { fill: #22c55e; }
}"#,
        description: "The fill color store is animatable and redraw-only. The preview uses a real SVG.",
        example: ExampleKind::Fill,
    },
    DocEntry {
        category: "Animated values",
        title: "left / right / top / bottom",
        css: r#"@keyframes demo {
  from { left: 12px; top: 8px; }
  to   { left: 110px; top: 36px; }
}"#,
        description: "Position offsets are layout-affecting. The runtime only requests relayout when the sampled value actually changes.",
        example: ExampleKind::Position,
    },
    DocEntry {
        category: "Animated values",
        title: "padding-left/right/top/bottom",
        css: r#"@keyframes demo {
  from { padding: 8px; }
  to   { padding: 56px; }
}"#,
        description: "All four padding stores are animatable and layout-affecting.",
        example: ExampleKind::Padding,
    },
    DocEntry {
        category: "Animated values",
        title: "horizontal-gap / vertical-gap",
        css: r#"@keyframes demo {
  from { horizontal-gap: 6px; }
  to   { horizontal-gap: 44px; }
}"#,
        description: "Row/column gaps are animatable layout values and participate in change-aware relayout ticking.",
        example: ExampleKind::Gap,
    },
    DocEntry {
        category: "Animated values",
        title: "width / height",
        css: r#"@keyframes demo {
  from { width: 120px; height: 90px; }
  to   { width: 280px; height: 170px; }
}

.target {
  animation: demo 1.6s steps(8, end) infinite alternate;
}"#,
        description: "Width and height support keyframe interpolation, including the existing auto-size animation resolution path.",
        example: ExampleKind::Size,
    },
    DocEntry {
        category: "Animated values",
        title: "min/max width/height + min/max gaps",
        css: r#"@keyframes demo {
  from { min-width: 120px; max-width: 170px; }
  to   { min-width: 220px; max-width: 320px; }
}

.target { width: 100%; animation: demo 1.6s steps(8, end) infinite alternate; }"#,
        description: "Constraint stores and min/max horizontal/vertical gap stores participate in CSS animation playback.",
        example: ExampleKind::Constraints,
    },
    DocEntry {
        category: "Animated values",
        title: "display",
        css: r#"@keyframes demo {
  0%, 64% { display: flex; }
  65%, 100% { display: none; }
}

.target { animation: demo 2.4s linear infinite; }"#,
        description: "Display is store-backed but fundamentally discrete rather than numerically interpolated.",
        example: ExampleKind::Display,
    },
    DocEntry {
        category: "Animated values",
        title: "typed custom properties",
        css: r#"@keyframes demo {
  from {
    --accent: #3b82f6;
    --size: 150px;
    --fade: .45;
  }
  to {
    --accent: #a855f7;
    --size: 300px;
    --fade: 1;
  }
}

.target {
  background-color: var(--accent);
  width: var(--size);
  opacity: var(--fade);
  animation: demo 1.8s ease-in-out infinite alternate;
}"#,
        description: "Typed Vizia custom property families use the same animation/composition machinery as built-in properties.",
        example: ExampleKind::StaticInfo,
    },
];

const TRANSFORM_VARIANTS: &[DocVariant] = &[
    DocVariant {
        label: "Translate",
        css: r#"@keyframes demo {
  from { transform: translateX(-120px); }
  to   { transform: translateX(120px); }
}

.target { animation: demo 1.5s ease-in-out infinite alternate; }"#,
        example: ExampleKind::TransformTranslateOnly,
    },
    DocVariant {
        label: "Rotate",
        css: r#"@keyframes demo {
  from { transform: rotate(-35deg); }
  to   { transform: rotate(325deg); }
}

.target { animation: demo 1.8s linear infinite; }"#,
        example: ExampleKind::TransformRotateOnly,
    },
    DocVariant {
        label: "All",
        css: r#"@keyframes demo {
  from { transform: translate(-100px, 0px) rotate(-25deg) scale(.78, .78); }
  to   { transform: translate(100px, 0px) rotate(335deg) scale(1.12, 1.12); }
}

.target { animation: demo 1.8s ease-in-out infinite alternate; }"#,
        example: ExampleKind::TransformCombined,
    },
];

const BACKGROUND_VARIANTS: &[DocVariant] = &[
    DocVariant {
        label: "All",
        css: r#"@keyframes demo {
  from {
    background-position: 0% 50%;
    background-size: 30% 55%;
    background-repeat: no-repeat;
  }
  to {
    background-position: 100% 50%;
    background-size: 100% 100%;
    background-repeat: repeat;
  }
}

.target {
  background-image: url("vizia.png");
  animation: demo 1.8s ease-in-out infinite alternate;
}"#,
        example: ExampleKind::BackgroundGeometry,
    },
    DocVariant {
        label: "Position",
        css: r#"@keyframes demo {
  from { background-position: 0% 50%; }
  to   { background-position: 100% 50%; }
}

.target {
  background-image: url("vizia.png");
  background-size: 48% 82%;
  background-repeat: no-repeat;
  animation: demo 1.6s ease-in-out infinite alternate;
}"#,
        example: ExampleKind::BackgroundPosition,
    },
    DocVariant {
        label: "Size",
        css: r#"@keyframes demo {
  from { background-size: 30% 55%; }
  to   { background-size: 100% 100%; }
}

.target {
  background-image: url("vizia.png");
  background-position: center;
  background-repeat: no-repeat;
  animation: demo 1.6s ease-in-out infinite alternate;
}"#,
        example: ExampleKind::BackgroundSize,
    },
    DocVariant {
        label: "Repeat",
        css: r#"@keyframes demo {
  0%, 45%   { background-repeat: no-repeat; }
  55%, 100% { background-repeat: repeat; }
}

.target {
  background-image: url("vizia.png");
  background-size: 56px 56px;
  animation: demo 1.8s steps(1, end) infinite alternate;
}"#,
        example: ExampleKind::BackgroundRepeat,
    },
];

const BORDER_WIDTH_VARIANTS: &[DocVariant] = &[
    DocVariant {
        label: "Top",
        css: r#"@keyframes demo {
  from { border-top-width: 2px; }
  to   { border-top-width: 24px; }
}
.target { animation: demo 1.4s ease-in-out infinite alternate; }"#,
        example: ExampleKind::BorderWidthTop,
    },
    DocVariant {
        label: "Right",
        css: r#"@keyframes demo {
  from { border-right-width: 2px; }
  to   { border-right-width: 24px; }
}
.target { animation: demo 1.4s ease-in-out infinite alternate; }"#,
        example: ExampleKind::BorderWidthRight,
    },
    DocVariant {
        label: "Bottom",
        css: r#"@keyframes demo {
  from { border-bottom-width: 2px; }
  to   { border-bottom-width: 24px; }
}
.target { animation: demo 1.4s ease-in-out infinite alternate; }"#,
        example: ExampleKind::BorderWidthBottom,
    },
    DocVariant {
        label: "Left",
        css: r#"@keyframes demo {
  from { border-left-width: 2px; }
  to   { border-left-width: 24px; }
}
.target { animation: demo 1.4s ease-in-out infinite alternate; }"#,
        example: ExampleKind::BorderWidthLeft,
    },
    DocVariant {
        label: "All",
        css: r#"@keyframes demo {
  from {
    border-top-width: 2px; border-right-width: 2px;
    border-bottom-width: 2px; border-left-width: 2px;
  }
  to {
    border-top-width: 18px; border-right-width: 18px;
    border-bottom-width: 18px; border-left-width: 18px;
  }
}
.target { animation: demo 1.4s ease-in-out infinite alternate; }"#,
        example: ExampleKind::BorderWidthAll,
    },
];

const BORDER_COLOR_VARIANTS: &[DocVariant] = &[
    DocVariant {
        label: "Top",
        css: r#"@keyframes demo {
  from { border-top-color: #3b82f6; }
  to   { border-top-color: #f97316; }
}"#,
        example: ExampleKind::BorderColorTop,
    },
    DocVariant {
        label: "Right",
        css: r#"@keyframes demo {
  from { border-right-color: #3b82f6; }
  to   { border-right-color: #f43f5e; }
}"#,
        example: ExampleKind::BorderColorRight,
    },
    DocVariant {
        label: "Bottom",
        css: r#"@keyframes demo {
  from { border-bottom-color: #22c55e; }
  to   { border-bottom-color: #a855f7; }
}"#,
        example: ExampleKind::BorderColorBottom,
    },
    DocVariant {
        label: "Left",
        css: r#"@keyframes demo {
  from { border-left-color: #06b6d4; }
  to   { border-left-color: #eab308; }
}"#,
        example: ExampleKind::BorderColorLeft,
    },
    DocVariant {
        label: "All",
        css: r#"@keyframes demo {
  from {
    border-top-color: #3b82f6; border-right-color: #3b82f6;
    border-bottom-color: #3b82f6; border-left-color: #3b82f6;
  }
  to {
    border-top-color: #f43f5e; border-right-color: #f43f5e;
    border-bottom-color: #f43f5e; border-left-color: #f43f5e;
  }
}"#,
        example: ExampleKind::BorderColorAll,
    },
];

const CORNER_RADIUS_VARIANTS: &[DocVariant] = &[
    DocVariant {
        label: "All",
        css: r#"@keyframes demo {
  from {
    corner-top-left-radius: 6px; corner-top-right-radius: 6px;
    corner-bottom-left-radius: 6px; corner-bottom-right-radius: 6px;
  }
  to {
    corner-top-left-radius: 72px; corner-top-right-radius: 72px;
    corner-bottom-left-radius: 72px; corner-bottom-right-radius: 72px;
  }
}"#,
        example: ExampleKind::CornerRadiusAll,
    },
    DocVariant {
        label: "Top left",
        css: r#"@keyframes demo {
  from { corner-top-left-radius: 4px; }
  to   { corner-top-left-radius: 90px; }
}"#,
        example: ExampleKind::CornerRadiusTopLeft,
    },
    DocVariant {
        label: "Top right",
        css: r#"@keyframes demo {
  from { corner-top-right-radius: 4px; }
  to   { corner-top-right-radius: 90px; }
}"#,
        example: ExampleKind::CornerRadiusTopRight,
    },
    DocVariant {
        label: "Bottom left",
        css: r#"@keyframes demo {
  from { corner-bottom-left-radius: 4px; }
  to   { corner-bottom-left-radius: 90px; }
}"#,
        example: ExampleKind::CornerRadiusBottomLeft,
    },
    DocVariant {
        label: "Bottom right",
        css: r#"@keyframes demo {
  from { corner-bottom-right-radius: 4px; }
  to   { corner-bottom-right-radius: 90px; }
}"#,
        example: ExampleKind::CornerRadiusBottomRight,
    },
];

pub(super) fn variants(title: &str) -> &'static [DocVariant] {
    match title {
        "transform" => TRANSFORM_VARIANTS,
        "background-image / position / repeat / size" => BACKGROUND_VARIANTS,
        "border-top/right/bottom/left-width" => BORDER_WIDTH_VARIANTS,
        "border-top/right/bottom/left-color" => BORDER_COLOR_VARIANTS,
        "corner-*-radius / smoothing" => CORNER_RADIUS_VARIANTS,
        _ => &[],
    }
}
