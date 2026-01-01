mod helpers;
use helpers::*;
use vizia::prelude::*;

fn main() -> Result<(), ApplicationError> {
    let (app, (title, size)) = Application::new_with_state(|cx| {
        let scroll_x = cx.state(0.0);
        let scroll_y = cx.state(0.0);
        let size_300 = cx.state(Pixels(300.0));
        let big = cx.state(Pixels(1000.0));
        let stretch_one = cx.state(Stretch(1.0));
        let align_center = cx.state(Alignment::Center);
        let gap_50 = cx.state(Pixels(50.0));

        ExamplePage::vertical(cx, move |cx| {
            HStack::new(cx, |cx| {
                ScrollView::new(cx, move |cx| {
                    Label::static_text(cx, "Vertical Scroll")
                        .height(big)
                        .width(stretch_one);
                })
                .size(size_300)
                .class("bg-default");

                ScrollView::new(cx, move |cx| {
                    Label::static_text(cx, "Horizontal Scroll")
                        .width(big)
                        .height(stretch_one);
                })
                .size(size_300)
                .class("bg-default");

                ScrollView::new(cx, move |cx| {
                    Label::static_text(cx, "Horizontal and Vertical Scroll")
                        .width(big)
                        .height(big);
                })
                .size(size_300)
                .class("bg-default");
            })
            .alignment(align_center)
            .gap(gap_50);

            HStack::new(cx, |cx| {
                ScrollView::new(cx, move |cx| {
                    Label::static_text(cx, "Vertical Scroll")
                        .height(big)
                        .width(stretch_one);
                })
                .scroll_y(scroll_y)
                .on_scroll({
                    let scroll_y = scroll_y;
                    move |cx, _, value| {
                        scroll_y.set(cx, value);
                    }
                })
                .size(size_300)
                .class("bg-default");

                ScrollView::new(cx, move |cx| {
                    Label::static_text(cx, "Horizontal Scroll")
                        .width(big)
                        .height(stretch_one);
                })
                .scroll_x(scroll_x)
                .on_scroll({
                    let scroll_x = scroll_x;
                    move |cx, value, _| {
                        scroll_x.set(cx, value);
                    }
                })
                .size(size_300)
                .class("bg-default");

                ScrollView::new(cx, move |cx| {
                    Label::static_text(cx, "Horizontal and Vertical Scroll")
                        .width(big)
                        .height(big);
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
                .size(size_300)
                .class("bg-default");
            })
            .alignment(align_center)
            .gap(gap_50);
        });
        (cx.state("Scrollview"), cx.state((1100, 800)))
    });

    app.title(title).inner_size(size).run()
}
