/// The DPI scaling policy of the window.
#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub enum WindowScalePolicy {
    /// Use the system's DPI scale factor.
    #[default]
    SystemScaleFactor,
    /// Use the given DPI scale factor (e.g. `1.0` = 96 dpi).
    ScaleFactor(f64),
}

/// Controls how often the window's frame loop is driven.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum FramePolicy {
    /// Drive a frame only when there is work to do (events, wakers, animations, timers).
    #[default]
    OnDemand,
    /// Drive a frame every vsync interval.
    ///
    /// This doesn't force a redraw of the window every frame. This just ensure
    /// on_idle will be called at every sync frame. Its the vizia app that decide
    /// if rect has to be redrawn or not based on its reactive system.
    Continuous,
}
