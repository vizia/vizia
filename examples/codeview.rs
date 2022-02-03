use vizia::*;

#[derive(Lens)]
pub struct AppData {
    code: String,
}

#[derive(Debug)]
pub enum AppEvent {

}

impl Model for AppData {
    fn event(&mut self, _: &mut Context, event: &mut Event) {
        // if let Some(app_event) = event.message.downcast() {
           
        // }
    }
}

fn main() {
    let window_description = WindowDescription::new().with_title("Textbox");
    Application::new(window_description, |cx| {
        AppData { code: "Some code".to_string() }.build(cx);

        HStack::new(cx, |cx| {
            //Binding::new(cx, AppData::code, |cx, text| {
                CodeView::new(cx, "Type code here".to_string())
                    // .on_edit(|cx, text|{
                    //     if let Ok(valid_number) = text.parse::<i32>() {
                    //         cx.emit(AppEvent::SetNumber(valid_number));
                    //         //cx.current.set_checked(cx, false);
                    //     } else {
                    //         //cx.current.set_checked(cx, true);
                    //     }
                    // })
                    .width(Pixels(200.0))
                    .height(Pixels(300.0))
                    .child_left(Pixels(5.0));
            //});

            // Binding::new(cx, AppData::number, |cx, text| {
            //     Label::new(cx, &text.get(cx).to_string())
            //         .width(Pixels(200.0))
            //         .height(Pixels(30.0))
            //         .child_left(Pixels(5.0));
            // });
        })
        .space(Stretch(1.0))
        .col_between(Pixels(10.0));
    })
    .run();
}
