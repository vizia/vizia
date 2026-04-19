use vizia::prelude::*;

use crate::components::DemoRegion;

struct ComboBoxState {
    options: Signal<Vec<&'static str>>,
    selected_option: Signal<usize>,
}

pub enum ComboBoxEvent {
    SetOption(usize),
}

impl Model for ComboBoxState {
    fn event(&mut self, _: &mut EventContext, event: &mut Event) {
        event.map(|app_event, _| match app_event {
            ComboBoxEvent::SetOption(index) => {
                self.selected_option.set(*index);
            }
        });

        let _ = self.options;
    }
}

pub fn combobox(cx: &mut Context) {
    let options = Signal::new(vec![
        "One", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine", "Ten",
    ]);
    let selected_option = Signal::new(0usize);

    ComboBoxState { options, selected_option }.build(cx);

    VStack::new(cx, |cx| {
        Markdown::new(cx, "# Combobox");

        Divider::new(cx);

        DemoRegion::new(cx, "Basic Combobox", move |cx| {
            ComboBox::new(cx, options, selected_option)
                .on_select(|cx, index| cx.emit(ComboBoxEvent::SetOption(index)))
                .width(Pixels(100.0));
        });
    })
    .class("panel");
}
