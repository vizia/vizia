mod helpers;
use helpers::*;
use vizia::prelude::*;

#[derive(Clone, Lens)]
struct AppData {
    svg_data: Vec<u8>,
}
impl Model for AppData {}

fn main() -> Result<(), ApplicationError> {
    Application::new(|cx| {
        AppData {
            svg_data: r#"<svg width="100" height="100" xmlns="http://www.w3.org/2000/svg">
                <circle cx="50" cy="50" r="30" stroke="red" stroke-width="2" fill="blue" />
                </svg>"#
                .as_bytes()
                .to_vec(),
        }
        .build(cx);

        ExamplePage::new(cx, |cx| {
            Svg::new(cx, *include_bytes!("../resources/images/Ghostscript_Tiger.svg"))
                .size(Stretch(1.0))
                .border_color(Color::black())
                .border_width(Pixels(1.0));

            Svg::new(
                cx,
                r#"<svg width="100" height="100" xmlns="http://www.w3.org/2000/svg">
                <circle cx="50" cy="50" r="40" stroke="green" stroke-width="4" fill="yellow" />
                </svg>"#,
            )
            .size(Pixels(100.0))
            .border_color(Color::black())
            .border_width(Pixels(1.0));

            Svg::new(cx, AppData::svg_data)
                .size(Pixels(100.0))
                .border_color(Color::black())
                .border_width(Pixels(1.0));
        });
    })
    .title("Svg")
    .inner_size((400, 200))
    .run()
}
