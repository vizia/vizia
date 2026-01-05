pub use vizia::prelude::*;

#[cfg(feature = "baseview")]
fn main() {
    panic!("This example is not supported on baseview");
}

#[cfg(not(feature = "baseview"))]
struct MultiWindowApp {
    red: Signal<f32>,
    green: Signal<f32>,
    blue: Signal<f32>,
    show_window: Signal<bool>,
    padding_20: Signal<Units>,
    align_center: Signal<Alignment>,
    gap_12: Signal<Units>,
    auto: Signal<Units>,
    window_title: Signal<&'static str>,
    window_size: Signal<(u32, u32)>,
    window_anchor: Signal<Anchor>,
}

#[cfg(not(feature = "baseview"))]
impl App for MultiWindowApp {
    fn new(cx: &mut Context) -> Self {
        Self {
            red: cx.state(1.0f32),
            green: cx.state(1.0f32),
            blue: cx.state(1.0f32),
            show_window: cx.state(false),
            padding_20: cx.state(Pixels(20.0)),
            align_center: cx.state(Alignment::Center),
            gap_12: cx.state(Pixels(12.0)),
            auto: cx.state(Auto),
            window_title: cx.state("Set color..."),
            window_size: cx.state((400, 200)),
            window_anchor: cx.state(Anchor::Center),
        }
    }

    fn on_build(self, cx: &mut Context) -> Self {
        let red = self.red;
        let green = self.green;
        let blue = self.blue;
        let show_window = self.show_window;
        let padding_20 = self.padding_20;
        let align_center = self.align_center;
        let gap_12 = self.gap_12;
        let auto = self.auto;
        let window_title = self.window_title;
        let window_size = self.window_size;
        let window_anchor = self.window_anchor;

        Binding::new(cx, show_window, move |cx| {
            if *show_window.get(cx) {
                Window::new(cx, move |cx| {
                    VStack::new(cx, move |cx: &mut Context| {
                        Slider::new(cx, red).two_way();
                        Slider::new(cx, green).two_way();
                        Slider::new(cx, blue).two_way();
                    })
                    .padding(padding_20)
                    .alignment(align_center)
                    .vertical_gap(gap_12);
                })
                .on_close(move |cx| show_window.set(cx, false))
                .title(window_title)
                .inner_size(window_size)
                .anchor(window_anchor);
            }
        });

        let color = cx.derived(move |cx| {
            let r = (*red.get(cx) * 255.0) as u8;
            let g = (*green.get(cx) * 255.0) as u8;
            let b = (*blue.get(cx) * 255.0) as u8;
            Color::rgb(r, g, b)
        });

        HStack::new(cx, move |cx| {
            Button::new(cx, |cx| Label::new(cx, "Show Window"))
                .on_press(move |cx| show_window.set(cx, true));
        })
        .size(auto)
        .padding(padding_20)
        .background_color(color);

        self
    }
}

#[cfg(not(feature = "baseview"))]
fn main() -> Result<(), ApplicationError> {
    MultiWindowApp::run()
}
