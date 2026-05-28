mod helpers;
pub use helpers::*;
use vizia::prelude::*;

pub struct AppData {
    tabs: Signal<Vec<&'static str>>,
    selected_tab: Signal<usize>,
    selected_vertical_tab: Signal<usize>,
}

pub enum AppEvent {
    SetSelectedTab(usize),
    SetSelectedVerticalTab(usize),
    CloseTab(usize),
}

fn adjust_selection(selected: usize, closed_index: usize, remaining: usize) -> usize {
    if remaining == 0 {
        0
    } else if closed_index == selected {
        selected.saturating_sub(1).min(remaining - 1)
    } else if closed_index < selected {
        selected.saturating_sub(1)
    } else {
        selected.min(remaining - 1)
    }
}

impl Model for AppData {
    fn event(&mut self, _cx: &mut EventContext, event: &mut Event) {
        event.map(|app_event, _| match app_event {
            AppEvent::SetSelectedTab(index) => self.selected_tab.set(*index),
            AppEvent::SetSelectedVerticalTab(index) => self.selected_vertical_tab.set(*index),
            AppEvent::CloseTab(index) => {
                let selected_tab = self.selected_tab.get();
                let selected_vertical_tab = self.selected_vertical_tab.get();

                self.tabs.update(|tabs| {
                    if *index < tabs.len() {
                        tabs.remove(*index);
                    }
                });

                let remaining = self.tabs.get().len();
                self.selected_tab.set(adjust_selection(selected_tab, *index, remaining));
                self.selected_vertical_tab.set(adjust_selection(
                    selected_vertical_tab,
                    *index,
                    remaining,
                ));
            }
        });

        let _ = self.tabs;
    }
}

fn main() -> Result<(), ApplicationError> {
    Application::new(|cx| {
        let tabs = Signal::new(vec![
            "Overview", "Files", "Search", "Problems", "Outline", "Logs", "History", "Preview",
            "Diff", "Debug",
        ]);
        let selected_tab = Signal::new(0usize);
        let selected_vertical_tab = Signal::new(0usize);

        AppData { tabs, selected_tab, selected_vertical_tab }.build(cx);

        ExamplePage::new(cx, |cx| {
            VStack::new(cx, |cx| {
                Label::new(cx, "Tab List").class("panel-title");
                Divider::new(cx);

                Label::new(
                    cx,
                    "Standalone TabList with scrollable tabs and optional close buttons.",
                )
                .hoverable(false);

                HStack::new(cx, |cx| {
                    VStack::new(cx, |cx| {
                        Label::new(cx, "Horizontal").hoverable(false);

                        TabList::new(cx, tabs, move |cx, index, item| {
                            let is_selected = selected_tab.map(move |selected| *selected == index);

                            if index != 0 {
                                Tab::new(cx, item)
                                    .checked(is_selected)
                                    .on_press(move |cx| cx.emit(AppEvent::SetSelectedTab(index)))
                                    .on_close(move |cx| cx.emit(AppEvent::CloseTab(index)));
                            } else {
                                Tab::new(cx, item)
                                    .checked(is_selected)
                                    .on_press(move |cx| cx.emit(AppEvent::SetSelectedTab(index)));
                            }
                        })
                        .width(Pixels(400.0))
                        .height(Auto);
                    })
                    .gap(Pixels(8.0))
                    .width(Auto)
                    .height(Auto);

                    VStack::new(cx, |cx| {
                        Label::new(cx, "Vertical").hoverable(false);

                        TabList::new(cx, tabs, move |cx, index, item| {
                            let is_selected =
                                selected_vertical_tab.map(move |selected| *selected == index);

                            if index != 0 {
                                Tab::new(cx, item)
                                    .checked(is_selected)
                                    .on_press(move |cx| {
                                        cx.emit(AppEvent::SetSelectedVerticalTab(index))
                                    })
                                    .on_close(move |cx| cx.emit(AppEvent::CloseTab(index)));
                            } else {
                                Tab::new(cx, item).checked(is_selected).on_press(move |cx| {
                                    cx.emit(AppEvent::SetSelectedVerticalTab(index))
                                });
                            }
                        })
                        .vertical(true)
                        .height(Pixels(150.0));
                    })
                    .gap(Pixels(8.0))
                    .width(Auto)
                    .height(Auto);
                })
                .gap(Pixels(24.0));
            })
            .gap(Pixels(12.0))
            .width(Stretch(1.0))
            .height(Auto);
        });
    })
    .title("Tab List")
    .run()
}
