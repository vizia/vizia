use vizia::*;

macro_rules! cursor_label {
    ($cx:ident, $name:ident) => {
        Label::new($cx, stringify!($name))
            .width(Pixels(110.0))
            .height(Pixels(30.0))
            .border_width(Pixels(1.0))
            .border_color(Color::black())
            .child_top(Stretch(1.0))
            .child_bottom(Stretch(1.0))
            .child_left(Pixels(5.0))
            .text_wrap(false)
            .on_hover(|cx| {
                cx.emit(WindowEvent::SetCursor(CursorIcon::$name));
            });
    };
}

fn main() {
    Application::new(WindowDescription::new().with_title("Cursor Icon"), |cx| {
        HStack::new(cx, |cx| {
            VStack::new(cx, |cx| {
                cursor_label!(cx, Default);
                cursor_label!(cx, Crosshair);
                cursor_label!(cx, Hand);
                cursor_label!(cx, Arrow);
                cursor_label!(cx, Move);
                cursor_label!(cx, Text);
                cursor_label!(cx, Wait);
                cursor_label!(cx, Help);
                cursor_label!(cx, Progress);
                cursor_label!(cx, NotAllowed);
                cursor_label!(cx, ContextMenu);
                cursor_label!(cx, Cell);
            })
            .child_space(Stretch(1.0))
            .row_between(Pixels(10.0));

            VStack::new(cx, |cx| {
                cursor_label!(cx, VerticalText);
                cursor_label!(cx, Alias);
                cursor_label!(cx, Copy);
                cursor_label!(cx, NoDrop);
                cursor_label!(cx, Grab);
                cursor_label!(cx, Grabbing);
                cursor_label!(cx, AllScroll);
                cursor_label!(cx, ZoomIn);
                cursor_label!(cx, ZoomOut);
                cursor_label!(cx, EResize);
                cursor_label!(cx, NResize);
                cursor_label!(cx, NeResize);
            })
            .child_space(Stretch(1.0))
            .row_between(Pixels(10.0));

            VStack::new(cx, |cx| {
                cursor_label!(cx, NwResize);
                cursor_label!(cx, SResize);
                cursor_label!(cx, SeResize);
                cursor_label!(cx, SwResize);
                cursor_label!(cx, WResize);
                cursor_label!(cx, EwResize);
                cursor_label!(cx, NsResize);
                cursor_label!(cx, NeswResize);
                cursor_label!(cx, NwseResize);
                cursor_label!(cx, ColResize);
                cursor_label!(cx, RowResize);
                cursor_label!(cx, None);
            })
            .child_space(Stretch(1.0))
            .row_between(Pixels(10.0));
        })
        .child_space(Stretch(1.0));
    })
    .run();
}
