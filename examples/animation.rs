use vizia::prelude::*;

const STYLE: &str = r#"
    @keyframes slidein {
        0% {
            left: 0px;
        }
        50% {
            left: 50px;
        }
        100% {
            left: 200px;
        }
    }
"#;

fn main() {
    Application::new(|cx| {
        cx.add_stylesheet(STYLE).expect("Failed to add stylesheet");

        let animation = AnimationBuilder::new()
            .keyframe(0.0, |key| key.scale("1"))
            .keyframe(1.0, |key| key.scale("2.5"));

        let anim_id = cx.add_animation(animation);

        Element::new(cx).background_color(Color::red()).size(Pixels(100.0)).id("elem");

        Button::new(
            cx,
            |cx| cx.play_animation_for("slidein", "elem", Duration::from_secs(2)),
            |cx| Label::new(cx, "Play 1"),
        );
        Button::new(
            cx,
            move |cx| cx.play_animation_for(anim_id, "elem", Duration::from_secs(2)),
            |cx| Label::new(cx, "Play 2"),
        );
    })
    .run();
}
