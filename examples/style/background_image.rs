#[cfg(not(target_arch = "wasm32"))]
use vizia::image;
#[allow(unused)]
use vizia::prelude::*;

#[allow(unused)]
const STYLE: &str = r#"
.auto-size {
    background-image: url("sample.png");
    width: auto;
    height: auto;
}

.fixed-size {
    background-image: url("sample.png");
    width: 600px;
    height: auto;
}

.web-image {
    background-image: url("https://download.samplelib.com/png/sample-bumblebee-400x300.png");
    width: auto;
    height: auto;
}

"#;

#[cfg(target_arch = "wasm32")]
fn main() {
    panic!("This example is not supported on wasm - threads are experimental");
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    Application::new(|cx| {
        cx.add_stylesheet(STYLE).expect("Failed to add stylesheet");

        cx.set_image_loader(|cx, path|{
            if path.starts_with("https://") {
                let path = path.to_string();
                cx.spawn(move |cx| {
                    let data = reqwest::blocking::get(&path).unwrap().bytes().unwrap();
                    cx.load_image(
                        path,
                        image::load_from_memory_with_format(
                            &data,
                            image::guess_format(&data).unwrap(),
                        )
                        .unwrap(),
                        ImageRetentionPolicy::DropWhenUnusedForOneFrame,
                    )
                    .unwrap();
                });
            }
        });

        // Load an image into the binary
        cx.load_image(
            "sample.png", 
            image::load_from_memory_with_format(
            include_bytes!("../resources/images/sample-hut-400x300.png"),
            image::ImageFormat::Png,
            )
            .unwrap(),
            ImageRetentionPolicy::DropWhenUnusedForOneFrame
        );

        Label::new(cx, "Any view can be styled with a background image. An Image view can be used to present a non-tiling background image.")
            .width(Stretch(1.0))
            .position_type(PositionType::SelfDirected)
            .space(Pixels(10.0));

        Element::new(cx).class("auto-size");
        Element::new(cx).class("fixed-size");
        Element::new(cx).class("web-image");
        Image::new(cx, "https://download.samplelib.com/png/sample-bumblebee-400x300.png");
        Label::new(cx, "Wait for the image to load :)");
    })
    .title("Image")
    .run()
}
