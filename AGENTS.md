# Agents.md – VIZIA Signals Development Guide

> Goal: Help optimize, extend, and maintain the Signals-based reactive state architecture in VIZIA. The Lens-to-Signals migration is complete. Focus is now on API polish, new features, and quality improvements.

Reference: Architecture documented in `ARCHITECTURE.md`. API patterns in `README.md`.

---

## 1. Project State

The `signals` branch contains the complete Signal-based reactive architecture:

- **Migration Status**: Complete (100%)
- **Current Focus**: API optimization, new features, documentation
- **Branch**: `signals` (active development)

Key accomplishments:
- All views migrated to Signal-based APIs
- App trait pattern standardized across examples
- `.two_way()` modifier for bidirectional binding
- `.drv()` method for ergonomic derived signals
- `Async<T, E>` for async state management

---

## 2. Signal API Reference

### Core Patterns

```rust
// State creation
let count = cx.state(0i32);

// Reading (in reactive context)
let value = count.get(store);
let value = count.try_get(store);  // Returns Option, safe for stale signals

// Writing
count.set(cx, 42);
count.upd(cx, |v| *v += 1);

// Derived state (preferred)
let doubled = count.drv(cx, |v, _| v * 2);

// Multi-signal derivation (use store param to access other signals)
let sum = a.drv(cx, move |a_val, s| a_val + b.get(s));

// Alternative: cx.derived() when no "primary" signal
let product = cx.derived(move |s| *x.get(s) * *y.get(s));

// Async state (see section 11 for full API)
let data: Signal<Async<T, E>> = cx.async_state();
cx.load_async(data, || fetch_data());
```

### View Binding Patterns

```rust
// Direct binding - view updates when signal changes
Label::new(cx, text_signal);
Slider::new(cx, value_signal).two_way();

// Conditional rendering
Binding::new(cx, condition, |cx| {
    if *condition.get(cx) {
        Label::new(cx, "Visible");
    }
});
```

### App Trait Pattern

```rust
struct MyApp {
    count: Signal<i32>,
}

impl App for MyApp {
    fn new(cx: &mut Context) -> Self {
        Self { count: cx.state(0) }
    }

    fn on_build(self, cx: &mut Context) -> Self {
        Label::new(cx, self.count.drv(cx, |v, _| format!("Count: {v}")));
        self
    }

    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|e, _| match e {
            AppEvent::Increment => self.count.upd(cx, |v| *v += 1),
        });
    }
}
```

---

## 3. Development Priorities

### API Optimization
- Reduce boilerplate in common patterns
- Improve ergonomics for derived signals
- Ensure consistent naming and behavior across views
- Add convenience methods where they reduce repetition

### New Features
When adding features:
1. Follow existing patterns in `crates/vizia_core/src/recoil/`
2. Add to both `Context` and `EventContext` where appropriate
3. Include unit tests
4. Update all relevant documentation (any .md files documenting the codebase)
5. Create example if non-trivial

### Code Quality
- Run `cargo clippy` and fix warnings
- Run `cargo fmt` before commits
- Prefer `#[derive]` over manual impls where possible
- Remove dead code, don't suppress warnings without reason

---

## 4. Modifying Views

When updating view components:

### Adding `.two_way()` Support
```rust
pub fn two_way(self) -> Self {
    self.modify(|view| {
        let signal = view.value;
        view.on_change = Some(Box::new(move |cx, val| signal.set(cx, val)));
    })
}
```

Views with `.two_way()`: Slider, Knob, XYPad, Textbox, NumberInput, Checkbox, Switch, ToggleButton, Rating

### Signal-Based Constructor Pattern
```rust
impl MyView {
    pub fn new(cx: &mut Context, value: Signal<T>) -> Handle<Self> {
        Self { value, on_change: None }
            .build(cx, |cx| {
                // Build child views, bind to signal
            })
    }
}
```

---

## 5. Testing and Validation

Before committing:

```bash
# Check compilation
cargo build

# Run tests
cargo test

# Check for warnings
cargo clippy --all-targets

# Format code
cargo fmt

# Test specific example
cargo run --example <name>
```

For new features:
- Add unit tests in the module (`#[cfg(test)] mod tests`)
- Create or update an example demonstrating usage
- Verify example compiles and runs

---

## 6. Commit Conventions

Format: `<type>(<scope>): <description>`

Types:
- `feat` - New feature
- `fix` - Bug fix
- `refactor` - Code restructuring without behavior change
- `docs` - Documentation only
- `test` - Adding tests
- `chore` - Maintenance tasks

Examples:
```
feat(async): Add Async<T,E> for async state management
fix(slider): Correct two_way binding behavior
refactor(views): Remove redundant Clone bounds
docs(readme): Update Signal API quick reference
```

---

## 7. File Organization

```
crates/vizia_core/src/
├── recoil/
│   ├── mod.rs          # Signal, Store, NodeId, exports
│   └── async_state.rs  # Async<T,E>, AsyncSignalExt
├── context/
│   ├── mod.rs          # Context - state(), derived(), async_state(), load_async()
│   └── event.rs        # EventContext - load_async(), spawn()
├── views/              # Individual view components
├── binding/            # Binding infrastructure
└── modifiers/          # Handle modifier traits
```

When adding new signal features:
1. Implementation in `recoil/` (new file if substantial)
2. Context methods in `context/mod.rs`
3. EventContext methods in `context/event.rs` (if needed in event handlers)
4. Export from `recoil/mod.rs`
5. Prelude exports via `lib.rs`

---

## 8. Common Patterns to Follow

### Avoiding Borrow Issues in Bindings
```rust
// Clone data before mutable cx operations
Binding::new(cx, signal, move |cx| {
    let data = signal.get(cx).clone();  // Clone first
    VStack::new(cx, |cx| {              // Now cx is free
        Label::new(cx, &data.name);
    });
});
```

### Derived Signals for Display
```rust
// Prefer .drv() for transformations
let display = value.drv(cx, |v, _| format!("{:.2}", v));
Label::new(cx, display);
```

### Event-Driven Updates
```rust
fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
    event.map(|app_event, _| match app_event {
        AppEvent::Action => {
            self.signal.upd(cx, |v| /* modify */);
        }
    });
}
```

---

## 9. What NOT to Do

- Don't add `#[allow(dead_code)]` without good reason
- Don't create new files for single small additions
- Don't add features without documentation
- Don't break existing examples
- Don't add unnecessary trait bounds
- Don't use `cx.derived()` when `.drv()` suffices
- Don't manually implement traits that can be derived
- Don't assume you know what files exist - always `ls` first before updating docs or making changes

---

## 10. Quality Checklist

Before considering work complete:

- [ ] Code compiles without warnings
- [ ] Clippy passes without new warnings
- [ ] Unit tests pass
- [ ] Affected examples still work
- [ ] Documentation updated if API changed
- [ ] Commit message follows conventions
- [ ] No dead code or unused imports

---

## 11. Async State API

Comprehensive async data loading with `Signal<Async<T, E>>`:

### States

```
Idle → Loading → Ready(T) / Error(E) / Timeout
         ↓
      Retrying(attempt, max, E)  [during retry]

Ready(T) → Reloading(T) → Ready(T) / Stale(T, E)  [refresh pattern]
```

### Basic Usage

```rust
// Create async signal
let users: Signal<Async<Vec<User>, String>> = cx.async_state();

// Basic load (with deduplication)
cx.load_async(users, || fetch_users());

// With cancellation
let handle = cx.load_async_cancelable(users, || fetch_users());
handle.cancel();

// Refresh - shows stale data while loading
cx.refresh_async(users, || fetch_users());
```

### With Options

```rust
// Timeout
cx.load_async_with(users, AsyncOptions::default()
    .timeout(Duration::from_secs(30)), || fetch_users());

// Retry with exponential backoff
cx.load_async_with(users, AsyncOptions::default()
    .retry(3)
    .retry_with_delay(3, Duration::from_millis(500)), || fetch_users());

// Presets
AsyncOptions::quick()      // 5s timeout, no retry
AsyncOptions::patient()    // 30s timeout, 3 retries
AsyncOptions::resilient()  // 60s timeout, 5 retries
```

### State Queries

```rust
// In bindings
let is_loading = users.drv(cx, |s, _| s.is_loading());
let is_retrying = users.drv(cx, |s, _| s.is_retrying());
let has_error = users.drv(cx, |s, _| s.is_error());

// Direct queries
users.is_idle(cx)      // No operation started
users.is_loading(cx)   // Loading, Reloading, or Retrying
users.is_ready(cx)     // Has data (Ready, Reloading, or Stale)
users.is_error(cx)     // Error or Stale
users.is_timeout(cx)   // Operation timed out
users.is_retrying(cx)  // Retry in progress

// Data access
users.data(cx)         // Option<&T> - any available data
users.fresh_data(cx)   // Option<&T> - only if Ready
users.error(cx)        // Option<&E> - error if present
users.retry_info(cx)   // Option<(attempt, max, &E)>
```

### TTL / Cache Freshness

```rust
// Check data age
if let Some(age) = users.age(cx) {
    println!("Data is {:?} old", age);
}

// Check expiration
if users.is_expired(cx, Duration::from_secs(60)) {
    cx.refresh_async(users, || fetch_users());
}

// Check freshness
if users.is_fresh_within(cx, Duration::from_secs(30)) {
    // Data is recent, skip refresh
}
```

### UI Pattern

```rust
Binding::new(cx, users.drv(cx, |s, _| s.is_loading()), |cx| {
    if *is_loading.get(cx) { Label::new(cx, "Loading..."); }
});

Binding::new(cx, users.drv(cx, |s, _| s.is_retrying()), |cx| {
    if let Some((attempt, max, err)) = users.get(cx).retry_info() {
        Label::new(cx, format!("Retry {}/{}: {}", attempt, max, err));
    }
});

Binding::new(cx, users.drv(cx, |s, _| s.is_ready()), |cx| {
    if let Some(data) = users.data(cx) {
        for user in data { Label::new(cx, &user.name); }
    }
});
```

### Example

See `examples/async_state.rs` for a complete demo with all features.

---

## 12. Undo/Redo API

Built-in undo/redo support for signals with automatic history tracking.

### Basic Usage

```rust
// Create undoable state (registers for auto-snapshot)
let circles = cx.state_undoable(Vec::<Circle>::new());

// Wrap mutations in undo groups (RAII-style)
cx.with_undo("Add Circle", |cx| {
    circles.upd(cx, |v| v.push(circle));
});

// Multiple mutations in one group
cx.with_undo("Batch Operation", |cx| {
    data.upd(cx, |v| v.items.push(item1));
    data.upd(cx, |v| v.items.push(item2));
    data.upd(cx, |v| v.count += 2);
});

// Trigger undo/redo
cx.undo();
cx.redo();
```

### Reactive UI State

```rust
// Reactive signals - update automatically when history changes
let can_undo = cx.can_undo_signal();
let can_redo = cx.can_redo_signal();

// Bind to button disabled state
let undo_disabled = can_undo.drv(cx, |v, _| !v);
let redo_disabled = can_redo.drv(cx, |v, _| !v);

Button::new(cx, |cx| Label::new(cx, "Undo"))
    .disabled(undo_disabled)
    .on_press(|cx| cx.undo());

Button::new(cx, |cx| Label::new(cx, "Redo"))
    .disabled(redo_disabled)
    .on_press(|cx| cx.redo());
```

### History Management

```rust
// Set max history size (default: 100)
cx.set_max_undo_history(50);

// Clear all history
cx.clear_undo_history();

// Check state directly (prefer reactive signals for UI)
cx.can_undo();
cx.can_redo();
```

### Manual Group Control

For complex scenarios where RAII doesn't fit:

```rust
// Begin/end pattern
cx.begin_undo_group("Complex Operation");
// ... mutations ...
cx.end_undo_group();
```

### How It Works

1. `state_undoable` registers a signal for undo tracking and stores its clone function
2. `with_undo` calls `begin_undo_group` which snapshots all registered signals before mutation
3. When mutation completes, `end_undo_group` saves the snapshot to the undo stack
4. `undo()` restores the previous snapshot, pushing current state to redo stack
5. `redo()` restores from redo stack, pushing current state back to undo stack
6. Reactive signals (`can_undo_signal`) track a version counter that increments on any history change

### Example

See `examples/7GUIs/circle_drawer.rs` for a complete demo with undo/redo.

---

## 13. Rich Text Labels

Unified rich text API through `Label::new` with method chaining.

### Markdown (Auto-Parsed)

Markdown syntax is automatically parsed. No extra methods needed.

```rust
Label::new(cx, "Press **Cmd+S** to *save*");
Label::new(cx, "Use `monospace` for code");
Label::new(cx, "This is __underlined__ and ~~strikethrough~~");
```

**Markdown syntax:**
- `**bold**` - bold
- `*italic*` or `_italic_` - italic
- `__underline__` - underline
- `~~strikethrough~~` - strikethrough
- `` `code` `` - monospace
- `"literal"` - escape (text in quotes is not parsed)

### Links & Custom Styles

Use `[tag]...[/tag]` syntax with `.link()` or `.rich_style()`. Requires `.build_rich()`.

```rust
// Clickable links - opens URL or file path
Label::new(cx, "Visit [docs]documentation[/docs] or [repo]GitHub[/repo]")
    .link("docs", "https://docs.vizia.dev")
    .link("repo", "https://github.com/vizia/vizia")
    .build_rich();

// Custom tag styles
Label::new(cx, "This is [highlight]important[/highlight]")
    .rich_style("highlight", |s| s.background_color(Color::yellow()))
    .build_rich();

// Mix markdown with custom tags
Label::new(cx, "**Click** [link]**here**[/link] to continue")
    .link("link", "https://vizia.dev")
    .build_rich();
```

### Reactive Bindings

Use `{name}` placeholders with `.rich_bind()`. Requires `.build_rich()`.

```rust
let count = cx.state(0i32);

Label::new(cx, "Counter: {count} clicks")
    .rich_bind("count", count)
    .rich_style("count", |s| s.font_weight(FontWeightKeyword::Bold))
    .build_rich();
```

### Conditionals

Use `{#if name}...{/if}` with `.cond()`. Requires `.build_rich()`.

```rust
let show_warning = cx.state(true);

Label::new(cx, "Status: {#if warn}**Warning!** {/if}All systems go.")
    .cond("warn", show_warning)
    .build_rich();
```

### Loops

Use `{#each name as item}...{/each}` with `.each()`. Requires `.build_rich()`.

```rust
let items = cx.state(vec!["Apple", "Banana", "Cherry"]);

Label::new(cx, "Fruits: {#each fruits as f}{f}, {/each}")
    .each("fruits", items, |item| item.to_string())
    .build_rich();
```

### Method Summary

| Method | Purpose | Requires `.build_rich()` |
|--------|---------|--------------------------|
| (markdown) | Bold, italic, etc. | No |
| `.link(tag, url)` | Clickable links | Yes |
| `.rich_style(tag, f)` | Custom tag styles | Yes |
| `.rich_bind(name, signal)` | Reactive placeholders | Yes |
| `.cond(name, signal)` | Conditional rendering | Yes |
| `.each(name, signal, f)` | Loop rendering | Yes |

### Example

See `examples/rich_text.rs` for a complete demo of all features.
