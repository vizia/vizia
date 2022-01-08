use vizia::*;

const STYLE: &str = r#"
    .test {
        space: 100px;
        background-color: green;
    }

    .test:hover {
        background-color: red;
    }
"#;

#[derive(Lens)]
pub struct AppData {
    color: Color,
}

impl Model for AppData {

}

// Example showing how to set a custom property on a view
fn main() {
    Application::new(WindowDescription::new().with_title("Test"), |cx| {
        cx.add_theme(STYLE);

        AppData {
            color: Color::red(),
        }.build(cx);

        Binding::new(cx, AppData::color, |cx, color|{
            Label::new(cx, "Test").custom(color);
            HStack::new(cx, move |cx|{
                Label::new(cx, "Test").custom(color);
            });
            Label::new(cx, "Test").custom(Color::green());
        });
    })
    .run();
}
