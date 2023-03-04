use vizia::prelude::*;

const STYLE: &str = r#"
    .one {
        size: 100px;
        background-color: green;
        overflow-x: visible;
        overflow-y: hidden;
    }

    .one:hover {
        background-color: red;
    }
    
    .two {
        size: 50px;
        background-color: red;
        top: 75px;
        transform: translateX(75px);
        overflow: visible;
    }

    .two:hover {
        background-color: blue;
    }

    .three {
        size: 75px;
        background-color: yellow;
    }

    .three:hover {
        background-color: maroon;
    }
"#;

#[derive(Lens)]
pub struct AppData {
    skew: f32,
}

pub enum AppEvent {
    SetSkew(f32),
}

impl Model for AppData {
    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|app_event, _| match app_event {
            AppEvent::SetSkew(val) => {
                self.skew = *val;
            }
        });
    }
}

fn main() {
    Application::new(|cx| {
        cx.add_theme(STYLE);

        // AppData { skew: 0.0 }.build(cx);
        // HStack::new(cx, |cx| {
        //     Element::new(cx)
        //         .size(Pixels(100.0))
        //         .space(Pixels(150.0))
        //         .background_color(Color::magenta());
        // })
        // .size(Pixels(200.0))
        // .class("test");
        HStack::new(cx, |cx| {
            HStack::new(cx, |cx| {
                HStack::new(cx, |cx| {}).class("three");
            })
            .min_size(Pixels(0.0))
            .class("two");
        })
        .transform(vec![Transform::SkewX(Angle::Deg(26.5650512))])
        .class("one")
        .min_size(Pixels(0.0));

        // Slider::new(cx, AppData::skew)
        //     .range(0.0..45.0)
        //     .on_changing(|ex, val| ex.emit(AppEvent::SetSkew(val)));

        // HStack::new(cx, |cx| {
        //     HStack::new(cx, |cx| {
        //         HStack::new(cx, |cx| {})
        //             .space(Pixels(150.0))
        //             .size(Pixels(100.0))
        //             .background_color(Color::blue());
        //     })
        //     // .left(Pixels(50.0))
        //     .size(Pixels(200.0))
        //     .background_color(Color::red())
        //     .min_size(Pixels(0.0))
        //     .class("bar");
        // })
        // .background_color(Color::green())
        // .size(Pixels(220.0))
        // .child_space(Stretch(1.0))
        // .overflow(Overflow::Hidden)
        // .min_size(Pixels(0.0));
    })
    .run();
}
