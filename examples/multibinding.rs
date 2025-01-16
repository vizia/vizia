use vizia::prelude::*;

#[derive(Lens)]
pub struct AppData {
    value: f32,
}

impl Model for AppData {
    // fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
    //     event.map(|app_event, _| match app_event {
    //         AppEvent::Change => self.value += 0.1,
    //     });
    // }
}

#[derive(Lens)]
pub struct OtherData {
    value: f32,
}

impl Model for OtherData {
    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|app_event, _| match app_event {
            AppEvent::Change => self.value += 0.1,
        });
    }
}

pub enum AppEvent {
    Change,
}

fn main() -> Result<(), ApplicationError> {
    Application::new(|cx| {
        AppData { value: std::f32::consts::PI }.build(cx);
        OtherData { value: 2.6 }.build(cx);

        Button::new(cx, |cx| Label::new(cx, "click me")).on_press(|cx| cx.emit(AppEvent::Change));

        // Binding::new(cx, (AppData::value, OtherData::value), |cx, (value, other)| {
        //     println!("rebuild: {} {}", value.get(cx), other.get(cx));
        //     Label::new(cx, value);
        //     Label::new(cx, other);
        // });

        Binding::new(cx, (AppData::value, OtherData::value).map(|(v, o)| (*v, *o)), |cx, m| {
            println!("rebuild: {} {}", m.get(cx).0, m.get(cx).1);
            // Label::new(cx, m.map(|m| m.0));
            // Label::new(cx, m.map(|m| m.1));
        });

        // Label::new(cx, "test").background_color((AppData::value, OtherData::value).map(
        //     |(v, o)| {
        //         if *v + *o > 8.0 {
        //             Color::green()
        //         } else {
        //             Color::red()
        //         }
        //     },
        // ));
    })
    .title("Lens Map")
    .run()
}
