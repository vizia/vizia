use vizia::*;

#[derive(Lens)]
pub struct AppData {
    text_list: Vec<String>,
}

pub enum AppEvent {
    SetText(usize, String),
}

impl Model for AppData {
    fn event(&mut self, _: &mut Context, event: &mut Event) {
        if let Some(app_event) = event.message.downcast() {
            match app_event {
                AppEvent::SetText(index, text) => {
                    self.text_list[*index] = text.clone();
                }
            }
        }
    }
}

fn main() {
    let window_description = WindowDescription::new();
    Application::new(window_description, |cx| {
        AppData {
            text_list: vec![
                "First".to_string(),
                "Second".to_string(),
                "Third".to_string(),
                "Fourth".to_string(),
            ],
        }
        .build(cx);

        List::new(cx, AppData::text_list, |cx, index, text_item| {
            HStack::new(cx, move |cx| {
                Binding::new(cx, text_item.clone(), move |cx, it| {
                    Textbox::new(cx, it)
                        .on_edit(move |cx, text| {
                            cx.emit(AppEvent::SetText(index, text));
                        })
                        .width(Pixels(200.0))
                        .height(Pixels(30.0));
                });

                Label::new(cx, text_item)
                    .width(Pixels(200.0))
                    .height(Pixels(30.0))
                    .child_left(Pixels(5.0));
            })
            .col_between(Pixels(50.0));
        })
        .space(Stretch(1.0))
        .row_between(Pixels(10.0));
    })
    .run();
}
