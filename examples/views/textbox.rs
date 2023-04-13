mod helpers;
use helpers::*;
use vizia::prelude::*;

#[derive(Lens, Setter, Model)]
pub struct AppData {
    text: String,
}

fn main() {
    Application::new(|cx| {
        AppData { text: "This is some editable text".to_string() }.build(cx);

        ExamplePage::vertical(cx, |cx| {
            Textbox::new(cx, AppData::text)
                .width(Pixels(300.0))
                .on_edit(|cx, text| cx.emit(AppDataSetter::Text(text)));
            Textbox::new(cx, AppData::text)
                .width(Pixels(300.0))
                .on_edit(|cx, text| cx.emit(AppDataSetter::Text(text)))
                .read_only(true);
        });

        // Textbox::new_multiline(
        //     cx,
        //     StaticLens::new(
        //         &"This text is editable, but will reset on blur. Good luck editing it, haha!",
        //     ),
        //     true,
        // )
        // .width(Pixels(200.0))
        // .height(Pixels(200.0));
    })
    .title("Textbox")
    .run();
}
