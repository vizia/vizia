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

// Example showing how to set a custom property on a view
fn main() {
    Application::new(WindowDescription::new().with_title("Test"), |cx| {
        cx.add_theme(STYLE);

        Element::new(cx).class("test").width(Pixels(100.0)).height(Pixels(100.0)).rotate(30.0);
        // VStack::new().build(cx, |cx| {
        //     Label::new("Hello").build(cx);
        //     Label::new("World").build(cx);
        // });
        // VStack::new(cx, |cx|{
        //     HStack::new(cx, |cx|{
        //         Label::new(cx, "Hello");
        //         Label::new(cx, "World");

        //         VStack::new(cx, |cx|{
        //             Label::new(cx, "Hello");
        //             Label::new(cx, "World");
        //         });

        //     }).width(Pixels(200.0)).height(Pixels(200.0)).background_color(Color::green()).custom_prop(cx, 3.14);
        // });
    })
    .run();
}
