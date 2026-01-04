mod helpers;
use helpers::*;
use vizia::prelude::*;

fn main() -> Result<(), ApplicationError> {
    ScrollviewApp::run()
}

struct ScrollviewApp {
    scroll_x: Signal<f32>,
    scroll_y: Signal<f32>,
}

impl App for ScrollviewApp {
    fn new(cx: &mut Context) -> Self {
        Self {
            scroll_x: cx.state(0.0),
            scroll_y: cx.state(0.0),
        }
    }

    fn on_build(self, cx: &mut Context) -> Self {
        let scroll_x = self.scroll_x;
        let scroll_y = self.scroll_y;

        ExamplePage::vertical(cx, move |cx| {
            HStack::new(cx, |cx| {
                ScrollView::new(cx, |cx| {
                    Label::new(cx, "Vertical Scroll").height(Pixels(1000.0)).width(Stretch(1.0));
                })
                .size(Pixels(300.0))
                .class("bg-default");

                ScrollView::new(cx, |cx| {
                    Label::new(cx, "Horizontal Scroll").width(Pixels(1000.0)).height(Stretch(1.0));
                })
                .size(Pixels(300.0))
                .class("bg-default");

                ScrollView::new(cx, |cx| {
                    Label::new(cx, "Horizontal and Vertical Scroll").width(Pixels(1000.0)).height(Pixels(1000.0));
                })
                .size(Pixels(300.0))
                .class("bg-default");
            })
            .alignment(Alignment::Center)
            .gap(Pixels(50.0));

            HStack::new(cx, |cx| {
                ScrollView::new(cx, |cx| {
                    Label::new(cx, "Vertical Scroll").height(Pixels(1000.0)).width(Stretch(1.0));
                })
                .scroll_y(scroll_y)
                .on_scroll({
                    let scroll_y = scroll_y;
                    move |cx, _, value| {
                        scroll_y.set(cx, value);
                    }
                })
                .size(Pixels(300.0))
                .class("bg-default");

                ScrollView::new(cx, |cx| {
                    Label::new(cx, "Horizontal Scroll").width(Pixels(1000.0)).height(Stretch(1.0));
                })
                .scroll_x(scroll_x)
                .on_scroll({
                    let scroll_x = scroll_x;
                    move |cx, value, _| {
                        scroll_x.set(cx, value);
                    }
                })
                .size(Pixels(300.0))
                .class("bg-default");

                ScrollView::new(cx, |cx| {
                    Label::new(cx, "Horizontal and Vertical Scroll").width(Pixels(1000.0)).height(Pixels(1000.0));
                })
                .scroll_x(scroll_x)
                .scroll_y(scroll_y)
                .on_scroll({
                    let scroll_x = scroll_x;
                    let scroll_y = scroll_y;
                    move |cx, x, y| {
                        scroll_x.set(cx, x);
                        scroll_y.set(cx, y);
                    }
                })
                .size(Pixels(300.0))
                .class("bg-default");
            })
            .alignment(Alignment::Center)
            .gap(Pixels(50.0));
        });
        self
    }

    fn window_config(&self) -> WindowConfig {
        window(|app| app.title("Scrollview").inner_size((1100, 800)))
    }
}
