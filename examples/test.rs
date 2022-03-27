use vizia::*;

const LOREM_IPSUM: &str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Orci a scelerisque purus semper eget duis. Commodo elit at imperdiet dui accumsan sit amet nulla. Sit amet est placerat in egestas erat imperdiet sed. Elementum eu facilisis sed odio morbi quis commodo odio. Nullam non nisi est sit amet facilisis. Egestas integer eget aliquet nibh praesent tristique. Dui faucibus in ornare quam viverra orci. Gravida dictum fusce ut placerat orci nulla pellentesque dignissim enim. Nibh praesent tristique magna sit amet purus gravida. Est pellentesque elit ullamcorper dignissim cras tincidunt lobortis feugiat. Semper viverra nam libero justo laoreet sit amet cursus. Enim ut sem viverra aliquet eget sit.";

const STYLE: &str = r#"
label {
    border-width: 1px;
    border-color: red;
}
"#;

fn main() {
    let mut window_description = WindowDescription::new();
    window_description.resizable = true;
    Application::new(window_description, |cx| {
        cx.add_theme(STYLE);

        HStack::new(cx, |cx| {
            VStack::new(cx, |cx| {
                Label::new(cx, LOREM_IPSUM);
                Label::new(cx, LOREM_IPSUM);
            })
            .min_width(Units::Pixels(0.0))
            .width(Units::Stretch(1.0));
            VStack::new(cx, |cx| {
                Label::new(cx, LOREM_IPSUM);
                Label::new(cx, LOREM_IPSUM);
            })
            .min_width(Units::Pixels(0.0))
            .width(Units::Stretch(2.0));
        })
        .height(Units::Auto);
    })
    .run();
}
