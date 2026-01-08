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

struct AnimationApp {
    red: Signal<Color>,
    size_100: Signal<Units>,
    position_absolute: Signal<PositionType>,
}

impl App for AnimationApp {
    fn new(cx: &mut Context) -> Self {
        Self {
            red: cx.state(Color::red()),
            size_100: cx.state(Pixels(100.0)),
            position_absolute: cx.state(PositionType::Absolute),
        }
    }

    fn on_build(self, cx: &mut Context) -> Self {
        cx.add_stylesheet(STYLE).expect("Failed to add stylesheet");
        let red = self.red;
        let size_100 = self.size_100;
        let position_absolute = self.position_absolute;

        let animation = AnimationBuilder::new()
            .keyframe(0.0, |key| key.scale("1"))
            .keyframe(1.0, |key| key.scale("2.5"));

        let anim_id = cx.add_animation(animation);

        Element::new(cx)
            .background_color(red)
            .size(size_100)
            .position_type(position_absolute)
            .id("elem");

        Button::new(cx, |cx| Label::new(cx, "Play 1")).on_press(|cx| {
            cx.play_animation_for("slidein", "elem", Duration::from_secs(2), Duration::default())
        });
        Button::new(cx, |cx| Label::new(cx, "Play 2")).on_press(move |cx| {
            cx.play_animation_for(anim_id, "elem", Duration::from_secs(2), Duration::default())
        });
        self
    }
}

fn main() -> Result<(), ApplicationError> {
    AnimationApp::run()
}
