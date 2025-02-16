use vizia::prelude::*;

const GLOBAL_STYLE: &str = r#"
    button {
        background-color: #f00;
    }
"#;

const GLOBAL_STYLE2: &str = r#"
    button {
        background-color: #00f;
    }
"#;

const SCOPED_STYLE: &str = r#"

    button {
        background-color: #0f0;
    }
"#;

#[derive(Clone, Copy, Data, PartialEq, Eq)]
pub enum Theme {
    Theme1,
    Theme2,
}

#[derive(Lens)]
pub struct AppData {
    theme: Theme,
}

pub enum AppEvent {
    ChangeTheme(Theme),
}

impl Model for AppData {
    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|app_event, _| match app_event {
            AppEvent::ChangeTheme(theme) => {
                self.theme = *theme;
            }
        });
    }
}

fn main() -> Result<(), ApplicationError> {
    Application::new(|cx| {
        AppData { theme: Theme::Theme1 }.build(cx);

        // This binding will be updated when the theme changes
        Binding::new(cx, AppData::theme, |cx, theme| {
            match theme.get(cx) {
                Theme::Theme1 => cx.add_stylesheet("glob", GLOBAL_STYLE),
                Theme::Theme2 => cx.add_stylesheet("glob", GLOBAL_STYLE2),
            };
        });

        HStack::new(cx, |cx| {
            Binding::new(cx, AppData::theme, |cx, theme| {
                match theme.get(cx) {
                    Theme::Theme1 => {
                        cx.add_stylesheet("style", "");
                    }
                    Theme::Theme2 => {
                        cx.add_stylesheet("style", SCOPED_STYLE);
                    }
                };
            });

            HStack::new(cx, |cx| {
                Button::new(cx, |cx| Label::new(cx, "Green button"));
            })
            .height(Auto);
        })
        .height(Auto);

        HStack::new(cx, |cx| {
            Button::new(cx, |cx| Label::new(cx, "Red button"))
                .on_press(|cx| cx.emit(AppEvent::ChangeTheme(Theme::Theme1)));
        })
        .height(Auto);

        Button::new(cx, |cx| Label::new(cx, "Red button"))
            .on_press(|cx| cx.emit(AppEvent::ChangeTheme(Theme::Theme2)));
    })
    .run()
}
