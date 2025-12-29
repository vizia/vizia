# Vizia Lens → Signals Migration Status

Last Updated: 2025-12-29

## Overview

This document tracks the progress of migrating from Lens-based state management to Signals-based architecture in the `signals` branch.

---

## Migration Progress Summary

| Category | Migrated | Remaining | Progress |
|----------|----------|-----------|----------|
| Core Views | 6 | ~20 | ~23% |
| Examples | ~10 | ~40 | ~20% |
| Infrastructure | Complete | - | 100% |

---

## Core Infrastructure (Complete)

- [x] `crates/vizia_core/src/recoil/mod.rs` - Signal implementation with `Signal<T>`, `cx.state()`, `cx.derived()`
- [x] `crates/vizia_winit/src/application_trait.rs` - App trait for application-level state
- [x] `crates/vizia_core/src/context/mod.rs` - Context modifications for signal creation
- [x] `crates/vizia_core/src/binding/binding_view.rs` - Signal binding support
- [x] Store access to DataContext trait

---

## Core Views - Migrated to Signals

| View | File | Commit | Notes |
|------|------|--------|-------|
| Checkbox | `crates/vizia_core/src/views/checkbox.rs` | 83f19f92 | |
| Collapsible | `crates/vizia_core/src/views/collapsible.rs` | 92f2764e | |
| Combobox | `crates/vizia_core/src/views/combobox.rs` | ec57e331 | Fixed signal assignment bugs |
| Textbox | `crates/vizia_core/src/views/textbox.rs` | 7b87d38d | Now requires `Signal<L>` |
| ToggleButton | `crates/vizia_core/src/views/toggle_button.rs` | 8614d7dd | |
| Tooltip | `crates/vizia_core/src/views/tooltip.rs` | 28c1f4cc | |

---

## Core Views - Pending Migration

### High Priority (Simple State)

These views have simple boolean or numeric state that maps directly to `Signal<T>`:

| View | File | State Type | Notes |
|------|------|------------|-------|
| Switch | `crates/vizia_core/src/views/switch.rs` | `Lens<bool>` | Similar to Checkbox |
| Radio | `crates/vizia_core/src/views/radio.rs` | `Lens<T: PartialEq>` | Selection state |
| Progressbar | `crates/vizia_core/src/views/progressbar.rs` | `Lens<f32>` | Progress value |
| Rating | `crates/vizia_core/src/views/rating.rs` | `Lens<u32>` | Rating value |

### Medium Priority (Numeric/Range State)

These have range-based or numeric state:

| View | File | State Type | Notes |
|------|------|------------|-------|
| Slider | `crates/vizia_core/src/views/slider.rs` | `Lens<f32>` | Range value |
| Knob | `crates/vizia_core/src/views/knob.rs` | `Lens<f32>` | Rotary control |
| Spinbox | `crates/vizia_core/src/views/spinbox.rs` | `Lens<T: Num>` | Numeric input |
| Scrollbar | `crates/vizia_core/src/views/scrollbar.rs` | `Lens<f32>` | Scroll position |
| XYPad | `crates/vizia_core/src/views/xypad.rs` | `Lens<(f32,f32)>` | 2D position |

### Lower Priority (Complex State)

These have more complex state or dependencies:

| View | File | State Type | Notes |
|------|------|------------|-------|
| Dropdown | `crates/vizia_core/src/views/dropdown.rs` | `Lens<T>` | Selection + open state |
| Picklist | `crates/vizia_core/src/views/picklist.rs` | `Lens<T>` | List selection |
| Datepicker | `crates/vizia_core/src/views/datepicker.rs` | `Lens<Date>` | **Blocked**: See Known Issues |
| TabView | `crates/vizia_core/src/views/tabview.rs` | `Lens<usize>` | Tab index |
| List | `crates/vizia_core/src/views/list.rs` | `Lens<Vec<T>>` | List data |
| VirtualList | `crates/vizia_core/src/views/virtual_list.rs` | `Lens<Vec<T>>` | Virtualized list |
| ScrollView | `crates/vizia_core/src/views/scrollview.rs` | Internal | Scroll position |
| ResizableStack | `crates/vizia_core/src/views/resizable_stack.rs` | `Lens<Vec<f32>>` | Resize state |
| Menu | `crates/vizia_core/src/views/menu.rs` | Internal | Menu state |
| Popup | `crates/vizia_core/src/views/popup.rs` | Internal | Popup visibility |
| Label | `crates/vizia_core/src/views/label.rs` | `Lens<impl ToString>` | Text content |
| Badge | `crates/vizia_core/src/views/badge.rs` | `Lens<impl ToString>` | Badge content |
| Chip | `crates/vizia_core/src/views/chip.rs` | Multiple | Chip state |
| Element | `crates/vizia_core/src/views/element.rs` | Generic | Base element |

---

## Examples - Migrated to Signals

| Example | File | Notes |
|---------|------|-------|
| Counter (Signal variant) | `examples/7GUIs/counter_signal.rs` | New signal-based counter |
| App State | `examples/app_state.rs` | App trait demonstration |
| Signal Map Demo | `examples/signal_map_demo.rs` | Signal mapping patterns |
| Checkbox | `examples/views/checkbox.rs` | Updated to signals |
| Chip | `examples/views/chip.rs` | Updated to signals |
| Collapsible | `examples/views/collapsible.rs` | Updated to signals |
| Label | `examples/views/label.rs` | Updated to signals |
| Textbox | `examples/views/textbox.rs` | Updated to signals |
| Toggle Button | `examples/views/toggle_button.rs` | Updated to signals |
| Tooltip | `examples/views/tooltip.rs` | Updated to signals |

---

## Examples - Pending Migration

Located in `examples/views/`:

- dropdown.rs, helpers/mod.rs, knob.rs, list.rs, markdown.rs
- picklist.rs, progressbar.rs, radiobutton.rs, rating.rs
- resizable_stack.rs, scrollview.rs, slider.rs, spinbox.rs
- svg.rs, switch.rs, tabview.rs, virtual_list.rs, xypad.rs

Located in `examples/`:

- 7GUIs/ (most examples except counter_signal.rs)
- todo/src/main.rs
- widget_gallery/ (all views)
- Various standalone examples using Lens

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
    fn on_build(self, cx: &mut Context) -> Self {
        Checkbox::new(cx, self.value);
        self
    }

    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        // Direct signal updates also supported:
        // self.value.update(cx, |v| *v = !*v);
    }
}
```

---

## Known Issues / Blockers

### Datepicker Year Input (Temporary Workaround)

**Issue**: `Textbox::new` now requires `Signal<L>` but Datepicker uses internal Lenses for state management.

**Workaround Applied**: Replaced `Textbox` with `Label` for year display. Year can still be changed via increment/decrement buttons, but direct text editing is temporarily disabled.

**Proper Fix**: Migrate Datepicker to use Signals for internal state, then restore Textbox for year input.

**Location**: `crates/vizia_core/src/views/datepicker.rs:135-142`

### Combobox Signal Assignment (Fixed)

**Issue**: Combobox had Signal fields but used direct assignment (`self.is_open = false`) instead of signal methods.

**Fix Applied**:
- Changed direct assignments to use `signal.set(cx, value)`
- Used `handle.modify2()` instead of `handle.modify()` for signal updates in bind closures
- Cloned filter text string to avoid borrow escaping closure

**Location**: `crates/vizia_core/src/views/combobox.rs`

---

## Next Steps

1. **Continue Component Migration**: Prioritize Switch, Radio, Progressbar, Rating (simple boolean/numeric state)
2. **Migrate Slider/Knob/Spinbox**: Range-based numeric controls
3. **Complex Components**: Dropdown, Picklist, TabView, List views
4. **Examples**: Update remaining examples as components are migrated
5. **Cleanup**: Remove dead Lens code once all consumers migrated

---

## Notes

- Lens infrastructure must remain until all consumers are migrated
- Migration is staged; both systems coexist during transition
- Refer to `SIGNALS_PROPOSAL.md` for architectural direction
