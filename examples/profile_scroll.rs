//! Profiles the work vizia performs when a long list inside a scroll view is scrolled.
//!
//! Run with:
//!
//! ```text
//! cargo run --release --example profile_scroll --features profiling [-- [wheel|jump] <amount>]
//! ```
//!
//! The app builds a list of 3000 rows inside a scroll view. Once the UI has settled, a timer
//! scrolls the list once per second, alternating between scrolling down and back up:
//!
//! - In `wheel` mode (the default) it sends a synthetic mouse wheel event over the list, scrolling
//!   by `<amount>` lines (3 by default), the same path a real mouse wheel takes.
//! - In `jump` mode it sets the vertical scroll position directly, jumping between the top and
//!   `<amount>` percent of the way down the list (50 by default), like dragging the scrollbar.
//!
//! Every update and draw pass triggered by each scroll is captured with [`vizia::profiling`] and
//! printed to stdout, showing which systems ran and how long each took.
//!
//! Keep the mouse outside the window while profiling, or real mouse events will be captured as
//! well.

use std::cell::Cell;

use vizia::prelude::*;
use vizia::profiling;

const ROWS: usize = 3000;
const ROW_HEIGHT: f32 = 24.0;
const INTERVAL: Duration = Duration::from_secs(1);
const WINDOW_SIZE: (u32, u32) = (400, 600);
const DEFAULT_LINES: f32 = 3.0;
const DEFAULT_PERCENT: f32 = 50.0;

#[derive(Clone, Copy, PartialEq)]
enum Mode {
    Wheel,
    Jump,
}

fn main() -> Result<(), ApplicationError> {
    let mut args = std::env::args().skip(1).peekable();
    let mode = match args.peek().map(String::as_str) {
        Some("jump") => Mode::Jump,
        _ => Mode::Wheel,
    };
    if matches!(args.peek().map(String::as_str), Some("wheel" | "jump")) {
        args.next();
    }
    let amount = args.next().and_then(|arg| arg.parse::<f32>().ok());
    let lines = amount.unwrap_or(DEFAULT_LINES);
    let percent = amount.unwrap_or(DEFAULT_PERCENT).clamp(0.0, 100.0);

    Application::new(move |cx| {
        let scroll_view = ScrollView::new(cx, |cx| {
            VStack::new(cx, |cx| {
                for index in 0..ROWS {
                    HStack::new(cx, |cx| {
                        Label::new(cx, format!("Row {index}"));
                    })
                    .height(Pixels(ROW_HEIGHT))
                    .padding_left(Pixels(8.0))
                    .alignment(Alignment::Left);
                }
            })
            .height(Auto);
        })
        .size(Stretch(1.0))
        .entity();

        let ticks = Cell::new(0usize);

        let timer = cx.add_timer(INTERVAL, None, move |cx, action| {
            let TimerAction::Tick(_) = action else { return };
            let tick = ticks.get();
            ticks.set(tick + 1);

            // The capture started on the previous tick holds every pass triggered by that scroll.
            if profiling::is_capturing() {
                let capture = profiling::stop_capture();
                let down = tick % 2 == 0;
                let description = match (mode, down) {
                    (Mode::Wheel, true) => format!("wheel down {lines} lines"),
                    (Mode::Wheel, false) => format!("wheel up {lines} lines"),
                    (Mode::Jump, true) => format!("jump to {percent}%"),
                    (Mode::Jump, false) => "jump to top".to_string(),
                };
                println!("\n##### Scroll #{}: {description} #####", tick - 1);
                println!("{capture}");
            }

            if tick == 0 {
                // Wheel events are sent to the hovered entity, so move the pointer over the list
                // first. This happens outside a capture so it doesn't skew the first scroll.
                if mode == Mode::Wheel {
                    let scale = cx.scale_factor();
                    let (x, y) = (50.0 * scale, WINDOW_SIZE.1 as f32 * 0.5 * scale);
                    emit_window_event(cx, Entity::root(), WindowEvent::MouseMove(x, y));
                }
                return;
            }

            let down = tick % 2 == 1;
            profiling::start_capture();
            match mode {
                Mode::Wheel => {
                    // Negative deltas scroll down, matching the deltas winit reports.
                    let delta = if down { -lines } else { lines };
                    emit_window_event(cx, Entity::root(), WindowEvent::MouseScroll(0.0, delta));
                }
                Mode::Jump => {
                    let position = if down { percent / 100.0 } else { 0.0 };
                    cx.emit_custom(
                        Event::new(ScrollEvent::SetY(position))
                            .target(scroll_view)
                            .propagate(Propagation::Direct),
                    );
                }
            }
        });
        cx.start_timer(timer);
    })
    .title(match mode {
        Mode::Wheel => format!("Profile: scrolling {ROWS} rows by {lines} lines"),
        Mode::Jump => format!("Profile: jumping {percent}% through {ROWS} rows"),
    })
    .inner_size(WINDOW_SIZE)
    .run()
}

/// Sends a window event as if it came from the window, so the event manager handles it like real
/// input (e.g. retargeting wheel events to the hovered entity).
fn emit_window_event(cx: &mut EventContext, window: Entity, event: WindowEvent) {
    cx.emit_custom(Event::new(event).origin(window).target(window).propagate(Propagation::Up));
}
