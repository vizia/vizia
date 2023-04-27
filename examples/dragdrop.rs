use vizia::prelude::*;
mod helpers;
use helpers::*;

fn main() {
    Application::new(|cx| {
        ExamplePage::vertical(cx, |cx| {
            HStack::new(cx, |cx| {
                Element::new(cx).size(Pixels(50.0)).background_color(Color::red()).on_drag(|ex| {
                    ex.set_drop_data(ex.current());
                });

                Element::new(cx).size(Pixels(50.0)).background_color(Color::green()).on_drag(
                    |ex| {
                        ex.set_drop_data(ex.current());
                    },
                );

                Element::new(cx).size(Pixels(50.0)).background_color(Color::blue()).on_drag(|ex| {
                    ex.set_drop_data(ex.current());
                });
            })
            .height(Pixels(100.0))
            .width(Auto)
            .col_between(Pixels(20.0))
            .child_space(Stretch(1.0));

            Element::new(cx)
                .size(Pixels(100.0))
                .background_color(Color::beige())
                .on_drop(|ex, data| {
                    if let DropData::Id(id) = data {
                        let bg = ex.with_current(id, |ex| ex.background_color());
                        ex.set_background_color(bg);
                        ex.emit(WindowEvent::SetCursor(CursorIcon::Default));
                    }
                    if let DropData::File(file) = data {
                        println!("Dropped File: {:?}", file);
                    }
                })
                .on_hover(|ex| {
                    if ex.has_drop_data() {
                        ex.emit(WindowEvent::SetCursor(CursorIcon::Copy));
                    } else {
                        ex.emit(WindowEvent::SetCursor(CursorIcon::Default));
                    }
                });
        });
    })
    .run();
}
