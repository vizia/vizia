use vizia::prelude::*;

use crate::DemoRegion;

struct SelectData {
    options: Signal<Vec<Signal<&'static str>>>,
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
        ["Red", "Green", "Blue", "Yellow", "Cyan", "Magenta"].map(Signal::new).to_vec(),
    );
    let selected_option_1 = Signal::new(Some(0usize));
    let selected_option_2 = Signal::new(Some(2usize));

    SelectData { options, selected_option_1, selected_option_2 }.build(cx);

    VStack::new(cx, |cx| {
        Label::new(cx, Localized::new("select")).class("panel-title");

        Divider::new(cx);

        DemoRegion::new(cx, "Select", move |cx| {
            VStack::new(cx, |cx| {
                Label::new(cx, "Color:").class("field-label");
                Select::new(cx, options, selected_option_1, true)
                    .on_select(|cx, index| cx.emit(SelectEvent::SetOption1(index)))
                    .width(Pixels(150.0));
            })
            .gap(Pixels(2.0))
            .size(Auto);
        });

        DemoRegion::new(cx, "Placeholder Select", move |cx| {
            VStack::new(cx, |cx| {
                Label::new(cx, "Color:").class("field-label");
                Select::new(cx, options, selected_option_2, true)
                    .placeholder(String::from("Select a color..."))
                    .on_select(|cx, index| cx.emit(SelectEvent::SetOption2(index)))
                    .width(Pixels(150.0));
            })
            .gap(Pixels(2.0))
            .size(Auto);
        });
    })
    .class("panel");
}
