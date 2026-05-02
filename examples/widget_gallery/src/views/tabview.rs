use vizia::prelude::*;

use crate::DemoRegion;

pub struct TabData {
    tabs: Signal<Vec<&'static str>>,
    selected_tab: Signal<usize>,
}

pub enum TabEvent {
    SetSelected(usize),
}

impl Model for TabData {
    fn event(&mut self, _cx: &mut EventContext, event: &mut Event) {
        event.map(|tab_event, _| match tab_event {
            TabEvent::SetSelected(index) => self.selected_tab.set(*index),
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

                _ => unreachable!(),
            })
            .with_selected(selected_tab)
            .on_select(|cx, index| cx.emit(TabEvent::SetSelected(index)))
            .width(Pixels(300.0))
            .height(Pixels(300.0));
        });
    })
    .class("panel");
}
