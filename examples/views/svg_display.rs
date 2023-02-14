use vizia::prelude::*;

fn main() {
    Application::new(|cx| {
        SvgDisplay::new(
            cx,
            SvgTree::from_str(
                include_str!("../resources/heroicons_cake.svg"),
                &SvgOptions::default(),
            ),
        );
    })
    .title("Svg Display")
    .run();
}
