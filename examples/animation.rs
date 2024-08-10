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

fn main() -> Result<(), ApplicationError> {
    Application::new(|cx| {
        cx.add_stylesheet(STYLE).expect("Failed to add stylesheet");

        let animation = AnimationBuilder::new()
            .keyframe(0.0, |key| key.scale("1"))
            .keyframe(1.0, |key| key.scale("2.5"));

        let anim_id = cx.add_animation(animation);

        Element::new(cx).background_color(Color::red()).size(Pixels(100.0)).id("elem");

        Button::new(cx, |cx| Label::new(cx, "Play 1")).on_press(|cx| {
            cx.play_animation_for("slidein", "elem", Duration::from_secs(2), Duration::default())
        });
        Button::new(cx, |cx| Label::new(cx, "Play 2")).on_press(move |cx| {
            cx.play_animation_for(anim_id, "elem", Duration::from_secs(2), Duration::default())
        });
    })
    .run()
}
