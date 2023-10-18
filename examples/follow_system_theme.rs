use vizia::prelude::*;

#[derive(Debug, Lens)]
pub struct AppData {
    current_theme: String,
}

impl Model for AppData {
    fn event(&mut self, _: &mut EventContext, event: &mut Event) {
        event.map(|event, _| {
            if let WindowEvent::ThemeChanged(theme) = event {
                self.current_theme = match theme {
                    ThemeMode::DarkMode => "Dark Mode",
                    ThemeMode::LightMode => "Light Mode",
                }
                .to_owned();
            }
        })
    }
}

fn main() {
    Application::new(|cx: &mut Context| {
        AppData { current_theme: "Light mode".to_owned() }.build(cx);

        cx.emit(EnvironmentEvent::SetThemeMode(AppTheme::System));

        VStack::new(cx, |cx| {
            Label::new(cx, "This example follow system theme change");
            Label::new(cx, "Change your theme to light or dark mode to see how it works\n");
            Label::new(cx, AppData::current_theme.map(|v| format!("Current system theme: {v}")));
        })
        .child_space(Stretch(1.0))
        .space(Stretch(1.0));
    })
    .title("Follow system theme")
    .inner_size((470, 320))
    .run();
}
