use vizia::{icons::ICON_CHECK, prelude::*};

mod helpers;
use helpers::*;
use vizia_core::icons::ICON_CHEVRON_RIGHT;

const STYLE: &str = r#"
    .container {
        child-space: 0px;
    }

    scrollview .scroll_content {
        child-space: 20px;
        row-between: 15px;
    }

    panel {
        height: auto;
        max-height: 40px;
        overflow: hidden;
        transition: max-height 200ms ease-out;
        border-color: black;
        border-width: 1px;
    }

    panel:checked {
        max-height: 110px;
        transition: max-height 200ms ease-out;
    }

    panel > .panel-header {
        child-top: 1s;
        child-bottom: 1s;
        col-between: 8px;
    }

    panel > .panel-header > .icon {
        font-size: 30;
        font-weight: bold;
        rotate: 0deg;
        transition: rotate 200ms ease-out;
    }

    panel:checked > .panel-header > .icon {
        rotate: 90deg;
        transition: rotate 200ms ease-out;
    }
"#;

#[derive(Lens)]
pub struct Panel {
    is_open: bool,
}

pub enum PanelEvent {
    ToggleOpen,
    Open,
    Close,
}

impl Panel {
    pub fn new<V: View>(
        cx: &mut Context,
        header: impl Fn(&mut Context) -> Handle<V>,
        content: impl Fn(&mut Context),
    ) -> Handle<Self> {
        Self { is_open: false }
            .build(cx, |cx| {
                HStack::new(cx, |cx| {
                    Label::new(cx, ICON_CHEVRON_RIGHT).class("icon").hoverable(false);
                    (header)(cx).hoverable(false);
                })
                .class("panel-header")
                .on_press(|cx| cx.emit(PanelEvent::ToggleOpen))
                .height(Pixels(40.0));
                VStack::new(cx, |cx| {
                    (content)(cx);
                })
                .height(Auto)
                .child_space(Pixels(10.0));
            })
            .checked(Panel::is_open)
    }
}

impl View for Panel {
    fn element(&self) -> Option<&'static str> {
        Some("panel")
    }

    fn event(&mut self, _: &mut EventContext, event: &mut Event) {
        event.map(|panel_event, _| match panel_event {
            PanelEvent::ToggleOpen => {
                self.is_open ^= true;
            }

            PanelEvent::Open => {
                self.is_open = true;
            }

            PanelEvent::Close => {
                self.is_open = false;
            }
        });
    }
}

#[derive(Lens)]
pub struct AppData {}

fn main() {
    Application::new(|cx| {
        ExamplePage::vertical(cx, |cx| {
            cx.add_stylesheet(STYLE).expect("Failed to add stylesheet");
            ScrollView::new(cx, 0.0, 0.0, false, true, |cx| {
                Label::new(cx, "Label").font_size(30.0).font_weight(FontWeightKeyword::Bold);
                label(cx);

                Label::new(cx, "Button").font_size(30.0).font_weight(FontWeightKeyword::Bold);
                button(cx);

                Label::new(cx, "Checkbox").font_size(30.0).font_weight(FontWeightKeyword::Bold);
                checkbox(cx);

                Label::new(cx, "Switch").font_size(30.0).font_weight(FontWeightKeyword::Bold);
                switch(cx);

                Label::new(cx, "Radiobutton").font_size(30.0).font_weight(FontWeightKeyword::Bold);
                radiobutton(cx);

                Label::new(cx, "Slider").font_size(30.0).font_weight(FontWeightKeyword::Bold);
                slider(cx);

                Label::new(cx, "Rating").font_size(30.0).font_weight(FontWeightKeyword::Bold);
                rating(cx);
            });
        });
    })
    .title("Widget Gallery")
    .run();
}

pub fn button(cx: &mut Context) -> Handle<impl View> {
    HStack::new(cx, |cx| {
        // Basic Button
        Button::new(cx, |_| {}, |cx| Label::new(cx, "Button"));
        // Accent Button
        Button::new(cx, |_| {}, |cx| Label::new(cx, "Accent Button")).class("accent");
        // Outline Button
        Button::new(cx, |_| {}, |cx| Label::new(cx, "Outline Button")).class("outline");
        // Ghost Button
        Button::new(cx, |_| {}, |cx| Label::new(cx, "Ghost Button")).class("ghost");
        // Button with Icon
        Button::new(
            cx,
            |_| {},
            |cx| {
                HStack::new(cx, |cx| {
                    Label::new(cx, ICON_CHECK).class("icon");
                    Label::new(cx, "Button with Icon");
                })
            },
        );
    })
    .col_between(Pixels(15.0))
    .size(Auto)
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
        Checkbox::new(cx, CheckboxData::check).on_toggle(|cx| cx.emit(CheckboxEvent::Toggle));

        HStack::new(cx, |cx| {
            Checkbox::new(cx, CheckboxData::check)
                .id("checky")
                .on_toggle(|cx| cx.emit(CheckboxEvent::Toggle));
            Label::new(cx, "Checkbox with label").describing("checky");
        })
        .size(Auto)
        .child_top(Stretch(1.0))
        .child_bottom(Stretch(1.0))
        .col_between(Pixels(5.0));
    })
    .height(Auto)
    .row_between(Pixels(15.0))
}

pub fn label(cx: &mut Context) {
    VStack::new(cx, |cx| {
        Label::new(cx, "This is some simple text");
        Label::new(cx, "This is some simple text");
    })
    .height(Auto)
    .row_between(Pixels(15.0));
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Options {
    First,
    Second,
    Third,
}

impl std::fmt::Display for Options {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match *self {
            Options::First => "First",
            Options::Second => "Second",
            Options::Third => "Third",
        };
        write!(f, "{}", str)
    }
}

#[derive(Lens)]
pub struct RadioData {
    option: Options,
}

pub enum RadioEvent {
    SetOption(Options),
}

impl Model for RadioData {
    fn event(&mut self, _: &mut EventContext, event: &mut Event) {
        event.map(|checkbox_event, _| match checkbox_event {
            RadioEvent::SetOption(option) => {
                self.option = *option;
            }
        });
    }
}

pub fn radiobutton(cx: &mut Context) -> Handle<impl View> {
    RadioData { option: Options::First }.build(cx);

    VStack::new(cx, |cx| {
        HStack::new(cx, |cx| {
            for i in 0..3 {
                let current_option = index_to_option(i);
                RadioButton::new(
                    cx,
                    RadioData::option.map(move |option| *option == current_option),
                )
                .on_select(move |cx| cx.emit(RadioEvent::SetOption(current_option)));
            }
        })
        .size(Auto)
        .col_between(Pixels(20.0));

        VStack::new(cx, |cx| {
            for i in 0..3 {
                let current_option = index_to_option(i);
                HStack::new(cx, move |cx| {
                    RadioButton::new(
                        cx,
                        RadioData::option.map(move |option| *option == current_option),
                    )
                    .on_select(move |cx| cx.emit(RadioEvent::SetOption(current_option)))
                    .id(format!("button_{i}"));
                    Label::new(cx, &current_option.to_string()).describing(format!("button_{i}"));
                })
                .size(Auto)
                .child_top(Stretch(1.0))
                .child_bottom(Stretch(1.0))
                .col_between(Pixels(5.0));
            }
        })
        .row_between(Pixels(10.0))
        .size(Auto);
    })
    .row_between(Pixels(15.0))
    .height(Auto)
}

fn index_to_option(index: usize) -> Options {
    match index {
        0 => Options::First,
        1 => Options::Second,
        2 => Options::Third,
        _ => unreachable!(),
    }
}

#[derive(Debug, Lens)]
pub struct SwitchData {
    pub option1: bool,
    pub option2: bool,
}

#[derive(Debug)]
pub enum SwitchEvent {
    ToggleOption1,
    ToggleOption2,
}

impl Model for SwitchData {
    fn event(&mut self, _cx: &mut EventContext, event: &mut Event) {
        event.map(|app_event, _| match app_event {
            SwitchEvent::ToggleOption1 => {
                self.option1 ^= true;
            }

            SwitchEvent::ToggleOption2 => {
                self.option2 ^= true;
            }
        });
    }
}

pub fn switch(cx: &mut Context) -> Handle<impl View> {
    SwitchData { option1: true, option2: false }.build(cx);

    Switch::new(cx, SwitchData::option1).on_toggle(|cx| cx.emit(SwitchEvent::ToggleOption1));

    HStack::new(cx, |cx| {
        Switch::new(cx, SwitchData::option2)
            .on_toggle(|cx| cx.emit(SwitchEvent::ToggleOption2))
            .id("Switch_1");
        Label::new(cx, "Switch with label").describing("Switch_1");
    })
    .size(Auto)
    .col_between(Pixels(5.0))
    .child_top(Stretch(1.0))
    .child_bottom(Stretch(1.0))
}

#[derive(Debug, Lens)]
pub struct SliderData {
    value: f32,
}

pub enum SliderEvent {
    SetValue(f32),
}

impl Model for SliderData {
    fn event(&mut self, _: &mut EventContext, event: &mut Event) {
        event.map(|app_event, _| match app_event {
            SliderEvent::SetValue(val) => {
                self.value = *val;
            }
        });
    }
}

pub fn slider(cx: &mut Context) -> Handle<impl View> {
    SliderData { value: 0.0 }.build(cx);

    Slider::new(cx, SliderData::value.map(|val| (val + 50.0) / 100.0))
        .range(0.0..1.0)
        .on_changing(move |cx, val| cx.emit(SliderEvent::SetValue(-50.0 + (val * 100.0))));

    HStack::new(cx, |cx| {
        Slider::new(cx, SliderData::value.map(|val| (val + 50.0) / 100.0))
            .range(0.0..1.0)
            .on_changing(move |cx, val| cx.emit(SliderEvent::SetValue(-50.0 + (val * 100.0))));
        Label::new(cx, SliderData::value.map(|val| format!("{:.2}", (val + 50.0) / 100.0)))
            .width(Pixels(50.0));
    })
    .child_top(Stretch(1.0))
    .child_bottom(Stretch(1.0))
    .height(Auto)
    .col_between(Pixels(8.0))
}

#[derive(Clone, Lens)]
struct RatingData {
    rating1: u32,
    rating2: u32,
}

impl Model for RatingData {
    fn event(&mut self, _cx: &mut EventContext, event: &mut Event) {
        event.map(|app_event, _| match app_event {
            RatingEvent::SetRating1(val) => self.rating1 = *val,
            RatingEvent::SetRating2(val) => self.rating2 = *val,
        })
    }
}

enum RatingEvent {
    SetRating1(u32),
    SetRating2(u32),
}

pub fn rating(cx: &mut Context) {
    RatingData { rating1: 3, rating2: 7 }.build(cx);

    Rating::new(cx, 5, RatingData::rating1)
        .on_change(|ex, rating| ex.emit(RatingEvent::SetRating1(rating)));

    Rating::new(cx, 10, RatingData::rating2)
        .on_change(|ex, rating| ex.emit(RatingEvent::SetRating2(rating)));
}
