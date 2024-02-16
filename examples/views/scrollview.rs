mod helpers;
use helpers::*;
use vizia::prelude::*;

// #[derive(Lens)]
// pub struct ScrollData {
//     scrollx: f32,
//     scrolly: f32,
// }

// pub enum ScrollEvent {
//     SetScroll(f32, f32),
// }

// impl Model for ScrollData {
//     fn event(&mut self, _cx: &mut EventContext, event: &mut Event) {
//         event.map(|scroll_event, _| match scroll_event {
//             ScrollEvent::SetScroll(x, y) => {
//                 self.scrollx = *x;
//                 self.scrolly = *y;
//             }
//         })
//     }
// }

fn main() -> Result<(), ApplicationError> {
    Application::new(|cx| {
        ExamplePage::new(cx, |cx| {
            HStack::new(cx, |cx| {
                // ScrollData { scrollx: 0.0, scrolly: 0.0 }.build(cx);

                ScrollView::new(cx, 0.0, 0.0, false, true, |cx| {
                    Label::new(cx, "Vertical Scroll")
                        .height(Pixels(1000.0))
                        .width(Stretch(1.0))
                        .background_color(Color::from("#EF5151"));
                })
                .size(Units::Pixels(300.0))
                .class("bg-default");

                ScrollView::new(cx, 0.0, 0.0, true, false, |cx| {
                    Label::new(cx, "Horizontal Scroll")
                        .width(Pixels(1000.0))
                        .height(Stretch(1.0))
                        .background_color(Color::from("#EF5151"));
                })
                .size(Units::Pixels(300.0))
                .class("bg-default");

                ScrollView::new(cx, 0.0, 0.0, true, true, |cx| {
                    Label::new(cx, "Horizontal and Vertical Scroll")
                        .width(Pixels(1000.0))
                        .height(Pixels(1000.0))
                        .background_color(Color::from("#EF5151"));
                })
                .size(Units::Pixels(300.0))
                .class("bg-default");
            })
            .child_space(Stretch(1.0))
            .col_between(Pixels(50.0));
        });
    })
    .title("Scrollview")
    .inner_size((1100, 400))
    .run()
}
