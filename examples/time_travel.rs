//! Time Travel Debugging Example
//!
//! Demonstrates the time travel debugging feature for navigating through signal history.
//!
//! ## Keybinds (debug builds only):
//! - `Cmd/Ctrl+[` - Step backward in history
//! - `Cmd/Ctrl+Shift+[` - Step forward in history
//! - `Cmd/Ctrl+`` ` `` - Toggle time travel overlay
//! - `Escape` - Exit time travel mode
//!
//! Run with: `cargo run --example time_travel`

use vizia::prelude::*;

struct TimeTravelApp {
    count: Signal<i32>,
    items: Signal<Vec<String>>,
}

impl App for TimeTravelApp {
    fn app_name() -> &'static str {
        "Time Travel Demo"
    }

    fn new(cx: &mut Context) -> Self {
        Self {
            // Use state_undoable so changes are tracked for time travel
            count: cx.state_undoable(0),
            items: cx.state_undoable(Vec::new()),
        }
    }

    fn on_build(self, cx: &mut Context) -> Self {
        // Add time travel overlay (debug builds only)
        #[cfg(debug_assertions)]
        {
            TtrvlOverlay::new(cx);

            // Add the overlay CSS
            cx.add_stylesheet(TTRVL_OVERLAY_STYLE).unwrap();
        }

        let count = self.count;
        let items = self.items;

        VStack::new(cx, move |cx| {
            // Instructions
            Label::new(cx, "Time Travel Demo")
                .font_size(24.0)
                .font_weight(FontWeightKeyword::Bold);

            #[cfg(debug_assertions)]
            {
                Label::new(cx, "**Cmd+`** overlay | **Cmd+[** back | **+Shift** fwd");
            }

            // Counter section
            HStack::new(cx, |cx| {
                Button::new(cx, |cx| Label::new(cx, "-"))
                    .on_press(move |cx| {
                        cx.with_undo("Decrement", |cx| {
                            count.upd(cx, |n| *n -= 1);
                        });
                    })
                    .width(Pixels(40.0));

                Label::new(cx, count)
                    .width(Pixels(60.0))
                    .text_align(TextAlign::Center);

                Button::new(cx, |cx| Label::new(cx, "+"))
                    .on_press(move |cx| {
                        cx.with_undo("Increment", |cx| {
                            count.upd(cx, |n| *n += 1);
                        });
                    })
                    .width(Pixels(40.0));
            })
            .alignment(Alignment::Center)
            .gap(Pixels(10.0))
            .height(Auto);

            // Items section
            VStack::new(cx, move |cx| {
                Label::new(cx, "Items:")
                    .font_weight(FontWeightKeyword::Bold);

                Button::new(cx, |cx| Label::new(cx, "Add Item"))
                    .on_press(move |cx| {
                        let item_num = items.get(cx).len() + 1;
                        cx.with_undo("Add Item", |cx| {
                            items.upd(cx, |v| v.push(format!("Item {}", item_num)));
                        });
                    });

                // Derive item count signal
                let item_count = items.drv(cx, |v, _| v.len());

                // List of items using Binding
                Binding::new(cx, item_count, move |cx| {
                    let len = *item_count.get(cx);
                    for i in 0..len {
                        let item_text = items.drv(cx, move |v, _| {
                            v.get(i).cloned().unwrap_or_default()
                        });
                        HStack::new(cx, move |cx| {
                            Label::new(cx, item_text);
                            Button::new(cx, |cx| Label::new(cx, "X"))
                                .on_press(move |cx| {
                                    cx.with_undo("Remove Item", |cx| {
                                        items.upd(cx, |v| {
                                            if i < v.len() {
                                                v.remove(i);
                                            }
                                        });
                                    });
                                })
                                .width(Pixels(24.0))
                                .height(Pixels(24.0));
                        })
                        .gap(Pixels(8.0))
                        .alignment(Alignment::Center)
                        .height(Auto);
                    }
                });
            })
            .gap(Pixels(8.0))
            .padding(Pixels(10.0));

            // Undo/Redo buttons
            HStack::new(cx, |cx| {
                // Create reactive signals for can_undo/can_redo
                let can_undo = cx.can_undo_signal();
                let can_redo = cx.can_redo_signal();

                let can_undo_val = can_undo.drv(cx, |v, _| !*v);
                let can_redo_val = can_redo.drv(cx, |v, _| !*v);

                Button::new(cx, |cx| Label::new(cx, "Undo"))
                    .on_press(|cx| { cx.undo(); })
                    .disabled(can_undo_val);

                Button::new(cx, |cx| Label::new(cx, "Redo"))
                    .on_press(|cx| { cx.redo(); })
                    .disabled(can_redo_val);
            })
            .gap(Pixels(10.0))
            .alignment(Alignment::Center)
            .height(Auto);

            // Time travel info (debug only)
            #[cfg(debug_assertions)]
            {
                // Show history
                let version_id = cx.data.get_store_mut().get_or_init_undo_version_signal();
                let history_info = cx.derived(move |store| {
                    store.track(&version_id);
                    let undo_mgr = store.undo_manager();
                    let pos = undo_mgr.ttrvl_position();
                    let len = undo_mgr.timeline_len();
                    let is_ttrvl = undo_mgr.is_ttrvl();
                    (pos, len, is_ttrvl)
                });

                let status_text = history_info.drv(cx, |info, _| {
                    if info.2 {
                        format!("Time Travel: {} / {}", info.0.unwrap_or(0) + 1, info.1)
                    } else {
                        format!("History: {} entries", info.1)
                    }
                });

                Label::new(cx, status_text)
                    .font_size(12.0)
                    .color(Color::gray());
            }
        })
        .padding(Pixels(20.0))
        .gap(Pixels(15.0))
        .alignment(Alignment::Center);

        self
    }

    fn window_config(&self) -> WindowConfig {
        window(|app| app.inner_size((400, 500)))
    }
}

fn main() -> Result<(), ApplicationError> {
    TimeTravelApp::run()
}
