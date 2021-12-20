use vizia::*;

fn main() {
    let app =
        Application::new(WindowDescription::new().with_title("Proxy"), |cx| {}).on_idle(|cx| {
            println!("On Idle: {:?}", std::time::Instant::now());
        });

    let proxy = app.get_proxy();

    std::thread::spawn(move || loop {
        proxy.send_event(Event::new(())).expect("Failed to send proxy event");
        std::thread::sleep(std::time::Duration::from_secs(2));
    });

    app.run();
}
