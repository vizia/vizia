//! This example showcases the different delta values of the mouse state.

use vizia::prelude::*;

fn main() {
    Application::new(|cx| {
        MouseDeltaView::new(cx);
    })
    .title("Mouse Delta")
    .run();
}

pub struct MouseDeltaView;

impl MouseDeltaView {
    pub fn new(cx: &mut Context) -> Handle<Self> {
        Self.build(cx, |_| {})
    }
}

impl View for MouseDeltaView {
    fn event(&mut self, cx: &mut EventContext, _: &mut Event) {
        println!("                     |            x |            y ");
        println!("---------------------|--------------|--------------");

        let frame_delta = cx.mouse.frame_delta();
        println!("Frame delta          | {:>12.4} | {:>12.4}", frame_delta.0, frame_delta.1);

        if cx.mouse.left.state == MouseButtonState::Pressed {
            let delta = cx.mouse.delta(MouseButton::Left);
            println!("Pressed left delta   | {:>12.4} | {:>12.4}", delta.0, delta.1);
        }

        if cx.mouse.right.state == MouseButtonState::Pressed {
            let delta = cx.mouse.delta(MouseButton::Right);
            println!("Pressed right delta  | {:>12.4} | {:>12.4}", delta.0, delta.1);
        }

        if cx.mouse.middle.state == MouseButtonState::Pressed {
            let delta = cx.mouse.delta(MouseButton::Middle);
            println!("Pressed middle delta | {:>12.4} | {:>12.4}", delta.0, delta.1);
        }

        println!("");
    }
}
