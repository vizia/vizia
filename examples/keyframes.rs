use instant::Duration;
use vizia::prelude::*;

const STYLE: &str = r#"
    @keyframes slidein {
        from {
            transform: translateX(0px);
        }

        to {
            transform: translateX(100px);
        }
    }
"#;

fn main() {
    Application::new(|cx| {
        cx.add_stylesheet(STYLE);

        let animation = AnimationBuilder::new()
            .keyframe(0.0, |key| key.scale("1"))
            .keyframe(1.0, |key| key.scale("2.5"));

        let anim_id = cx.add_animation(animation);

        Element::new(cx).size(Pixels(100.0)).background_color(Color::red()).id("elem");

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
