use vizia::prelude::*;

fn main() {
    Application::new(|cx| {
        SvgDisplay::new(
            cx,
            SvgTree::from_str(
                include_str!("../resources/heroicons_cake.svg"),
                &SvgOptions::default(),
                None,
            ),
        )
        .size(Pixels(200.));

        SvgDisplay::new(
            cx,
            SvgTree::from_str(
                include_str!("../resources/heroicons_cake.svg"),
                &SvgOptions::default(),
                Some(Color::green()),
            ),
        )
        .size(Pixels(200.));
    })
    .title("Svg Display")
    .run();
}
