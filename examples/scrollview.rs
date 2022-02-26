use vizia::*;

fn main() {
    Application::new(WindowDescription::new(), |cx| {
        ScrollView::new(cx, 0.0, 0.0, true, true, |cx| {
            Label::new(cx, "Label 1").width(Units::Pixels(1000.0)).background_color(Color::green());
            Label::new(cx, "Label 2").height(Units::Pixels(1000.0)).background_color(Color::blue());
        })
        .size(Units::Pixels(300.0))
        .overflow(Overflow::Hidden);
    })
    .run();
}
