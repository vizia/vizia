use vizia::prelude::*;

use crate::DemoRegion;

pub fn picklist(cx: &mut Context) {
    let options = cx.state(vec!["Red", "Green", "Blue", "Yellow", "Cyan", "Magenta"]);
    let selected_option_1 = cx.state(0usize);
    let selected_option_2 = cx.state(200usize);
    let width_150 = cx.state(Pixels(150.0));
    let gap_2 = cx.state(Pixels(2.0));
    let auto = cx.state(Auto);

    VStack::new(cx, |cx| {
        Markdown::new(cx, "# Picklist
A view which allows the user to select an option from a list.
        ");

        Divider::new(cx);

        Markdown::new(cx, "### Basic picklist");

        DemoRegion::new(
            cx,
            |cx| {
                VStack::new(cx, |cx| {
                    Label::new(cx, "Color:").class("field-label");
                    PickList::new(cx, options, selected_option_1, true)
                        .on_select(move |cx, index| selected_option_1.set(cx, index))
                        .width(width_150);
                })
                .gap(gap_2)
                .size(auto);
            },
r#"let options = cx.state(vec!["Red", "Green", "Blue", "Yellow", "Cyan", "Magenta"]);
let selected_option = cx.state(0usize);
let width_150 = cx.state(Pixels(150.0));
PickList::new(cx, options, selected_option, true)
    .on_select(|cx, index| selected_option.set(cx, index))
    .width(width_150);"#,
        );

        Markdown::new(cx, "### Placeholder
The placeholder text prompts a user to select an option from the picker menu when the selected index is greater than the list length. It disappears once a user selects an option.
        ");

        DemoRegion::new(
            cx,
            |cx| {
                VStack::new(cx, |cx| {
                    Label::new(cx, "Color:").class("field-label");
                    let placeholder = cx.state("Select a color...");
                    PickList::new(cx, options, selected_option_2, true)
                        .placeholder(placeholder)
                        .on_select(move |cx, index| selected_option_2.set(cx, index))
                        .width(width_150);
                })
                .gap(gap_2)
                .size(auto);
            },
r#"let options = cx.state(vec!["Red", "Green", "Blue", "Yellow", "Cyan", "Magenta"]);
let selected_option = cx.state(200usize);
let placeholder = cx.state("Select a color...");
let width_150 = cx.state(Pixels(150.0));
PickList::new(cx, options, selected_option, true)
    .placeholder(placeholder)
    .on_select(|cx, index| selected_option.set(cx, index))
    .width(width_150);"#,
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
