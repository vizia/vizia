use vizia::prelude::*;

#[allow(dead_code)]
const DARK_THEME: &str = "crates/vizia_core/resources/themes/dark_theme.css";
#[allow(dead_code)]
const LIGHT_THEME: &str = "crates/vizia_core/resources/themes/light_theme.css";

const ICON_PLUS: &str = "\u{2b}";

const STYLE: &str = r#"

    .title {
        font-size: 30.0;
        font: "roboto-bold";
        top: 10px;
        bottom: 10px;
    }

    .heading {
        font-size: 20.0;
        font: "roboto-bold";
        top: 10px;
        bottom: 6px;
    }

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

fn tab<T: ToString + Data>(cx: &mut Context, item: impl Lens<Target = T>) {
    Label::new(cx, item);
    Element::new(cx).class("indicator");
}

fn main() {
    Application::new(|cx| {
        cx.add_stylesheet(DARK_THEME).expect("Failed to find stylesheet");
        cx.add_theme(STYLE);
        AppData { items: vec!["Label", "Button", "Checkbox", "Slider"] }.build(cx);
        //cx.add_stylesheet("examples/test_style.css").unwrap();

        TabView::new(cx, AppData::items, |cx, item| match item.get(cx) {
            "Label" => TabPair::new(move |cx| tab(cx, item), label),

            "Button" => TabPair::new(
                move |cx| {
                    tab(cx, item);
                },
                button,
            ),

            "Checkbox" => TabPair::new(
                move |cx| {
                    tab(cx, item);
                },
                checkbox,
            ),

            "Slider" => TabPair::new(
                move |cx| {
                    tab(cx, item);
                },
                slider,
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

pub fn button(cx: &mut Context) {
    VStack::new(cx, |cx| {
        Label::new(cx, "Button").class("title");

        Label::new(cx, "A button with a text a label").class("heading");

        Button::new(cx, |_| {}, |cx| Label::new(cx, "Simple Button"));

        Label::new(cx, "An accent button with a text label").class("heading");

        Button::new(cx, |_| {}, |cx| Label::new(cx, "Accent Button")).class("accent");

        Label::new(cx, "A simple button with an icon label").class("heading");

        Button::new(cx, |_| {}, |cx| Label::new(cx, ICON_PLUS).font("icons"));

        Label::new(cx, "A button with icon and text labels").class("heading");

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
                .col_between(Pixels(4.0))
            },
        );
    })
    .child_space(Pixels(10.0))
    .class("bg-darker");
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

pub fn checkbox(cx: &mut Context) {
    CheckboxData { check: false }.build(cx);

    VStack::new(cx, |cx| {
        Label::new(cx, "Checkbox").class("title");

        Label::new(cx, "A simple 2-state checkbox").class("heading");

        Checkbox::new(cx, CheckboxData::check).on_toggle(|cx| cx.emit(CheckboxEvent::Toggle));

        Label::new(cx, "A simple 2-state checkbox with a text label").class("heading");

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
    .child_space(Pixels(10.0))
    .class("bg-darker");
}

pub fn label(cx: &mut Context) {
    VStack::new(cx, |cx| {
        Label::new(cx, "Label").class("title");

        Label::new(cx, "A simple label").class("heading");

        Label::new(cx, "This is some simple text");

        Label::new(cx, "A styled label").class("heading");

        Label::new(cx, "This is some styled text").color(Color::red());

        Label::new(cx, "A multiline label").class("heading");

        Label::new(cx, "This is some text which is wrapped")
            .width(Pixels(100.0))
            .bottom(Pixels(10.0));
    })
    .child_space(Pixels(10.0))
    .class("bg-darker");
}

#[derive(Lens)]
pub struct SliderData {
    val: f32,
}

pub enum SliderEvent {
    SetValue(f32),
}

impl Model for SliderData {
    fn event(&mut self, _cx: &mut EventContext, event: &mut Event) {
        event.map(|slider_event, _| match slider_event {
            SliderEvent::SetValue(val) => {
                self.val = *val;
            }
        });
    }
}

pub fn slider(cx: &mut Context) {
    SliderData { val: 0.5 }.build(cx);

    VStack::new(cx, |cx| {
        Label::new(cx, "Label").class("title");

        Label::new(cx, "A simple slider").class("heading");

        Slider::new(cx, SliderData::val).on_changing(|cx, val| cx.emit(SliderEvent::SetValue(val)));

        Label::new(cx, "A slider and label").class("heading");

        HStack::new(cx, |cx| {
            Slider::new(cx, SliderData::val)
                .on_changing(|cx, val| cx.emit(SliderEvent::SetValue(val)));
            Label::new(cx, SliderData::val.map(|val| format!("{:.2}", val))).width(Pixels(50.0));
        })
        .height(Auto)
        .child_top(Stretch(1.0))
        .child_bottom(Stretch(1.0))
        .col_between(Pixels(10.0));

        Label::new(cx, "A multiline label").class("heading");

        Label::new(cx, "This is some text which is wrapped")
            .width(Pixels(100.0))
            .bottom(Pixels(10.0));
    })
    .child_space(Pixels(10.0))
    .class("bg-darker");
}
