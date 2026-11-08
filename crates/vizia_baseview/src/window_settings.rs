/// The DPI scaling policy of the window.
#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub enum WindowScalePolicy {
    /// Use the system's DPI scale factor.
    #[default]
    SystemScaleFactor,
    /// Use the system's DPI scale factor, but fall back to the given factor when
    /// the platform cannot report one (e.g. `1.0` = 96 dpi).
    ScaleFactor(f64),
}

/// Controls how often the window's frame loop is driven.
///
/// Only [`FramePolicy::Continuous`] is currently offered. `OnDemand` requires a
/// working sync-effect waker so that off-thread `SyncSignal` writes and disposals
/// can wake the event loop; without one those updates would stall indefinitely.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum FramePolicy {
    /// Drive a frame every vsync interval.
    ///
    /// This doesn't force a redraw of the window every frame. It only ensures
    /// `on_idle` is called on every sync frame; the vizia app decides whether
    /// anything has to be redrawn based on its reactive system.
    #[default]
    Continuous,
    /// Drive a frame only when there is work to do (events, wakers, animations, timers).
    OnDemand,
}
