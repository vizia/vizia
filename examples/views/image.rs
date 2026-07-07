mod helpers;
use helpers::*;
use vizia::prelude::*;

fn main() -> Result<(), ApplicationError> {
    Application::new(|cx| {
        let rust_status = Signal::new(LoadingStatus::Loaded);
        let broken_status = Signal::new(LoadingStatus::Error);

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
                        Image::new(
                            cx,
                            "https://raw.githubusercontent.com/rust-lang/www.rust-lang.org/main/static/images/rust-logo-blk.svg",
                        )
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
                        Image::new(cx, "https://example.invalid/missing-image.png")
                            .width(Pixels(300.0))
                            .height(Pixels(300.0))
                            .background_color(Color::rgb(240, 240, 240));
                    }
                });

                VStack::new(cx, |cx| {
                    Label::new(cx, "Loaded from file").color(Color::rgb(80, 80, 80));
                    Image::new(cx, "examples/resources/images/sample-hut-400x300.png")
                        .width(Pixels(300.0))
                        .height(Pixels(300.0))
                        .background_size(vec![BackgroundSize::Contain])
                        .background_position(vec![Position::center()])
                        .background_color(Color::rgb(240, 240, 240));
                })
                .gap(Pixels(8.0));
            })
            .gap(Pixels(15.0))
            .padding(Pixels(20.0))
            .width(Stretch(1.0));
        });
    })
    .title("Image View Example")
    .run()
}
