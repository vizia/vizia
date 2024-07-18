#[allow(unused)]
use vizia::{icons::ICON_X, prelude::*};
mod helpers;
use helpers::*;

#[cfg(feature = "baseview")]
fn main() {
    panic!("This example is not supported on baseview");
}

#[derive(Lens)]
pub struct AppData {
    color: Color,
    show_window: bool,
}

impl Model for AppData {
    fn event(&mut self, _cx: &mut EventContext, event: &mut Event) {
        event.map(|app_event, _| match app_event {
            AppEvent::CloseWindow => {
                self.show_window = false;
            }

            AppEvent::OpenWindow => {
                self.show_window = true;
            }

            AppEvent::SetRed(val) => {
                self.color = Color::rgb((*val * 255.0) as u8, self.color.g(), self.color.b())
            }
            AppEvent::SetGreen(val) => {
                self.color = Color::rgb(self.color.r(), (*val * 255.0) as u8, self.color.b())
            }
            AppEvent::SetBlue(val) => {
                self.color = Color::rgb(self.color.r(), self.color.g(), (*val * 255.0) as u8)
            }
        });
    }
}

pub enum AppEvent {
    CloseWindow,
    OpenWindow,
    SetRed(f32),
    SetGreen(f32),
    SetBlue(f32),
}

fn sliders(cx: &mut Context) {
    VStack::new(cx, |cx: &mut Context| {
        Slider::new(cx, AppData::color.map(|c| c.r() as f32 / 255.0))
            .on_changing(|cx, val| cx.emit(AppEvent::SetRed(val)));
        Slider::new(cx, AppData::color.map(|c| c.g() as f32 / 255.0))
            .on_changing(|cx, val| cx.emit(AppEvent::SetGreen(val)));
        Slider::new(cx, AppData::color.map(|c| c.b() as f32 / 255.0))
            .on_changing(|cx, val| cx.emit(AppEvent::SetBlue(val)));
    })
    .child_space(Pixels(20.0))
    .child_top(Stretch(1.0))
    .child_bottom(Stretch(1.0))
    .row_between(Pixels(12.0));
}

#[cfg(not(feature = "baseview"))]
fn main() -> Result<(), ApplicationError> {
    Application::new(|cx| {
        AppData { color: Color::white(), show_window: false }.build(cx);

        Window::new(cx, |cx| {
            HStack::new(cx, |cx| {
                IconButton::new(cx, ICON_X).on_press(|cx| cx.emit(AppEvent::CloseWindow));
            })
            .on_press_down(|cx| cx.emit(WindowEvent::DragWindow))
            .background_color(Color::rgb(100, 100, 100))
            .height(Pixels(40.0))
            .child_left(Stretch(1.0))
            .child_top(Stretch(1.0))
            .child_bottom(Stretch(1.0))
            .child_right(Pixels(4.0));
            sliders(cx);
        })
        //.on_create(|cx| cx.emit(WindowEvent::DragWindow))
        .on_close(|cx| {
            cx.emit(AppEvent::CloseWindow);
        })
        .title("Set color...")
        .decorations(false)
        .bind(AppData::show_window, |mut handle, show| {
            if show.get(&handle) {
                handle.context().emit(WindowEvent::SetVisible(true));
                let x = handle.context().mouse.cursor_x as u32;
                let y = handle.context().mouse.cursor_y as u32;
                handle.context().emit(WindowEvent::SetPosition((x, y).into()));
                handle.context().emit(WindowEvent::DragWindow);
            } else {
                handle.context().emit(WindowEvent::SetVisible(false));
            }
        })
        .inner_size((200, 600))
        //.position((500, 500))
        .background_color(Color::gray());

        HStack::new(cx, |cx| {
            ExamplePage::vertical(cx, |cx| {
                Element::new(cx).size(Pixels(50.0)).background_color(Color::red()).on_drag(|cx| {
                    cx.emit(AppEvent::OpenWindow);
                });
            });
            Binding::new(cx, AppData::show_window, |cx, show_subwindow| {
                if !show_subwindow.get(cx) {
                    VStack::new(cx, |cx| {
                        Element::new(cx)
                            .height(Pixels(40.0))
                            .background_color(Color::rgb(100, 100, 100))
                            .on_drag(|cx| {
                                cx.emit(AppEvent::OpenWindow);
                            });
                        sliders(cx);
                    })
                    .background_color(Color::gray())
                    .width(Pixels(200.0));
                }
            });
        })
        .background_color(AppData::color);
    })
    .position((100, 100))
    .run()
}
