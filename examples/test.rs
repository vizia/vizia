use vizia::*;


const STYLE: &str = r#"
    hstack {
        color: green;
    }
"#;


#[derive(Default, Lens)]
pub struct AppData {
    value: bool,
}

impl Model for AppData {
    fn event(&mut self, cx: &mut Context, event: &mut Event) {
        if let Some(app_event) = event.message.downcast() {
            match app_event {
                AppEvent::ToggleValue => {
                    self.value ^= true;
                }
            }
        } 
    }
}

#[derive(Debug)]
pub enum AppEvent {
    ToggleValue,
}

fn main() {
    Application::new(WindowDescription::new().with_title("Test"), |cx| {
        
        cx.add_theme(STYLE);
        
        AppData::default().build(cx);
        
        HStack::new(cx, |cx|{
            Binding::new(cx, AppData::value, |cx, value|{
                Checkbox::new(cx, *value.get(cx))
                    .on_toggle(cx, |cx| cx.emit(AppEvent::ToggleValue));
            });
            Label::new(cx, "Press Me");
        }).col_between(Pixels(5.0)).color(Color::red()).disabled(true);
    })
    .run();
}

// .border_shape_top_left(BorderCornerShape::Bevel)
