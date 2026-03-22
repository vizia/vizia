pub use vizia::prelude::*;

#[cfg(feature = "baseview")]
fn main() {
    panic!("This example is not supported on baseview");
}

#[cfg(all(feature = "winit", not(feature = "baseview")))]
struct AppData {
    color: Signal<Color>,
    show_window: Signal<bool>,
}

#[cfg(all(feature = "winit", not(feature = "baseview")))]
impl Model for AppData {
    fn event(&mut self, _cx: &mut EventContext, event: &mut Event) {
        event.map(|app_event, _| match app_event {
            AppEvent::ShowWindow => self.show_window.set(true),
            AppEvent::WindowClosed => self.show_window.set(false),
            AppEvent::SetRed(val) => {
                self.color.update(|color| {
                    *color = Color::rgb((*val * 255.0) as u8, color.g(), color.b())
                });
            }
            AppEvent::SetGreen(val) => {
                self.color.update(|color| {
                    *color = Color::rgb(color.r(), (*val * 255.0) as u8, color.b())
                });
            }
            AppEvent::SetBlue(val) => {
                self.color.update(|color| {
                    *color = Color::rgb(color.r(), color.g(), (*val * 255.0) as u8)
                });
            }
        })
    }
}

#[cfg(all(feature = "winit", not(feature = "baseview")))]
pub enum AppEvent {
    ShowWindow,
    WindowClosed,
    SetRed(f32),
    SetGreen(f32),
    SetBlue(f32),
}

#[cfg(all(feature = "winit", not(feature = "baseview")))]
fn main() -> Result<(), ApplicationError> {
    Application::new(|cx| {
        let color = Signal::new(Color::white());
        let show_window = Signal::new(false);
        let red = Memo::new(move |_| color.get().r() as f32 / 255.0);
        let green = Memo::new(move |_| color.get().g() as f32 / 255.0);
        let blue = Memo::new(move |_| color.get().b() as f32 / 255.0);

        AppData { color, show_window }.build(cx);

        Binding::new(cx, show_window, move |cx, show_subwindow| {
            if show_subwindow {
                Window::new(cx, move |cx| {
                    VStack::new(cx, |cx: &mut Context| {
                        Slider::new(cx, red).on_change(|cx, val| cx.emit(AppEvent::SetRed(val)));
                        Slider::new(cx, green)
                            .on_change(|cx, val| cx.emit(AppEvent::SetGreen(val)));
                        Slider::new(cx, blue).on_change(|cx, val| cx.emit(AppEvent::SetBlue(val)));
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
        .background_color(color);
    })
    .title("Main")
    .run()
}
