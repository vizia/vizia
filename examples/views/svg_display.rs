use vizia::prelude::*;

fn main() {
    Application::new(|cx| {
        SvgDisplay::new(
            cx,
            SvgTree::from_str(
                include_str!("../resources/heroicons_cake.svg"),
                &SvgOptions::default(),
            ),
        )
        .size(Pixels(200.));

        SvgDisplay::new(
            cx,
            SvgTree::from_str(
                include_str!("../resources/heroicons_cake.svg"),
                &SvgOptions::default(),
            ),
        )
        .size(Pixels(200.));
    })
    .title("Svg Display")
    .run();
}
