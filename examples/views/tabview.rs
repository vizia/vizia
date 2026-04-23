mod helpers;
pub use helpers::*;
use vizia::prelude::*;

pub struct AppData {
    tabs: Signal<Vec<&'static str>>,
    selected_tab: Signal<usize>,
}

pub enum AppEvent {
    SetSelectedTab(usize),
}

impl Model for AppData {
    fn event(&mut self, _cx: &mut EventContext, event: &mut Event) {
        event.map(|app_event, _| match app_event {
            AppEvent::SetSelectedTab(index) => self.selected_tab.set(*index),
        });

        let _ = self.tabs;
    }
}

fn main() -> Result<(), ApplicationError> {
    Application::new(|cx| {
        let tabs = Signal::new(vec!["Tab1", "Tab2", "Tab3", "Tab4", "Tab5", "Tab6"]);
        let selected_tab = Signal::new(0usize);

        AppData { tabs, selected_tab }.build(cx);

        ExamplePage::new(cx, |cx| {
            TabView::new(cx, tabs, |_, _, item| match item {
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
            .with_selected(selected_tab)
            .on_select(|cx, index| cx.emit(AppEvent::SetSelectedTab(index)))
            .width(Pixels(500.0))
            .height(Pixels(300.0));
        });
    })
    .title(Localized::new("view-title-tabview"))
    .run()
}
