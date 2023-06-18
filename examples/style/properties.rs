use lazy_static::lazy_static;
use vizia::prelude::*;

lazy_static! {
    pub static ref STATIC_LIST: Vec<&'static str> = vec!["Background Image", "Background Size", "Border", "Box Shadow"];
}

const BACKGROUND_IMAGE_STYLE: &str = r#"
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

const BACKGROUND_SIZE_STYLE: &str = r#"
    .container {
        row-between: 20px;
        child-space: 1s;
    }

    .image_element {
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

const BORDER_STYLE: &str = r#"

    .row {
        child-space: 1s;
        col-between: 20px;
    }

    .border_element {
        size: 100px;
        background-color: rgb(200, 200, 200);
    }

    .border {
        border: black 5px;
    }

    .border:hover {
        border: 10px blue;
        transition: border 0.1s;
    }

    .border_radius {
        border-radius: 5px 10px 15px 20px;
    }

    .border_radius:hover {
        border-radius: 10px 20px 30px 40px;
        transition: border-radius 0.1s;
    }

    .border_shape {
        border-radius: 30px;
        border-corner-shape: round bevel round bevel;
    }

    .border_shape:hover {
        border-radius: 30px;
        border-corner-shape: bevel round bevel round;
    }
"#;

const STYLE: &str = r#"

    :root {
        child-space: 1s;
    }

    hstack {
        child-space: 1s;
        col-between: 40px;
    }

    .shadow_element {
        size: 100px;
        background-color: rgb(200, 200, 200);
    }

    .shadow-offsetx {
        box-shadow: 5px 0px red;
    }

    .shadow-offsetx:hover {
        box-shadow: 15px 0px red;
        transition: box-shadow 100ms;
    }

    .shadow-offsety {
        box-shadow: 0px 5px red;
    }

    .shadow-offsety:hover {
        box-shadow: 0px 15px red;
        transition: box-shadow 100ms;
    }

    .shadow-offset {
        box-shadow: 5px 5px red;
    }

    .shadow-offset:hover {
        box-shadow: 15px 15px red;
        transition: box-shadow 100ms;
    }

    .shadow-color {
        box-shadow: 5px 5px red;
    }

    .shadow-color:hover {
        box-shadow: 5px 5px blue;
        transition: box-shadow 200ms;
    }

    .shadow-blur {
        box-shadow: 5px 5px 5px red;
    }

    .shadow-blur:hover {
        box-shadow: 5px 5px 15px red;
        transition: box-shadow 100ms;
    }
    
    .shadow {
        box-shadow: 5px 5px blue, 10px 10px red, 15px 15px green;
    }

    .shadow:hover {
        box-shadow: 10px 10px 16px 8px blue, 20px 20px 16px 8px red, 30px 30px 16px 8px green;
        transition: box-shadow 200ms;
    }

    .shadow-inset-offsetx {
        box-shadow: 5px 0px red inset;
    }

    .shadow-inset-offsetx:hover {
        box-shadow: 15px 0px red inset;
        transition: box-shadow 100ms;
    }

    .shadow-inset-offsety {
        box-shadow: 0px 5px red inset;
    }

    .shadow-inset-offsety:hover {
        box-shadow: 0px 15px red inset;
        transition: box-shadow 100ms;
    }

    .shadow-inset-offset {
        box-shadow: 5px 5px red inset;
    }

    .shadow-inset-offset:hover {
        box-shadow: 15px 15px red inset;
        transition: box-shadow 100ms;
    }

    .shadow-inset-color {
        box-shadow: 5px 5px red inset;
    }

    .shadow-inset-color:hover {
        box-shadow: 5px 5px blue inset;
        transition: box-shadow 200ms;
    }

    .shadow-inset-blur {
        box-shadow: 5px 5px 5px red inset;
    }

    .shadow-inset-blur:hover {
        box-shadow: 5px 5px 15px red inset;
        transition: box-shadow 100ms;
    }

    .shadow-inset {
        box-shadow: 5px 5px blue inset, 10px 10px red inset, 15px 15px green inset;
    }

    .shadow-inset:hover {
        box-shadow: 10px 10px 16px blue inset, 20px 20px 16px red inset, 30px 30px 16px green inset;
        transition: box-shadow 200ms;
    }
"#;

fn main() {
    Application::new(|cx| {
        cx.set_image_loader(|cx, name|{
            if name == "sample.png" {
                // Load an image into the binary
                cx.load_image(
                    String::from("sample.png"),
                    image::load_from_memory_with_format(
                        include_bytes!("../resources/images/sample-hut-400x300.png"),
                        image::ImageFormat::Png,
                    )
                    .unwrap(),
                    ImageRetentionPolicy::DropWhenUnusedForOneFrame,
                );
            }
        });

        cx.add_stylesheet(BACKGROUND_IMAGE_STYLE).expect("Failed to add stylesheet");
        cx.add_stylesheet(BACKGROUND_SIZE_STYLE).expect("Failed to add stylesheet");
        cx.add_stylesheet(BORDER_STYLE).expect("Failed to add stylesheet");

        TabView::new(cx, StaticLens::new(STATIC_LIST.as_ref()), |cx, item| match item.get(cx) {
            "Background Image" => TabPair::new(
                move |cx| {
                    Label::new(cx, item).hoverable(false);
                    Element::new(cx).class("indicator");
                },
                |cx| {
                    background_image(cx);
                },
            ),

            "Background Size" => TabPair::new(
                move |cx| {
                    Label::new(cx, item).hoverable(false);
                    Element::new(cx).class("indicator");
                },
                |cx| {
                    background_size(cx);
                },
            ),

            "Border" => TabPair::new(
                move |cx| {
                    Label::new(cx, item).hoverable(false);
                    Element::new(cx).class("indicator");
                },
                |cx| {
                    border(cx);
                },
            ),

            "Box Shadow" => TabPair::new(
                move |cx| {
                    Label::new(cx, item).hoverable(false);
                    Element::new(cx).class("indicator");
                },
                |cx| {
                    box_shadow(cx);
                },
            ),

            _ => unreachable!(),
        })
        .class("vertical");
    })
    .run();
}

fn background_image(cx: &mut Context) {
    VStack::new(cx, |cx|{
        Label::new(cx, "Any view can be styled with a background image. An Image view can be used to present a non-tiling background image.")
                .width(Stretch(1.0))
                .position_type(PositionType::SelfDirected)
                .space(Pixels(10.0));
    
        Element::new(cx).class("auto-size");
        Element::new(cx).class("fixed-size");
        Element::new(cx).class("web-image");
        Image::new(cx, "https://download.samplelib.com/png/sample-bumblebee-400x300.png");
        Label::new(cx, "Wait for the image to load :)");
    }).class("container");
}

fn background_size(cx: &mut Context) {
    VStack::new(cx, |cx|{
        Element::new(cx).class("auto").class("image_element");
        Element::new(cx).class("length").class("image_element");
        Element::new(cx).class("percentage").class("image_element");
        Element::new(cx).class("contain").class("image_element");
        Element::new(cx).class("cover").class("image_element");
    }).class("container");
}

fn border(cx: &mut Context) {
    HStack::new(cx, |cx| {
        Element::new(cx).class("border").class("border_element");
        Element::new(cx).class("border_radius").class("border_element");
        Element::new(cx).class("border_shape").class("border_element");
    })
    .class("row");
}

fn box_shadow(cx: &mut Context) {
    HStack::new(cx, |cx| {
        Element::new(cx).class("shadow-offsetx");
        Element::new(cx).class("shadow-offsety");
        Element::new(cx).class("shadow-offset");
        Element::new(cx).class("shadow-color");
        Element::new(cx).class("shadow-blur");
        Element::new(cx).class("shadow");
    });

    HStack::new(cx, |cx| {
        Element::new(cx).class("shadow-inset-offsetx");
        Element::new(cx).class("shadow-inset-offsety");
        Element::new(cx).class("shadow-inset-offset");
        Element::new(cx).class("shadow-inset-color");
        Element::new(cx).class("shadow-inset-blur");
        Element::new(cx).class("shadow-inset");
    });
}