use vizia::prelude::*;

use crate::DemoRegion;

pub struct TabData {
    tabs: Signal<Vec<&'static str>>,
    selected_tab: Signal<usize>,
}

pub enum TabEvent {
    SetSelected(usize),
    CloseTab(usize),
    AddTab,
    PrevTab,
    NextTab,
}

impl Model for TabData {
    fn event(&mut self, _cx: &mut EventContext, event: &mut Event) {
        event.map(|tab_event, _| match tab_event {
            TabEvent::SetSelected(index) => self.selected_tab.set(*index),

            TabEvent::CloseTab(index) => {
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

            TabEvent::AddTab => {
                let mut tabs = self.tabs.get();
                let new_name = format!("Tab{}", tabs.len() + 1);
                let new_tab = Box::leak(Box::new(new_name));
                tabs.push(new_tab);
                self.tabs.set(tabs);
                self.selected_tab.set(self.tabs.get().len() - 1);
            }

            TabEvent::PrevTab => {
                let len = self.tabs.get().len();
                if len > 0 {
                    let current = self.selected_tab.get();
                    let prev = if current == 0 { len - 1 } else { current - 1 };
                    self.selected_tab.set(prev);
                }
            }

            TabEvent::NextTab => {
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

pub fn tabview(cx: &mut Context) {
    let tabs = Signal::new(vec!["Tab1", "Tab2"]);
    let selected_tab = Signal::new(0usize);
    TabData { tabs, selected_tab }.build(cx);

    VStack::new(cx, |cx| {
        Label::new(cx, Localized::new("tabview")).class("panel-title");

        Divider::new(cx);

        DemoRegion::new(cx, "TabView", move |cx| {
            VStack::new(cx, |cx| {
                HStack::new(cx, |cx| {
                    Button::new(cx, |cx| Label::new(cx, "Add Tab").hoverable(false))
                        .on_press(|cx| cx.emit(TabEvent::AddTab));
                    Button::new(cx, |cx| Label::new(cx, "Previous").hoverable(false))
                        .on_press(|cx| cx.emit(TabEvent::PrevTab));
                    Button::new(cx, |cx| Label::new(cx, "Next").hoverable(false))
                        .on_press(|cx| cx.emit(TabEvent::NextTab));
                })
                .height(Auto)
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
                            Element::new(cx)
                                .size(Pixels(200.0))
                                .background_color(Color::rgb(60, 160, 80));
                        },
                    )
                    .closeable(true),
                })
                .with_selected(selected_tab)
                .on_select(|cx, index| cx.emit(TabEvent::SetSelected(index)))
                .on_close(|cx, index| cx.emit(TabEvent::CloseTab(index)))
                .width(Pixels(300.0))
                .height(Pixels(300.0));
            })
            .height(Auto)
            .vertical_gap(Pixels(8.0));
        });
    })
    .class("panel");
}
