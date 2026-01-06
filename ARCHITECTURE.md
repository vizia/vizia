# Architecture

This document explains the architecture and structure of the vizia codebase.

## Crates

Vizia is split into a number of internal sub-crates for specific purposes:
- `vizia_baseview` - Windowing backend utilising [Baseview], used primarily for audio plugins as it allows for parented windows.
- `vizia_core` - The main crate where most of the user-facing types and traits live.
- `vizia_derive` - Derive macros such as `Data` and other helpers.
- `vizia_id` - A utility crate for providing generational IDs.
- `vizia_input` - Types which are specific to user input such as mouse state, keyboard modifiers, and keymaps.
- `vizia_storage` - Storage types used by core. This includes a sparse set and a tree, as well as various iterators for tree traversal.
- `vizia_style` - Style property types as well as style parsing and matching.
- `vizia_window` - Types specific to a window such as the window description and cursor icon.
- `vizia_winit` - Windowing backend utilising [Winit], which is the default windowing backend.

**External Crates**
- `skia` - 2D drawing crate.
- `morphorm` - Provides adaptive layout for a tree of nodes.
- `fluent` - Provides localization including text translation substitution.
- `accesskit` - Provides integration with platform accessibility APIs for use with assistive technologies such as screen readers.
- `winit` - Provides window management.
- `baseview` - An alternative crate for window management.
- `glutin` - Provides OpenGL context management for the winit backend.

## Overview

At the core of Vizia is a simple ECS-like model. Views are assigned an entity id, which is used to get/set view properties (components), and a series of systems update these properties and draw the views to the window.

State management uses **Signals** - reactive primitives that automatically track dependencies and update views when their values change.

## Glossary

### Application

The `Application` struct is the entry point of a Vizia application. The `Application::new()` method creates a `Context`, which is a global store for all retained application state:

```rust
Application::new(|cx| {
    let count = cx.state(0i32);  // Create a signal
    Label::new(cx, count);       // Bind signal to view
}).run();
```

For structured applications, use the `App` trait:

```rust
struct MyApp {
    count: Signal<i32>,
}

impl App for MyApp {
    fn new(cx: &mut Context) -> Self {
        Self { count: cx.state(0) }
    }

    fn on_build(self, cx: &mut Context) -> Self {
        Label::new(cx, self.count);
        self
    }
}

fn main() {
    MyApp::run();
}
```

When `run()` is called, a window is created with an OpenGL context and a Skia `Canvas`, then the event loop starts.

### Context

The `Context` is where all retained application state lives. This includes:
- The `Store` (signal values and dependency tracking)
- Views and their properties
- Style data
- Mouse/keyboard state
- Event queue
- Timers

### Store

The `Store` manages all signal state. It contains:
- Signal values (keyed by `NodeId`)
- Dependency graph (which signals depend on which)
- Derived signal compute functions
- Observer callbacks for bindings

### Signal

A `Signal<T>` is a reactive primitive that holds a value and tracks its dependents:

```rust
// Create state
let count = cx.state(0i32);

// Read value (in a reactive context)
let value = count.get(store);

// Safe read (returns Option)
let value = count.try_get(store);

// Update value
count.set(cx, 42);
count.update(cx, |v| *v += 1);

// Derive new signal
let doubled = count.drv(cx, |v, _| v * 2);
```

When a signal's value changes, all dependent bindings and derived signals automatically update.

### Derived Signals

Created via `signal.drv()` or `cx.derived()`, these compute values from other signals:

```rust
let count = cx.state(5i32);
let doubled = count.drv(cx, |v, _| v * 2);
let label = count.drv(cx, |v, _| format!("Count: {v}"));

// Multi-signal derivation
let sum = cx.derived(move |s| {
    *a.get(s) + *b.get(s)
});
```

Derived signals recompute automatically when their dependencies change.

### Async Signals

Comprehensive async data loading with signals:

```rust
// Create async state
let users: Signal<Async<Vec<User>, String>> = cx.async_state();

// Basic load (with deduplication)
cx.load_async(users, || fetch_users());

// With cancellation
let handle = cx.load_async_cancelable(users, || fetch_users());
handle.cancel(); // Cancel if needed

// Refresh (shows stale data while loading)
cx.refresh_async(users, || fetch_users());

// With options (timeout, retry)
cx.load_async_with(users, AsyncOptions::patient(), || fetch_users());
```

**States:** `Idle`, `Loading`, `Ready(T)`, `Reloading(T)`, `Error(E)`, `Stale(T, E)`

**Features:**
- **Deduplication** - Multiple calls while loading are ignored
- **Stale-while-revalidate** - Show old data while refreshing
- **Cancellation** - Cancel in-flight requests
- **Timeout** - Auto-fail after duration
- **Retry** - Exponential backoff retries

**Option Presets:**
- `AsyncOptions::quick()` - 5s timeout, no retry
- `AsyncOptions::patient()` - 30s timeout, 3 retries
- `AsyncOptions::resilient()` - 60s timeout, 5 retries

### View

The `View` trait describes a visual element. It has 4 methods:
- `build()` - Builds the view and sub-views into the context. Called in constructors.
- `element()` - Returns an element name for CSS styling.
- `event()` - Handles events sent to this view.
- `draw()` - Custom drawing logic. If not implemented, draws based on style properties.

Built-in views include `Label`, `Button`, `Slider`, `Textbox`, and many more.

### Binding

A `Binding` creates a reactive scope that rebuilds when a signal changes:

```rust
Binding::new(cx, some_signal, |cx| {
    // This closure re-runs when some_signal changes
    let value = *some_signal.get(cx);
    if value > 10 {
        Label::new(cx, "High");
    }
});
```

Views that accept signals internally create bindings for reactive updates.

### Entity

Each view is assigned a generational `Entity` ID when created. This ID is used to:
- Store/retrieve view properties
- Navigate the view tree
- Associate signals with their owning views

### Tree

The `Tree` describes the hierarchy of views. It provides iterators for traversal used by systems to update the hierarchy.

### Style

View properties are stored in separate sparse-set stores in the `Style` struct, not in the tree itself. The entity ID is used to get/set style properties.

### Handle

A `Handle` wraps an `Entity` and a mutable `Context` reference. Returned when building views:

```rust
let handle = Label::new(cx, "Hello");
handle.background_color(Color::red());
```

### Modifiers

Modifiers are traits on `Handle` that provide methods for setting view properties:

```rust
Label::new(cx, "Hello")
    .background_color(Color::red())  // StyleModifiers
    .on_press(|_| println!("clicked"))  // ActionModifiers
    .width(Pixels(100.0));  // LayoutModifiers
```

Modifiers accept either values or signals for reactive properties:

```rust
let color = cx.state(Color::red());
Label::new(cx, "Hello").background_color(color);  // Updates when signal changes
```

### Model

The `Model` trait describes data that responds to events. It complements signals for event-driven updates:

```rust
struct AppData {
    count: Signal<i32>,
}

impl Model for AppData {
    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|app_event, _| match app_event {
            AppEvent::Increment => self.count.update(cx, |v| *v += 1),
        });
    }
}
```

Models are optional - for simple apps, direct signal updates suffice.

## Systems

Systems update application state each cycle:
- **Event Manager** - Routes events to Models and Views, calls `event()` methods.
- **Style System** - Links entities to shared style data and applies inheritance.
- **Image System** - Loads/unloads images as needed.
- **Animation System** - Applies animations to style properties.
- **Layout System** - Computes size and position of views.
- **Accessibility System** - Updates the accessibility tree.
- **Draw System** - Draws views to the window, calls `draw()` methods.

### Cache

The `Cache` contains computed data from systems. For example, `Style` contains desired size/position, but after layout the `Cache` contains the computed bounds used for drawing.

## Events

An `Event` contains a type-erased message plus metadata describing origin, target, and propagation behavior.

During each event loop cycle:
1. Window events are translated to `WindowEvent` and queued
2. On `MainEventsCleared`, vizia processes the queue
3. Events route through the tree to Views and Models
4. Signal changes trigger binding updates
5. Systems run (style, layout, draw)

## Data Flow

```
User Input → Events → Models/Views → Signal Updates → Bindings → View Rebuilds → Draw
                         ↓
                    Derived Signals
                         ↓
                    More Bindings
```

Signals provide fine-grained reactivity: only views bound to changed signals update, not the entire tree.
