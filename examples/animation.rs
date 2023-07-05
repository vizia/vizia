use vizia::prelude::*;

fn main() {
    Application::new(|cx| {
        let animation = AnimationBuilder::new()
            .keyframe(0.0, |key| key.scale("1"))
            .keyframe(1.0, |key| key.scale("2.5"));

        let anim_id = cx.add_animation(animation);

        Button::new(
            cx,
            move |cx| cx.play_animation(anim_id, Duration::from_secs(2)),
            |cx| Label::new(cx, "Animate"),
        );
    })
    .run();
}
