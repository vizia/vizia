#[allow(unused)]
use vizia::prelude::*;

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
    let app = Application::new(|_| {})
        .on_idle(|_| {
            println!("On Idle: {:?}", Instant::now());
        })
        .title("Proxy");

    let mut proxy = app.get_proxy();

    std::thread::spawn(move || loop {
        proxy.emit(()).expect("Failed to send proxy event");
        std::thread::sleep(std::time::Duration::from_secs(2));
    });

    app.run();
}
