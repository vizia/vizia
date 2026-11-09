//! Profiles the work vizia performs when the colour of a single row in a long list changes.
//!
//! Run with:
//!
//! ```text
//! cargo run --release --example profile_list --features profiling [-- [hover|resize] <row>]
//! ```
//!
//! The app builds a list of 3000 rows inside a scroll view. Once the UI has settled, a timer
//! changes the UI once per second:
//!
//! - By default it toggles the background colour of a single row (row 10 by default; pass a row
//!   index to pick another, e.g. an off-screen row such as 2500).
//! - In `hover` mode it sends a synthetic mouse move which alternates the hovered row between the
//!   target row and the row below it, exercising the hover and style systems.
//! - In `resize` mode it alternates the window width between two sizes, exercising the resize
//!   path (layout and a full repaint of the window).
//!
//! Every update and draw pass triggered by the change is captured with [`vizia::profiling`] and
//! printed to stdout, showing which systems ran, how long each took, and how many tree nodes each
//! one visited.
//!
//! Keep the mouse outside the window while profiling, or real mouse-move events will be captured
//! as well.

use std::cell::Cell;

use vizia::prelude::*;
use vizia::profiling;

const ROWS: usize = 3000;
const ROW_HEIGHT: f32 = 24.0;
const DEFAULT_ROW: usize = 10;
const INTERVAL: Duration = Duration::from_secs(1);
const WINDOW_SIZE: (u32, u32) = (400, 600);
const RESIZED_WIDTH: u32 = 440;

#[derive(Clone, Copy, PartialEq)]
enum Mode {
    Color,
    Hover,
    Resize,
}

fn main() -> Result<(), ApplicationError> {
    let mut args = std::env::args().skip(1).peekable();
    let mode = match args.peek().map(String::as_str) {
        Some("hover") => Mode::Hover,
        Some("resize") => Mode::Resize,
        _ => Mode::Color,
    };
    if mode != Mode::Color {
        args.next();
    }
    let target_row =
        args.next().and_then(|arg| arg.parse::<usize>().ok()).unwrap_or(DEFAULT_ROW).min(ROWS - 2);

    Application::new(move |cx| {
        let colors: Vec<Signal<Color>> =
            (0..ROWS).map(|_| Signal::new(Color::transparent())).collect();
        let target = colors[target_row];

        ScrollView::new(cx, move |cx| {
            VStack::new(cx, move |cx| {
                for (index, color) in colors.iter().enumerate() {
                    HStack::new(cx, |cx| {
                        Label::new(cx, format!("Row {index}"));
                    })
                    .height(Pixels(ROW_HEIGHT))
                    .padding_left(Pixels(8.0))
                    .alignment(Alignment::Left)
                    .background_color(*color);
                }
            })
            .height(Auto);
        })
        .size(Stretch(1.0));

        let highlighted = Cell::new(false);
        let changes = Cell::new(0usize);

        let timer = cx.add_timer(INTERVAL, None, move |cx, action| {
            let TimerAction::Tick(_) = action else { return };

            // The capture started on the previous tick holds every pass triggered by that change.
            if profiling::is_capturing() {
                let capture = profiling::stop_capture();
                let description = match (mode, highlighted.get()) {
                    (Mode::Hover, true) => {
                        format!("hover row {target_row} -> row {}", target_row + 1)
                    }
                    (Mode::Hover, false) => {
                        format!("hover row {} -> row {target_row}", target_row + 1)
                    }
                    (Mode::Resize, true) => format!("window width -> {RESIZED_WIDTH}"),
                    (Mode::Resize, false) => format!("window width -> {}", WINDOW_SIZE.0),
                    (Mode::Color, true) => format!("row {target_row} background -> highlighted"),
                    (Mode::Color, false) => format!("row {target_row} background -> transparent"),
                };
                println!("\n##### Change #{}: {description} #####", changes.get());
                println!("{capture}");
            }

            highlighted.set(!highlighted.get());
            changes.set(changes.get() + 1);

            profiling::start_capture();
            if mode == Mode::Hover {
                let row = if highlighted.get() { target_row + 1 } else { target_row };
                let scale = cx.scale_factor();
                let x = 50.0 * scale;
                let y = (row as f32 + 0.5) * ROW_HEIGHT * scale;
                // Window events only update hover state when they originate from a window.
                cx.emit_custom(
                    Event::new(WindowEvent::MouseMove(x, y))
                        .origin(Entity::root())
                        .target(Entity::root())
                        .propagate(Propagation::Direct),
                );
            } else if mode == Mode::Resize {
                let width = if highlighted.get() { RESIZED_WIDTH } else { WINDOW_SIZE.0 };
                cx.emit_custom(
                    Event::new(WindowEvent::SetSize(WindowSize::new(width, WINDOW_SIZE.1)))
                        .origin(Entity::root())
                        .target(Entity::root())
                        .propagate(Propagation::Direct),
                );
            } else {
                target.set(if highlighted.get() {
                    Color::rgb(255, 120, 60)
                } else {
                    Color::transparent()
                });
            }
        });
        cx.start_timer(timer);
    })
    .title(match mode {
        Mode::Hover => {
            format!("Profile: {ROWS} rows, hovering rows {target_row}/{}", target_row + 1)
        }
        Mode::Resize => format!("Profile: {ROWS} rows, resizing the window"),
        Mode::Color => format!("Profile: {ROWS} rows, toggling row {target_row}"),
    })
    .inner_size(WINDOW_SIZE)
    .run()
}
