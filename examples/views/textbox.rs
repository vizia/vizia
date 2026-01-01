mod helpers;
use helpers::*;
use vizia::icons::ICON_SEARCH;
use vizia::prelude::*;

fn main() -> Result<(), ApplicationError> {
    let (app, title) = Application::new_with_state(|cx| {
        let editable_text = cx.state("Editable text".to_string());
        let multiline_text =
            cx.state("This is some text which is editable and spans multiple lines".to_string());
        let non_editable_text = cx.state("This text can be selected but not edited".to_string());
        let non_editable_multiline_text = cx
            .state("This text can be selected but not edited and spans multiple lines".to_string());
        let width_300 = cx.state(Pixels(300.0));
        let stretch_one = cx.state(Stretch(1.0));
        let auto = cx.state(Auto);
        let type_placeholder = cx.state("Type something...");
        let search_placeholder = cx.state("Search");
        let icon_search = cx.state(ICON_SEARCH);
        let gray = cx.state(Color::gray());
        let position_absolute = cx.state(PositionType::Absolute);
        let read_only = cx.state(true);

        ExamplePage::vertical(cx, |cx| {
            Textbox::new(cx, editable_text)
                .width(width_300)
                .placeholder(type_placeholder)
                .on_edit(move |cx, text| editable_text.set(cx, text));

            HStack::new(cx, |cx| {
                Textbox::new(cx, editable_text)
                    .class("icon-before")
                    .width(stretch_one)
                    .placeholder(search_placeholder)
                    .on_edit(move |cx, text| editable_text.set(cx, text));
                Svg::new(cx, icon_search)
                    .color(gray)
                    .position_type(position_absolute)
                    .top(stretch_one)
                    .bottom(stretch_one);
            })
            .height(auto)
            .width(width_300);

            Textbox::new_multiline(cx, multiline_text, true)
                .width(width_300)
                .on_edit(move |cx, text| multiline_text.set(cx, text));

            Textbox::new(cx, non_editable_text)
                .width(auto)
                .read_only(read_only);
            Textbox::new_multiline(cx, non_editable_multiline_text, true)
                .width(width_300)
                .read_only(read_only);
        });
        cx.state("Textbox")
    });

    app.title(title).run()
}
