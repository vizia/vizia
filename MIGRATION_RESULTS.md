# Vizia Signals Migration Results

> **TL;DR**: Complete migration from Lens to Signals architecture. All 33 views migrated, Lens infrastructure removed, `Res<T>` restored for ergonomics, plus new features: async state management, undo/redo, and keyed list diffing.

---

## Executive Summary

This PR replaces vizia's Lens-based state management with a Signals-based reactive architecture. The migration is **100% complete** with all views, examples, and framework internals converted.

**Key outcomes:**
- Simpler API: No `#[derive(Lens)]`, no `#[derive(Data)]`, no `Model` trait required
- Better ergonomics: `Res<T>` allows both values and signals in view constructors/modifiers
- New features: Async state (`Async<T,E>`), undo/redo, keyed list diffing
- Proper lifetime management: Binding scope cleanup prevents signal leaks

---

## Migration Progress

| Category | Status | Details |
|----------|--------|---------|
| Core Views | ✅ 100% | 33 signal-based + 6 structural views |
| Examples | ✅ 100% | All examples use signal APIs |
| Framework Internals | ✅ 100% | PopupData, ModalModel, Environment, Theme |
| Lens Infrastructure | ✅ Removed | `binding/lens.rs`, `#[derive(Lens)]`, etc. |
| Res-based APIs | ✅ Restored | Value-or-signal modifiers, view constructors |
| Doc Comments | ✅ Updated | Signal-first documentation |

---

## Owner Concerns & Solutions

### Concern 1: Loss of `Res<T>` Ergonomics

**Problem**: Removing `Res<T>` forced `Signal<T>` for all modifiers/constructors, making literal values noisy.

**Solution**: Restored `Res<T>` as value-or-signal abstraction.

```rust
// Both work - signal auto-binds, value is static
label.width(Pixels(100.0));      // Static value
label.width(width_signal);        // Reactive signal
```

**Implementation**:
- `Res::resolve()` (renamed from `get` to avoid stdlib conflicts)
- `Res::set_or_bind()` - values apply immediately, signals create bindings
- `internal::bind_res()` helper in modifiers
- `+ 'static` bounds on all `impl Res<T>` parameters

**Files**: `binding/res.rs`, `modifiers/*.rs`, `vizia_winit/src/*.rs`

### Concern 2: Signal Lifetime Management

**Problem**: Signals in `Binding` closures accumulate on repeated updates (lists), leaking memory.

**Solution**: Two-pronged approach:

1. **Binding Scope Cleanup**: Each `Binding` owns a disposable scope entity. On update, the scope is destroyed (cleaning all signals) and recreated.

2. **Unified Keyed API**: Opt-in entity reuse for list-like views:
```rust
List::new(cx, items.keyed(|item| item.id), |cx, index, item| { ... });
TabView::new(cx, tabs.keyed(|tab| tab.id), |cx, tab| { ... });
```

**Files**: `binding/binding_view.rs`, `views/list.rs`, `views/tabview.rs`, `views/picklist.rs`

---

## What Changed

### Core Views Migrated (33 views)

All view constructors now accept `impl Res<T>` (value-or-signal):

| View | Notes |
|------|-------|
| Label | `Res<impl ToString>` for text |
| Button | `Res<ButtonVariant>` for variants |
| Checkbox, Switch, ToggleButton | `Res<bool>` + `.two_way()` |
| Slider, Knob | `Res<f32>` + `.two_way()` |
| Textbox, NumberInput | `Res<T>` + `.two_way()` |
| Rating | `Res<u32>` + `.two_way()` |
| XYPad | `Res<(f32, f32)>` + `.two_way()` |
| Progressbar | `Res<f32>` |
| List, VirtualList | `Res<Vec<T>>` + `.keyed()` opt-in |
| TabView, TabBar | `Res<Vec<T>>` + `.keyed()` opt-in |
| PickList, Combobox | `Res<Vec<T>>`, `Res<usize>` |
| Datepicker | `Res<NaiveDate>` |
| Image, Svg | `Res` for source |
| Badge, Chip | `Res` for placement/variant |
| Popup, Dropdown, Menu | Internal signals for state |
| Scrollbar, ScrollView | Signal-based scroll state |
| Divider | Signal-based orientation |
| Spinbox | `Res<T>` |
| ResizableStack | `Res<Units>` |
| Collapsible | `Signal<bool>` for open state |
| Tooltip | Internal signals |

### Structural Views (6 views, no external signal params)

| View | Notes |
|------|-------|
| Element, Spacer | Stateless |
| VStack, HStack, ZStack | Internal layout signals |
| Grid | Internal grid track signals |
| Markdown | Internal parsed content signals |

### Framework Internals

| Model | Signal Fields |
|-------|---------------|
| PopupData | `Signal<bool>` for is_open |
| ModalModel | `Signal<(bool, bool)>`, `Signal<bool>` for tooltip/menu |
| Environment | `Signal<LanguageIdentifier>` for locale |
| Theme | Plain struct (removed `#[derive(Lens)]`) |

### Lens Infrastructure Removed

| Removed | Notes |
|---------|-------|
| `binding/lens.rs` | Lens trait and impls |
| `binding/store.rs` | Lens store |
| `binding/map.rs` | Lens map |
| `systems/binding.rs` | Binding system |
| `vizia_derive/src/lens.rs` | `#[derive(Lens)]` proc macro |
| Prelude exports | `Lens` removed |

---

## Signal API vs Original Proposal

### ✅ Core Signal System (Proposal Lines 101-116)

| Feature | Status |
|---------|--------|
| `cx.state(initial_value)` | ✅ Implemented |
| `signal.upd(cx, \|v\| ...)` | ✅ Implemented |
| `signal.set(cx, value)` | ✅ Implemented |
| `cx.derived(closure)` | ✅ Implemented (prefer `signal.drv()`) |
| Automatic dependency tracking | ✅ Built into Signal internals |

### ✅ Simplified API (Proposal Lines 230-242)

| Feature | Status |
|---------|--------|
| No `#[derive(Lens)]` | ✅ Lens infrastructure removed |
| No `#[derive(Data)]` required | ✅ Only for complex equality checks |
| No `Model` trait required | ✅ Views use `View::event` directly |

### ✅ App Trait Pattern (Proposal Lines 307-349)

```rust
struct Counter {
    count: Signal<i32>,
}

impl App for Counter {
    // app_name() auto-derives: "Counter"

    fn new(cx: &mut Context) -> Self {
        Self { count: cx.state(0) }
    }

    fn on_build(self, cx: &mut Context) -> Self {
        Label::new(cx, self.count);
        Button::new(cx, |cx| Label::new(cx, "+"))
            .on_press(move |cx| self.count.upd(cx, |v| *v += 1));
        self
    }

    fn window_config(&self) -> WindowConfig {
        window(|app| app.inner_size((400, 300)))
    }
}

fn main() -> Result<(), ApplicationError> { Counter::run() }
```

### ✅ Direct Signal Updates (Proposal Lines 190-226)

Both patterns work - choose per use case:

```rust
// Direct updates - no events needed
Button::new(cx, |cx| Label::new(cx, "+"))
    .on_press(move |cx| count.upd(cx, |v| *v += 1));

// Or with events - still works
Button::new(cx, |cx| Label::new(cx, "+"))
    .on_press(|cx| cx.emit(CounterEvent::Increment));
```

### ✅ Derived State (Proposal Lines 244-306)

```rust
let count = cx.state(5i32);
let doubled = count.drv(cx, |v, _| v * 2);
let label = count.drv(cx, |v, _| format!("Count: {v}"));

// Multi-signal derivation
let sum = cx.derived(move |s| *a.get(s) + *b.get(s));
```

---

## New Features Beyond Proposal

### From Proposal Future Work (Lines 434-438)

### ✅ Async State Management

Comprehensive async data loading with `Signal<Async<T, E>>`:

```rust
// Create async signal
let users: Signal<Async<Vec<User>, String>> = cx.async_state();

// Basic load (with deduplication)
cx.load_async(users, || fetch_users());

// With cancellation
let handle = cx.load_async_cancelable(users, || fetch_users());
handle.cancel();

// Refresh (stale-while-revalidate)
cx.refresh_async(users, || fetch_users());

// With timeout + retry
cx.load_async_with(users, AsyncOptions::patient(), || fetch_users());

// TTL / cache freshness
if users.is_expired(cx, Duration::from_secs(60)) {
    cx.refresh_async(users, || fetch_users());
}
```

**States**: `Idle` → `Loading` → `Ready(T)` / `Error(E)` / `Timeout`
**Intermediate**: `Reloading(T)`, `Stale(T, E)`, `Retrying(attempt, max, E)`

**Presets**: `AsyncOptions::quick()` (5s), `patient()` (30s, 3 retries), `resilient()` (60s, 5 retries)

**Files**: `recoil/async_state.rs`, `context/mod.rs`, `context/event.rs`

### ✅ Undo/Redo Support

Framework-level undo/redo with automatic snapshot management:

```rust
// Register signals for undo tracking
let document = cx.state_undoable(Document::new());

// RAII-style undo groups
cx.with_undo("Edit Document", |cx| {
    document.upd(cx, |d| d.apply_edit(edit));
});

// Trigger undo/redo
cx.undo();
cx.redo();

// Reactive signals for UI
let can_undo = cx.can_undo_signal();
let can_redo = cx.can_redo_signal();

Button::new(cx, |cx| Label::new(cx, "Undo"))
    .disabled(can_undo.drv(cx, |v, _| !v))
    .on_press(|cx| cx.undo());

// History management
cx.set_max_undo_history(50);
cx.clear_undo_history();
```

**Files**: `recoil/mod.rs` (UndoManager), `context/mod.rs`, `context/event.rs`

### Additional Improvements (Not in Proposal)

### ✅ Unified Keyed API

Opt-in keyed diffing for list-like views:

```rust
// List - reuses entities when keys match
List::new(cx, items.keyed(|item| item.id), |cx, index, item| {
    Label::new(cx, item.map(|i| i.name.clone()));
});

// TabView - reuses tab entities
TabView::new(cx, tabs.keyed(|tab| tab.id), |cx, tab| {
    TabPair::new(
        move |cx| Label::new(cx, tab.map(|t| t.title.clone())),
        move |cx| Label::new(cx, tab.map(|t| t.content.clone())),
    )
});
```

Works with: `List`, `TabView`, `TabBar`, `PickList`

**Files**: `views/list.rs` (`Keyed`, `KeyedExt`), `views/tabview.rs`, `views/picklist.rs`

### ✅ Two-Way Binding

Auto-wire `on_change` to update bound signal:

```rust
// Before (verbose)
Slider::new(cx, value).on_change(move |cx, val| value.set(cx, val));

// After (concise)
Slider::new(cx, value).two_way();
```

Available on: `Slider`, `Knob`, `XYPad`, `Textbox`, `NumberInput`, `Checkbox`, `Switch`, `ToggleButton`, `Rating`

### ✅ Rich Text Labels

Unified rich text through `Label::new` with method chaining:

```rust
// Markdown (auto-parsed)
Label::new(cx, "Press **Cmd+S** to *save*");

// Links & styles (requires .build_rich())
Label::new(cx, "Visit [docs]documentation[/docs]")
    .link("docs", "https://docs.vizia.dev")
    .build_rich();

// Reactive bindings (requires .build_rich())
Label::new(cx, "Counter: {count}")
    .rich_bind("count", count_signal)
    .build_rich();

// Conditionals & loops
Label::new(cx, "{#if warn}Warning!{/if}")
    .cond("warn", show_warning)
    .build_rich();
```

**Syntax**: `**bold**`, `*italic*`, `__underline__`, `~~strike~~`, `` `code` ``, `"escaped"`, `[tag]`, `{bind}`, `{#if}`, `{#each}`

**Files**: `views/label.rs`

### Other Improvements

| Feature | Description |
|---------|-------------|
| `Signal::try_get()` | Safe access returning `Option` for stale signals |
| `signal.drv()` | Ergonomic derived signal creation |
| `cx.modify_timer()` | Dynamic timer interval changes |
| `window()` helper | Creates `WindowConfig` closure for App trait |

---

## API Patterns & Examples

### DrawContext Cache Pattern

`DrawContext` doesn't implement `DataContext`, so cache signal values for `draw()`:

```rust
struct MyCanvas {
    data: Signal<Vec<Data>>,
    data_cache: Vec<Data>,
}

impl MyCanvas {
    fn new(cx: &mut Context, data: Signal<Vec<Data>>) -> Handle<Self> {
        Self { data, data_cache: data.get(cx).clone() }
            .build(cx, |_| {})
            .bind(data, |handle, d| {
                let cached = d.get(&handle).clone();
                handle.modify(|c| c.data_cache = cached).needs_redraw();
            })
    }
}

impl View for MyCanvas {
    fn draw(&self, cx: &mut DrawContext, canvas: &Canvas) {
        // Use self.data_cache, not self.data.get()
    }
}
```

### Before/After Comparison

**Before (Lens)**:
```rust
#[derive(Lens)]
pub struct AppData {
    value: bool,
}

impl Model for AppData {
    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|e, _| match e {
            AppEvent::Toggle => self.value = !self.value,
        });
    }
}

Checkbox::new(cx, AppData::value);
```

**After (Signal)**:
```rust
struct MyApp {
    value: Signal<bool>,
}

impl App for MyApp {
    fn new(cx: &mut Context) -> Self {
        Self { value: cx.state(false) }
    }

    fn on_build(self, cx: &mut Context) -> Self {
        Checkbox::new(cx, self.value).two_way();
        self
    }
}
```

---

## API Reference

### Event Handling Methods

| Method | Signature | Behavior |
|--------|-----------|----------|
| `event.map()` | `fn(&M, &mut EventMeta)` | Borrows message, does NOT auto-consume |
| `event.take()` | `fn(M, &mut EventMeta)` | Takes ownership, auto-consumes event |
| `meta.consume()` | `fn()` | Manually stop propagation |

### Res Auto-Binding

When a modifier accepts `impl Res<T>`:
- **Plain values**: Applied immediately, no binding created
- **Signals**: Creates internal `Binding` that updates on change

Use `Binding::new()` only when you need to **rebuild view structure** based on signal changes.

### Hot Reload

Press **F5** in debug builds to reload stylesheets.

---

## File Reference

### Core Infrastructure

| File | Purpose |
|------|---------|
| `recoil/mod.rs` | `Signal<T>`, `Store`, dependency tracking, `UndoManager` |
| `recoil/async_state.rs` | `Async<T, E>`, `AsyncOptions`, retry/timeout |
| `recoil/persistence.rs` | `PersistenceManager`, auto-save/load |
| `binding/res.rs` | `Res<T>` trait for value-or-signal |
| `binding/binding_view.rs` | `Binding` with disposable scope |
| `context/mod.rs` | `cx.state()`, `cx.derived()`, `cx.async_state()`, `cx.state_persistent()`, undo methods |
| `context/event.rs` | `cx.load_async()`, `cx.undo()`, `cx.redo()`, `cx.with_undo()` |
| `modifiers/mod.rs` | `internal::bind_res()` helper |
| `vizia_winit/application_trait.rs` | `App` trait, `window()` helper |

### Views with Notable Changes

| File | Changes |
|------|---------|
| `views/list.rs` | `Keyed`, `KeyedExt`, `ListSource` |
| `views/tabview.rs` | `TabSource`, keyed support |
| `views/picklist.rs` | `PickListSource`, keyed support |
| `views/slider.rs` | `.two_way()` |
| `views/checkbox.rs` | `.two_way()` |
| `views/textbox.rs` | `.two_way()` |

---

### ✅ State Persistence

Automatic save/restore of signal state across app restarts:

```rust
#[derive(Serialize, Deserialize, Clone, Default)]
struct Settings {
    theme: String,
    volume: f32,
}

// Just use it!
let settings = cx.state_persistent("app.settings", Settings::default());

// Use like any other signal
settings.upd(cx, |s| s.volume = 0.8);
```

**Configuration:** `app_name()` is auto-derived from the struct name (e.g., `MyApp` → "My", `CRUDApp` → "CRUD"). Override only for custom names:

```rust
fn app_name() -> &'static str { "Custom Name" }
```

**Features:**
- **App isolation:** Data stored in `{data_local_dir}/{app_name}/signals/`
- Loads from disk on signal creation (falls back to default if not found)
- Auto-saves when value changes (debounced 500ms to reduce I/O)
- Flushes pending saves on app exit
- **Versioned format:** `{"v": 1, "data": {...}}` for future migration support
- **Secure permissions:** Files created with 0600 on Unix
- **Error tracking:** Call `cx.persistence_errors()` to check for issues

**Type constraints:** `T: Serialize + DeserializeOwned + Clone + 'static`

**Files:** `recoil/persistence.rs`, `context/mod.rs`

---

### ✅ Time Travel Debugging

Navigate through signal history in debug builds:

```rust
// Add overlay to your app (debug builds only)
#[cfg(debug_assertions)]
{
    TtrvlOverlay::new(cx);
    cx.add_stylesheet(TTRVL_OVERLAY_STYLE).unwrap();
}

// Programmatic API
cx.ttrvl_to(index);    // Jump to specific point
cx.ttrvl_back();       // Step backward
cx.ttrvl_forward();    // Step forward
cx.ttrvl_exit();       // Return to present
```

**Keybinds** (debug builds only):
- ``Cmd/Ctrl+` `` - Toggle time travel overlay
- `Cmd/Ctrl+[` - Step backward
- `Cmd/Ctrl+Shift+[` - Step forward
- `Escape` - Exit time travel mode

**Files**: `views/ttrvl_overlay.rs`, `events/event_manager.rs`, `recoil/mod.rs`

---

## Examples

| Example | Demonstrates |
|---------|--------------|
| `examples/7GUIs/circle_drawer.rs` | Undo/redo with reactive buttons |
| `examples/async_state.rs` | Async loading with retry/timeout |
| `examples/timers.rs` | `cx.modify_timer()` for dynamic intervals |
| `examples/window_modifiers.rs` | Reactive window title binding |
| `examples/widget_gallery/` | All views with signal APIs |
| `examples/rich_text.rs` | Rich text with markdown, links, bindings, conditionals, loops |
| `examples/time_travel.rs` | Time travel debugging overlay |

---

## Summary

The Signals migration is complete. All views, examples, and framework internals are converted. The implementation:

1. ✅ Covers everything in the original proposal
2. ✅ Adds `Res<T>` for value-or-signal ergonomics
3. ✅ Solves signal lifetime issues with binding scope cleanup
4. ✅ Adds keyed API for list performance
5. ✅ Implements async state management (originally "future work")
6. ✅ Implements undo/redo (originally "future work")
7. ✅ Implements time travel debugging (originally "future work")
8. ✅ Adds unified rich text via `Label::new` with method chaining

No Lens usage remains in the codebase.
