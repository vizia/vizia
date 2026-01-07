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
    <a href="https://discord.gg/aNkTPsRm2w"> Discord </a>
  </h3>
</div>

<br/>

# Features

- ### __Cross-platform (Windows, Linux, MacOS)__

  Build desktop applications which look and behave the same for Windows, Mac, and Linux.

- ### __Declarative__

  Write GUI code in a declarative way in pure Rust (no DSL macros).

- ### __Reactive Signals__

  Fine-grained reactivity with `Signal<T>`. Create state with `cx.state()`, derive computed values with `cx.derived()`, and watch views update automatically when signals change.

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

## Simple Counter

A minimal counter showing signal creation and direct updates.

Run with [`cargo run --example readme_counter`](examples/readme/counter.rs)

```rust
use vizia::prelude::*;

struct Counter {
    count: Signal<i32>,
}

impl App for Counter {
    fn app_name() -> &'static str { "Counter" }

    fn new(cx: &mut Context) -> Self {
        Self { count: cx.state(0) }
    }

    fn on_build(self, cx: &mut Context) -> Self {
        HStack::new(cx, move |cx| {
            Button::new(cx, |cx| Label::static_text(cx, "-"))
                .on_press(move |cx| self.count.upd(cx, |n| *n -= 1));

            Label::new(cx, self.count);

            Button::new(cx, |cx| Label::static_text(cx, "+"))
                .on_press(move |cx| self.count.upd(cx, |n| *n += 1));
        });
        self
    }

    fn window_config(&self) -> WindowConfig {
        window(|app| app.inner_size((200, 100)))
    }
}

fn main() -> Result<(), ApplicationError> {
    Counter::run()
}
```

## Derived State

Computed values that automatically update when dependencies change.

Run with [`cargo run --example readme_derived_state`](examples/readme/derived_state.rs)

```rust
use vizia::prelude::*;

struct DerivedDemo {
    number: Signal<i32>,
}

impl App for DerivedDemo {
    fn app_name() -> &'static str { "Derived State" }

    fn new(cx: &mut Context) -> Self {
        Self { number: cx.state(5) }
    }

    fn on_build(self, cx: &mut Context) -> Self {
        // Derived signals recompute automatically
        let squared = self.number.drv(cx, |v, _| v * v);
        let is_even = self.number.drv(cx, |v, _| v % 2 == 0);
        let parity = is_even.drv(cx, |v, _| if *v { "even" } else { "odd" });

        VStack::new(cx, move |cx| {
            Label::new(cx, self.number);
            Label::new(cx, squared);
            Label::new(cx, parity);

            HStack::new(cx, move |cx| {
                Button::new(cx, |cx| Label::static_text(cx, "-"))
                    .on_press(move |cx| self.number.upd(cx, |n| *n -= 1));
                Button::new(cx, |cx| Label::static_text(cx, "+"))
                    .on_press(move |cx| self.number.upd(cx, |n| *n += 1));
            });
        });
        self
    }

    fn window_config(&self) -> WindowConfig {
        window(|app| app.inner_size((300, 200)))
    }
}

fn main() -> Result<(), ApplicationError> {
    DerivedDemo::run()
}
```

## Signal Synchronization

Keep multiple signals in sync with cross-updates.

Run with [`cargo run --example readme_signal_sync`](examples/readme/signal_sync.rs).

```rust
use vizia::prelude::*;

fn celsius_to_fahrenheit(c: f32) -> f32 { c * 9.0 / 5.0 + 32.0 }
fn fahrenheit_to_celsius(f: f32) -> f32 { (f - 32.0) * 5.0 / 9.0 }

struct TempConverter {
    celsius: Signal<f32>,
    fahrenheit: Signal<f32>,
}

impl App for TempConverter {
    fn app_name() -> &'static str { "Temperature Converter" }

    fn new(cx: &mut Context) -> Self {
        Self {
            celsius: cx.state(20.0),
            fahrenheit: cx.state(celsius_to_fahrenheit(20.0)),
        }
    }

    fn on_build(self, cx: &mut Context) -> Self {
        HStack::new(cx, |cx| {
            Textbox::new(cx, self.celsius)
                .on_submit(move |cx, val, _| {
                    self.fahrenheit.set(cx, celsius_to_fahrenheit(val));
                });
            Label::static_text(cx, "C");

            Textbox::new(cx, self.fahrenheit)
                .on_submit(move |cx, val, _| {
                    self.celsius.set(cx, fahrenheit_to_celsius(val));
                });
            Label::static_text(cx, "F");
        });
        self
    }

    fn window_config(&self) -> WindowConfig {
        window(|app| app.inner_size((400, 100)))
    }
}

fn main() -> Result<(), ApplicationError> {
    TempConverter::run()
}
```

## App Trait with Derived State

Encapsulate state and UI with derived computations.

Run with [`cargo run --example readme_app_trait`](examples/readme/app_trait.rs)

```rust
use vizia::prelude::*;

struct CounterWithDouble {
    count: Signal<i32>,
}

impl App for CounterWithDouble {
    fn app_name() -> &'static str { "App Trait Example" }

    fn new(cx: &mut Context) -> Self {
        Self { count: cx.state(0) }
    }

    fn on_build(self, cx: &mut Context) -> Self {
        let doubled = self.count.drv(cx, |v, _| v * 2);

        VStack::new(cx, move |cx| {
            Label::new(cx, self.count);
            Label::new(cx, doubled);

            HStack::new(cx, move |cx| {
                Button::new(cx, |cx| Label::static_text(cx, "Decrement"))
                    .on_press(move |cx| self.count.upd(cx, |n| *n -= 1));
                Button::new(cx, |cx| Label::static_text(cx, "Increment"))
                    .on_press(move |cx| self.count.upd(cx, |n| *n += 1));
            });
        });
        self
    }

    fn window_config(&self) -> WindowConfig {
        window(|app| app.inner_size((300, 150)))
    }
}

fn main() -> Result<(), ApplicationError> {
    CounterWithDouble::run()
}
```

## Signal API Quick Reference

| Operation | Code |
|-----------|------|
| Create state | `let sig = cx.state(initial_value)` |
| Read value | `sig.get(store)` (in derived) |
| Safe read | `sig.try_get(store)` (returns `Option`) |
| Set value | `sig.set(cx, new_value)` |
| Update value | `sig.upd(cx, \|v\| *v += 1)` |
| Derived state | `sig.drv(cx, \|v, _\| v * 2)` |
| Bind to view | `Label::new(cx, sig)` |
| Window props | `app.title(cx.state("Title")).inner_size(cx.state((w, h)))` |
| Async state | `let data = cx.async_state::<T, E>()` |
| Load async | `cx.load_async(data, \|\| fetch_data())` |
| Cancelable load | `let h = cx.load_async_cancelable(data, \|\| ...); h.cancel()` |
| Refresh (stale) | `cx.refresh_async(data, \|\| fetch_data())` |
| With options | `cx.load_async_with(data, AsyncOptions::patient(), \|\| ...)` |
| Undoable state | `let sig = cx.state_undoable(initial_value)` |
| Undo group | `cx.with_undo("desc", \|cx\| { sig.upd(cx, ...) })` |
| Undo/Redo | `cx.undo()` / `cx.redo()` |
| Reactive state | `cx.can_undo_signal()` / `cx.can_redo_signal()` |
| Persistent state | `let sig = cx.state_persistent("key", default)` |

<br />

# Learning

## Book

A quickstart guide for vizia is available [here](https://book.vizia.dev).

## Docs

Auto-generated code documentation can be found [here](https://docs.vizia.dev).

## Examples

A list of [examples](examples) is included in the repository.

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
