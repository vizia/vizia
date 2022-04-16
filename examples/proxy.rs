#[allow(unused)]
use vizia::*;

#[cfg(target_arch = "wasm32")]
fn main() {
    panic!("This example is not supported on wasm - threads are experimental");
}

#[cfg(feature = "baseview")]
fn main() {
    panic!("This example is not supported on baseview - proxies are winit only");
}

#[cfg(all(not(target_arch = "wasm32"), not(feature = "baseview")))]
fn main() {
    let app =
        Application::new(WindowDescription::new().with_title("Proxy"), |_| {}).on_idle(|_| {
            println!("On Idle: {:?}", instant::Instant::now());
        });

    let proxy = app.get_proxy();

    std::thread::spawn(move || loop {
        proxy.send_event(Event::new(())).expect("Failed to send proxy event");
        std::thread::sleep(std::time::Duration::from_secs(2));
    });

    app.run();
}
