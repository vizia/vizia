mod helpers;
use helpers::*;
use vizia::prelude::*;

#[derive(Lens)]
pub struct AppData {
    scroll_x: f32,
    scroll_y: f32,
}

pub enum AppEvent {
    ScrollX(f32),
    ScrollY(f32),
}

impl Model for AppData {
    fn event(&mut self, _cx: &mut EventContext, event: &mut Event) {
        event.map(|app_event, _| match app_event {
            AppEvent::ScrollX(val) => self.scroll_x = *val,
            AppEvent::ScrollY(val) => self.scroll_y = *val,
        });
    }
}

fn main() -> Result<(), ApplicationError> {
    Application::new(|cx| {
        AppData { scroll_x: 0.0, scroll_y: 0.0 }.build(cx);

        ExamplePage::vertical(cx, |cx| {
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
                    Label::new(cx, "Horizontal and Vertical Scroll")
                        .width(Pixels(1000.0))
                        .height(Pixels(1000.0));
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
                .scroll_y(AppData::scroll_y)
                .on_scroll(|cx, _, scroll_y| cx.emit(AppEvent::ScrollY(scroll_y)))
                .size(Pixels(300.0))
                .class("bg-default");

                ScrollView::new(cx, |cx| {
                    Label::new(cx, "Horizontal Scroll").width(Pixels(1000.0)).height(Stretch(1.0));
                })
                .scroll_x(AppData::scroll_x)
                .on_scroll(|cx, scroll_x, _| cx.emit(AppEvent::ScrollX(scroll_x)))
                .size(Pixels(300.0))
                .class("bg-default");

                ScrollView::new(cx, |cx| {
                    Label::new(cx, "Horizontal and Vertical Scroll")
                        .width(Pixels(1000.0))
                        .height(Pixels(1000.0));
                })
                .scroll_x(AppData::scroll_x)
                .scroll_y(AppData::scroll_y)
                .on_scroll(|cx, scroll_x, scroll_y| {
                    cx.emit(AppEvent::ScrollX(scroll_x));
                    cx.emit(AppEvent::ScrollY(scroll_y));
                })
                .size(Pixels(300.0))
                .class("bg-default");
            })
            .alignment(Alignment::Center)
            .gap(Pixels(50.0));
        });
    })
    .title("Scrollview")
    .inner_size((1100, 800))
    .run()
}
