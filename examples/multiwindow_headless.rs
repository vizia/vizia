/// Demonstrates `Application::headless` – no implicit primary window, starting
/// with zero windows.
///
/// The application starts with no OS windows at all. After a startup timer completes
/// it sets a signal that causes a `Binding` to create the main window. A button
/// inside that window then opens a second window the same way.
pub use vizia::prelude::*;

#[cfg(feature = "baseview")]
fn main() {
    panic!("This example is not supported on baseview");
}

#[cfg(all(feature = "winit", not(feature = "baseview")))]
#[derive(Debug)]
enum AppEvent {
    /// Fired by the startup timer – triggers window creation via signal.
    OpenMain,
    ShowSecondary,
    HideSecondary,
}

#[cfg(all(feature = "winit", not(feature = "baseview")))]
struct AppData {
    show_main: Signal<bool>,
    show_secondary: Signal<bool>,
}

#[cfg(all(feature = "winit", not(feature = "baseview")))]
impl Model for AppData {
    fn event(&mut self, _cx: &mut EventContext, event: &mut Event) {
        event.map(|e, _| match e {
            AppEvent::OpenMain => {
                if !self.show_main.get() {
                    self.show_main.set(true)
                }
            }
            AppEvent::ShowSecondary => self.show_secondary.set(true),
            AppEvent::HideSecondary => self.show_secondary.set(false),
        });
    }
}

#[cfg(all(feature = "winit", not(feature = "baseview")))]
fn main() -> Result<(), ApplicationError> {
    Application::headless(|cx| {
        // No windows created here – zero OS windows at startup.
        let show_main = Signal::new(false);
        let show_secondary = Signal::new(false);
        AppData { show_main, show_secondary }.build(cx);

        // When show_main becomes true, create the main window.
        Binding::new(cx, show_main, move |cx| {
            if show_main.get() {
                Window::new(cx, move |cx| {
                    VStack::new(cx, move |cx| {
                        Label::new(cx, "Main window – opened by a timer event");
                        Button::new(cx, |cx| Label::new(cx, "Open secondary window"))
                            .on_press(|cx| cx.emit(AppEvent::ShowSecondary));
                    })
                    .padding(Pixels(20.0))
                    .vertical_gap(Pixels(8.0));
                })
                .title("Main (spawned by timer)")
                .inner_size((340, 180));
            }
        });

        // Secondary window - created at root scope so it is a separate top-level window.
        Binding::new(cx, show_secondary, move |cx| {
            if show_secondary.get() {
                Window::new(cx, |cx| {
                    VStack::new(cx, |cx| {
                        Label::new(cx, "Secondary window");
                        Label::new(cx, "Independent top-level window.");
                    })
                    .padding(Pixels(20.0))
                    .vertical_gap(Pixels(8.0));
                })
                .on_close(|cx| cx.emit(AppEvent::HideSecondary))
                .title("Secondary")
                .inner_size((280, 120));
            }
        });

        // Start a bounded timer and create the main window when it stops.
        let timer = cx.add_timer(
            std::time::Duration::from_millis(100),
            Some(std::time::Duration::from_millis(5000)),
            |cx, action| {
                if let TimerAction::Stop = action {
                    cx.emit(AppEvent::OpenMain);
                }
            },
        );
        cx.start_timer(timer);
    })
    .run()
}
