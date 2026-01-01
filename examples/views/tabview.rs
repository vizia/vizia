mod helpers;
pub use helpers::*;
use vizia::prelude::*;

fn main() -> Result<(), ApplicationError> {
    let (app, title) = Application::new_with_state(|cx| {
        let tabs = cx.state(vec!["Tab1", "Tab2", "Tab3", "Tab4", "Tab5", "Tab6"]);
        let no_hover = cx.state(false);
        let size_200 = cx.state(Pixels(200.0));
        let width_500 = cx.state(Pixels(500.0));
        let height_300 = cx.state(Pixels(300.0));
        let color_red = cx.state(Color::red());
        let color_blue = cx.state(Color::blue());
        let color_gray = cx.state(Color::gray());

        ExamplePage::new(cx, move |cx| {
            TabView::new(cx, tabs, move |cx, item| match *item.get(cx) {
                "Tab1" => TabPair::new(
                    move |cx| {
                        Label::new(cx, item).hoverable(no_hover);
                        Element::new(cx).class("indicator");
                    },
                    move |cx| {
                        Element::new(cx).size(size_200).background_color(color_red);
                    },
                ),

                "Tab2" => TabPair::new(
                    move |cx| {
                        Label::new(cx, item).hoverable(no_hover);
                        Element::new(cx).class("indicator");
                    },
                    move |cx| {
                        Element::new(cx).size(size_200).background_color(color_blue);
                    },
                ),

                _ => TabPair::new(
                    move |cx| {
                        Label::new(cx, item).hoverable(no_hover);
                        Element::new(cx).class("indicator");
                    },
                    move |cx| {
                        Element::new(cx).size(size_200).background_color(color_gray);
                    },
                ),
            })
            .width(width_500)
            .height(height_300);
        });
        cx.state("Tabview")
    });

    app.title(title).run()
}
