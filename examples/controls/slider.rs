use vizia::*;

fn main() {
    Application::new(WindowDescription::new().with_title("Slider"), |cx| {
        //cx.add_theme(STYLE);

        AppData { value: 0.5 }.build(cx);

        HStack::new(cx, |cx| {
            HStack::new(cx, |cx| {
                Slider::new(cx, AppData::value)
                    .on_changing(move |cx, val| cx.emit(AppEvent::SetValue(val)));
                Label::new(cx, AppData::value.map(|val| format!("{:.2}", val)));
            })
            .height(Pixels(50.0))
            .child_top(Stretch(1.0))
            .child_bottom(Stretch(1.0))
            .col_between(Pixels(10.0));

            VStack::new(cx, |cx| {
                Slider::new(cx, AppData::value)
                    .on_changing(move |cx, val| cx.emit(AppEvent::SetValue(val)))
                    .class("vertical");
                Label::new(cx, AppData::value.map(|val| format!("{:.2}", val)));
            })
            .width(Pixels(50.0))
            .child_left(Stretch(1.0))
            .child_right(Stretch(1.0))
            .row_between(Pixels(10.0));
        })
        .child_space(Pixels(50.0))
        .col_between(Pixels(50.0));
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
