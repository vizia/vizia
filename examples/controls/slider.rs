use vizia::*;

const STYLE: &str = r#"
    slider {
        height: 10px;
        top: 1s;
        bottom: 1s;
        width: 1s;
        background-color: #dfdfdf;
        border-radius: 4.5px;
    }

    slider.vertical {
        top: auto;
        bottom: auto;
        height: 1s;
        width: 10px;
    }

    slider track {
    }

    slider .active {
        background-color: #f74c00;
        border-radius: 4.5px;
    }

    slider .thumb {
        background-color: white;
        top: 1s;
        bottom: 1s;
        border-radius: 14.5px;
        border-color: #757575;
        border-width: 1px;
        width: 40px;
        height: 40px;
    }

    label {
        width: 100px;
        height: 30px;
        top: 1s;
        bottom: 1s;
        border-color: #757575;
        border-width: 1px;
    }
"#;

fn main() {
    Application::new(WindowDescription::new().with_title("Slider"), |cx| {
        cx.add_theme(STYLE);

        AppData { value: 0.5 }.build(cx);

        for _ in 0..5 {
            HStack::new(cx, |cx| {
                Slider::new(cx, AppData::value, Orientation::Horizontal)
                    .on_changing(move |cx, val| cx.emit(AppEvent::SetValue(val)));
                Label::new(cx, AppData::value.map(|val| format!("{:.2}", val)));
            })
            .height(Pixels(50.0))
            .child_space(Pixels(50.0))
            .col_between(Pixels(50.0));
        }

        // HStack::new(cx, |cx| {
        //     Binding::new(cx, SliderData::value, |cx, value| {
        //         Slider::new(cx, *value.get(cx), Orientation::Vertical)
        //             .class("vertical")
        //             .on_press(cx, |_| println!("Press"));
        //         let value = *value.get(cx);
        //         Label::new(cx, &format!("{:.*}", 2, value));
        //     });
        // })
        // .child_space(Pixels(50.0))
        // .col_between(Pixels(50.0));
    })
    .run();
}

#[derive(Debug, Lens)]
pub struct AppData {
    value: f32,
}

pub enum AppEvent {
    SetValue(f32),
}

impl Model for AppData {
    fn event(&mut self, _: &mut Context, event: &mut Event) {
        if let Some(slider_event) = event.message.downcast() {
            match slider_event {
                AppEvent::SetValue(val) => {
                    self.value = *val;
                }
            }
        }
    }
}
