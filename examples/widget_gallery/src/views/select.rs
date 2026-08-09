use vizia::prelude::*;

use crate::DemoRegion;

struct SelectData {
    options: Signal<Vec<Localized>>,
    selected_option_1: Signal<Option<usize>>,
    selected_option_2: Signal<Option<usize>>,
}

pub enum SelectEvent {
    SetOption1(usize),
    SetOption2(usize),
}

impl Model for SelectData {
    fn event(&mut self, _: &mut EventContext, event: &mut Event) {
        event.map(|select_event, _| match select_event {
            SelectEvent::SetOption1(index) => {
                self.selected_option_1.set(Some(*index));
            }

            SelectEvent::SetOption2(index) => {
                self.selected_option_2.set(Some(*index));
            }
        });

        let _ = self.options;
    }
}

pub fn select(cx: &mut Context) {
    let options = Signal::new(
        ["red", "green", "blue", "yellow", "cyan", "magenta"].map(Localized::new).to_vec(),
    );
    let selected_option_1 = Signal::new(Some(0usize));
    let selected_option_2 = Signal::new(None);

    SelectData { options, selected_option_1, selected_option_2 }.build(cx);

    VStack::new(cx, |cx| {
        Label::new(cx, Localized::new("select")).class("panel-title");

        Divider::new(cx);

        DemoRegion::new(cx, Localized::new("demo-region-select"), move |cx| {
            Select::new(cx, options, selected_option_1, true)
                .on_select(|cx, index| cx.emit(SelectEvent::SetOption1(index)))
                .width(Pixels(150.0));
        });

        DemoRegion::new(cx, Localized::new("demo-region-placeholder-select"), move |cx| {
            Select::new(cx, options, selected_option_2, true)
                .placeholder(Localized::new("select-color-placeholder"))
                .on_select(|cx, index| cx.emit(SelectEvent::SetOption2(index)))
                .width(Pixels(150.0));
        });
    })
    .class("panel");
}
