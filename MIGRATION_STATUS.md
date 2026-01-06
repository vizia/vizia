# Vizia Lens → Signals Migration Status

Last Updated: 2026-01-06

## Overview

This document tracks the progress of migrating from Lens-based state management to Signals-based architecture in the `signals` branch.

---

## Migration Progress Summary

| Category | Migrated | Remaining | Progress |
|----------|----------|-----------|----------|
| Examples | Signal-based view APIs + modifier literals migrated | 0 | 100% |
| Core Views | 33 signal-based + 6 structural | 0 | 100% |
| Framework Internals | 4 (PopupData, ModalModel, Environment, Theme) | 0 | 100% |
| Lens Infrastructure | Removed | 0 | 100% |
| Res-based APIs | Restored (value-or-signal modifiers, window modifiers, view constructors) | 0 | 100% |
| Doc Comments | Updated where APIs changed | 0 | 100% |

**Overall Status**: Signal-based internals, Res-based view APIs, example modifiers, core internals, and doc comments are migrated. Lens infrastructure remains removed; Res has been reintroduced for value-or-signal modifiers, window modifiers, and view constructors. Example-only `#[derive(Data)]` remnants removed; core types may still derive `Data` where required. No lens usage remains in `.rs` sources.

**Recent Polish**:
- Added `.two_way()` to Checkbox, Switch, ToggleButton for API consistency
- Fixed clippy warnings: `unused_mut`, `derivable_impls`, `unnecessary_map_or`
- Fixed empty line after doc comment in tooltip.rs
- Used modern `is_none_or()` instead of `map_or(true, ...)`
- Added unified keyed API (`items.keyed(|t| t.id)`) for opt-in item reuse without full rebuilds (addresses Concern 2 performance)
  - Works with `List::new`, `TabView::new`, `TabBar::new`, and `PickList::new`
  - Unified `Keyed` wrapper and `KeyedExt` trait in `list.rs` (shared across all list-like views)
  - Note: For PickList, keyed reuse only helps when list changes while popup is open (rare case)
- Added comprehensive async state management (`Async<T, E>`) with:
  - States: Idle, Loading, Ready, Reloading, Error, Stale, Timeout, Retrying
  - Cancellation support via `AsyncHandle`
  - Stale-while-revalidate pattern (`refresh_async`)
  - Timeout with error state
  - Retry with exponential backoff + UI progress feedback
  - TTL / cache freshness (`age()`, `is_expired()`, `is_fresh_within()`)
  - Deduplication (skip if already loading)
  - `AsyncOptions` presets: `quick()`, `patient()`, `resilient()`
  - Example: `examples/async_state.rs`

---

## Examples - Migrated to Signals (APIs)

Examples appear signal-based; no `Lens` or `Model` usage found in `examples/`.

Recent fixes:
- `examples/window_modifiers.rs` now binds the window title to a `Signal<String>` and uses `Application::new_with_state()` so the title updates while typing (`Textbox::on_edit`).
- `examples/timers.rs` uses a 100ms base interval, a toggleable 1s interval button (via `cx.modify_timer`), and a one-shot reset timer.
- All examples (including widget_gallery + gallery) now use signal-backed modifiers for width/height/padding/gap/placement/etc., and window modifiers are signal-driven via `Application::new_with_state()`.
- Doc comments in core modules and views now reflect signal-first patterns (no Lens).
- Removed legacy `#[derive(Data)]` on example-only types (`circle_drawer`, `todo`, `spinbox`, `gallery/civitai`) since signals no longer require Data on those structs.

---

## Framework Internals - Migrated to Signals

All framework internal models now use Signal-based state management:

| Model | File | Signal Fields | Notes |
|-------|------|---------------|-------|
| `PopupData` | `views/popup.rs` | `Signal<bool>` | `is_open` |
| `ModalModel` | `modifiers/actions.rs` | `Signal<(bool, bool)>`, `Signal<bool>` | Tooltip/menu visibility |
| `Environment` | `environment.rs` | `Signal<LanguageIdentifier>` | Locale (theme remains plain field) |
| `Theme` | `environment.rs` | - | Removed `#[derive(Lens)]`, plain struct |

### Bindings Now Use Signals Directly

```rust
// modifiers/actions.rs - tooltip()
let (tooltip_visible, _) = build_modal_model(self.cx, entity);
Binding::new(cx, tooltip_visible, move |cx| { ... });

// localization/mod.rs
let locale_signal = cx.environment().locale;
Binding::new(cx, locale_signal, move |cx| { ... });
```

---

## Core Views - Migrated to Signals

View constructors now accept `impl Res<T>` (value-or-signal) where applicable; notes below call out internal signal usage or other specifics.

| View | File | Commit | Notes |
|------|------|--------|-------|
| Checkbox | `crates/vizia_core/src/views/checkbox.rs` | 83f19f92 | |
| Collapsible | `crates/vizia_core/src/views/collapsible.rs` | 92f2764e | Refactored: removed dead `#[derive(Lens)]`, `open()` now takes `Signal<bool>` |
| Combobox | `crates/vizia_core/src/views/combobox.rs` | ec57e331 | Fixed signal assignment bugs |
| Textbox | `crates/vizia_core/src/views/textbox.rs` | 7b87d38d | Value-or-signal input (`Res<L>`) |
| ToggleButton | `crates/vizia_core/src/views/toggle_button.rs` | 8614d7dd | |
| Tooltip | `crates/vizia_core/src/views/tooltip.rs` | 28c1f4cc | |
| Label | `crates/vizia_core/src/views/label.rs` | - | Value-or-signal text (`Res<T>`) for Label + TextSpan |
| Switch | `crates/vizia_core/src/views/switch.rs` | - | Value-or-signal input (`Res<bool>`) |
| RadioButton | `crates/vizia_core/src/views/radio.rs` | - | Value-or-signal input (`Res<bool>`) |
| Rating | `crates/vizia_core/src/views/rating.rs` | - | Value-or-signal input (`Res<u32>`) with internal preview signal |
| Slider | `crates/vizia_core/src/views/slider.rs` | - | Value-or-signal input (`Res<f32>`); range remains `Signal<Range<f32>>` for UI binding |
| Knob | `crates/vizia_core/src/views/knob.rs` | - | Value-or-signal input (`Res<f32>`); ArcTrack/TickKnob/Ticks use `Signal<f32>` for normalized_value |
| Scrollbar | `crates/vizia_core/src/views/scrollbar.rs` | - | Value-or-signal input (`Res<f32>`) for value + ratio |
| ScrollView | `crates/vizia_core/src/views/scrollview.rs` | - | Signal-based scroll state |
| Badge | `crates/vizia_core/src/views/badge.rs` | - | Value-or-signal placement (`Res<BadgePlacement>`) |
| Avatar | `crates/vizia_core/src/views/avatar.rs` | - | Signal-based variant (AvatarGroup uses internal signals for layout/gap) |
| Button | `crates/vizia_core/src/views/button.rs` | - | Value-or-signal variants (`Res<ButtonVariant>`) for Button + ButtonGroup |
| Chip | `crates/vizia_core/src/views/chip.rs` | - | Value-or-signal text + variant |
| Image | `crates/vizia_core/src/views/image.rs` | - | Value-or-signal source (Image + Svg) |
| Dropdown | `crates/vizia_core/src/views/dropdown.rs` | - | Signal-based popup state |
| Popup | `crates/vizia_core/src/views/popup.rs` | - | Signal-based placement/arrow |
| Divider | `crates/vizia_core/src/views/divider.rs` | - | Signal-based orientation |
| Progressbar | `crates/vizia_core/src/views/progressbar.rs` | - | Value-or-signal input (`Res<f32>`) |
| Menu | `crates/vizia_core/src/views/menu.rs` | - | Signal-based menu state |
| Picklist | `crates/vizia_core/src/views/picklist.rs` | - | Value-or-signal input (`Res<Vec<T>>`, `Res<usize>`) |
| Spinbox | `crates/vizia_core/src/views/spinbox.rs` | - | Value-or-signal input (`Res<T>`) |
| TabView | `crates/vizia_core/src/views/tabview.rs` | - | Value-or-signal list input (`Res<Vec<T>>`) for TabView + TabBar |
| List | `crates/vizia_core/src/views/list.rs` | - | Value-or-signal list input (`Res<Vec<T>>`) for List + ListItem |
| VirtualList | `crates/vizia_core/src/views/virtual_list.rs` | - | Value-or-signal list input (`Res<Vec<T>>`) |
| XYPad | `crates/vizia_core/src/views/xypad.rs` | - | Value-or-signal input (`Res<(f32, f32)>`) |
| ResizableStack | `crates/vizia_core/src/views/resizable_stack.rs` | - | Value-or-signal input (`Res<Units>`) (ResizeHandle uses internal signals) |
| Datepicker | `crates/vizia_core/src/views/datepicker.rs` | - | Value-or-signal input (`Res<NaiveDate>`) |
| NumberInput | `crates/vizia_core/src/views/number_input.rs` | - | Wraps Textbox with min/max validation; value-or-signal input (`Res<T>`) |

---

## Core Views - Structural/Helper (No external signal parameters)

These views don't expose signal-based APIs; they use internal signals for layout/behavior or are stateless:

| View | File | Notes |
|------|------|-------|
| Element, Spacer | `crates/vizia_core/src/views/element.rs` | Stateless structural views |
| Grid | `crates/vizia_core/src/views/grid.rs` | Internal signals for layout/grid tracks |
| VStack, HStack, ZStack | `crates/vizia_core/src/views/stack.rs` | Internal signals for layout type |
| Markdown | `crates/vizia_core/src/views/markdown.rs` | Internal signals for parsed content; API takes `&str` |
| MenuButton | `crates/vizia_core/src/views/menu.rs` | Internal signals only; no signal params |
| ResizeHandle | `crates/vizia_core/src/views/resizable_stack.rs` | Internal signals only; no signal params |

---

## Lens Infrastructure - Removed

The Lens system has been deleted from `signals`:

| Removed Item | Notes |
|-------------|-------|
| `binding/lens.rs` | Lens trait and impls removed |
| `binding/store.rs` | Lens store removed |
| `binding/map.rs` | Lens map removed |
| `binding/binding_view.rs` | Refactored to signal-only `Binding` |
| `systems/binding.rs` | Binding system removed from event flow |
| `vizia_derive/src/lens.rs` | `#[derive(Lens)]` proc macro removed |
| `lib.rs` prelude | `Lens` export removed |

`Res` has been reintroduced for value-or-signal modifier ergonomics (Signal-based binding only; no lens compatibility).

---

## Doc Comments

Signal-first documentation is updated where APIs changed in core modules, views, and layout modifiers.

---

## Next Steps (Priority Order)

1. **None** - Doc and README sweeps complete (proposal/agent docs intentionally retain lens history)

---

## Core Infrastructure (Complete)

- [x] `crates/vizia_core/src/recoil/mod.rs` - Signal implementation with `Signal<T>`, `cx.state()`, `signal.drv()`, `cx.derived()`
- [x] `crates/vizia_winit/src/application_trait.rs` - App trait for application-level state
- [x] `crates/vizia_core/src/context/mod.rs` - Context modifications for signal creation
- [x] `crates/vizia_core/src/binding/binding_view.rs` - Signal-only `Binding` with disposable scope per update
- [x] `crates/vizia_core/src/modifiers/mod.rs` - `internal::bind_res` helper for value-or-signal modifiers
- [x] Store access to DataContext trait
- [x] `crates/vizia_winit/src/application.rs` - `Application::new_with_state()` to return signals created during app build
- [x] `crates/vizia_core/src/context/mod.rs` - Timer tick logic updated to allow one-shot ticks when frames are late
- [x] `crates/vizia_core/src/context/mod.rs` + `crates/vizia_core/src/context/event.rs` - Timer heap mutation/query avoids `BinaryHeap::peek()` infinite loop
- [x] `crates/vizia_core/src/recoil/async_state.rs` - Async state management with `Async<T, E>`, cancellation, timeout, retry, TTL

---

## Migration Patterns Reference

### Before (Lens)

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

// Usage
Checkbox::new(cx, AppData::value);
```

---

## Owner Concerns, Solutions, and Roadmap (2026-01-02)

### Concern 1 — Loss of `Res<T>` value-or-signal ergonomics
**Problem**: Removing `Res<T>` forces modifiers, window modifiers, and view constructors to accept `Signal<T>` only, which makes literal/constant usage noisy and breaks existing expectations for passing raw values.
**Solution**: Restored `Res<T>` as a value-or-signal abstraction in `binding/res.rs`, then applied it across modifiers, window modifiers, and view constructors:

**Implementation Details**:
- **Method renamed**: `Res::get` → `Res::resolve` to avoid conflicts with standard library methods (`Vec::get`, `HashMap::get`, etc.). When `Res` trait was in scope, Rust would resolve `.get()` calls to `Res::get` instead of the native collection methods.
- **Lifetime bounds**: All modifier functions using `impl Res<T>` require `+ 'static` bounds since values are captured in closures for binding.
- **Core trait** (`binding/res.rs`):
  ```rust
  pub trait Res<T>: Sized {
      fn resolve(&self, cx: &impl DataContext) -> T;
      fn set_or_bind<F>(self, cx: &mut Context, entity: Entity, closure: F);
      fn into_signal(self, cx: &mut Context) -> Signal<T>;
  }
  ```
- **Value behavior**: Plain values call the closure immediately via `set_or_bind`.
- **Signal behavior**: `Signal<T>` impl creates a `Binding::new` so updates propagate reactively.
- **Modifier helper**: `internal::bind_res()` in `modifiers/mod.rs` handles the binding pattern for style modifiers.
- **Builder types**: `LinearGradientBuilder` and `ShadowBuilder` implement `Res` for ergonomic builder patterns.
- **Window modifiers**: `vizia_winit` window modifiers (title, size, position, etc.) use `Res::resolve` for initial values and `set_or_bind` for reactive updates.
- **View constructors**: Views that previously required `Signal<T>` now accept `impl Res<T>` where applicable for value-or-signal ergonomics.

**Files Modified**:
- `crates/vizia_core/src/binding/res.rs` — Core `Res<T>` trait and impls
- `crates/vizia_core/src/modifiers/{mod,style,layout,text,abilities,accessibility}.rs` — Added `+ 'static` bounds
- `crates/vizia_winit/src/{application,window}.rs` — Window modifiers use `Res::resolve`

### Concern 2 — Signal lifetime management (lists + dynamic bindings)
**Problem**: Signals created inside `Binding` closures are owned by the binding entity, which is long-lived; repeated updates (e.g., lists) can accumulate signals and leak memory/state.
**Solution**: Two-pronged approach: (1) disposable **binding scope entity** for automatic cleanup, and (2) **unified keyed API** for opt-in entity reuse.

**Implementation Details — Binding Scope Cleanup**:
- Each `Binding` owns a `scope` child entity used exclusively for building content.
- On update, the previous scope entity is removed and a fresh scope is created.
- Signal ownership is entity-scoped in the `recoil::Store` (`entity_signals`), so removing the scope triggers `entity_destroyed` and cleans all signals created during the prior build.
- This directly addresses list-like views (`List`, `VirtualList`, `TabView`) without changing their public APIs.

**Implementation Details — Unified Keyed API**:
- **`Keyed<T, K, R, F>` wrapper** (`views/list.rs`): Wraps a list with a key function for stable-identity item reuse.
- **`KeyedExt<T>` trait**: Extension trait providing `.keyed(|item| item.id)` method on any `Res<Vec<T>>`.
- **Unified across views**: The same `Keyed` wrapper works with `List::new`, `TabView::new`, `TabBar::new`, and `PickList::new`.
- **Internal dispatch traits**: `ListSource<T>`, `TabSource<T>`, `PickListSource<T>` select between normal and keyed build paths.
- **Keyed diffing**: Maintains `HashMap<K, VecDeque<KeyedItem>>` to reuse entities by stable key across updates.
- **Signal-based index**: Keyed items use `Signal<usize>` for index so selection/focus updates correctly on reorder.
- **Note for PickList**: Keyed reuse only helps when list changes while popup is open (rare case since popup is destroyed on close).

**Files Modified**:
- `crates/vizia_core/src/binding/binding_view.rs` — Disposable scope entity
- `crates/vizia_core/src/views/list.rs` — `Keyed`, `KeyedExt`, `ListSource`, keyed diffing logic
- `crates/vizia_core/src/views/tabview.rs` — `TabSource` trait, `TabHeader` with `Signal<usize>` index
- `crates/vizia_core/src/views/picklist.rs` — `PickListSource` trait, `PickListItemKeyed` with `Signal<usize>` index

### Roadmap — Status
1. ✅ **Binding scope cleanup**: Scope-entity rebuild shipped in `Binding`.
2. ✅ **`Res<T>` reinstatement**: `binding/res.rs` restored, exported in prelude, modifier + window modifier signatures updated.
3. ✅ **Keyed list performance**: Unified `items.keyed(|t| t.id)` API for stable-key item reuse across all list-like views (`List`, `TabView`, `TabBar`, `PickList`).
4. 🔄 **Behavior verification**: Ongoing — stress test list updates and binding-heavy examples.
5. 📋 **Optional hardening**: `Signal::try_get` or debug guard for stale handles (future work).

### After (Signal)

```rust
struct MyView {
    value: Signal<bool>,
}

impl MyView {
    fn new(cx: &mut Context) -> Handle<'_, Self> {
        Self { value: cx.state(false) }.build(cx, |cx| {})
    }
}

impl View for MyView {
    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        // Direct signal updates:
        // self.value.update(cx, |v| *v = !*v);
    }
}
```

---

## Novel Constructs & Concepts

Patterns formalized during migration:

| Pattern | Description |
|---------|-------------|
| DrawContext Cache | Cache signal values for `draw()` since DrawContext doesn't impl DataContext |
| `signal.drv()` for Signal APIs | Use `signal.drv(cx, \|v, s\| ...)` (preferred) or `cx.derived()` when APIs expect `Signal<T>` |
| Clone in `bind_res` | Clone values before mutable `cx` operations to avoid borrow conflicts |
| `static_text()` Convenience | Framework method for static text without fake signals |
| `.two_way()` Convenience | Auto-wires `on_change` to update bound signal (Slider, Knob, XYPad, Textbox, NumberInput, Checkbox, Switch, ToggleButton, Rating) |
| `internal::bind_res` | Modifier helper for binding `Res<T>` (values or signals) to style/state updates |
| Signal-only `Binding::new` | Bindings accept `Signal<T>` directly (Res is used at modifier/window-modifier level) |
| `Application::new_with_state()` | Returns app + builder state (e.g., signals) for window modifiers and external bindings |
| `cx.modify_timer(...)` toggle | Use a signal-backed flag to swap timer intervals at runtime (see `examples/timers.rs`) |
| Signal-backed modifier updates | Drive modifier changes via internal signals inside binds (e.g., Badge placement, ProgressBar sizing) |
| Unified `.keyed()` API | Opt-in keyed reuse for list-like views via `Keyed` wrapper + `KeyedExt` trait (List, TabView, TabBar, PickList) |
| Async State Pattern | `Signal<Async<T, E>>` for async data loading with states, retry, timeout, TTL (see below) |

---

## Known Patterns

### DrawContext Cache Pattern

```rust
struct MyCanvas {
    data: Signal<Vec<Data>>,
    data_cache: Vec<Data>, // Cache for draw()
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
```

### Two-Way Binding Pattern

For controls that modify a signal (Slider, Knob, XYPad), use `.two_way()` instead of manual `on_change`:

```rust
// Before (verbose):
let value = cx.state(0.5f32);
Slider::new(cx, value).on_change(move |cx, val| value.set(cx, val));

// After (concise):
let value = cx.state(0.5f32);
Slider::new(cx, value).two_way();
```

Available on: `Slider`, `Knob`, `XYPad`, `Textbox`, `NumberInput`, `Checkbox`, `Switch`, `ToggleButton`, `Rating`

### Unified Keyed API Pattern

For list-like views with stable item identities, use `.keyed()` for opt-in entity reuse instead of full rebuilds:

```rust
// List - reuses item entities when keys match
let items: Signal<Vec<Item>> = cx.state(vec![...]);
List::new(cx, items.keyed(|item| item.id), |cx, index, item| {
    Label::new(cx, item.map(|i| i.name.clone()));
});

// TabView - reuses tab header entities
let tabs: Signal<Vec<Tab>> = cx.state(vec![...]);
TabView::new(cx, tabs.keyed(|tab| tab.id), |cx, tab| {
    TabPair::new(
        move |cx| Label::new(cx, tab.map(|t| t.title.clone())),
        move |cx| Label::new(cx, tab.map(|t| t.content.clone())),
    )
});

// PickList - reuses popup items (only helps when list changes while popup is open)
let options: Signal<Vec<Option>> = cx.state(vec![...]);
PickList::new(cx, options.keyed(|opt| opt.id), selected, true);
```

The `Keyed` wrapper and `KeyedExt` trait are defined in `list.rs` and shared across all list-like views for API consistency.

### Async State Pattern

For async data loading with comprehensive state management:

```rust
// Create async signal
let users: Signal<Async<Vec<User>, String>> = cx.async_state();

// Basic load
cx.load_async(users, || fetch_users());

// With cancellation
let handle = cx.load_async_cancelable(users, || fetch_users());
handle.cancel();

// Refresh (stale-while-revalidate)
cx.refresh_async(users, || fetch_users());

// With options (timeout, retry)
cx.load_async_with(users, AsyncOptions::default()
    .timeout(Duration::from_secs(30))
    .retry(3), || fetch_users());

// React to state changes
Binding::new(cx, users.drv(cx, |s, _| s.is_loading()), |cx| { ... });
Binding::new(cx, users.drv(cx, |s, _| s.is_retrying()), |cx| { ... });

// TTL / cache freshness
if users.is_expired(cx, Duration::from_secs(60)) {
    cx.refresh_async(users, || fetch_users());
}
```

**States**: `Idle` → `Loading` → `Ready(T)` / `Error(E)` / `Timeout`, with `Reloading(T)`, `Stale(T, E)`, `Retrying(attempt, max, E)` for intermediate states.

**Files**: `recoil/async_state.rs`, `context/mod.rs`, `context/event.rs`
