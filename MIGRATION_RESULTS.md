# ✅ Core Signal System (Proposal Lines 101-114)


  | Feature                    | Proposed | Current Status                                 |
  |----------------------------|----------|------------------------------------------------|
  | cx.state(initial_value)    | ✅       | Implemented in recoil/mod.rs                   |
  | signal.update(cx, |v| ...) | ✅       | Implemented                                    |
  | signal.set(cx, value)      | ✅       | Implemented                                    |
  | cx.derived(closure)        | ✅       | Implemented with automatic dependency tracking. Prefer `signal.drv(cx, \|v, s\| ...)` |
  | Automatic change detection | ✅       | Built into Signal internals                    |


# ✅ Simplified API (Lines 236-248)


  | Feature                     | Proposed | Current Status                          |
  |-----------------------------|----------|-----------------------------------------|
  | No #[derive(Lens)]          | ✅       | Lens infrastructure fully removed       |
  | No #[derive(Data)] required | ✅       | Only needed for complex equality checks |
  | No Model trait required     | ✅       | Views use View::event directly          |


# ✅ Derived State & Multi-State (Lines 250-312)

Works exactly as proposed - multiple signals compose naturally. Prefer `signal.drv(cx, |v, s| ...)` for ergonomics; `cx.derived()` also available.

```rust
let welcome_message = cx.derived(move |s| {
    if *self.is_logged_in.get(s) {
        format!("Welcome {}! You have {} messages",
            self.user_name.get(s),
            self.message_count.get(s))
    } else {
        "Please log in".to_string()
    }
});
```


# ✅ Alternative: Direct Signal Updates (Lines 194-232)

Proposed: Direct signal updates without events for simple cases
Implemented: Works exactly as proposed - both patterns supported!

```rust
// Direct updates - no events needed
Button::new(cx, |cx| Label::new(cx, "Increment"))
    .on_press(move |cx| self.count.update(cx, |v| *v += 1));

// Or with events - still works
Button::new(cx, |cx| Label::new(cx, "Increment"))
    .on_press(|cx| cx.emit(CounterEvent::Increment));
```

Both approaches coexist - developers choose the appropriate pattern for each use case.


# ✅ Application-Level State (Lines 313-355)


  Proposed: Application trait pattern with MyApp::run()
  Implemented: Full `App` trait with `MyApp::run()` pattern - exactly as proposed! Plus exposed window property config:

  **App trait pattern w/ static window config:**
  ```rust
  struct Counter {
      count: Signal<i32>,
  }

  impl App for Counter {
      fn new(cx: &mut Context) -> Self {
          Self { count: cx.state(0) }
      }

      fn on_build(self, cx: &mut Context) -> Self {
          // Build UI...
          self
      }

      fn window_config(&self) -> WindowConfig {
          window(|app| app.title("Counter").inner_size((800, 600)))
      }
  }

  fn main() -> Result<(), ApplicationError> { Counter::run() }
  ```

  **App trait pattern w/ reactive window config:**
  ```rust
  struct Counter {
      count: Signal<i32>,
      title: Signal<String>,
      size: Signal<(u32, u32)>,
  }

  impl App for Counter {
      fn new(cx: &mut Context) -> Self {
          Self {
              count: cx.state(0),
              title: cx.state(String::from("Counter: 0")),
              size: cx.state((800, 600)),
          }
      }

      fn on_build(self, cx: &mut Context) -> Self {
          // Build UI...
          self
      }

      fn window_config(&self) -> WindowConfig {
          let title = self.title;
          let size = self.size;
          window(move |app| {
              app.title(title)
                 .inner_size(size)
          })
      }
  }

  fn main() -> Result<(), ApplicationError> { Counter::run() }
  ```


# ✅ Migration Phases (Lines 417-435)



  We completed all three phases:

- Phase 1: ~~Parallel implementation~~ (skipped)
- Phase 2: ~~Gradual migration~~ (done rapidly)
- Phase 3: ✅ Full transition - lens removed, all views migrated



---

# ⬆️ Improvements Beyond Proposal


  | Enhancement                   | Description                                                      |
  |-------------------------------|------------------------------------------------------------------|
  | Unified .keyed() API          | Opt-in keyed diffing for List/TabView/PickList performance       |
  | Binding scope cleanup         | Automatic signal lifetime management via disposable scope entity |
  | .two_way() convenience        | Auto-wires on_change to update bound signal                      |
  | App::window_config()          | Reactive window properties with access to signals                |
  | window() helper               | Creates WindowConfig closure for App trait                       |
  | DrawContext cache pattern     | Cache signals for draw() since DrawContext isn't DataContext     |
  | Timer integration             | cx.modify_timer() for dynamic intervals                          |
  | Signal::try_get()             | Safe access returning Option for potentially stale signals       |
  | Async<T, E>                   | Async state with cancel/timeout/retry/TTL/stale-while-revalidate |


# ✅ Advantages of Signals (Lines 357-393)

| Proposed Advantage          | Status | Implementation                                      |
|-----------------------------|--------|-----------------------------------------------------|
| Reduced Boilerplate         | ✅     | No Lens/Data derives, no Model trait required       |
| Better Performance          | ✅     | Automatic dependency tracking, efficient updates    |
| Intuitive Mental Model      | ✅     | Direct state access, clear ownership                |
| Composability               | ✅     | Multiple signals combine naturally in derived()     |
| Type Safety                 | ✅     | Strong typing preserved, clear borrowing semantics  |
| Local State Management      | ✅     | Views have own state, no global state for simple cases |
| Application-Level State     | ✅     | App trait with window_config() for global state     |


# ✅ Disadvantages Addressed (Lines 395-415)

| Proposed Disadvantage       | Status | How Addressed                                       |
|-----------------------------|--------|-----------------------------------------------------|
| Learning Curve              | ⚠️     | Mitigated by familiar View-like patterns            |
| Memory Overhead             | ✅     | Acceptable - signals are lightweight handles        |
| Debugging Complexity        | ✅     | Standard Rust debugging; try_get() for safe access  |
| Migration Cost              | ✅     | Completed - all views migrated                      |
| Runtime Dependency Tracking | ✅     | Efficient implementation in recoil::Store           |


# ✅ Signal Mapping Ergonomics

**Implemented:** `Signal::drv()` method for concise derived signal creation.

```rust
// Instead of:
let count = self.count;
let doubled = cx.derived(move |s| count.get(s) * 2);
let label = cx.derived(move |s| format!("Count: {}", count.get(s)));

// You can now write:
let doubled = self.count.drv(cx, |v| v * 2);
let label = self.count.drv(cx, |v| format!("Count: {v}"));
```

The closure receives `&T` (the dereferenced value), not the store. This eliminates the need to capture the signal and call `.get(s)`.

---

# 💡 Future API Ideas (Under Consideration)

## Accessibility Improvements

Many views already set appropriate roles (Button, Checkbox, Slider, etc. - see `crates/vizia_core/src/views/`). Potential improvements:

- Auto-generate `live_region` announcements for value changes in controls
- Consider default `name()` bindings based on nearby Label associations
- Explore auto-announcing state changes (e.g., "Checkbox checked")

---

# 📚 API Reference Notes

## Event Handling Methods

Three methods for handling events in `View::event()`:

| Method | Signature | Behavior |
|--------|-----------|----------|
| `event.map()` | `fn(&M, &mut EventMeta)` | Borrows message, does NOT auto-consume |
| `event.take()` | `fn(M, &mut EventMeta)` | Takes ownership, auto-consumes event |
| `meta.consume()` | `fn()` | Manually stop propagation |

**Usage**:
- Use `map()` when multiple handlers might need the event, manually `consume()` when done
- Use `take()` when this handler is the final destination

```rust
// map() - doesn't auto-consume, message borrowed
event.map(|window_event, meta| match window_event {
    WindowEvent::KeyDown(code, _) => {
        // Handle key...
        meta.consume(); // Manual consume
    }
    _ => {}
});

// take() - auto-consumes, message owned
event.take(|app_event, _meta| match app_event {
    AppEvent::Increment => count.update(cx, |v| *v += 1),
    AppEvent::Decrement => count.update(cx, |v| *v -= 1),
});
```

## Auto-Binding via Res<T>

When a modifier accepts `impl Res<T>`, it automatically handles both values and signals:

- **Plain values**: Applied immediately, no binding created
- **Signals**: Creates internal `Binding` that updates on signal change

```rust
// Both work - signal auto-binds, value is static
label.width(Pixels(100.0));      // Static
label.width(width_signal);        // Reactive, auto-binds
```

Use `Binding::new()` only when you need to **rebuild view structure** (add/remove children) based on signal changes.

## Hot Reload

Press **F5** in debug builds to reload stylesheets. Implementation: `crates/vizia_core/src/events/event_manager.rs:525-527`.

---

# SUMMARY

  The implementation covers everything in the proposal and adds significant improvements. The `App` trait now matches the proposal exactly with `MyApp::run()` pattern, plus we added `window_config()` for reactive window properties. The restored `Res<T>` trait brings greater ergonomics and the keyed API + binding scope cleanup address performance/lifetime concerns not originally considered.

---

# 📋 Future Work (Lines 440-445)

These were explicitly listed as future possibilities in the original proposal:

- ~~Async state management~~ ✅ **Implemented** - `Async<T, E>` with load/cancel/refresh/retry/timeout/TTL
- Time-travel debugging
- State persistence
- Undo/redo functionality *(manual pattern demonstrated in `circle_drawer.rs`; framework support TBD)*

---

## 🔮 Time-Travel Debugging

Record signal state changes over time, allowing developers to step backwards/forwards through history and inspect state at any point.

### API Approaches

**Approach 1: Global Automatic Recording**
```rust
Application::new(|cx| { ... })
    .enable_time_travel()
    .run();

// Navigate
cx.time_travel_back();
cx.time_travel_forward();
cx.time_travel_to(index);
cx.time_travel_history();  // Vec<StateSnapshot>
```
*Pros*: Simple, captures everything
*Cons*: Memory overhead, noisy with high-frequency updates (animations, mouse position)

**Approach 2: Selective Recording**
```rust
let important = cx.state_recorded(0);   // Recorded
let ephemeral = cx.state(0.0);          // Not recorded

signal.enable_recording(cx);
signal.disable_recording(cx);
```
*Pros*: Focused debugging, less memory
*Cons*: May miss related state changes

**Approach 3: Explicit Snapshots**
```rust
let snap = cx.snapshot();
cx.restore(snap);

cx.snapshot_named("before_submit");
cx.restore_named("before_submit");
```
*Pros*: Predictable, explicit control
*Cons*: Not true "time travel", requires forethought

**Approach 4: Action Log (Redux-style)**
```rust
#[derive(TimeTravel)]
enum AppAction { Increment, SetUser(User) }

cx.dispatch(AppAction::Increment);  // Logged with resulting state
cx.replay_from(action_index);
```
*Pros*: Shows causality ("why did this change?")
*Cons*: More boilerplate, requires discipline

### Implementation Considerations

| Concern | Options |
|---------|---------|
| Storage | Full state copies vs deltas (deltas more memory-efficient) |
| Side effects | Skip async/timers on replay, or re-execute? |
| UI | DevTools panel with slider, step buttons, diff view |
| Memory limit | Ring buffer (last N changes), or explicit clear |
| Serialization | Save/load history for sharing bug reports |
| Granularity | Every `set()`/`update()`, or batch per frame? |

### Suggested Hybrid
```rust
impl App for MyApp {
    fn debug_config(&self) -> DebugConfig {
        DebugConfig::default()
            .time_travel(true)
            .max_history(1000)
            .exclude_signals(vec![self.mouse_pos, self.animation_t])
    }
}

let transient = cx.state_transient(0.0);  // Never recorded
cx.checkpoint("user_logged_in");          // Named milestone
```

---

## 💾 State Persistence

Save and restore signal state across app restarts (localStorage equivalent for desktop apps).

### API Approaches

**Approach 1: Declarative Persistence**
```rust
let settings = cx.state_persistent("app.settings", Settings::default());
// Auto-saves on change, auto-loads on startup
```
*Pros*: Zero boilerplate for simple cases
*Cons*: Implicit I/O, unclear when saves happen

**Approach 2: Explicit Save/Load**
```rust
let settings = cx.state(Settings::default());

// Manual control
cx.save_state("settings", &settings);
cx.load_state("settings", &mut settings);

// Or batch
cx.save_all_persistent();
cx.load_all_persistent();
```
*Pros*: Clear control flow, predictable
*Cons*: More code, easy to forget

**Approach 3: Session-based**
```rust
impl App for MyApp {
    fn persist(&self) -> Vec<(&str, &dyn Persist)> {
        vec![
            ("window_size", &self.size),
            ("theme", &self.theme),
            ("recent_files", &self.recent),
        ]
    }
}
// Auto-called on close, restored on open
```
*Pros*: Centralized, automatic lifecycle
*Cons*: Requires trait impl on all persisted types

### Implementation Considerations

| Concern | Options |
|---------|---------|
| Format | JSON, MessagePack, bincode, RON |
| Location | XDG config dir, app data folder, custom path |
| Versioning | Schema migrations when app updates |
| Encryption | Optional for sensitive data |
| Debouncing | Don't save on every keystroke - batch saves |
| Errors | What if file is corrupted? Fallback to defaults? |

### Suggested Hybrid
```rust
// Simple cases - auto-persist
let theme = cx.state_persistent("theme", Theme::Dark);

// Complex cases - manual with derive
#[derive(Persist)]
struct AppSettings { ... }

impl App for MyApp {
    fn on_close(&self, cx: &mut Context) {
        cx.persist("settings", &self.settings);
    }

    fn on_open(&mut self, cx: &mut Context) {
        if let Some(s) = cx.restore("settings") {
            self.settings.set(cx, s);
        }
    }
}
```

---

## ↩️ Undo/Redo Functionality

Allow users to undo/redo state changes, common in editors, drawing apps, form workflows.

> **Note**: `examples/7GUIs/circle_drawer.rs` already demonstrates manual undo/redo using the command pattern with `Signal<Vec<UndoRedoAction>>` for undo/redo stacks. The approaches below explore framework-level support to reduce boilerplate.

### API Approaches

**Approach 1: Global Undo Stack**
```rust
cx.enable_undo();

// All signal changes automatically tracked
button.on_press(|cx| {
    cx.begin_undo_group("Change Color");
    color.set(cx, new_color);
    cx.end_undo_group();
});

cx.undo();  // Ctrl+Z
cx.redo();  // Ctrl+Shift+Z
```
*Pros*: Works automatically once enabled
*Cons*: May capture unwanted changes (hover states, selections)

**Approach 2: Selective Undo Signals**
```rust
let document = cx.state_undoable(Document::new());  // Tracked
let ui_state = cx.state(UiState::default());        // Not tracked

document.set_undoable(cx, "Edit text", new_doc);
```
*Pros*: Precise control over what's undoable
*Cons*: Must explicitly mark each undoable operation

**Approach 3: Command Pattern**
```rust
trait UndoableCommand {
    fn execute(&self, cx: &mut EventContext);
    fn undo(&self, cx: &mut EventContext);
    fn description(&self) -> &str;
}

struct SetColor { old: Color, new: Color, target: Signal<Color> }

impl UndoableCommand for SetColor {
    fn execute(&self, cx: &mut EventContext) { self.target.set(cx, self.new); }
    fn undo(&self, cx: &mut EventContext) { self.target.set(cx, self.old); }
    fn description(&self) -> &str { "Change Color" }
}

cx.execute(SetColor { ... });
```
*Pros*: Full control, custom undo logic, mergeable commands
*Cons*: Significant boilerplate per action

**Approach 4: Snapshot Diffing**
```rust
cx.begin_transaction("Draw Shape");
// ... multiple signal changes ...
cx.commit_transaction();  // Snapshot delta saved

cx.undo();  // Restore previous snapshot
```
*Pros*: Groups multiple changes naturally
*Cons*: Memory for snapshots, no partial undo within transaction

### Implementation Considerations

| Concern | Options |
|---------|---------|
| Granularity | Per-signal-change, or grouped transactions? |
| Stack limit | Unlimited, or cap at N operations? |
| Merge | Combine rapid similar changes (typing in textbox)? |
| Branches | Linear stack, or tree (redo after undo + new change)? |
| Persistence | Save undo history with document? |
| UI | Show undo history list? Descriptions? |

### Suggested Hybrid
```rust
// Mark signals as undoable
let content = cx.state_undoable(String::new());

// Group related changes
cx.undo_group("Format Text", || {
    content.set(cx, formatted);
    style.set(cx, new_style);
});

// Keyboard shortcuts auto-wired
// Ctrl+Z -> cx.undo()
// Ctrl+Shift+Z / Ctrl+Y -> cx.redo()

// Access history for UI
let history = cx.undo_history();  // Vec<UndoEntry>
for entry in history {
    println!("{}: {}", entry.index, entry.description);
}
```

### Relationship with Time-Travel

Time-travel and undo/redo overlap but serve different purposes:

| Aspect | Time-Travel | Undo/Redo |
|--------|-------------|-----------|
| Purpose | Debugging | User feature |
| Scope | All signals | Selected "document" signals |
| Granularity | Every change | Grouped actions |
| UI | DevTools | App menu, Ctrl+Z |
| Persistence | Optional (bug reports) | Often saved with document |

Could share underlying infrastructure (state snapshots, delta storage) but expose different APIs.
