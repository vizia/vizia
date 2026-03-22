use vizia::prelude::*;

#[cfg(feature = "baseview")]
fn main() {
    panic!("This example is not supported on baseview");
}

pub enum AppEvent {
    SetTitle(String),
}

pub struct AppData {
    title: Signal<String>,
}

impl Model for AppData {
    fn event(&mut self, _cx: &mut EventContext, event: &mut Event) {
        event.map(|app_event, _| match app_event {
            AppEvent::SetTitle(title) => self.title.set(title.clone()),
        })
    }
}

#[cfg(not(feature = "baseview"))]
fn main() -> Result<(), ApplicationError> {
    let title = Signal::new("Window Modifiers".to_string());

    Application::new(move |cx| {
        AppData { title }.build(cx);

        VStack::new(cx, |cx| {
            Label::new(cx, "Window title:");
            Textbox::new(cx, title)
                .on_edit(|ex, txt| ex.emit(AppEvent::SetTitle(txt)))
                .width(Stretch(1.0));
        })
        .padding(Pixels(8.0))
        .gap(Pixels(8.0));
    })
    .title(title)
    .inner_size((400, 100))
    .run()
}
