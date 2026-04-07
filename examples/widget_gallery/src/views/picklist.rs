use vizia::prelude::*;

use crate::DemoRegion;

struct PicklistData {
    options: Signal<Vec<Signal<&'static str>>>,
    selected_option_1: Signal<usize>,
    selected_option_2: Signal<usize>,
}

pub enum PicklistEvent {
    SetOption1(usize),
    SetOption2(usize),
}

impl Model for PicklistData {
    fn event(&mut self, _: &mut EventContext, event: &mut Event) {
        event.map(|picklist_event, _| match picklist_event {
            PicklistEvent::SetOption1(index) => {
                self.selected_option_1.set(*index);
            }

            PicklistEvent::SetOption2(index) => {
                self.selected_option_2.set(*index);
            }
        });

        let _ = self.options;
    }
}

pub fn picklist(cx: &mut Context) {
    let options = Signal::new(
        ["Red", "Green", "Blue", "Yellow", "Cyan", "Magenta"].map(Signal::new).to_vec(),
    );
    let selected_option_1 = Signal::new(0usize);
    let selected_option_2 = Signal::new(200usize);

    PicklistData { options, selected_option_1, selected_option_2 }.build(cx);

    VStack::new(cx, |cx| {
        Markdown::new(cx, "# Picklist
A view which allows the user to select an option from a list.
        ");

        Divider::new(cx);

        Markdown::new(cx, "### Basic picklist");

        DemoRegion::new(
            cx,
            move |cx| {
                VStack::new(cx, |cx| {
                    Label::new(cx, "Color:").class("field-label");
                    PickList::new(cx, options, selected_option_1, true)
                        .on_select(|cx, index| cx.emit(PicklistEvent::SetOption1(index)))
                        .width(Pixels(150.0));
                })
                .gap(Pixels(2.0))
                .size(Auto);
            },
                r#"PickList::new(cx, options, selected_option_1, true)
            .on_select(|cx, index| cx.emit(PicklistEvent::SetOption1(index)))
            .width(Pixels(150.0));"#,
        );

        Markdown::new(cx, "### Placeholder
The placeholder text prompts a user to select an option from the picker menu when the selected index is greater than the list length. It disappears once a user selects an option.
        ");

        DemoRegion::new(
            cx,
            move |cx| {
                VStack::new(cx, |cx| {
                    Label::new(cx, "Color:").class("field-label");
                    PickList::new(cx, options, selected_option_2, true)
                        .placeholder(String::from("Select a color..."))
                        .on_select(|cx, index| cx.emit(PicklistEvent::SetOption2(index)))
                        .width(Pixels(150.0));
                })
                .gap(Pixels(2.0))
                .size(Auto);
            },
                r#"PickList::new(cx, options, selected_option_2, true)
            .placeholder(String::from("Select a color..."))
            .on_select(|cx, index| cx.emit(PicklistEvent::SetOption2(index)))
            .width(Pixels(150.0));"#,
        );

        Markdown::new(cx, "## Keyboard interactions");
        Divider::new(cx);
        Markdown::new(cx, "When the picklist menu is closed:");
        Markdown::new(cx, "
| Key       | Interaction |
| --------- | ------- |
| Space or Enter | Opens the picklist menu. The focus is set on the menu item selected. |
");
        Markdown::new(cx, "When the picklist menu is open:");
        Markdown::new(cx, "
| Key       | Interaction |
| --------- | ------- |
| Space or Enter | Selects the list item in focus, closes the popup list and moves focus to the picklist button. |
| Up or Down Arrow | Moves focus to previous or next item in the popup list. |
| Esc | Closes the popoup list and moves focus to the picklist button. |
");

    })
    .class("panel");
}
