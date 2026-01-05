pub use vizia::prelude::*;

#[cfg(feature = "baseview")]
fn main() {
    panic!("This example is not supported on baseview");
}

#[cfg(not(feature = "baseview"))]
struct PopupWindowApp {
    red: Signal<f32>,
    green: Signal<f32>,
    blue: Signal<f32>,
    show_popup: Signal<bool>,
    popup_title: Signal<String>,
    popup_size: Signal<(u32, u32)>,
    popup_anchor: Signal<Anchor>,
    padding_20: Signal<Units>,
    gap_12: Signal<Units>,
    align_center: Signal<Alignment>,
}

#[cfg(not(feature = "baseview"))]
impl App for PopupWindowApp {
    fn new(cx: &mut Context) -> Self {
        Self {
            red: cx.state(1.0f32),
            green: cx.state(1.0f32),
            blue: cx.state(1.0f32),
            show_popup: cx.state(false),
            popup_title: cx.state("Set color...".to_string()),
            popup_size: cx.state((400, 200)),
            popup_anchor: cx.state(Anchor::Center),
            padding_20: cx.state(Pixels(20.0)),
            gap_12: cx.state(Pixels(12.0)),
            align_center: cx.state(Alignment::Center),
        }
    }

    fn on_build(self, cx: &mut Context) -> Self {
        let red = self.red;
        let green = self.green;
        let blue = self.blue;
        let show_popup = self.show_popup;
        let popup_title = self.popup_title;
        let popup_size = self.popup_size;
        let popup_anchor = self.popup_anchor;
        let padding_20 = self.padding_20;
        let gap_12 = self.gap_12;
        let align_center = self.align_center;

        Binding::new(cx, show_popup, move |cx| {
            if *show_popup.get(cx) {
                Window::popup(cx, false, move |cx| {
                    VStack::new(cx, move |cx: &mut Context| {
                        Slider::new(cx, red).on_change(move |cx, val| red.set(cx, val));
                        Slider::new(cx, green).on_change(move |cx, val| green.set(cx, val));
                        Slider::new(cx, blue).on_change(move |cx, val| blue.set(cx, val));
                    })
                    .padding(padding_20)
                    .alignment(align_center)
                    .vertical_gap(gap_12);
                })
                .on_close(move |cx| show_popup.set(cx, false))
                .title(popup_title)
                .inner_size(popup_size)
                .anchor(popup_anchor);
            }
        });

        let color = cx.derived(move |cx| {
            let r = (*red.get(cx) * 255.0) as u8;
            let g = (*green.get(cx) * 255.0) as u8;
            let b = (*blue.get(cx) * 255.0) as u8;
            Color::rgb(r, g, b)
        });

        HStack::new(cx, move |cx| {
            Button::new(cx, |cx| Label::new(cx, "Show Popup"))
                .on_press(move |cx| show_popup.set(cx, true));
        })
        .padding(padding_20)
        .background_color(color);

        self
    }

    fn window_config(&self) -> WindowConfig {
        window(|app| app.title("Main").position((100, 100)))
    }
}

#[cfg(not(feature = "baseview"))]
fn main() -> Result<(), ApplicationError> {
    PopupWindowApp::run()
}
