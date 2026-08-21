use super::{DocEntry, ExampleKind};

pub(super) const DOCS: &[DocEntry] = &[
    DocEntry {
        category: "Animation properties",
        title: "animation",
        css: r#"@keyframes move {
  from { translate: -110px 0; opacity: .45; }
  to   { translate: 110px 0; opacity: 1; }
}

.target {
  animation: move 1.6s ease-in-out
             -300ms infinite alternate both;
}"#,
        description: "The shorthand wires name, duration, easing, delay, iteration count, direction, fill mode and play state into one CSS animation occurrence.",
        example: ExampleKind::Motion,
    },
    DocEntry {
        category: "Animation properties",
        title: "animation-name",
        css: r#".target {
  animation-name: move;
  animation-duration: 1.6s;
  animation-iteration-count: infinite;
  animation-direction: alternate;
}"#,
        description: "Selects the @keyframes rule by name. Vizia starts, updates and cancels the CSS animation automatically from computed style.",
        example: ExampleKind::Motion,
    },
    DocEntry {
        category: "Animation properties",
        title: "animation-duration",
        css: r#".target {
  animation-name: move;
  animation-duration: 3.5s;
  animation-iteration-count: infinite;
  animation-direction: alternate;
}"#,
        description: "Controls the active interval for one iteration. Negative durations are rejected.",
        example: ExampleKind::Duration,
    },
    DocEntry {
        category: "Animation properties",
        title: "animation-delay",
        css: r#".target {
  animation: move 1.6s ease-in-out;
  animation-delay: -800ms;
  animation-iteration-count: infinite;
  animation-direction: alternate;
}"#,
        description: "Positive and negative delays are supported. A negative delay begins as if the animation had already been running.",
        example: ExampleKind::Delay,
    },
    DocEntry {
        category: "Animation properties",
        title: "animation-timing-function",
        css: r#".target {
  animation: move 1.8s
    cubic-bezier(.2, .8, .2, 1)
    infinite alternate;
}"#,
        description: "Supports CSS easing keywords, validated cubic-bezier() and steps() timing functions.",
        example: ExampleKind::Motion,
    },
    DocEntry {
        category: "Animation properties",
        title: "steps() / step-start / step-end",
        css: r#".target {
  animation: move 1.8s
    steps(6, end)
    infinite alternate;
}"#,
        description: "Discrete timing functions jump between sampled positions instead of interpolating continuously.",
        example: ExampleKind::Steps,
    },
    DocEntry {
        category: "Animation properties",
        title: "animation-iteration-count",
        css: r#".target {
  animation: move 900ms ease-in-out;
  animation-iteration-count: 3;
  animation-fill-mode: forwards;
}"#,
        description: "Finite, fractional and infinite iteration counts are supported. This example runs three times and keeps its final value.",
        example: ExampleKind::Iterations,
    },
    DocEntry {
        category: "Animation properties",
        title: "animation-direction",
        css: r#".target {
  animation: move 1.4s ease-in-out
             infinite alternate-reverse;
}"#,
        description: "normal, reverse, alternate and alternate-reverse affect iteration progress without changing the keyframe data.",
        example: ExampleKind::Direction,
    },
    DocEntry {
        category: "Animation properties",
        title: "animation-fill-mode",
        css: r#".target {
  animation: settle 900ms ease-out;
  animation-fill-mode: forwards;
}"#,
        description: "none, forwards, backwards and both control whether sampled values apply before or after the active phase.",
        example: ExampleKind::FillMode,
    },
    DocEntry {
        category: "Animation properties",
        title: "animation-play-state",
        css: r#".target {
  animation: move 1.6s ease-in-out
             -800ms infinite alternate;
  animation-play-state: paused;
}"#,
        description: "running and paused operate on the same CSS occurrence and preserve local progress when resumed.",
        example: ExampleKind::Paused,
    },
    DocEntry {
        category: "Animation properties",
        title: "animation-composition",
        css: r#".target {
  animation:
    move-x 1.8s infinite alternate,
    move-y 1.1s infinite alternate;

  animation-composition: add, add;
}"#,
        description: "replace, add and accumulate are resolved inside Vizia's property stores using a stable per-property effect stack.",
        example: ExampleKind::Composition,
    },
    DocEntry {
        category: "Animation properties",
        title: "animation-timeline",
        css: r#".target {
  animation: reveal 1s linear both;
  animation-timeline: auto;
}

/* Also supported:
   --named-timeline
   scroll(nearest block)
   view(block)
*/"#,
        description: "The effect can be sampled from document time, a named scroll source, scroll() or view() progress.",
        example: ExampleKind::DocumentTimeline,
    },
];
