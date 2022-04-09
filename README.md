<div align="center"><img src="./assets/branding/vizia-logo-01.png" width="128px" height="128px"/><h1>Vizia</h1></div>

<div align="center">
  <!-- License -->
  <a href="https://github.com/vizia/vizia/blob/main/LICENSE">
    <img src="https://img.shields.io/crates/l/vizia"
    alt="License" />
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

Vizia is a (in development) declarative GUI framework for the Rust programming language.

> **WARNING** - Vizia is currently experimental and *not* considered stable.

```rust
#[derive(Lens)]
pub struct AppData {
    count: i32,
}

pub enum AppEvent {
    Increment,
}

impl Model for AppData {
    fn event(&mut self, _: &mut Context, event: &mut Event) {
        if let Some(app_event) = event.message.downcast() {
            match app_event {
                AppEvent::Increment => {
                    self.count += 1;
                }
            }
        }
    }
}

Application::new(WindowDescription::new(), |cx| {
    AppData { count: 0 }.build(cx);

    HStack::new(cx, |cx| {
        Button::new(cx, |cx| cx.emit(AppEvent::Increment), |cx| Label::new(cx, "Increment"));
        Label::new(cx, AppData::count).width(Pixels(50.0));
    })
    .child_space(Stretch(1.0))
    .col_between(Pixels(50.0));
}).run();
```

## Features
 - Multiplatform (Windows, Linux, MacOS, Web)
 - Declarative API
 - Reactive with an elm-like archtecture
 - Flexible layout engine, powered by [morphorm](https://github.com/vizia/morphorm)
 - CSS-like styling with hot reloading
 - Style property animations

## Learning Vizia
<!-- Add docs to this when ready -->
<div align="center">
  <h3>
    <a href="https://book.vizia.dev/"> Getting Started </a>
    <span> | </span>
    <a href="https://book.vizia.dev/"> The Book </a>
    <span> | </span>
    <a href="https://demo.vizia.dev/"> Web Demo </a>
  </h3>
</div>


## Example Projects

Example projects go here... with pictures and stuff...

Vizia is also being used for the [Meadowlark project](https://github.com/MeadowlarkDAW/Meadowlark). An attempt to build a modern Digital Audio Workstation (DAW) with Rust.

## Architecture

## Roadmap

## License and Attribution
Vizia is licensed under [MIT](https://github.com/vizia/vizia/blob/main/LICENSE).

Fonts used in Vizia:
- Roboto licensed under [Apache license](http://www.apache.org/licenses/LICENSE-2.0).
- Entypo licensed under CC BY-SA 4.0.
- Material Icons licensed under [Apache license](https://github.com/google/material-design-icons/blob/master/LICENSE).

Vizia logo designed by [Lunae Somnia](https://github.com/LunaeSomnia).
