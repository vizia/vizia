<div align="center"><img src="https://raw.githubusercontent.com/vizia/vizia/main/assets/branding/vizia-logo-01.png" width="128px" height="128px"/><h1>Vizia</h1></div>

<div align="center">
  <!-- License -->
  <a href="https://github.com/vizia/vizia/blob/main/LICENSE">
    <img src="https://img.shields.io/crates/l/vizia"
    alt="License" />
  </a>
  <!-- Docs -->
  <a href="https://docs.vizia.dev">
    <img src="https://img.shields.io/badge/docs-website-blue" 
      alt="Documentation" />
  </a>
  <!-- CI -->
  <a href="https://github.com/vizia/vizia/actions/workflows/build.ym">
    <img src="https://github.com/vizia/vizia/actions/workflows/build.yml/badge.svg"
      alt="CI status" />
  </a>
  <!-- docs (TODO) -->
  <!-- Dependency status -->
  <a href="https://deps.rs/repo/github/vizia/vizia">
    <img src="https://deps.rs/repo/github/vizia/vizia/status.svg"
      alt="Dependency status" />
  </a>
  <!-- Audit -->
  <a href="https://github.com/vizia/vizia/actions/workflows/audit.yml">
    <img src="https://github.com/vizia/vizia/actions/workflows/audit.yml/badge.svg"
      alt="Audit status" />
  </a>
  <!-- Discord -->
  <a href="https://discord.gg/aNkTPsRm2w">
    <img src="https://img.shields.io/discord/791142189005537332.svg?label=&logo=discord&logoColor=ffffff&color=7389D8&labelColor=6A7EC2" 
      alt="Discord Link" />
  </a>
</div>

<!-- [![Crates.io](https://img.shields.io/crates/v/vizia)](https://crates.io/crates/vizia) -->
<!-- [![docs.rs](https://img.shields.io/badge/docs-website-blue)](https://docs.rs/vizia/) -->

<br/>

<h4 align="center">A declarative desktop GUI framework for the <a href="https://www.rust-lang.org/">Rust</a> programming language.</h4>

<br/>

<div align="center">
  <h3>
    <a href="https://book.vizia.dev/"> Book </a>
    <span> • </span>
    <a href="https://docs.vizia.dev/"> Docs </a>
    <span> • </span>
    <a href="https://demo.vizia.dev/"> Demo </a>
    <span> • </span>
    <a href="https://discord.gg/aNkTPsRm2w"> Discord </a>
  </h3>
</div>

<br/>

# Features
- ### __Cross-platform (Windows, Linux, MacOS)__
  Build desktop applications which look and behave the same for Windows, Mac, and Linux.
- ### __Declarative__
  Write GUI code in a declarative way in pure Rust (no DSL macros).
- ### __Reactive__
  Views derive from application state. Change the state and the views which bind to it update automatically.
- ### __Flexible layout__
  Create flexible layouts which adapt to changes in size. Powered by [morphorm](https://github.com/vizia/morphorm).
- ### __Powerful styling__
  Take advantage of stylesheets with hot-reloading to fully customize the look of your application.
- ### __Animations__
  Bring your applications to life with animations.
- ### __Built-in views and themes__
  Utilize over 25 ready-made views as well as two built-in themes (light and dark) to get you started. Includes 4250+ SVG icons, provided by [Tabler Icons](https://tabler-icons.io).
- ### __Accessibility__
  Make you applications accessible to assistive technologies such as screen readers, powered by [accesskit](https://github.com/accesskit/accesskit).
- ### __Localization__
  Adapt your application to different locales, including translating text with [fluent](https://github.com/projectfluent/fluent-rs).
- ### __Optimised rendering__
  Vizia leverages the powerful and robust [skia](https://github.com/rust-skia/rust-skia) library for rendering, with further optimizations to only draw what is necessary.  
- ### __Audio plugin development__
  Vizia provides an alternative [baseview](https://github.com/RustAudio/baseview) windowing backend for audio plugin development, for example with the [nih-plug](https://github.com/robbert-vdh/nih-plug) framework.

<br />

# At a Glance
A simple counter application. Run with `cargo run --example counter`.
```rust, no_run
use vizia::prelude::*;

// Define some model data
#[derive(Lens)]
pub struct AppData {
    count: i32,
}

// Define events to mutate the data
pub enum AppEvent {
    Increment,
}

// Describe how the data is mutated in response to events
impl Model for AppData {
    fn event(&mut self, _: &mut EventContext, event: &mut Event) {
        event.map(|app_event, _| match app_event {
            AppEvent::Increment => {
                self.count += 1;
            }
        });
    }
}

fn main() {
    // Create an application
    Application::new(|cx| {

        // Build the model data into the tree
        AppData { count: 0 }.build(cx);

        // Declare views which make up the UI
        HStack::new(cx, |cx| {
          
            // Declare a button which emits an event
            Button::new(cx, |cx| Label::new(cx, "Increment"))
              .on_press(|cx| cx.emit(AppEvent::Increment));

            // Declare a label which is bound to part of the model, updating when it changes
            Label::new(cx, AppData::count).width(Pixels(50.0));
        })
        .alignment(Alignment::Center)  // Apply style and layout modifiers
        .horizontal_gap(Pixels(50.0));
    })
    .title("Counter") // Configure window properties
    .inner_size((400, 100))
    .run();
}
```
<div align="center"><img src="https://raw.githubusercontent.com/vizia/vizia/main/assets/images/counter.png" width="400px" height="130px"/></div>


# Learning
## Book
A quickstart guide for vizia is available [here](https://book.vizia.dev).
## Docs
Auto-generated code documentation can be found [here](https://docs.vizia.dev).
## Examples

A list of [examples](https://github.com/vizia/vizia/tree/main/examples) is included in the repository.

To run an example with the [winit](https://github.com/rust-windowing/winit) (default) windowing backend:
```bash
cargo run --release --example name_of_example
```

### Baseview

To run an example with the [baseview](https://github.com/RustAudio/baseview) windowing backend:

```bash
cargo run --release --example name_of_example --no-default-features --features baseview
```

# Contributing and Community
For help with vizia, or to get involved with contributing to the project, come join us on [our discord](https://discord.gg/aNkTPsRm2w).

# License and Attribution
Vizia is licensed under [MIT](https://github.com/vizia/vizia/blob/main/LICENSE).

Vizia logo designed by [Lunae Somnia](https://github.com/LunaeSomnia).
