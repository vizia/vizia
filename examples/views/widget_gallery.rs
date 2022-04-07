use vizia::*;

const ICON_PLUS: &str = "\u{2b}";

#[derive(Lens)]
pub struct AppData {}

fn main() {
    let window_description = WindowDescription::new();
    Application::new(window_description, |cx| {
        //cx.add_stylesheet("examples/test_style.css").unwrap();

        //buttons(cx)
        //    .space(Pixels(30.0));
        checkbox(cx).space(Pixels(30.0));
        // label(cx);
    })
    .background_color(Color::rgb(249, 249, 249))
    .run();
}

pub fn buttons(cx: &mut Context) -> Handle<impl View> {
    VStack::new(cx, |cx| {
        Label::new(cx, "Button").font_size(30.0).font("roboto-bold");

        Label::new(cx, "A simple Button with a text label").font_size(24.0).font("roboto-bold");

        Button::new(cx, |_| {}, |cx| Label::new(cx, "Simple Button"));

        Label::new(cx, "A simple Button with an icon label").font_size(24.0).font("roboto-bold");

        Button::new(cx, |_| {}, |cx| Label::new(cx, ICON_PLUS).font("icons"));

        Label::new(cx, "A simple Button with icon and text labels")
            .font_size(24.0)
            .font("roboto-bold");

        Button::new(
            cx,
            |_| {},
            |cx| {
                HStack::new(cx, |cx| {
                    Label::new(cx, ICON_PLUS).font("icons");
                    Label::new(cx, "Button");
                })
                .size(Auto)
                .child_space(Stretch(1.0))
                .col_between(Pixels(2.0))
            },
        );

        Label::new(cx, "An accented Button with a text label").font_size(24.0).font("roboto-bold");

        Button::new(cx, |_| {}, |cx| Label::new(cx, "Simple Button")).class("accent");
    })
    .row_between(Pixels(15.0))
}

#[derive(Lens)]
pub struct CheckboxData {
    check: bool,
}

pub enum CheckboxEvent {
    Toggle,
}

impl Model for CheckboxData {
    fn event(&mut self, _: &mut Context, event: &mut Event) {
        if let Some(checkbox_event) = event.message.downcast() {
            match checkbox_event {
                CheckboxEvent::Toggle => {
                    self.check ^= true;
                }
            }
        }
    }
}

pub fn checkbox(cx: &mut Context) -> Handle<impl View> {
    CheckboxData { check: false }.build(cx);

    VStack::new(cx, |cx| {
        Label::new(cx, "Checkbox").font_size(30.0).font("roboto-bold");

        Label::new(cx, "A simple 2-state checkbox").font_size(24.0).font("roboto-bold");

        Checkbox::new(cx, CheckboxData::check).on_toggle(|cx| cx.emit(CheckboxEvent::Toggle));

        Label::new(cx, "A simple 2-state checkbox with a text label")
            .font_size(24.0)
            .font("roboto-bold");

        HStack::new(cx, |cx| {
            Checkbox::new(cx, CheckboxData::check);
            Label::new(cx, "Two-state checkbox");
        })
        .size(Auto)
        .child_top(Stretch(1.0))
        .child_bottom(Stretch(1.0))
        .col_between(Pixels(5.0))
        .on_press(|cx| cx.emit(CheckboxEvent::Toggle));
    })
    .row_between(Pixels(15.0))
}

pub fn label(cx: &mut Context) {
    VStack::new(cx, |cx| {
        Label::new(cx, "A simple label").font_size(20.0).font("roboto-bold");

        Label::new(cx, "This is some simple text");

        Label::new(cx, "A styled label").font_size(20.0).font("roboto-bold");

        Label::new(cx, "This is some simple text");
    })
    .row_between(Pixels(15.0));
}
