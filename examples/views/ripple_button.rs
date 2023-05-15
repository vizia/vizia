use instant::Duration;
use vizia::prelude::*;

mod helpers;
use helpers::*;
use vizia_core::style::Scale;

const STYLE: &str = r#"

    button.accent:active {
        
    }

    ripple {
        width: auto;
        height: auto;
        overflow: hidden;     
    }

    ripple > .ink {
        position-type: self-directed;
        border-radius: 50%;
        size: 60px;
        background-color: #00000040;
        translate: -50% -50%;
    }
"#;

#[derive(Lens)]
pub struct RippleButton {
    press_pos: (f32, f32),
    animation: Animation,
}

impl RippleButton {
    pub fn new(cx: &mut Context) -> Handle<Self> {
        let animation = AnimationBuilder::new()
            .keyframe(0.0, |key| key.scale("0.0").opacity("1.0"))
            .keyframe(1.0, |key| key.scale("2.5").opacity("0.0"));
        let animation = cx.add_animation(animation);

        Self { press_pos: (0.0, 0.0), animation }.build(cx, |cx| {
            Button::new(cx, |cx| {}, |cx| Label::new(cx, "Button")).class("accent");
            Element::new(cx)
                .class("ink")
                .hoverable(false)
                .position_type(PositionType::SelfDirected)
                .scale("0");
        })
    }
}

impl View for RippleButton {
    fn element(&self) -> Option<&'static str> {
        Some("ripple")
    }

    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|window_event, _| match window_event {
            WindowEvent::MouseDown(button) if *button == MouseButton::Left => {
                let bounds = cx.bounds();

                if let Some(child) = cx.nth_child(1) {
                    cx.with_current(child, |cx| {
                        cx.play_animation(self.animation, Duration::from_millis(500));
                        let posx = cx.mouse().left.pos_down.0 - bounds.left();
                        let posy = cx.mouse().left.pos_down.1 - bounds.top();
                        let scale_factor = cx.scale_factor();
                        println!("{} {}", posx / scale_factor, posy / scale_factor);
                        cx.set_left(Pixels(posx / scale_factor));
                        cx.set_top(Pixels(posy / scale_factor));
                    });
                }
            }

            _ => {}
        });
    }
}

fn main() {
    Application::new(|cx| {
        cx.add_theme(STYLE);

        ExamplePage::new(cx, |cx| {
            RippleButton::new(cx);
        });
    })
    .run();
}
