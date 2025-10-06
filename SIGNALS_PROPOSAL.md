# Proposal: Migrating from Lenses to Signals for State Management

## Summary

This proposal outlines a migration from the current lens-based state management system to a signals-based approach in Vizia. Signals provide a more intuitive, efficient, and composable way to manage reactive state in GUI applications, offering better performance characteristics and a more ergonomic API.

## Current State: Lens-Based System

The current Vizia framework uses a lens-based system for state management, as demonstrated in the counter example:

```rust
#[derive(Lens)]
pub struct AppData {
    count: i32,
}

pub enum AppEvent {
    Increment,
    Decrement,
}

impl Model for AppData {
    fn event(&mut self, _cx: &mut EventContext, event: &mut Event) {
        event.map(|app_event, _| match app_event {
            AppEvent::Increment => {
                self.count += 1;
            }
            AppEvent::Decrement => {
                self.count -= 1;
            }
        });
    }
}

fn main() -> Result<(), ApplicationError> {
    Application::new(|cx| {
        AppData { count: 0 }.build(cx);

        HStack::new(cx, move |cx| {
            Button::new(cx, |cx| Label::new(cx, "Increment"))
                .on_press(|cx| cx.emit(AppEvent::Increment));

            Button::new(cx, |cx| Label::new(cx, "Decrement"))
                .on_press(|cx| cx.emit(AppEvent::Decrement));

            Label::new(cx, AppData::count);
        })
        .alignment(Alignment::Center)
        .gap(Pixels(50.0));
    })
    .title("Counter")
    .inner_size((400, 100))
    .run()
}
```

## Proposed State: Signal-Based System

The new signals-based approach would simplify state management significantly:

```rust
fn main() -> Result<(), ApplicationError> {
    Application::new(|cx| {
        Counter::new(cx);
    })
    .title("Counter")
    .inner_size((400, 100))
    .run()
}

struct Counter {
    count: Signal<i32>,
}

impl Counter {
    fn new(cx: &mut Context) -> Handle<'_, Self> {
        Self { count: cx.state(0) }.build(cx, |cx| {})
    }
}

impl View for Counter {
    fn element(&self) -> Option<&'static str> {
        Some("counter")
    }

    fn on_build(self, cx: &mut Context) -> Self {
        HStack::new(cx, move |cx| {
            Button::new(cx, |cx| Label::new(cx, "Increment"))
                .on_press(move |cx| self.count.update(cx, |v| *v += 1));

            Button::new(cx, |cx| Label::new(cx, "Decrement"))
                .on_press(move |cx| self.count.update(cx, |v| *v -= 1));

            Label::new(cx, self.count);

            // Derived state - only recomputed when the count changes
            let doubled = cx.derived(move |s| *self.count.get(s) * 2);

            Label::new(cx, doubled);
        })
        .alignment(Alignment::Center)
        .gap(Pixels(50.0));

        self
    }
}
```

## Key Differences and Improvements

### 1. Simplified API

**Lenses (Current):**
- Requires deriving `Lens` trait
- Requires deriving `Data` trait for change detection
- Complex data access through lens composition
- Requires `Model` trait implementation for state updates

**Signals (Proposed):**
- Direct state creation with `cx.state(initial_value)`
- Direct updates with `signal.update(cx, |value| *value += 1)`
- No need for lens derivation, data derivation, or model implementations
- Simple, direct state access and mutation
- Built-in change detection and reactivity

### 2. Derived State Support

Signals provide built-in support for derived/computed state:

```rust
// Automatically recomputes when count changes
let doubled = cx.derived(move |s| *self.count.get(s) * 2);
let is_even = cx.derived(move |s| *self.count.get(s) % 2 == 0);
```

This is much more ergonomic than manually managing derived state with lenses.

### 3. Application-Level State Management (Future Enhancement)

As part of the signals migration, we anticipate converting the `Application` struct to a trait, similar to the `View` trait pattern. This would enable application-level state management:

```rust
struct MyApp {
    user_settings: Signal<UserSettings>,
    app_state: Signal<AppState>,
}

impl Application for MyApp {
    fn new(cx: &mut Context) -> Self {
        Self {
            user_settings: cx.state(UserSettings::load()),
            app_state: cx.state(AppState::new()),
        }
    }

    fn on_build(self, cx: &mut Context) -> Self {
        // Build the main application UI
        VStack::new(cx, |cx| {
            // Main content that can access app-level state
            MainView::new(cx, self.app_state, self.user_settings);
        });

        self
    }
}

fn main() -> Result<(), ApplicationError> {
    MyApp::run()
        .title("My Application")
        .inner_size((800, 600))
}
```

This pattern would provide:
- **Global state management**: Application-level signals accessible throughout the widget tree
- **Consistent API**: Same pattern as views for state management
- **Better organization**: Clear separation between app-level and component-level state
- **Improved composability**: Application state can be easily passed to child components

The current closure-based application setup would be maintained initially for backwards compatibility.

## Advantages of Signals

### 1. **Reduced Boilerplate**
- No need for `#[derive(Lens)]`
- No need for `#[derive(Data)]` 
- No `Model` trait implementation required
- Direct state manipulation
- Simplified data access patterns

### 2. **Better Performance**
- Automatic dependency tracking
- Efficient derived state computation

### 3. **More Intuitive Mental Model**
- State is just a value that can change over time
- Direct mutations with clear ownership
- Clear dependency relationships through derived state

### 4. **Composability**
- Easy to combine multiple signals
- Derived state can depend on multiple sources
- Natural composition patterns

### 5. **Type Safety**
- Strong typing preserved throughout
- Clear ownership and borrowing semantics

### 6. **Local State Management**
- Views can have their own local state
- No need for global state management for simple cases
- Better encapsulation

### 7. **Application-Level State (Future)**
- Planned Application trait will enable global state management
- Consistent patterns between application and view state
- Clear separation of concerns between global and local state
- Better state organization for complex applications

## Disadvantages of Signals

### 1. **Learning Curve**
- Developers familiar with the current lens system need to learn new patterns
- Different mental model for state management

### 2. **Memory Overhead**
- Each signal has some overhead for dependency tracking
- May use more memory than simple struct fields

### 3. **Debugging Complexity**
- Reactive updates can make debugging more complex
- Dependency chains might be hard to trace

### 4. **Migration Cost**
- Existing codebases would need significant refactoring
- Breaking change requiring major version bump

### 5. **Runtime Dependency Tracking**
- Some overhead for tracking which views depend on which signals
- More complex runtime behavior

## Migration Strategy

### Phase 1: Parallel Implementation
- Implement signals alongside existing lens system
- Provide compatibility layer
- Update examples to demonstrate both approaches

### Phase 2: Gradual Migration
- Migrate built-in views to use signals
- Implement Application trait for app-level state management
- Provide migration guides and tools
- Deprecate lens-based APIs

### Phase 3: Full Transition
- Remove lens-based system in major version update
- Finalize Application trait API
- Clean up codebase
- Update all documentation

## Conclusion

The migration from lenses to signals represents a significant improvement in Vizia's state management capabilities. While there are migration costs and some learning curve, the benefits of reduced boilerplate, better performance, and more intuitive APIs make this a worthwhile evolution.

The signals approach aligns Vizia with modern reactive frameworks and provides a foundation for more sophisticated features like:
- Async state management
- Time-travel debugging
- State persistence
- Undo/redo functionality

This proposal positions Vizia for future growth while maintaining its core strengths in performance and developer experience.
