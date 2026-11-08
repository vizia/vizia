/// Demonstrates a system tray icon managed by the application.
///
/// The tray icon is configured with the `with_tray_icon` application modifier, which takes a
/// closure returning a `tray_icon::TrayIconBuilder`. Tray icon and tray menu events are
/// forwarded into the application and propagated through the whole tree, so any view or model
/// can react to them.
///
/// The window can be hidden to the tray with the button, by closing the window, or from the
/// tray menu, and shown again from the tray menu (or by left-clicking the tray icon on
/// platforms which report click events). The application only exits via the `Quit` menu option.
///
/// Run with: `cargo run --example tray_icon --features tray`
pub use vizia::prelude::*;

#[cfg(feature = "baseview")]
fn main() {
    panic!("This example is not supported on baseview");
}

#[cfg(all(feature = "winit", not(feature = "baseview")))]
struct AppData {
    quitting: bool,
}

#[cfg(all(feature = "winit", not(feature = "baseview")))]
impl Model for AppData {
    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        // Tray menu events, identified by the ids given to the menu items.
        event.map(|menu_event: &tray_icon::menu::MenuEvent, _| match menu_event.id().0.as_str() {
            "show" => cx.emit(WindowEvent::SetVisible(true)),
            "hide" => cx.emit(WindowEvent::SetVisible(false)),
            "quit" => {
                self.quitting = true;
                cx.emit(WindowEvent::WindowClose);
            }
            _ => {}
        });

        // Intercept the window close and minimize to the tray instead. This model is built on
        // the window entity and models receive events before views, so consuming the event here
        // prevents the window from closing. Quitting from the tray menu lets the close through.
        event.map(|window_event: &WindowEvent, meta| {
            if matches!(window_event, WindowEvent::WindowClose) && !self.quitting {
                meta.consume();
                cx.emit(WindowEvent::SetVisible(false));
            }
        });

        // Tray icon events. Not emitted on Linux when using the default appindicator backend.
        event.map(|tray_event: &tray_icon::TrayIconEvent, _| {
            if let tray_icon::TrayIconEvent::Click {
                button: tray_icon::MouseButton::Left,
                button_state: tray_icon::MouseButtonState::Up,
                ..
            } = tray_event
            {
                cx.emit(WindowEvent::SetVisible(true));
            }
        });
    }
}

/// Decodes the vizia logo into RGBA pixels, returning the pixels and the image dimensions.
#[cfg(all(feature = "winit", not(feature = "baseview")))]
fn vizia_logo_rgba() -> (Vec<u8>, u32, u32) {
    use vizia::vg;

    let data = vg::Data::new_copy(include_bytes!("../assets/branding/vizia-logo-01.png"));
    let image = vg::Image::from_encoded(data).expect("Failed to decode the vizia logo");
    let (width, height) = (image.width(), image.height());

    let info =
        vg::ImageInfo::new((width, height), vg::ColorType::RGBA8888, vg::AlphaType::Unpremul, None);
    let mut rgba = vec![0u8; (width * height * 4) as usize];
    let row_bytes = (width * 4) as usize;
    assert!(
        image.read_pixels(&info, &mut rgba, row_bytes, (0, 0), vg::image::CachingHint::Allow),
        "Failed to read the vizia logo pixels"
    );

    (rgba, width as u32, height as u32)
}

#[cfg(all(feature = "winit", not(feature = "baseview")))]
fn main() -> Result<(), ApplicationError> {
    // The vizia logo, used as both the window icon and the tray icon.
    let (logo_rgba, logo_width, logo_height) = vizia_logo_rgba();
    let window_logo = logo_rgba.clone();

    Application::new(move |cx| {
        Window::new(cx, |cx| {
            AppData { quitting: false }.build(cx);

            VStack::new(cx, |cx| {
                Label::new(cx, "This application has a tray icon.");
                Label::new(cx, "Closing or hiding the window minimizes it to the tray.");
                Label::new(cx, "Use the tray menu to show the window again or quit.");
                Button::new(cx, |cx| Label::new(cx, "Hide to tray"))
                    .on_press(|cx| cx.emit(WindowEvent::SetVisible(false)));
            })
            .padding(Pixels(20.0))
            .vertical_gap(Pixels(8.0));
        })
        .title("Tray icon")
        .inner_size((420, 180))
        .icon(logo_width, logo_height, window_logo);
    })
    .with_tray_icon(move || {
        use tray_icon::menu::{IsMenuItem, Menu, MenuItem, PredefinedMenuItem};

        let show = MenuItem::with_id("show", "Show", true, None);
        let hide = MenuItem::with_id("hide", "Hide", true, None);
        let separator = PredefinedMenuItem::separator();
        let quit = MenuItem::with_id("quit", "Quit", true, None);

        let menu = Menu::new();
        menu.append_items(&[&show as &dyn IsMenuItem, &hide, &separator, &quit])
            .expect("Failed to build tray menu");

        let icon = tray_icon::Icon::from_rgba(logo_rgba, logo_width, logo_height)
            .expect("Failed to create tray icon");

        tray_icon::TrayIconBuilder::new()
            .with_menu(Box::new(menu))
            .with_tooltip("Vizia tray icon example")
            .with_icon(icon)
    })
    .run()
}
