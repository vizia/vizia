use vizia::*;

#[derive(Lens)]
pub struct ScrollData {
    scroll_x: f32,
    scroll_y: f32,
    ratio_x: f32,
    ratio_y: f32,
}

pub enum ScrollUpdate {
    ScrollX(f32),
    ScrollY(f32),
}

impl Model for ScrollData {
    fn event(&mut self, cx: &mut Context, event: &mut Event) {
        if let Some(msg) = event.message.downcast() {
            match msg {
                ScrollUpdate::ScrollX(f) => self.scroll_x = *f,
                ScrollUpdate::ScrollY(f) => self.scroll_y = *f,
            }
            event.consume();
        }
    }
}

fn main() {
    Application::new(WindowDescription::new(), |cx|{
        ScrollData {
            scroll_x: 0.0,
            scroll_y: 0.0,
            ratio_x: 0.5,
            ratio_y: 0.5
        }.build(cx);

        Scrollbar::new(cx, ScrollData::scroll_y, ScrollData::ratio_y, Orientation::Horizontal, |cx, value| {
            cx.emit(ScrollUpdate::ScrollY(value));
        });
    }).run();
}
