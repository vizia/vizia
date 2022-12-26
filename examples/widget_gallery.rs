use vizia::prelude::*;

#[allow(dead_code)]
const DARK_THEME: &str = "crates/vizia_core/resources/themes/dark_theme.css";
#[allow(dead_code)]
const LIGHT_THEME: &str = "crates/vizia_core/resources/themes/light_theme.css";

const ICON_PLUS: &str = "\u{2b}";

const STYLE: &str = r#"

    .tabview-tabheader-wrapper {
        width: 200px;
    }

    tabheader {
        width: 1s;
    }

    tabheader label {
        width: 1s;
    }

"#;

#[derive(Lens)]
pub struct AppData {
    items: Vec<&'static str>,
}

impl Model for AppData {}

fn main() {
    Application::new(|cx| {
        cx.add_stylesheet(DARK_THEME).expect("Failed to find stylesheet");
        cx.add_theme(STYLE);
        AppData { items: vec!["Button", "Checkbox"] }.build(cx);
        //cx.add_stylesheet("examples/test_style.css").unwrap();

        TabView::new(cx, AppData::items, |cx, item| match item.get(cx) {
            "Button" => TabPair::new(
                move |cx| {
                    Label::new(cx, item);
                    Element::new(cx).class("indicator");
                },
                |cx| {
                    buttons(cx).child_space(Pixels(20.0));
                },
            ),

            "Checkbox" => TabPair::new(
                move |cx| {
                    Label::new(cx, item);
                    Element::new(cx).class("indicator");
                },
                |cx| {
                    checkbox(cx).child_space(Pixels(20.0));
                },
            ),

            _ => TabPair::new(|_| {}, |_| {}),
        })
        .class("vertical");

        //buttons(cx)
        //    .space(Pixels(30.0));
        // checkbox(cx).space(Pixels(30.0));
        // label(cx);
    })
    .title("Widget Gallery")
    //.background_color(Color::rgb(249, 249, 249))
    .ignore_default_theme()
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
    fn event(&mut self, _: &mut EventContext, event: &mut Event) {
        event.map(|checkbox_event, _| match checkbox_event {
            CheckboxEvent::Toggle => {
                self.check ^= true;
            }
        });
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
