#[allow(unused)]
use vizia::prelude::*;
#[cfg(not(target_arch = "wasm32"))]
use vizia_core::resource::ImageRetentionPolicy;

#[allow(unused)]
const STYLE: &str = r#"
element {
    background-image: "sample.png";
    width: 1s;
    height: 1s;
}
"#;

#[cfg(target_arch = "wasm32")]
fn main() {
    panic!("This example is not supported on wasm - threads are experimental");
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    Application::new(|cx| {
        cx.add_theme(STYLE);
        cx.set_image_loader(|cx, path| {
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
            } else if path == "sample.png" {
                cx.load_image(
                    path.to_owned(),
                    image::load_from_memory_with_format(
                        include_bytes!("../resources/sample-hut-400x300.png"),
                        image::ImageFormat::Png,
                    )
                    .unwrap(),
                    ImageRetentionPolicy::DropWhenUnusedForOneFrame,
                );
            } else {
                panic!();
            }
        });

        Label::new(cx, "Any view can be styled with a background image. An Image view can be used to present a non-tiling background image.")
            .width(Stretch(1.0))
            .position_type(PositionType::SelfDirected)
            .space(Pixels(10.0));

        Element::new(cx);
        Image::new(cx, "https://download.samplelib.com/png/sample-bumblebee-400x300.png");
        Label::new(cx, "Wait for the image to load :)");
    })
    .title("Image")
    .run()
}
