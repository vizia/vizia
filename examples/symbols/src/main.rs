use symbols::{Symbol, SYMBOLS};
use vizia::prelude::*;

mod app_data;
use app_data::*;

mod app_event;

mod categories;
mod controls;

mod symbols;

fn main() {
    Application::new(|cx| {
        cx.add_stylesheet("examples/symbols/src/style.css").expect("Failed to load stylsheet");

        AppData::new().build(cx);
        VStack::new(cx, |cx| {
            // Top Bar
            HStack::new(cx, |cx| {
                // Searchbox here
            })
            .class("top-bar");

            // Partition into three panels (categories, icons, info)
            HStack::new(cx, |cx| {
                //ScrollView::new(cx, 0.0, 0.0, true, true, |cx| {
                // List::new(cx, AppData::categories, |cx, index, item| {
                //     Label::new(cx, item);
                // });
                //});

                // Icon View
                VStack::new(cx, |cx| {
                    HStack::new(cx, |cx| {
                        Label::new(cx, "symbol").class("symbol-label");
                        Label::new(cx, "name").class("name-label");
                        Label::new(cx, "unicode").class("unicode-label");
                    })
                    .class("headings");
                    ScrollView::new(cx, 0.0, 0.0, false, true, |cx| {
                        List::new(cx, StaticLens::new(SYMBOLS.as_ref()), |cx, index, item| {
                            HStack::new(cx, |cx| {
                                Label::new(cx, item.then(Symbol::txt))
                                    .class("symbol")
                                    .class("symbol-label")
                                    .class("icon");
                                Label::new(cx, item.then(Symbol::name)).class("name-label");
                                Label::new(
                                    cx,
                                    item.then(Symbol::txt.map(|txt| {
                                        format!("{:x}", txt.chars().nth(0).unwrap() as u32)
                                    })),
                                )
                                .class("unicode-label");
                            })
                            .class("list-row")
                            .toggle_class("odd", index % 2 != 0);
                        })
                        .class("symbol-list")
                        .width(Stretch(1.0));
                    });
                })
                .class("icon-view");
            });
        });
    })
    .run();
}
