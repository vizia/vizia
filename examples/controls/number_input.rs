use vizia::*;

#[derive(Lens)]
pub struct AppData {
    number: i32,
}

#[derive(Debug)]
pub enum AppEvent {
    SetNumber(i32),
}

impl Model for AppData {
    fn event(&mut self, _: &mut Context, event: &mut Event) {
        if let Some(app_event) = event.message.downcast() {
            match app_event {
                AppEvent::SetNumber(num) => {
                    self.number = *num;
                }
            }
        }
    }
}

fn main() {
    let window_description = WindowDescription::new().with_title("Textbox");
    Application::new(window_description, |cx| {
        AppData { number: 5 }.build(cx);

        HStack::new(cx, |cx| {
            //Binding::new(cx, AppData::number, |cx, text| {
                Textbox::new(cx, AppData::number)
                    .on_edit(|cx, text|{
                        if let Ok(valid_number) = text.parse::<i32>() {
                            cx.emit(AppEvent::SetNumber(valid_number));
                            //cx.current.set_checked(cx, false);
                        } else {
                            //cx.current.set_checked(cx, true);
                        }
                    })
                    .width(Pixels(200.0))
                    .child_left(Pixels(5.0));
            //});

            Binding::new(cx, AppData::number, |cx, text| {
                Label::new(cx, &text.get(cx).to_string())
                    .width(Pixels(200.0))
                    .height(Pixels(30.0))
                    .child_left(Pixels(5.0));
            });
        })
        .space(Stretch(1.0))
        .col_between(Pixels(10.0));
    })
    .run();
}
