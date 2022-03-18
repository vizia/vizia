use vizia::*;

const STYLE: &str = r#"
    .test {
        width: 100px;
        height: 100px;
        background-color: red;
        transition: background-color 2.0 0.0;
    }

    .test:hover {
        background-color: blue;
        transition: background-color 2.0 0.0;
    }
"#;

fn main() {
    let window_description = WindowDescription::new();
    Application::new(window_description, |cx| {
        cx.add_theme(STYLE);
        // Transition
        Element::new(cx).class("test");

        // Animation
        let animation = cx
            .add_animation(std::time::Duration::from_secs(1))
            .add_keyframe(0.0, |keyframe| keyframe.set_background_color(Color::red()))
            .add_keyframe(1.0, |keyframe| keyframe.set_background_color(Color::blue()))
            .build();

        let animation_persistent = cx
            .add_animation(std::time::Duration::from_secs(1))
            .persistent()
            .add_keyframe(0.0, |keyframe| keyframe.set_background_color(Color::red()))
            .add_keyframe(1.0, |keyframe| keyframe.set_background_color(Color::blue()))
            .build();

        Element::new(cx)
            .size(Pixels(200.0))
            .background_color(Color::red())
            .on_press(move |cx| cx.play_animation(animation));

        Element::new(cx)
            .size(Pixels(200.0))
            .background_color(Color::red())
            .on_press(move |cx| cx.play_animation(animation_persistent));
    })
    .run();
}
