mod helpers;
use helpers::*;
use vizia::prelude::*;

const RUST_LOGO_URL: &str = "https://raw.githubusercontent.com/rust-lang/www.rust-lang.org/main/static/images/rust-logo-blk.svg";
const BROKEN_IMAGE_URL: &str = "https://example.invalid/missing-image.png";
const RUST_LOGO_NAME: &str = "rust-logo";
const BROKEN_IMAGE_NAME: &str = "broken-image";
const SAMPLE_HUT_PATH: &str = "examples/resources/images/sample-hut-400x300.png";
const SAMPLE_HUT_NAME: &str = "sample-hut";

fn main() -> Result<(), ApplicationError> {
    Application::new(|cx| {
        let rust_status =
            cx.add_image_encoded(RUST_LOGO_NAME, RUST_LOGO_URL, ImageRetentionPolicy::Forever);
        let broken_status = cx.add_image_encoded(
            BROKEN_IMAGE_NAME,
            BROKEN_IMAGE_URL,
            ImageRetentionPolicy::DropWhenNoObservers,
        );
        let sample_status = cx.add_image_encoded(
            SAMPLE_HUT_NAME,
            SAMPLE_HUT_PATH,
            ImageRetentionPolicy::DropWhenNoObservers,
        );

        ExamplePage::vertical(cx, |cx| {
            HStack::new(cx, |cx| {
                Binding::new(cx, rust_status, move |cx| match rust_status.get() {
                    LoadingStatus::Loading => {
                        VStack::new(cx, |cx| {
                            Label::new(cx, "Loading...").color(Color::rgb(100, 100, 100));
                        })
                        .width(Pixels(300.0))
                        .height(Pixels(300.0))
                        .background_color(Color::rgb(240, 240, 240));
                    }

                    LoadingStatus::Error => {
                        VStack::new(cx, |cx| {
                            Label::new(cx, "Failed to load").color(Color::rgb(220, 53, 69));
                        })
                        .width(Pixels(300.0))
                        .height(Pixels(300.0))
                        .background_color(Color::rgb(240, 240, 240));
                    }

                    _ => {
                        Image::new(cx, RUST_LOGO_NAME)
                            .width(Pixels(300.0))
                            .height(Pixels(300.0))
                            .background_size(vec![BackgroundSize::Contain])
                            .background_position(vec![Position::center()])
                            .background_color(Color::rgb(240, 240, 240));
                    }
                });

                Binding::new(cx, broken_status, move |cx| match broken_status.get() {
                    LoadingStatus::Loading => {
                        VStack::new(cx, |cx| {
                            Label::new(cx, "Loading...").color(Color::rgb(100, 100, 100));
                        })
                        .width(Pixels(300.0))
                        .height(Pixels(300.0))
                        .background_color(Color::rgb(240, 240, 240));
                    }

                    LoadingStatus::Error => {
                        VStack::new(cx, |cx| {
                            Label::new(cx, "Failed to load").color(Color::rgb(220, 53, 69));
                            Label::new(cx, "Check your internet connection")
                                .font_size(12.0)
                                .color(Color::rgb(100, 100, 100));
                        })
                        .width(Pixels(300.0))
                        .height(Pixels(300.0))
                        .background_color(Color::rgb(240, 240, 240));
                    }

                    _ => {
                        Image::new(cx, BROKEN_IMAGE_NAME)
                            .width(Pixels(300.0))
                            .height(Pixels(300.0))
                            .background_color(Color::rgb(240, 240, 240));
                    }
                });

                Binding::new(cx, sample_status, move |cx| match sample_status.get() {
                    LoadingStatus::Loading => {
                        VStack::new(cx, |cx| {
                            Label::new(cx, "Loaded from file").color(Color::rgb(80, 80, 80));
                            Label::new(cx, "Loading...").color(Color::rgb(100, 100, 100));
                        })
                        .width(Pixels(300.0))
                        .height(Pixels(300.0))
                        .background_color(Color::rgb(240, 240, 240));
                    }

                    LoadingStatus::Error => {
                        VStack::new(cx, |cx| {
                            Label::new(cx, "Loaded from file").color(Color::rgb(80, 80, 80));
                            Label::new(cx, "Failed to load").color(Color::rgb(220, 53, 69));
                        })
                        .width(Pixels(300.0))
                        .height(Pixels(300.0))
                        .background_color(Color::rgb(240, 240, 240));
                    }

                    _ => {
                        VStack::new(cx, |cx| {
                            Label::new(cx, "Loaded from file").color(Color::rgb(80, 80, 80));
                            Image::new(cx, SAMPLE_HUT_NAME)
                                .width(Pixels(300.0))
                                .height(Pixels(300.0))
                                .background_size(vec![BackgroundSize::Contain])
                                .background_position(vec![Position::center()])
                                .background_color(Color::rgb(240, 240, 240));
                        })
                        .gap(Pixels(8.0));
                    }
                });
            })
            .gap(Pixels(15.0))
            .padding(Pixels(20.0))
            .width(Stretch(1.0));
        });
    })
    .title("Image View Example")
    .run()
}
