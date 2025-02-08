use vizia::prelude::*;

const STYLE: &str = r#"
    .grid-test {
        layout-type: grid;
        grid-columns: 200px 200px;
        grid-rows: 150px 100px;
    }
"#;

fn main() -> Result<(), ApplicationError> {
    Application::new(|cx| {
        cx.add_stylesheet(STYLE).expect("Failed to load stylesheet");

        Grid::new(
            cx,
            vec![Pixels(200.0), Pixels(200.0)],
            vec![Pixels(150.0), Pixels(100.0)],
            |cx| {
                Element::new(cx).column_start(0).row_start(0).background_color(Color::red());
                Element::new(cx).column_start(1).row_start(0).background_color(Color::blue());
                Element::new(cx).column_start(0).row_start(1).background_color(Color::green());
                Element::new(cx).column_start(1).row_start(1).background_color(Color::yellow());
            },
        );

        VStack::new(cx, |cx| {
            Element::new(cx).column_start(0).row_start(0).background_color(Color::red());
            Element::new(cx).column_start(1).row_start(0).background_color(Color::blue());
            Element::new(cx).column_start(0).row_start(1).background_color(Color::green());
            Element::new(cx).column_start(1).row_start(1).background_color(Color::yellow());
        })
        .class("grid-test");
    })
    .run()
}
