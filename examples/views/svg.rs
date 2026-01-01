mod helpers;
use helpers::*;
use vizia::prelude::*;

fn main() -> Result<(), ApplicationError> {
    let (app, (title, size)) = Application::new_with_state(|cx| {
        let svg_data = cx.state(
            r#"<svg width="100" height="100" xmlns="http://www.w3.org/2000/svg">
                <circle cx="50" cy="50" r="30" stroke="red" stroke-width="2" fill="blue" />
                </svg>"#
                .as_bytes()
                .to_vec(),
        );
        let tiger_svg = cx.state(include_bytes!("../resources/images/Ghostscript_Tiger.svg"));
        let inline_svg = cx.state(
            r#"<svg width="100" height="100" xmlns="http://www.w3.org/2000/svg">
                <circle cx="50" cy="50" r="40" stroke="green" stroke-width="4" fill="yellow" />
                </svg>"#,
        );
        let stretch_one = cx.state(Stretch(1.0));
        let size_100 = cx.state(Pixels(100.0));
        let border_color = cx.state(Color::black());
        let border_width = cx.state(Pixels(1.0));

        ExamplePage::new(cx, move |cx| {
            Svg::new(cx, tiger_svg)
                .size(stretch_one)
                .border_color(border_color)
                .border_width(border_width);

            Svg::new(cx, inline_svg)
                .size(size_100)
                .border_color(border_color)
                .border_width(border_width);

            Svg::new(cx, svg_data)
                .size(size_100)
                .border_color(border_color)
                .border_width(border_width);
        });
        (cx.state("Svg"), cx.state((400, 200)))
    });

    app.title(title).inner_size(size).run()
}
