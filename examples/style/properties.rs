use lazy_static::lazy_static;
use vizia::prelude::*;

lazy_static! {
    pub static ref STATIC_LIST: Vec<&'static str> = vec!["Background Image", "Background Size"];
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

    .element {
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

fn main() {
    Application::new(|cx| {
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

        cx.add_stylesheet(BACKGROUND_IMAGE_STYLE).expect("Failed to add stylesheet");
        cx.add_stylesheet(BACKGROUND_SIZE_STYLE).expect("Failed to add stylesheet");

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
        Element::new(cx).class("auto").class("element");
        Element::new(cx).class("length").class("element");
        Element::new(cx).class("percentage").class("element");
        Element::new(cx).class("contain").class("element");
        Element::new(cx).class("cover").class("element");
    }).class("container");

}
