use vizia::image;
#[allow(unused)]
use vizia::prelude::*;

#[allow(unused)]
const STYLE: &str = r#"

:root {
    layout-type: row;
    col-between: 20px;
    child-space: 1s;
}

element {
    background-image: url("sample.png");
    width: 250px;
    height: 200px;
}

.auto {
    background-size: auto;
}

.length {
    background-size: 100px 100px;
    transition: background-size 200ms;
}

.length:hover {
    background-size: 200px 200px;
    transition: background-size 200ms;
}

.percentage {
    background-size: 100% 100%;
}

.contain {
    background-size: contain;
}

.cover {
    background-size: cover;
}

"#;

fn main() -> Result<(), ApplicationError> {
    Application::new(|cx| {
        cx.add_stylesheet(STYLE).expect("Failed to add stylesheet");

        // Load an image into the binary
        cx.load_image(
            "sample.png",
            image::load_from_memory_with_format(
                include_bytes!("../resources/images/sample-hut-400x300.png"),
                image::ImageFormat::Png,
            )
            .unwrap(),
            ImageRetentionPolicy::DropWhenUnusedForOneFrame,
        );

        Element::new(cx).class("auto");
        Element::new(cx).class("length");
        Element::new(cx).class("percentage");
        Element::new(cx).class("contain");
        Element::new(cx).class("cover");
    })
    .title("Background Size")
    .inner_size((1400, 600))
    .run()
}
