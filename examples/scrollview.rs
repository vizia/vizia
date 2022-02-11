

use vizia::*;

fn main() {
    Application::new(WindowDescription::new(), |cx|{

        ScrollData {
            height_ratio: 0.0,
            container_height: 0.0,
            width_ratio: 0.0,
            container_width: 0.0,
            scroll_height: 0.0,
        }.build(cx);

        HStack::new(cx, |cx|{
            ScrollView::new(cx, |cx|{
                ForEach::new(cx, 0..15, |cx, index|{
                    Label::new(cx, &format!("Content: {}", index))
                        .height(Pixels(50.0))
                        .width(Pixels(400.0))
                        .child_space(Stretch(1.0))
                        .background_color(Color::red())
                        .overflow(Overflow::Hidden);
                });
            }).width(Pixels(300.0)).min_size(Pixels(0.0)).background_color(Color::blue());

            Binding::new(cx, ScrollData::root, |cx, val|{
                let scrollh = val.get(cx).scroll_height;
                let scrolly = val.get(cx).height_ratio;
                Scrollbar::new(cx, scrolly, scrollh, Orientation::Vertical)
                    .on_changing(|cx, val|{
                        cx.emit(ScrollEvent::SetHeightRatio(val));
                    })
                    .class("vertical");
            });

        });


        // Binding::new(cx, ScrollData::root, |cx, val|{
        //     Scrollbar::new(cx, val., Orientation::Horizontal)
        //         .on_changing(|cx, val|{
        //             cx.emit(ScrollEvent::SetWidthRatio(val));
        //         });
        // });
    }).run();
}