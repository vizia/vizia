mod helpers;
use helpers::*;
use vizia::prelude::*;

fn main() {
    Application::new(|cx| {
        view_controls(cx);

        PopupData::default().build(cx);

        VStack::new(cx, |cx| {
            Button::new(cx, |cx| cx.emit(PopupEvent::Switch), |cx| Label::new(cx, "Open"));

            Popup::new(cx, PopupData::is_open, true, |_| {})
                .on_blur(|cx| cx.emit(PopupEvent::Close))
                .size(Pixels(200.0))
                .background_color(Color::red());
        })
        .disabled(ControlsData::disabled)
        .class("container");
    })
    .title("Popup")
    .run();
}
