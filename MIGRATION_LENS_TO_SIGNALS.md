# Migration Guide: Lens-Based State to Signals

Use this guide to migrate from lens-driven state to signal-driven state.

## What changed

- Reactive updates are signal-driven.
- Model fields that were plain values are typically Signal<T>.
- Views/modifiers take signal sources in common paths.
- Writes move from assignment to .set(...) / .update(...).
- Lens derives are no longer the primary app-state pattern.

## Quick checklist

1. Remove lens derives from app models.
2. Change reactive fields from T to Signal<T>.
3. Create signals first with Signal::new(...), then pass into models and views.
4. Replace writes (field = value, field += 1) with .set / .update.
5. Replace lens widget inputs (AppData::field) with signal handles.
6. Update event handlers and bind closures to new value signatures.
7. Migrate derived values to derived signals (use map for simple transforms, Memo for computed state).
8. If setup/use is split across files, access model/view state via `cx.data()`.

## Core patterns

### 1) Model fields

```rust
// Before
#[derive(Lens)]
struct AppData {
    count: i32,
}

// After
struct AppData {
    count: Signal<i32>,
}
```

### 2) Initialize signals first, then wire model + views

```rust
// Before
AppData { count: 0 }.build(cx);

// After
let count = Signal::new(0);
AppData { count }.build(cx);

HStack::new(cx, |cx| {
    Label::new(cx, count);
});
```

If setup and usage are in separate files and you cannot pass the signal through constructors:

```rust
// setup module
let count = Signal::new(0);
AppData { count }.build(cx);

// consumer module
let count = cx.data::<AppData>().unwrap().count;
Label::new(cx, count);
```

### 3) Event writes

```rust
// Before
self.count += 1;
self.date = *date;

// After
self.count.update(|count| *count += 1);
self.date.set(*date);
```

### 4) View inputs

```rust
// Before
Label::new(cx, AppData::count);
Calendar::new(cx, AppState::date);

// After
Label::new(cx, count);
Calendar::new(cx, date);
```

### 5) Collection shapes

List common pattern: Signal<Vec<Signal<T>>>.

```rust
let list = Signal::new((0..15u32).map(Signal::new).collect::<Vec<_>>());

List::new(cx, list, |cx, _, item| {
    Label::new(cx, item).hoverable(false);
});
```

VirtualList common pattern: Signal<Vec<T>>.

```rust
let list = Signal::new((1..100u32).collect::<Vec<_>>());

VirtualList::new(cx, list, 40.0, |cx, index, item| {
    Label::new(cx, item).toggle_class("dark", index % 2 == 0).hoverable(false)
});
```

### 6) bind callback expectation

Handle::bind now passes the resolved value directly.

```rust
// old
handle.bind(source, |handle, lens_like| {
    let value = lens_like.get(&handle);
});

// new
handle.bind(source, |handle, value| {
    // value is already resolved
});
```

### 7) Res: get(...) -> get_value(...)

Res is the unifying input type behind many modifiers/constructors. It can represent values, lenses, and signals.

When reading from a Res source directly, use `.get_value(cx)`.

```rust
// Before
let v = source.get(cx);

// After
let v = source.get_value(cx);
```

Important: this does not apply to `Handle::bind` callback parameters. In `bind`, the callback already receives the resolved value.

```rust
// Do this in bind callbacks
handle.bind(source, |_, value| {
    // use value directly
});
```

### 8) Derived signals with Memo and map

Use derived signals when one UI value depends on other signals.

- Use Memo for computed values that combine one or more signals.
- Use map for lightweight projection/formatting from one signal.

Memo example:

```rust
let orientation = Signal::new(Orientation::Vertical);
let is_horizontal = Memo::new(move |_| orientation.get() == Orientation::Horizontal);

Switch::new(cx, is_horizontal)
    .on_toggle(|cx| cx.emit(AppEvent::ToggleHorizontal));
```

map example:

```rust
let count = Signal::new(0);
let label_text = count.map(|v| format!("Count: {v}"));

Label::new(cx, label_text);
```

Rule of thumb:

- Prefer map for simple one-signal transformations.
- Prefer Memo for multi-signal or reusable computed state.

