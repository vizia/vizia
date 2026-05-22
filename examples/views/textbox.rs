mod helpers;
use helpers::*;
use vizia::icons::{ICON_EYE, ICON_EYE_OFF, ICON_SEARCH};
use vizia::prelude::*;

pub struct AppData {
    editable_text: Signal<String>,
    password_text: Signal<String>,
    password_visible: Signal<bool>,
    multiline_text: Signal<String>,
    _non_editable_text: Signal<String>,
    _non_editable_multiline_text: Signal<String>,
}

impl Model for AppData {
    fn event(&mut self, _: &mut EventContext, event: &mut Event) {
        event.map(|app_event, _| match app_event {
            AppEvent::SetEditableText(text) => self.editable_text.set(text.clone()),
            AppEvent::SetPasswordText(text) => self.password_text.set(text.clone()),
            AppEvent::TogglePasswordVisible => {
                self.password_visible.update(|visible| *visible ^= true)
            }
            AppEvent::SetMultilineText(text) => self.multiline_text.set(text.clone()),
        });
    }
}

pub enum AppEvent {
    SetEditableText(String),
    SetPasswordText(String),
    TogglePasswordVisible,
    SetMultilineText(String),
}

fn main() -> Result<(), ApplicationError> {
    Application::new(|cx| {
        let editable_text = Signal::new("".to_string());
        let password_text = Signal::new("".to_string());
        let password_visible = Signal::new(false);
        let multiline_text =
            Signal::new("This is some text which is editable and spans multiple lines".to_string());
        let non_editable_text = Signal::new("This text can be selected but not edited".to_string());
        let non_editable_multiline_text = Signal::new(
            "This text can be selected but not edited and spans multiple lines".to_string(),
        );

        AppData {
            editable_text,
            password_text,
            password_visible,
            multiline_text,
            _non_editable_text: non_editable_text,
            _non_editable_multiline_text: non_editable_multiline_text,
        }
        .build(cx);

        ExamplePage::vertical(cx, |cx| {
            Textbox::new(cx, editable_text)
                .width(Pixels(300.0))
                .placeholder(Localized::new("textbox-placeholder-type"))
                .on_edit(|cx, text| cx.emit(AppEvent::SetEditableText(text)));

            ZStack::new(cx, |cx| {
                let password_entity = Textbox::new(cx, password_text)
                    .width(Stretch(1.0))
                    .placeholder("Password")
                    .mask_char(Some('*'))
                    .on_edit(|cx, text| cx.emit(AppEvent::SetPasswordText(text)))
                    .padding_right(Pixels(30.0))
                    .entity();

                ToggleButton::with_contents(
                    cx,
                    password_visible,
                    |cx| Svg::new(cx, ICON_EYE),
                    |cx| Svg::new(cx, ICON_EYE_OFF),
                )
                //.variant(ButtonVariant::Text)
                .on_toggle(move |cx| {
                    cx.emit(AppEvent::TogglePasswordVisible);
                    cx.emit_to(password_entity, TextEvent::ToggleMaskVisible);
                });
            })
            .width(Pixels(300.0))
            .height(Auto)
            .alignment(Alignment::Right);

            HStack::new(cx, |cx| {
                Textbox::new(cx, editable_text)
                    .class("icon-before")
                    .width(Stretch(1.0))
                    .placeholder(Localized::new("textbox-placeholder-search"))
                    .on_edit(|cx, text| cx.emit(AppEvent::SetEditableText(text)));
                Svg::new(cx, ICON_SEARCH)
                    .color(Color::gray())
                    .position_type(PositionType::Absolute)
                    .left(Pixels(5.0))
                    .top(Stretch(1.0))
                    .bottom(Stretch(1.0));
            })
            .height(Auto)
            .width(Pixels(300.0));

            Textbox::new_multiline(cx, multiline_text, true)
                .width(Pixels(300.0))
                .on_edit(|cx, text| cx.emit(AppEvent::SetMultilineText(text)));

            Textbox::new(cx, non_editable_text).width(Auto).read_only(true);
            Textbox::new_multiline(cx, non_editable_multiline_text, true)
                .width(Pixels(300.0))
                .read_only(true);
        });
    })
    .title(Localized::new("view-title-textbox"))
    .run()
}
