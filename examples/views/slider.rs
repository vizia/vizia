use vizia::*;

fn main() {
    Application::new(|cx| {
        AppData { value: 0.0 }.build(cx);

        HStack::new(cx, |cx| {
            VStack::new(cx, |cx| {
                HStack::new(cx, |cx| {
                    Slider::new(cx, AppData::value.map(|val| (val + 50.0) / 100.0)).on_changing(
                        move |cx, val| cx.emit(AppEvent::SetValue(-50.0 + (val * 100.0))),
                    );
                    Label::new(
                        cx,
                        AppData::value.map(|val| format!("{:.2}", (val - 50.0) / 100.0)),
                    )
                    .width(Pixels(50.0));
                })
                .height(Pixels(50.0))
                .child_top(Stretch(1.0))
                .child_bottom(Stretch(1.0))
                .col_between(Pixels(10.0));

                HStack::new(cx, |cx| {
                    Slider::new(cx, AppData::value)
                        .range(-50.0..50.0)
                        .on_changing(move |cx, val| cx.emit(AppEvent::SetValue(val)));
                    Label::new(cx, AppData::value.map(|val| format!("{:.2}", val)))
                        .width(Pixels(50.0));
                })
                .height(Pixels(50.0))
                .child_top(Stretch(1.0))
                .child_bottom(Stretch(1.0))
                .col_between(Pixels(10.0));
            });

            VStack::new(cx, |cx| {
                Slider::new(cx, AppData::value)
                    .range(-50.0..50.0)
                    .on_changing(move |cx, val| cx.emit(AppEvent::SetValue(val)))
                    .class("vertical");
                Label::new(cx, AppData::value.map(|val| format!("{:.2}", val)))
                    .child_space(Stretch(1.0))
                    .width(Pixels(50.0));
            })
            .width(Pixels(50.0))
            .child_left(Stretch(1.0))
            .child_right(Stretch(1.0))
            .row_between(Pixels(10.0));
        })
        .child_space(Pixels(50.0))
        .col_between(Pixels(50.0));
    })
    .title("Slider")
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
        event.map(|app_event, _| match app_event {
            AppEvent::SetValue(val) => {
                self.value = *val;
            }
        });
    }
}
