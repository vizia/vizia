mod helpers;
use helpers::*;
use vizia::prelude::*;

#[derive(Lens)]
pub struct AppData {}

impl Model for AppData {}

fn main() -> Result<(), ApplicationError> {
    Application::new(|cx| {
        let list: Vec<u32> = (10..14u32).collect();
        AppData {}.build(cx);

        //         Markdown::new(
        //             cx,
        //             r#"
        // # This is a heading

        // ## This is a subheading

        // This is **some strong text**. ~~And some more text~~.

        // - List A
        //   - List A1
        //     - Item
        //   - List A2
        //     - Item with *emphasis*

        // ```
        // fn some_code() {

        // }
        // ```

        // A [link](https://github.com/vizia/vizia)
        // "#,
        //         );
        ScrollView::new(cx, 0.0, 0.0, false, true, |cx| {
            Markdown::new(
                cx,
                r#"
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
  Take advantage of CSS with hot-reloading to fully customize the style of your application.
- ### __Animations__
  Bring your application to life with animatable style properties.
- ### __Built-in views and themes__
  Utilize over 25 ready-made views as well as two built-in themes (light and dark) to get you started. Includes 4250+ icons, provided by [Tabler Icons](https://tabler-icons.io).
- ### __Accessibility__
  Make you applications accessible to assistive technologies such as screen readers, powered by [accesskit](https://github.com/accesskit/accesskit).
- ### __Localization__
  Adapt your application to different locales, including translating text with [fluent](https://github.com/projectfluent/fluent-rs).
- ### __GPU accelerated rendering__
  Vizia leverages the GPU for fast graphical updates, powered by [femtovg](https://github.com/femtovg/femtovg).
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
        .child_space(Stretch(1.0))  // Apply style and layout modifiers
        .col_between(Pixels(50.0));
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
"#,
            );
        });
    })
    .title("Markdown")
    .run()
}
