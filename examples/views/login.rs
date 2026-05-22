mod helpers;
use helpers::*;
use vizia::icons::{ICON_EYE, ICON_EYE_OFF};
use vizia::prelude::*;

const LOGIN_STYLES: &str = r#"
    .login-card {
        width: 340px;
        height: auto;
        padding: 16px;
        gap: 12px;
        alignment: center;
        border-width: 1px;
        border-color: var(--border);
        corner-radius: 8px;
        background-color: var(--card);
    }

    .login-title {
        font-size: 28px;
        height: auto;
    }

    .error-text {
        color: #b42318;
        font-size: 12px;
    }

    .status-text {
        font-size: 12px;
    }
"#;

#[derive(Clone, Copy)]
struct LoginData {
    username: Signal<String>,
    password: Signal<String>,
    password_visible: Signal<bool>,
    username_error: Signal<String>,
    password_error: Signal<String>,
    can_submit: Signal<bool>,
    status: Signal<String>,
}

impl LoginData {
    fn new() -> Self {
        Self {
            username: Signal::new(String::new()),
            password: Signal::new(String::new()),
            password_visible: Signal::new(false),
            username_error: Signal::new(String::new()),
            password_error: Signal::new(String::new()),
            can_submit: Signal::new(false),
            status: Signal::new(String::new()),
        }
    }

    fn username_error(username: &str) -> Option<&'static str> {
        if username.is_empty() {
            Some("Username is required")
        } else if username.chars().count() < 3 {
            Some("Username must be at least 3 characters")
        } else if !username.chars().all(|ch| ch.is_ascii_alphanumeric() || ch == '_') {
            Some("Use letters, numbers, or _")
        } else {
            None
        }
    }

    fn password_error(password: &str) -> Option<&'static str> {
        let has_letter = password.chars().any(|ch| ch.is_ascii_alphabetic());
        let has_number = password.chars().any(|ch| ch.is_ascii_digit());

        if password.is_empty() {
            Some("Password is required")
        } else if password.chars().count() < 8 {
            Some("Password must be at least 8 characters")
        } else if !has_letter || !has_number {
            Some("Password must include a letter and a number")
        } else {
            None
        }
    }

    fn recompute_validation(&self) {
        let username = self.username.get();
        let password = self.password.get();

        let username_error = Self::username_error(&username).unwrap_or_default().to_string();
        let password_error = Self::password_error(&password).unwrap_or_default().to_string();

        self.username_error.set(username_error.clone());
        self.password_error.set(password_error.clone());
        self.can_submit.set(username_error.is_empty() && password_error.is_empty());
    }
}

enum LoginEvent {
    SetUsername(String),
    SetPassword(String),
    TogglePasswordVisible,
    Submit,
}

impl Model for LoginData {
    fn event(&mut self, _: &mut EventContext, event: &mut Event) {
        event.map(|login_event, _| match login_event {
            LoginEvent::SetUsername(username) => {
                self.username.set(username.clone());
                self.recompute_validation();
                self.status.set(String::new());
            }

            LoginEvent::SetPassword(password) => {
                self.password.set(password.clone());
                self.recompute_validation();
                self.status.set(String::new());
            }

            LoginEvent::TogglePasswordVisible => {
                self.password_visible.update(|visible| *visible ^= true);
            }

            LoginEvent::Submit => {
                if self.can_submit.get() {
                    self.status.set("Login successful (demo)".to_string());
                } else {
                    self.status.set("Please fix validation errors before logging in".to_string());
                }
            }
        });
    }
}

fn main() -> Result<(), ApplicationError> {
    Application::new(|cx| {
        cx.add_stylesheet(LOGIN_STYLES).expect("Failed to add login stylesheet");

        let &LoginData {
            username,
            password,
            password_visible,
            username_error,
            password_error,
            can_submit,
            status,
        } = LoginData::new().build(cx);

        ExamplePage::vertical(cx, |cx| {
            VStack::new(cx, |cx| {
                Label::new(cx, "Welcome Back").class("login-title");

                VStack::new(cx, |cx| {
                    Label::new(cx, "Username").height(Auto).font_weight(FontWeightKeyword::Bold);
                    Textbox::new(cx, username)
                        .width(Stretch(1.0))
                        .placeholder("Enter username")
                        .validate(|value: &String| LoginData::username_error(value).is_none())
                        .on_edit(|cx, text| cx.emit(LoginEvent::SetUsername(text)));
                    Label::new(cx, username_error).class("error-text");
                })
                .height(Auto)
                .gap(Pixels(4.0));
                VStack::new(cx, |cx| {
                    Label::new(cx, "Password").height(Auto).font_weight(FontWeightKeyword::Bold);
                    ZStack::new(cx, |cx| {
                        let password_entity = Textbox::new(cx, password)
                            .width(Stretch(1.0))
                            .placeholder("Enter password")
                            .mask_char(Some('*'))
                            .validate(|value: &String| LoginData::password_error(value).is_none())
                            .on_edit(|cx, text| cx.emit(LoginEvent::SetPassword(text)))
                            .padding_right(Pixels(50.0))
                            .entity();

                        ToggleButton::with_contents(
                            cx,
                            password_visible,
                            |cx| Svg::new(cx, ICON_EYE),
                            |cx| Svg::new(cx, ICON_EYE_OFF),
                        )
                        .on_toggle(move |cx| {
                            cx.emit(LoginEvent::TogglePasswordVisible);
                            cx.emit_to(password_entity, TextEvent::ToggleMaskVisible);
                        });
                    })
                    .width(Stretch(1.0))
                    .height(Auto)
                    .alignment(Alignment::Right);

                    Label::new(cx, password_error).class("error-text");
                })
                .height(Auto)
                .gap(Pixels(4.0));
                Button::new(cx, |cx| Label::new(cx, "Login"))
                    .width(Stretch(1.0))
                    .on_press(|cx| cx.emit(LoginEvent::Submit))
                    .disabled(can_submit);

                Label::new(cx, status).class("status-text");
            })
            .class("login-card");
        });
    })
    .title("Login Example")
    .inner_size((700, 400))
    .run()
}
