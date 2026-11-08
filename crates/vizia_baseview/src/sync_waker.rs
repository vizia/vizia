use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, LazyLock};

use baseview::WindowWaker;
use parking_lot::RwLock;

#[derive(Clone)]
struct RegisteredWindow {
    id: u64,
    waker: WindowWaker,
}

static NEXT_REGISTRATION: AtomicU64 = AtomicU64::new(0);
static REGISTERED_WINDOWS: LazyLock<RwLock<Arc<[RegisteredWindow]>>> =
    LazyLock::new(|| RwLock::new(Arc::from([])));

/// Keeps this window in the sync-effect wake registry for exactly as long as
/// it is alive. Store it in `ViziaWindow`; dropping the window deregisters it.
#[must_use]
pub(crate) struct SyncWakerGuard {
    id: u64,
}

impl Drop for SyncWakerGuard {
    fn drop(&mut self) {
        let mut windows = REGISTERED_WINDOWS.write();
        let next: Vec<_> = windows.iter().filter(|win| win.id != self.id).cloned().collect();
        *windows = Arc::from(next);
    }
}

pub(crate) fn register_window(waker: WindowWaker) -> SyncWakerGuard {
    let id = NEXT_REGISTRATION.fetch_add(1, Ordering::Relaxed);
    let mut windows = REGISTERED_WINDOWS.write();
    let mut next: Vec<_> = (**windows).to_vec();
    next.push(RegisteredWindow { id, waker });
    *windows = Arc::from(next);
    SyncWakerGuard { id }
}

/// Wake all the registered window when there is a SyncSignal set off the UI thread.
pub(crate) fn wake() {
    let windows = REGISTERED_WINDOWS.read().clone();
    for RegisteredWindow { waker, .. } in windows.iter() {
        waker.request_redraw();
    }
}
