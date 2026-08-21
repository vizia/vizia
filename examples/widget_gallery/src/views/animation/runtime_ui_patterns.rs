use super::{DocEntry, DocVariant, ExampleKind};

pub(super) const DOCS: &[DocEntry] = &[
    DocEntry {
        category: "Runtime & UI patterns",
        title: "runtime controls",
        css: r#".target {
  animation: move 6s linear infinite alternate;
}

/* Rust */
cx.pause_css_animation(id);
cx.resume_css_animation(id);
cx.seek_css_animation(id, seconds);
cx.reverse_css_animation(id);
cx.set_css_animation_playback_rate(id, 2.0);"#,
        description: "Runtime control operates on the same stable CSS occurrence and the same property-store clocks that render the effect.",
        example: ExampleKind::Runtime,
    },
    DocEntry {
        category: "Runtime & UI patterns",
        title: "blur reveal popover",
        css: r#"@keyframes reveal {
  from { opacity: 0; filter: blur(8px); translate: 0 -5px; }
  to   { opacity: 1; filter: blur(0); translate: 0 0; }
}

popover { animation: reveal 180ms ease-out both; }"#,
        description: "A native Vizia Popover can use the same CSS animation engine for a short entrance effect.",
        example: ExampleKind::Popover,
    },
    DocEntry {
        category: "Runtime & UI patterns",
        title: "throbber variants",
        css: r#"/* Generic GalleryThrobber variants:
   dots · ring · waveform · orbit · pulse
   spinner · grid · bounce · ripple · equalizer */

.throbber { animation: loader 900ms ease-in-out infinite; }"#,
        description: "Reusable loading indicators cover staggered dots, a rotating ring, equalizer bars, an orbit and a pulse.",
        example: ExampleKind::Throbber,
    },
    DocEntry {
        category: "Runtime & UI patterns",
        title: "skeleton variants",
        css: r#"/* Generic Skeleton variants:
   text · circle · rectangle · button · card
   profile · list · table · article · media */

.skeleton-bone {
  animation: shimmer 1.4s ease-in-out infinite alternate;
}"#,
        description: "Reusable skeleton compositions share one animated bone primitive and expose common content-layout variants.",
        example: ExampleKind::Skeleton,
    },
    DocEntry {
        category: "Runtime & UI patterns",
        title: "SVG icon morph sequence",
        css: r#"/* Three Tabler SVGs are layered and cross-morphed
   with opacity, scale and rotation. */
.icon { animation: icon-morph 3s ease-in-out infinite; }"#,
        description: "Tabler heart, star and bolt icons cross-morph through coordinated opacity, scale and rotation while preserving native SVG rendering.",
        example: ExampleKind::SvgMorph,
    },
];

const THROBBER_VARIANTS: &[DocVariant] = &[
    DocVariant { label: "All", css: DOCS[2].css, example: ExampleKind::Throbber },
    DocVariant {
        label: "Dots",
        css: ".throbber-dots { animation: staggered-dots 900ms infinite; }",
        example: ExampleKind::ThrobberDots,
    },
    DocVariant {
        label: "Ring",
        css: ".throbber-ring { animation: spin 800ms linear infinite; }",
        example: ExampleKind::ThrobberRing,
    },
    DocVariant {
        label: "Bars",
        css: ".throbber-bars { animation: equalize 720ms ease-in-out infinite; }",
        example: ExampleKind::ThrobberBars,
    },
    DocVariant {
        label: "Orbit",
        css: ".throbber-orbit { animation: spin 1100ms linear infinite; }",
        example: ExampleKind::ThrobberOrbit,
    },
    DocVariant {
        label: "Pulse",
        css: ".throbber-pulse { animation: pulse 1200ms ease-out infinite; }",
        example: ExampleKind::ThrobberPulse,
    },
    DocVariant {
        label: "Spinner",
        css: r#".spinner-arm {
  opacity: 0;
  background-color: var(--primary);
  animation: reveal-arm 800ms steps(1, end) infinite;
}

/* Reverse the staggered delays for the opposite direction. */"#,
        example: ExampleKind::ThrobberSpinner,
    },
    DocVariant {
        label: "Grid",
        css: ".throbber-grid { animation: grid 750ms ease-in-out infinite; }",
        example: ExampleKind::ThrobberGrid,
    },
    DocVariant {
        label: "Bounce",
        css: ".throbber-bounce { animation: bounce 620ms ease-in-out infinite; }",
        example: ExampleKind::ThrobberBounce,
    },
    DocVariant {
        label: "Ripple",
        css: ".throbber-ripple { animation: ripple 1.3s ease-out infinite; }",
        example: ExampleKind::ThrobberRipple,
    },
    DocVariant {
        label: "Equalizer",
        css: ".throbber-equalizer { animation: waveform 520ms ease-in-out infinite; }",
        example: ExampleKind::ThrobberEqualizer,
    },
];

const SKELETON_VARIANTS: &[DocVariant] = &[
    DocVariant { label: "All", css: DOCS[3].css, example: ExampleKind::Skeleton },
    DocVariant {
        label: "Text",
        css: ".skeleton.text { /* three responsive lines */ }",
        example: ExampleKind::SkeletonText,
    },
    DocVariant {
        label: "Card",
        css: ".skeleton.card { /* media + text */ }",
        example: ExampleKind::SkeletonCard,
    },
    DocVariant {
        label: "Profile",
        css: ".skeleton.profile { /* avatar + copy */ }",
        example: ExampleKind::SkeletonProfile,
    },
    DocVariant {
        label: "List",
        css: ".skeleton.list { /* repeated profile rows */ }",
        example: ExampleKind::SkeletonList,
    },
    DocVariant {
        label: "Table",
        css: ".skeleton.table { /* header + cells */ }",
        example: ExampleKind::SkeletonTable,
    },
    DocVariant {
        label: "Circle",
        css: ".skeleton.circle { corner-radius: 50%; }",
        example: ExampleKind::SkeletonCircle,
    },
    DocVariant {
        label: "Rectangle",
        css: ".skeleton.rectangle { corner-radius: 0; }",
        example: ExampleKind::SkeletonRectangle,
    },
    DocVariant {
        label: "Button",
        css: ".skeleton.button { width: 140px; height: 38px; }",
        example: ExampleKind::SkeletonButton,
    },
    DocVariant {
        label: "Article",
        css: ".skeleton.article { /* title + paragraphs */ }",
        example: ExampleKind::SkeletonArticle,
    },
    DocVariant {
        label: "Media",
        css: ".skeleton.media { /* thumbnail + metadata */ }",
        example: ExampleKind::SkeletonMedia,
    },
];

pub(super) fn variants(title: &str) -> &'static [DocVariant] {
    match title {
        "throbber variants" => THROBBER_VARIANTS,
        "skeleton variants" => SKELETON_VARIANTS,
        _ => &[],
    }
}
