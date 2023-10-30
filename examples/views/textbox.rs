mod helpers;
use helpers::*;
use vizia::icons::ICON_SEARCH;
use vizia::prelude::*;

#[derive(Lens, Setter, Model)]
pub struct AppData {
    editable_text: String,
    multiline_text: String,
    non_editable_text: String,
    non_editable_multiline_text: String,
}

fn main() {
    Application::new(|cx| {
        AppData {
            editable_text: "".to_string(),
            multiline_text: "This is some text which is editable and spans multiple lines"
                .to_string(),
            non_editable_text: "This text can be selected but not edited".to_string(),
            non_editable_multiline_text:
                "This text can be selected but not edited and spans multiple lines".to_string(),
        }
        .build(cx);

        ExamplePage::vertical(cx, |cx| {
            Textbox::new(cx, AppData::editable_text)
                .width(Pixels(300.0))
                .placeholder("Type something...")
                .on_edit(|cx, text| cx.emit(AppDataSetter::EditableText(text)));

            HStack::new(cx, |cx| {
                Textbox::new(cx, AppData::editable_text)
                    .class("icon-before")
                    .width(Stretch(1.0))
                    .placeholder("Search")
                    .on_edit(|cx, text| cx.emit(AppDataSetter::EditableText(text)));
                Icon::new(cx, ICON_SEARCH)
                    .color(Color::gray())
                    .position_type(PositionType::SelfDirected);
            })
            .height(Auto)
            .width(Pixels(300.0));

            Textbox::new_multiline(cx, AppData::multiline_text, true)
                .width(Pixels(300.0))
                .on_edit(|cx, text| cx.emit(AppDataSetter::MultilineText(text)));

            Textbox::new(cx, AppData::non_editable_text).width(Auto).read_only(true);
            Textbox::new_multiline(cx, AppData::non_editable_multiline_text, true)
                .width(Pixels(300.0))
                .read_only(true);
        });
    })
    .title("Textbox")
    .run();
}
