mod helpers;
pub use helpers::*;
use vizia::prelude::*;

pub struct AppData {
    tabs: Signal<Vec<&'static str>>,
    selected_tab: Signal<usize>,
}

pub enum AppEvent {
    SetSelectedTab(usize),
    CloseTab(usize),
    AddTab,
    PrevTab,
    NextTab,
}

impl Model for AppData {
    fn event(&mut self, _cx: &mut EventContext, event: &mut Event) {
        event.map(|app_event, _| match app_event {
            AppEvent::SetSelectedTab(index) => self.selected_tab.set(*index),

            AppEvent::CloseTab(index) => {
                let mut tabs = self.tabs.get();
                if *index < tabs.len() {
                    tabs.remove(*index);
                    let len = tabs.len();
                    self.tabs.set(tabs);

                    if len == 0 {
                        self.selected_tab.set(0);
                    } else {
                        let current = self.selected_tab.get();
                        let next =
                            if current > *index { current.saturating_sub(1) } else { current };
                        self.selected_tab.set(next.min(len.saturating_sub(1)));
                    }
                }
            }

            AppEvent::AddTab => {
                let mut tabs = self.tabs.get();
                let new_name = format!("Tab{}", tabs.len() + 1);
                let new_tab = Box::leak(Box::new(new_name));
                tabs.push(new_tab);
                self.tabs.set(tabs);
                self.selected_tab.set(self.tabs.get().len() - 1);
            }

            AppEvent::PrevTab => {
                let len = self.tabs.get().len();
                if len > 0 {
                    let current = self.selected_tab.get();
                    let prev = if current == 0 { len - 1 } else { current - 1 };
                    self.selected_tab.set(prev);
                }
            }

            AppEvent::NextTab => {
                let len = self.tabs.get().len();
                if len > 0 {
                    let current = self.selected_tab.get();
                    let next = (current + 1) % len;
                    self.selected_tab.set(next);
                }
            }
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
            VStack::new(cx, |cx| {
                HStack::new(cx, |cx| {
                    Button::new(cx, |cx| Label::new(cx, "Add Tab").hoverable(false))
                        .on_press(|cx| cx.emit(AppEvent::AddTab));
                    Button::new(cx, |cx| Label::new(cx, "Previous").hoverable(false))
                        .on_press(|cx| cx.emit(AppEvent::PrevTab));
                    Button::new(cx, |cx| Label::new(cx, "Next").hoverable(false))
                        .on_press(|cx| cx.emit(AppEvent::NextTab));
                })
                .horizontal_gap(Pixels(8.0));

                TabView::new(cx, tabs, |_, _, item| match item {
                    "Tab1" => TabPair::new(
                        move |cx| {
                            Label::new(cx, item).hoverable(false);
                            Element::new(cx).class("indicator");
                        },
                        |cx| {
                            Element::new(cx).size(Pixels(200.0)).background_color(Color::red());
                        },
                    )
                    .closeable(false),

                    "Tab2" => TabPair::new(
                        move |cx| {
                            Label::new(cx, item).hoverable(false);
                            Element::new(cx).class("indicator");
                        },
                        |cx| {
                            Element::new(cx).size(Pixels(200.0)).background_color(Color::blue());
                        },
                    )
                    .closeable(true),

                    _ => TabPair::new(
                        move |cx| {
                            Label::new(cx, item).hoverable(false);
                            Element::new(cx).class("indicator");
                        },
                        |cx| {
                            Element::new(cx).size(Pixels(200.0)).background_color(Color::gray());
                        },
                    )
                    .closeable(true),
                })
                .with_selected(selected_tab)
                .on_select(|cx, index| cx.emit(AppEvent::SetSelectedTab(index)))
                .on_close(|cx, index| cx.emit(AppEvent::CloseTab(index)))
                .width(Pixels(500.0))
                .height(Pixels(300.0));
            })
            .vertical_gap(Pixels(8.0));
        });
    })
    .title(Localized::new("view-title-tabview"))
    .run()
}
