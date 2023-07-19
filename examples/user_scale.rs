use vizia::prelude::*;

fn main() {
    Application::new(app_main).title("Window resize demo").inner_size((600, 300)).run();
}

#[derive(Lens)]
pub struct AppData {
    user_scale_factor: f64,
    window_size: WindowSize,
}

#[cfg(not(feature = "baseview"))]
fn app_main(cx: &mut Context) {
    Label::new(cx, "This example only works with the baseview backend").space(Stretch(100.0));
}

#[cfg(feature = "baseview")]
fn app_main(cx: &mut Context) {
    // This stores the current scale factor so we can display it in the textbox below
    AppData { user_scale_factor: cx.user_scale_factor(), window_size: cx.window_size() }.build(cx);

    VStack::new(cx, |cx| {
        VStack::new(cx, |cx| {
            Label::new(cx, "User scale")
                .width(Percentage(100.0))
                .child_space(Stretch(1.0))
                .bottom(Pixels(5.0))
                .top(Pixels(-5.0));
            Textbox::new(cx, AppData::user_scale_factor).width(Percentage(100.0)).on_submit(
                |cx, value, blur| {
                    if blur {
                        cx.set_user_scale_factor(value);
                    }
                },
            );

            Label::new(cx, "Window size")
                .width(Percentage(100.0))
                .child_space(Stretch(1.0))
                .bottom(Pixels(5.0))
                .top(Pixels(5.0));
            Textbox::new(
                cx,
                AppData::window_size
                    .map(|WindowSize { width, height }| format!("{width}x{height}")),
            )
            .width(Percentage(100.0))
            .on_submit(|cx, value, success| {
                if success {
                    let parsed = value
                        .split_once('x')
                        .map(|(width, height)| (width.parse(), height.parse()));
                    if let Some((Ok(width), Ok(height))) = parsed {
                        cx.set_window_size(WindowSize { width, height });
                    }
                }
            });
        })
        .width(Pixels(100.0));
    })
    .space(Stretch(1.0))
    .width(Auto)
    .child_space(Pixels(10.0))
    .background_color(Color::from("#fafafa"))
    .border_color(Color::from("#dadada"))
    .border_width(Pixels(1.0))
    .border_radius(Pixels(5.0));

    // TODO: Resize handle
}

impl Model for AppData {
    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|window_event, _| {
            if let WindowEvent::GeometryChanged(_) = window_event {
                self.user_scale_factor = cx.user_scale_factor();
                self.window_size = cx.window_size();
            }
        });
    }
}
