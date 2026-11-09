//! This example isn't the best because `Runtime::set_sync_effect_waker` is
//! set once per process and register the window.waker() registration would
//! work. However, there isn't an easy way to open / close multiple.
//!
//!
//! Run with:
//!
//! ```text
//! cargo run -p vizia_baseview --example sync_counter
//! ```

use std::{thread, time::Duration};

use vizia_baseview::{Application, ApplicationError};
use vizia_core::prelude::*;

fn main() -> Result<(), ApplicationError> {
    Application::new(|cx| {
        let count = SyncSignal::new(0i32);

        // Stands in for an audio/host thread writing UI state off the UI thread.
        thread::spawn(move || {
            loop {
                thread::sleep(Duration::from_millis(50));
                count.update(|c| *c += 1);
            }
        });

        HStack::new(cx, |cx| {
            Label::new(cx, "Off-thread count:").width(Pixels(150.0));
            Label::new(cx, count).width(Pixels(60.0)).live(Live::Polite);
        })
        .alignment(Alignment::Center)
        .horizontal_gap(Pixels(20.0))
        .size(Stretch(1.0));
    })
    .title("baseview + SyncSignal")
    .with_frame_policy(vizia_baseview::FramePolicy::OnDemand)
    .inner_size((400, 100))
    .run()
}
