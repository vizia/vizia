# Proposal: Migrating from Lenses to Signals for State Management

## Summary

This proposal outlines a migration from the current lens-based state management system to a signals-based approach in Vizia. Signals provide a more intuitive, efficient, and composable way to manage reactive state in GUI applications, offering better performance characteristics and a more ergonomic API.

## Current State: Lens-Based System

The current Vizia framework uses a lens-based system for state management. Lenses provide a functional programming approach to accessing and updating nested data structures. They work by creating composable "accessors" that can focus on specific parts of a data structure, allowing views to bind to particular fields without needing to know about the entire data model.

### How Lenses Work

In the lens system:

1. **Data Structure Definition**: State is defined as a struct with fields that need to be accessed by views
2. **Lens Derivation**: The `#[derive(Lens)]` macro automatically generates lens accessors for each field
3. **Data Trait**: Types must implement the `Data` trait for change detection and efficient updates
4. **Model Implementation**: A `Model` trait implementation handles state mutations through event processing
5. **View Binding**: Views bind to specific parts of the state using lens composition (e.g., `AppData::count`)

This approach provides type-safe access to nested data but requires significant boilerplate and indirection, some of which is hidden behind derive macros. Here's how it looks in practice:

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

In this example:
- `AppData` defines the state structure and derives `Lens` to generate field accessors
- `AppEvent` enum defines possible state mutations
- The `Model` implementation processes events and updates state accordingly
- Views bind to `AppData::count` using the generated lens
- State changes flow through the event system: user interaction → event emission → model update → view re-render

### Challenges with Multiple State Dependencies

One significant limitation of the lens system becomes apparent when views need to depend on multiple pieces of state. Consider a more complex example:

```rust
#[derive(Lens, Data, Clone)]
pub struct AppData {
    user_name: String,
    is_logged_in: bool,
    message_count: i32,
    theme: Theme,
}
```

If you want to create a view that displays different content based on both `is_logged_in` and `message_count`, you face several challenges:

1. **No Direct Multi-Binding**: You cannot easily bind to multiple lenses simultaneously
2. **Complex Derived State**: Creating computed values requires manual state management
3. **Verbose Workarounds**: You must either:
   - Bind to the entire `AppData` struct (losing granular updates)
   - Create complex lens compositions or nested binding views
   - Manually manage state dependencies in the view

This results in complex, hard-to-maintain code.

## Proposed State: Signal-Based System

The new signals-based approach represents a paradigm shift towards reactive programming for state management. Signals are reactive primitives that automatically track dependencies and propagate changes through the system. They provide a direct, imperative way to manage state while maintaining the benefits of reactive updates.

### How Signals Work

In the signals system:

1. **Direct State Creation**: State is created directly using `cx.state(initial_value)` without requiring struct definitions or trait derivations
2. **Automatic Change Detection**: Signals internally track when their values change, eliminating the need for manual comparison logic
3. **Dependency Tracking**: The system automatically tracks which views and derived computations depend on which signals
4. **Direct Mutation**: State can be updated directly using `signal.update(cx, |value| *value += 1)` or `signal.set(cx, new_value)`
5. **Derived State**: Computed values are created with `cx.derived(closure)` and automatically recompute when dependencies change
6. **Reactive Binding**: Views can bind directly to signals, and the framework ensures they update only when necessary

This approach eliminates boilerplate while providing fine-grained reactivity and automatic dependency management. Here's how it looks in practice:

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

pub enum CounterEvent {
    Increment,
    Decrement,
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
                .on_press(|cx| cx.emit(CounterEvent::Increment));

            Button::new(cx, |cx| Label::new(cx, "Decrement"))
                .on_press(|cx| cx.emit(CounterEvent::Decrement));

            Label::new(cx, self.count);

            // Derived state - only recomputed when the count changes
            let doubled = cx.derived(move |s| *self.count.get(s) * 2);

            Label::new(cx, doubled);
        })
        .alignment(Alignment::Center)
        .gap(Pixels(50.0));

        self
    }

    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|app_event, _| match app_event {
            CounterEvent::Increment => {
                self.count.update(cx, |count| *count += 1);
            }
            CounterEvent::Decrement => {
                self.count.update(cx, |count| *count -= 1);
            }
        });
    }
}
```

In this example:
- `Counter` stores a single `Signal<i32>` created with `cx.state(0)`
- User interactions still emit events (`CounterEvent::Increment`/`Decrement`) for consistency with the existing event system
- State updates happen in the `event` handler via `self.count.update(cx, |count| *count += 1)` - no Model trait needed
- Views bind directly to signals: `Label::new(cx, self.count)`
- Derived state (`doubled`) automatically tracks dependencies and recomputes only when `count` changes
- The reactive system works automatically: user interaction → event emission → signal update → affected views re-render

### Alternative: Direct Signal Updates (Optional)

While the example above shows signals working with the existing event system, signals also support direct updates without events. This provides flexibility in how developers choose to structure their applications:

```rust
impl View for Counter {
    fn on_build(self, cx: &mut Context) -> Self {
        HStack::new(cx, move |cx| {
            // Direct signal updates - no events needed
            Button::new(cx, |cx| Label::new(cx, "Increment"))
                .on_press(move |cx| self.count.update(cx, |v| *v += 1));

            Button::new(cx, |cx| Label::new(cx, "Decrement"))
                .on_press(move |cx| self.count.update(cx, |v| *v -= 1));

            Label::new(cx, self.count);

            // Derived state works the same way
            let doubled = cx.derived(move |s| *self.count.get(s) * 2);
            Label::new(cx, doubled);
        })
        .alignment(Alignment::Center)
        .gap(Pixels(50.0));

        self
    }

    // No event method needed for direct updates
}
```

**When to use each approach:**

- **Event-based updates**: Best for complex applications, cross-component communication, when you need centralized state logic, or when following established patterns
- **Direct updates**: Ideal for simple local state, rapid prototyping, or when the overhead of events isn't justified

Both approaches can coexist in the same application, allowing developers to choose the most appropriate pattern for each use case.

## Key Differences and Improvements

### 1. Simplified API

**Lenses (Current):**
- Requires deriving `Lens` trait
- Requires deriving `Data` trait for change detection
- Complex data access through lens composition

**Signals (Proposed):**
- Direct state creation with `cx.state(initial_value)`
- Direct updates with `signal.update(cx, |value| *value += 1)` in event handlers
- No need for lens derivation or data derivation
- Simple, direct state access and mutation
- Built-in change detection and reactivity

### 2. Derived State Support

Signals provide built-in support for derived/computed state:

```rust
// Automatically recomputes when count changes
let doubled = cx.derived(move |s| *self.count.get(s) * 2);
let is_even = cx.derived(move |s| *self.count.get(s) % 2 == 0);
```

This replaces the need for multi- binding and is much more ergonomic than manually managing derived state with lenses.

### 3. Multi-State Dependencies Made Simple

Signals excel when views need to depend on multiple pieces of state. Using the same complex example from the lens section:

```rust
struct UserDashboard {
    user_name: Signal<String>,
    is_logged_in: Signal<bool>,
    message_count: Signal<i32>,
    theme: Signal<Theme>,
}

impl UserDashboard {
    fn new(cx: &mut Context) -> Handle<'_, Self> {
        Self {
            user_name: cx.state(String::new()),
            is_logged_in: cx.state(false),
            message_count: cx.state(0),
            theme: cx.state(Theme::default()),
        }.build(cx, |cx| {})
    }
}

impl View for UserDashboard {
    fn on_build(self, cx: &mut Context) -> Self {
        // Easy multi-signal derived state
        let welcome_message = cx.derived(move |s| {
            if *self.is_logged_in.get(s) {
                format!(
                    "Welcome {}! You have {} messages", 
                    self.user_name.get(s), 
                    self.message_count.get(s)
                )
            } else {
                "Please log in".to_string()
            }
        });

        // Only recomputes when any dependency changes
        Label::new(cx, welcome_message);

        self
    }
}
```

Benefits:
- **Natural Composition**: Multiple signals combine easily in derived state
- **Automatic Dependency Tracking**: The system knows which signals the derived state depends on
- **Efficient Updates**: Only recomputes when any dependency actually changes

### 4. Application-Level State Management (Future Enhancement)

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
