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
        Markdown::new(cx, "# Select
A view which allows the user to select an option from a list.
        ");

        Divider::new(cx);

        DemoRegion::new(
            cx,
            "Basic Select",
            move |cx| {
                VStack::new(cx, |cx| {
                    Label::new(cx, "Color:").class("field-label");
                    Select::new(cx, options, selected_option_1, true)
                        .on_select(|cx, index| cx.emit(SelectEvent::SetOption1(index)))
                        .width(Pixels(150.0));
                })
                .gap(Pixels(2.0))
                .size(Auto);
            });

        Markdown::new(cx, "### Placeholder
The placeholder text prompts a user to select an option from the picker menu when the selected index is greater than the list length. It disappears once a user selects an option.
        ");

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

        Markdown::new(cx, "## Keyboard interactions");
        Divider::new(cx);
        Markdown::new(cx, "When the select menu is closed:");
        Markdown::new(cx, "
| Key       | Interaction |
| --------- | ------- |
| Space or Enter | Opens the select menu. The focus is set on the menu item selected. |
");
        Markdown::new(cx, "When the select menu is open:");
        Markdown::new(cx, "
| Key       | Interaction |
| --------- | ------- |
| Space or Enter | Selects the list item in focus, closes the popup list and moves focus to the select button. |
| Up or Down Arrow | Moves focus to previous or next item in the popup list. |
| Esc | Closes the popoup list and moves focus to the select button. |
");

    })
    .class("panel");
}
