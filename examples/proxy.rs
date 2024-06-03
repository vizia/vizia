#[allow(unused)]
use vizia::prelude::*;

#[cfg(feature = "baseview")]
fn main() {
    panic!("This example is not supported on baseview - proxies are winit only");
}

#[cfg(not(feature = "baseview"))]
fn main() -> Result<(), ApplicationError> {
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

    app.run()
}
