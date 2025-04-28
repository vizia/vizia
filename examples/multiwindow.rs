pub use vizia::prelude::*;

#[cfg(feature = "baseview")]
fn main() {
    panic!("This example is not supported on baseview");
}

#[derive(Lens)]
struct AppData {
    color: Color,
    show_window: bool,
}

impl Model for AppData {
    fn event(&mut self, _cx: &mut EventContext, event: &mut Event) {
        event.map(|app_event, _| match app_event {
            AppEvent::ShowWindow => self.show_window = true,
            AppEvent::WindowClosed => self.show_window = false,
            AppEvent::SetRed(val) => {
                self.color = Color::rgb((*val * 255.0) as u8, self.color.g(), self.color.b())
            }
            AppEvent::SetGreen(val) => {
                self.color = Color::rgb(self.color.r(), (*val * 255.0) as u8, self.color.b())
            }
            AppEvent::SetBlue(val) => {
                self.color = Color::rgb(self.color.r(), self.color.g(), (*val * 255.0) as u8)
            }
        })
    }
}

pub enum AppEvent {
    ShowWindow,
    WindowClosed,
    SetRed(f32),
    SetGreen(f32),
    SetBlue(f32),
}

#[cfg(not(feature = "baseview"))]
fn main() -> Result<(), ApplicationError> {
    Application::new(|cx| {
        AppData { color: Color::white(), show_window: false }.build(cx);

        Binding::new(cx, AppData::show_window, |cx, show_subwindow| {
            if show_subwindow.get(cx) {
                Window::new(cx, |cx| {
                    VStack::new(cx, |cx: &mut Context| {
                        Slider::new(cx, AppData::color.map(|c| c.r() as f32 / 255.0))
                            .on_change(|cx, val| cx.emit(AppEvent::SetRed(val)));
                        Slider::new(cx, AppData::color.map(|c| c.g() as f32 / 255.0))
                            .on_change(|cx, val| cx.emit(AppEvent::SetGreen(val)));
                        Slider::new(cx, AppData::color.map(|c| c.b() as f32 / 255.0))
                            .on_change(|cx, val| cx.emit(AppEvent::SetBlue(val)));
                    })
                    .padding(Pixels(20.0))
                    .alignment(Alignment::Center)
                    .vertical_gap(Pixels(12.0));
                })
                .on_close(|cx| {
                    cx.emit(AppEvent::WindowClosed);
                })
                .title("Set color...")
                .inner_size((400, 200))
                .anchor(Anchor::Center);
            }
        });

        HStack::new(cx, |cx| {
            Button::new(cx, |cx| Label::new(cx, "Show Window"))
                .on_press(|cx| cx.emit(AppEvent::ShowWindow));
        })
        .size(Auto)
        .padding(Pixels(20.0))
        .background_color(AppData::color);
    })
    .title("Main")
    .run()
}
