use vizia::*;

const STYLE: &str = r#"
    button:focus {
        border-width: 1px;
        border-color: blue;
    }
"#;

#[derive(Lens)]
pub struct AppData {
    text: String,
}

pub enum AppEvent {
    SetText(String),
}

impl Model for AppData {
    fn event(&mut self, _: &mut Context, event: &mut Event) {
        event.map(|app_event, _| match app_event {
            AppEvent::SetText(text) => {
                self.text = text.clone();
            }
        });
    }
}

fn main() {
    Application::new(|cx| {
        cx.add_theme(STYLE);

        AppData { text: "".to_string() }.build(cx);

        VStack::new(cx, |cx| {
            HStack::new(cx, |cx| {
                Button::new(cx, |_| {}, |cx| Label::new(cx, "Button 1"))
                    .on_focus_in(|cx| cx.emit(AppEvent::SetText("Button 1".to_string())))
                    .on_focus_out(|cx| cx.emit(AppEvent::SetText("".to_string())));
                Button::new(cx, |_| {}, |cx| Label::new(cx, "Button 2"))
                    .on_focus_in(|cx| cx.emit(AppEvent::SetText("Button 2".to_string())))
                    .on_focus_out(|cx| cx.emit(AppEvent::SetText("".to_string())));
                Button::new(cx, |_| {}, |cx| Label::new(cx, "Button 3"))
                    .on_focus_in(|cx| cx.emit(AppEvent::SetText("Button 3".to_string())))
                    .on_focus_out(|cx| cx.emit(AppEvent::SetText("".to_string())));
            })
            .col_between(Pixels(10.0))
            .height(Auto);

            Label::new(cx, AppData::text.map(|text| format!("Focused: {}", text)));
        })
        .child_space(Pixels(10.0))
        .row_between(Pixels(10.0));
    })
    .title("Focus Order")
    .run();
}
