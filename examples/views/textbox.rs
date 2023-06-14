mod helpers;
use helpers::*;
use vizia::prelude::*;

#[derive(Lens, Setter, Model)]
pub struct AppData {
    editable_text: String,
    multiline_text: String,
    non_editable_text: String,
}

fn main() {
    Application::new(|cx| {
        AppData {
            editable_text: "This is some editable text".to_string(),
            multiline_text: "This is some text which is editable and spans multiple lines"
                .to_string(),
            non_editable_text: "This text can be selected but not edited".to_string(),
        }
        .build(cx);

        ExamplePage::vertical(cx, |cx| {
            Textbox::new(cx, AppData::editable_text)
                .width(Pixels(300.0))
                .on_edit(|cx, text| cx.emit(AppDataSetter::EditableText(text)));
            Textbox::new_multiline(cx, AppData::multiline_text, true)
                .width(Pixels(300.0))
                .on_edit(|cx, text| cx.emit(AppDataSetter::MultilineText(text)));
            Textbox::new(cx, AppData::non_editable_text).width(Pixels(300.0)).read_only(true);
        });
    })
    .title("Textbox")
    .run();
}
