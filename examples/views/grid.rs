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
        let col0 = cx.state(0);
        let col1 = cx.state(1);
        let row0 = cx.state(0);
        let row1 = cx.state(1);
        let red = cx.state(Color::red());
        let blue = cx.state(Color::blue());
        let green = cx.state(Color::green());
        let yellow = cx.state(Color::yellow());

        Grid::new(
            cx,
            vec![Pixels(200.0), Pixels(200.0)],
            vec![Pixels(150.0), Pixels(100.0)],
            |cx| {
                Element::new(cx)
                    .column_start(col0)
                    .row_start(row0)
                    .background_color(red);
                Element::new(cx)
                    .column_start(col1)
                    .row_start(row0)
                    .background_color(blue);
                Element::new(cx)
                    .column_start(col0)
                    .row_start(row1)
                    .background_color(green);
                Element::new(cx)
                    .column_start(col1)
                    .row_start(row1)
                    .background_color(yellow);
            },
        );

        VStack::new(cx, |cx| {
            Element::new(cx)
                .column_start(col0)
                .row_start(row0)
                .background_color(red);
            Element::new(cx)
                .column_start(col1)
                .row_start(row0)
                .background_color(blue);
            Element::new(cx)
                .column_start(col0)
                .row_start(row1)
                .background_color(green);
            Element::new(cx)
                .column_start(col1)
                .row_start(row1)
                .background_color(yellow);
        })
        .class("grid-test");
    })
    .run()
}
