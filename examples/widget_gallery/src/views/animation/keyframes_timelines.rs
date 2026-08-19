use super::{DocEntry, ExampleKind};

pub(super) const DOCS: &[DocEntry] = &[
    DocEntry {
        category: "Keyframes & timelines",
        title: "@keyframes",
        css: r#"@keyframes pulse {
  from { opacity: .35; scale: .8; }
  to   { opacity: 1; scale: 1.15; }
}

.target {
  animation: pulse 1.4s ease-in-out infinite alternate;
}"#,
        description: "from/to and percentage offsets are parsed into the same animation storage used by Rust-side animations and transitions.",
        example: ExampleKind::Motion,
    },
    DocEntry {
        category: "Keyframes & timelines",
        title: "percentage keyframes",
        css: r#"@keyframes orbit {
  0%   { translate: -120px 0; }
  35%  { translate: -20px -35px; }
  70%  { translate: 80px 24px; }
  100% { translate: 120px 0; }
}

.target {
  animation: orbit 1.8s ease-in-out infinite alternate;
}"#,
        description: "Multiple offsets, duplicate offsets, implicit endpoints and underlying values are normalized by the CSS animation runtime.",
        example: ExampleKind::PercentageKeyframes,
    },
    DocEntry {
        category: "Keyframes & timelines",
        title: "multiple animations",
        css: r#".target {
  animation:
    move 1.6s ease-in-out infinite alternate,
    color 2.4s linear infinite alternate;
}"#,
        description: "Comma-separated animation lists create independent occurrences. CSS list repetition rules resolve shorter longhand lists.",
        example: ExampleKind::Multiple,
    },
    DocEntry {
        category: "Keyframes & timelines",
        title: "document timeline",
        css: r#".target {
  animation: move 2s linear infinite alternate;
  animation-timeline: auto;
}"#,
        description: "The default document timeline advances from wall-clock time and requests frames only while sampled values are changing.",
        example: ExampleKind::DocumentTimeline,
    },
    DocEntry {
        category: "Keyframes & timelines",
        title: "named scroll timeline",
        css: r#".target {
  animation: reveal 1s linear both;
  animation-timeline: --docs-scroll;
}

/* Rust source */
ScrollView::new(cx, content)
  .timeline_name("--docs-scroll");"#,
        description: "A Vizia ScrollView can publish a named normalized progress source that CSS animations consume directly.",
        example: ExampleKind::ScrollTimeline,
    },
    DocEntry {
        category: "Keyframes & timelines",
        title: "scroll()",
        css: r#".target {
  animation: reveal 1s linear both;
  animation-timeline: scroll(nearest block);
}"#,
        description: "scroll() uses the selected scroll source and axis as the animation clock. Stop scrolling and the animation freezes.",
        example: ExampleKind::ScrollTimeline,
    },
    DocEntry {
        category: "Keyframes & timelines",
        title: "view()",
        css: r#".target {
  animation: reveal 1s linear both;
  animation-timeline: view(block);
}"#,
        description: "view() derives progress from the subject entering and leaving its nearest scroll viewport.",
        example: ExampleKind::ViewTimeline,
    },
];
