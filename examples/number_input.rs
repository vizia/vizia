use vizia::prelude::*;

const STYLE: &str = r#"
    :root {
        alignment: center;
    }
    number-input:invalid textbox {
        border-color: #ff0000;
    }
"#;

fn main() -> Result<(), ApplicationError> {
    NumberInputApp::run()
}

struct NumberInputApp {
    number: Signal<i32>,
    width_200: Signal<Units>,
    padding_5: Signal<Units>,
    height_32: Signal<Units>,
    align_center: Signal<Alignment>,
    auto: Signal<Units>,
    stretch_one: Signal<Stretch>,
    gap_10: Signal<Units>,
    title: Signal<&'static str>,
    size: Signal<(u32, u32)>,
}

impl App for NumberInputApp {
    fn new(cx: &mut Context) -> Self {
        Self {
            number: cx.state(5i32),
            width_200: cx.state(Pixels(200.0)),
            padding_5: cx.state(Pixels(5.0)),
            height_32: cx.state(Pixels(32.0)),
            align_center: cx.state(Alignment::Center),
            auto: cx.state(Auto),
            stretch_one: cx.state(Stretch(1.0)),
            gap_10: cx.state(Pixels(10.0)),
            title: cx.state("Number Input"),
            size: cx.state((600, 300)),
        }
    }

    fn on_build(self, cx: &mut Context) -> Self {
        cx.add_stylesheet(STYLE).expect("Failed to add stylesheet");
        let number = self.number;
        let width_200 = self.width_200;
        let padding_5 = self.padding_5;
        let height_32 = self.height_32;
        let align_center = self.align_center;
        let auto = self.auto;
        let stretch_one = self.stretch_one;
        let gap_10 = self.gap_10;

        HStack::new(cx, |cx| {
            // NumberInput with max validation and two-way binding
            NumberInput::new(cx, number).max(49).two_way().width(width_200).padding_left(padding_5);

            Label::new(cx, number)
                .width(width_200)
                .height(height_32)
                .alignment(align_center)
                .padding_left(padding_5);
        })
        .alignment(align_center)
        .height(auto)
        .space(stretch_one)
        .alignment(align_center)
        .horizontal_gap(gap_10);
        
        self
    }

    fn window_config(&self) -> WindowConfig {
        let title = self.title;
        let size = self.size;
        window(move |app| app.title(title).inner_size(size))
    }
}
