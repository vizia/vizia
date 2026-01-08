mod helpers;
use helpers::*;
use vizia::icons::ICON_SEARCH;
use vizia::prelude::*;

struct TextboxApp {
    editable_text: Signal<String>,
    multiline_text: Signal<String>,
    non_editable_text: Signal<String>,
    non_editable_multiline_text: Signal<String>,
    placeholder: Signal<&'static str>,
    search_placeholder: Signal<&'static str>,
}

impl App for TextboxApp {
    fn new(cx: &mut Context) -> Self {
        Self {
            editable_text: cx.state("Editable text".to_string()),
            multiline_text: cx.state("This is some text which is editable and spans multiple lines".to_string()),
            non_editable_text: cx.state("This text can be selected but not edited".to_string()),
            non_editable_multiline_text: cx.state("This text can be selected but not edited and spans multiple lines".to_string()),
            placeholder: cx.state("Type something..."),
            search_placeholder: cx.state("Search"),
        }
    }

    fn on_build(self, cx: &mut Context) -> Self {
        let editable_text = self.editable_text;
        let multiline_text = self.multiline_text;
        let non_editable_text = self.non_editable_text;
        let non_editable_multiline_text = self.non_editable_multiline_text;
        let placeholder = self.placeholder;
        let search_placeholder = self.search_placeholder;

        ExamplePage::vertical(cx, |cx| {
            Textbox::new(cx, editable_text)
                .width(Pixels(300.0))
                .placeholder(placeholder)
                .on_edit(move |cx, text| editable_text.set(cx, text));

            HStack::new(cx, |cx| {
                Textbox::new(cx, editable_text)
                    .class("icon-before")
                    .width(Stretch(1.0))
                    .placeholder(search_placeholder)
                    .on_edit(move |cx, text| editable_text.set(cx, text));
                Svg::new(cx, ICON_SEARCH)
                    .color(Color::gray())
                    .position_type(PositionType::Absolute)
                    .top(Stretch(1.0))
                    .bottom(Stretch(1.0));
            })
            .height(Auto)
            .width(Pixels(300.0));

            Textbox::new_multiline(cx, multiline_text, true)
                .width(Pixels(300.0))
                .on_edit(move |cx, text| multiline_text.set(cx, text));

            Textbox::new(cx, non_editable_text).width(Auto).read_only(true);
            Textbox::new_multiline(cx, non_editable_multiline_text, true)
                .width(Pixels(300.0))
                .read_only(true);
        });
        self
    }
}

fn main() -> Result<(), ApplicationError> {
    TextboxApp::run()
}
