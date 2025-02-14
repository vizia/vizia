use vizia::prelude::*;

#[cfg(feature = "baseview")]
fn main() {
    panic!("This example is not supported on baseview");
}

#[derive(Lens)]
pub struct AppData {
    is_saved: bool,
    show_dialog: bool,
}

impl Model for AppData {
    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|window_event, meta| {
            // Intercept WindowClose event to show a dialog if not 'saved'.
            if let WindowEvent::WindowClose = window_event {
                if !self.is_saved {
                    self.show_dialog = true;
                    meta.consume();
                }
            }
        });

        event.map(|app_event, _| match app_event {
            AppEvent::CloseModal => {
                self.show_dialog = false;
            }

            AppEvent::Save => {
                self.is_saved = true;
            }

            AppEvent::SaveAndClose => {
                self.is_saved = true;
                self.show_dialog = false;
                cx.emit(WindowEvent::WindowClose);
            }
        });
    }
}

pub enum AppEvent {
    CloseModal,
    Save,
    SaveAndClose,
}

#[cfg(not(feature = "baseview"))]
fn main() -> Result<(), ApplicationError> {
    Application::new(|cx| {
        AppData { is_saved: false, show_dialog: false }.build(cx);

        HStack::new(cx, |cx| {
            Button::new(cx, |cx| Label::new(cx, "Close"))
                .on_press(|cx| cx.emit(WindowEvent::WindowClose));
            Button::new(cx, |cx| Label::new(cx, "Save")).on_press(|cx| cx.emit(AppEvent::Save));
        })
        .gap(Pixels(10.0))
        .padding(Pixels(50.0))
        .alignment(Alignment::TopCenter);

        Binding::new(cx, AppData::show_dialog, |cx, show_dialog| {
            if show_dialog.get(cx) {
                Window::popup(cx, true, |cx| {
                    VStack::new(cx, |cx| {
                        Label::new(cx, "Save before close?")
                            .width(Stretch(1.0))
                            .alignment(Alignment::Center);
                        HStack::new(cx, |cx| {
                            Button::new(cx, |cx| Label::new(cx, "Save & Close"))
                                .on_press(|cx| cx.emit(AppEvent::SaveAndClose))
                                .width(Pixels(120.0))
                                .class("accent");

                            Button::new(cx, |cx| Label::new(cx, "Cancel"))
                                .on_press(|cx| cx.emit(AppEvent::CloseModal))
                                .width(Pixels(120.0));
                        })
                        .horizontal_gap(Pixels(10.0))
                        .size(Auto);
                    })
                    .alignment(Alignment::Center)
                    .vertical_gap(Pixels(20.0));
                })
                .on_close(|cx| cx.emit(AppEvent::CloseModal))
                .title("Save work?")
                .inner_size((400, 100))
                .anchor(Anchor::Center);
            }
        });

        Element::new(cx)
            .size(Stretch(1.0))
            .position_type(PositionType::Absolute)
            .backdrop_filter(Filter::Blur(Pixels(2.0).into()))
            .display(AppData::show_dialog);
    })
    .run()
}
