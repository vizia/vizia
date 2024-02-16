mod helpers;
use helpers::*;
use vizia::prelude::*;

fn main() {
    Application::new(|cx| {
        PopupData::default().build(cx);

        ExamplePage::new(cx, |cx| {
            Button::new(cx, |cx| Label::new(cx, "Open")).on_press(|cx| cx.emit(PopupEvent::Switch));

            Popup::new(cx, |_| {})
                .on_blur(|cx| cx.emit(PopupEvent::Close))
                .size(Pixels(200.0))
                .background_color(Color::red());
        });
    })
    .title("Popup")
    .run();
}
