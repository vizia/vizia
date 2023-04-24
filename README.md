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
    <a href="https://docs.vizia.dev/"> Docs </a>
    <span>  • </span>
    <!-- <a href="https://book.vizia.dev/"> Guide </a>
    <span> | </span> -->
    <a href="https://demo.vizia.dev/"> Demo </a>
    <span>  • </span>
    <a href="https://discord.gg/aNkTPsRm2w"> Discord </a>
  </h3>
</div>

<br/>

# Features
 - Multiplatform (Windows, Linux, MacOS, Web)
 - Declarative API
 - Fine-grained data-driven reactivity
 - Adaptive layout, powered by [morphorm](https://github.com/vizia/morphorm)
 - GPU rendering, powered by [femtovg](https://github.com/femtovg/femtovg)
 - CSS theming with hot reloading
 - Animations
 - Rich text rendering, powered by [cosmic-text](https://github.com/pop-os/cosmic-text) 
 - Accessibility and screen reader support, powered by [accesskit](https://github.com/accesskit/accesskit)
 - Localization, powered by [fluent](https://github.com/projectfluent/fluent-rs)
 - Alternative [baseview](https://github.com/RustAudio/baseview) backend for audio plugin development.

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
            Button::new(cx, |cx| cx.emit(AppEvent::Increment), |cx| Label::new(cx, "Increment"));

            // Declare a label which is bound to part of the model, updating when it changes
            Label::new(cx, AppData::count).width(Pixels(50.0));
        })
        .child_space(Stretch(1.0))  // Apply style and layout modifiers
        .col_between(Pixels(50.0));
    })
    .title("Counter") // Configure window peoperties
    .inner_size((400, 100))
    .run();
}
```
<div align="center"><img src="https://raw.githubusercontent.com/vizia/vizia/main/assets/images/counter.png" width="400px" height="130px"/></div>


# Running the Examples

A list of [examples](https://github.com/vizia/vizia/tree/main/examples) is included in the repository.

To run an example with the [winit](https://github.com/rust-windowing/winit) (default) windowing backend:
```bash
cargo run --release --example name_of_example
```
 <details>
<summary>Baseview</summary>
To run an example with the [baseview](https://github.com/RustAudio/baseview) windowing backend:
```bash
cargo run --release --example name_of_example --no-default-features --features baseview
```
</details>

<details>
<summary>Web</summary>
To run an example as a web application, first ensure that the `wasm32-unknown-unknown` toolchain is installed:
```bash
rustup target add wasm32-unknown-unknown
```
Then run an example with the following:
```bash
cargo run-wasm --release --example name_of_example
```
> **NOTE** - Some examples are not compatible with the web target and will intentionally panic if run on web.
</details>

<br />

# Contributing and Community

For help with vizia, or to get involved with contributing to the project, come join us on [our discord](https://discord.gg/aNkTPsRm2w).

# License and Attribution
Vizia is licensed under [MIT](https://github.com/vizia/vizia/blob/main/LICENSE).

Vizia logo designed by [Lunae Somnia](https://github.com/LunaeSomnia).
