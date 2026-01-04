mod helpers;
pub use helpers::*;
use vizia::prelude::*;

fn main() -> Result<(), ApplicationError> {
    TabviewApp::run()
}

struct TabviewApp {
    tabs: Signal<Vec<&'static str>>,
}

impl App for TabviewApp {
    fn new(cx: &mut Context) -> Self {
        Self {
            tabs: cx.state(vec!["Tab1", "Tab2", "Tab3", "Tab4", "Tab5", "Tab6"]),
        }
    }

    fn on_build(self, cx: &mut Context) -> Self {
        let tabs = self.tabs;

        ExamplePage::new(cx, move |cx| {
            TabView::new(cx, tabs, move |cx, item| match *item.get(cx) {
                "Tab1" => TabPair::new(
                    move |cx| {
                        Label::new(cx, item).hoverable(false);
                        Element::new(cx).class("indicator");
                    },
                    |cx| {
                        Element::new(cx).size(Pixels(200.0)).background_color(Color::red());
                    },
                ),

                "Tab2" => TabPair::new(
                    move |cx| {
                        Label::new(cx, item).hoverable(false);
                        Element::new(cx).class("indicator");
                    },
                    |cx| {
                        Element::new(cx).size(Pixels(200.0)).background_color(Color::blue());
                    },
                ),

                _ => TabPair::new(
                    move |cx| {
                        Label::new(cx, item).hoverable(false);
                        Element::new(cx).class("indicator");
                    },
                    |cx| {
                        Element::new(cx).size(Pixels(200.0)).background_color(Color::gray());
                    },
                ),
            })
            .width(Pixels(500.0))
            .height(Pixels(300.0));
        });
        self
    }

    fn window_config(&self) -> WindowConfig {
        window(|app| app.title("Tabview"))
    }
}
