# VIZIA

[![License (MIT)](https://img.shields.io/crates/l/vizia)](https://github.com/vizia/vizia/blob/main/LICENSE)
[![Build Status](https://github.com/vizia/vizia/actions/workflows/build.yml/badge.svg)](https://github.com/vizia/vizia/actions/workflows/build.yml)
[![Audit Status](https://github.com/vizia/vizia/actions/workflows/audit.yml/badge.svg)](https://github.com/vizia/vizia/actions/workflows/audit.yml)
[![Discord](https://img.shields.io/discord/791142189005537332.svg?label=&logo=discord&logoColor=ffffff&color=7389D8&labelColor=6A7EC2)](https://discord.gg/aNkTPsRm2w)
<!-- [![Crates.io](https://img.shields.io/crates/v/vizia)](https://crates.io/crates/vizia) -->
<!-- [![docs.rs](https://img.shields.io/badge/docs-website-blue)](https://docs.rs/vizia/) -->



VIZIA is a (in development) declarative GUI framework for the Rust programming language.

**WARNING** - VIZIA is currently experimental and *not* a fully functioning framework. The code in this repositiory is not considered stable and is likely to change.

## Views
Views form the basic building blocks of a GUI. A view could describe a widget like a label or button, or a more complex container such as a list.

### Composing Views
Views can be easily composed to form a more complex GUI application:
```rust
Application::new(|cx|{
	HStack::new(cx, |cx|{
		Label::new(cx, "Hello");
		Label::new(cx, "World");
	});
}).run();
```

### Layout and Styling
Inline properties used for layout and styling can be set directly on views:
```rust
Application::new(|cx|{
	HStack::new(cx, |cx|{
		Label::new(cx, "Hello")
			.width(Pixels(200.0));
		Label::new(cx, "World")
			.background_color(Color::blue());
	}).col_between(Pixels(10.0));
}).run();
```
Styling can also be done with CSS stylesheets:
```css
/* style.css */
label {
	width: 100px;
	height: 30px;
	border-width: 1px;
	border-color: black;
}

.foo {
	background-color: red;
}
```
```rust
Application::new(window_description, |cx|{

	cx.add_style("style.css");

	HStack::new(cx, |cx|{
		Label::new(cx, "Hello");
		Label::new(cx, "World").class("foo");
	});
}).run();
```
## State
State describes the data provided by the user which the GUI will modify. VIZIA is reactive, which means that changes to the state will update the views.
 
### Defining State
State is defined as a struct/enum which implements the `Model` trait:
```rust
#[derive(Lens)]
struct AppData {
	count: i32,
}

impl Model for AppData {}

```
The `Lens` derive macro enables binding views to fields of some state.

### Binding to State
To use the state we need to `build` it into the view tree and then `bind` to it:
```rust
Application::new(window_description, |cx|{
	
	// Build the state into the tree
	AppData{
		count: 0,
	}.build(cx);

	HStack::new(cx, |cx|{
		// Bind to the state
		Binding::new(cx, AppData::count, |cx, some_data|{
			Label::new(cx, *count.get(cx));
		});
	});
}).run();
```
Anything within the body of the binding view will be updated when the bound data changes.

### Mutating State
To keep the separation of data changes and updates to the view tree, mutations of the data are done through events:
```rust
pub enum AppEvent {
	Increment,
	Decrement,
}

impl Model for AppData {
	fn event(&mut self, cx: &mut Context, event: &mut MyEvent) {
		event.map(|app_event, _| match app_event {
				AppEvent::Increment => self.count += 1,	
				AppEvent::Decrement => self.count -= 1,
		})
	}
}
```

### Events and Callbacks
Some views have a built-in action or can be modified to add an action. Actions are callbacks which can be used to send events:
```rust
Application::new(|cx|{
	
	AppData{
		count: 0,
	}.build(cx);

	HStack::new(cx, |cx|{
		// Buttons take an action callback and a label
		Button::new(cx, |cx| cx.emit(AppEvent::Increment), |cx|{
			Label::new(cx, "Increment")
		});

		// The action will be called when the button is pressed
		Button::new(cx, |cx| cx.emit(AppEvent::Decrement), |cx|{
			Label::new(cx, "Decrement")
		});

		Binding::new(cx, AppData::count, |cx, some_data|{
			Label::new(cx, &count.get(cx).to_string());
		});
	});
}).run();
```
