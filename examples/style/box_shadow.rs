use vizia::prelude::*;

const STYLE: &str = r#"
    element {
        size: 200px;
        left: 100px;
        top: 100px;
        background-color: cyan;
        box-shadow: 5px 5px blue, 10px 10px red, 15px 15px green;
    }

    element:hover {
        box-shadow: 10px 10px 16px blue, 20px 20px 16px red, 30px 30px 16px green;
        transition: box-shadow 0.5s;
    }
"#;

fn main() {
    Application::new(|cx| {
        cx.add_theme(STYLE);
        Element::new(cx);
    })
    .run();
}
