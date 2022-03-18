use vizia::*;

fn main() {
    Application::new(WindowDescription::new().with_title("Slider"), |cx| {
        //cx.add_theme(STYLE);

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
