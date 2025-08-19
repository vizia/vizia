use vizia::{animation::Keyframes, prelude::*};

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

    #elem2 {
        animation: slidein 2s ease-in-out 1s forwards reverse infinite;
    }
"#;

fn main() -> Result<(), ApplicationError> {
    Application::new(|cx| {
        cx.add_stylesheet(STYLE).expect("Failed to add stylesheet");

        let keyframes = Keyframes::new()
            .keyframe(0.0, |key| key.background_color(Color::red()))
            .keyframe(0.5, |key| key.background_color(Color::green()))
            .keyframe(1.0, |key| key.background_color(Color::blue()));

        let anim_id = cx.add_animation_keyframes(keyframes);

        let anim = Animation::new().duration(Duration::from_secs(2)).delay(Duration::from_secs(1));

        VStack::new(cx, |cx| {
            Element::new(cx)
                .background_color(Color::yellow())
                .size(Pixels(200.0))
                .position_type(PositionType::Absolute)
                .id("elem");

            Button::new(cx, |cx| Label::new(cx, "Play 1")).on_press(|cx| {
                cx.play_animation_for(
                    "slidein",
                    "elem",
                    Animation::new().duration(Duration::from_secs(2)),
                );
            });

            Button::new(cx, |cx| Label::new(cx, "Play 2"))
                .on_press(move |cx| cx.play_animation_for(anim_id, "elem", anim));
        });
    })
    .run()
}
