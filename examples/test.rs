use vizia::*;

#[derive(Lens)]
pub struct AppData {
    val: f32,
}

pub enum AppEvent {
    SetValue(f32),
}

impl Model for AppData {
    fn event(&mut self, cx: &mut Context, event: &mut Event) {
        if let Some(app_event) = event.message.downcast() {
            match app_event {
                AppEvent::SetValue(val) => {
                    self.val = *val;
                }
            }
        }
    }
}

fn main() {
    let mut window_description = WindowDescription::new();
    window_description.resizable = false;
    Application::new(window_description, |cx| {
        AppData { val: 0.5 }.build(cx);

        HStack::new(cx, |cx| {
            Slider::new(cx, AppData::val, Orientation::Horizontal)
                .on_changing(|cx, val| cx.emit(AppEvent::SetValue(val)));
            Textbox::new(cx, AppData::val.map(|val| format!("{:.2}", val)))
                .on_submit(|cx, txt| {
                    if let Ok(val) = txt.parse::<f32>() {
                        let val = val.clamp(0.0, 1.0);
                        cx.emit(AppEvent::SetValue(val));
                    }
                })
                .width(Pixels(100.0));
        })
        .height(Auto)
        .col_between(Pixels(20.0))
        .child_space(Pixels(20.0));
    })
    .run();
}
