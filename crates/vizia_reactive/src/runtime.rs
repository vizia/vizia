use std::{
    any::{Any, TypeId},
    cell::{Cell, RefCell},
    cmp::Reverse,
    collections::{HashMap, HashSet},
    rc::Rc,
    sync::{
        Mutex, OnceLock,
        atomic::{AtomicBool, Ordering},
    },
    thread::{self, ThreadId},
};

use smallvec::SmallVec;

use crate::{
    effect::{EffectPriority, EffectTrait, run_effect},
    id::Id,
    state::SignalState,
    sync_runtime::SYNC_RUNTIME,
};

/// Type alias for context storage within a scope.
pub(crate) type ScopeContexts = HashMap<TypeId, Box<dyn Any>>;

thread_local! {
pub static RUNTIME: Runtime = Runtime::new();
}

static UI_THREAD_REGISTRY: OnceLock<Mutex<HashSet<ThreadId>>> = OnceLock::new();
#[cfg(debug_assertions)]
static UI_THREAD_SET_LOCATION: OnceLock<
    Mutex<HashMap<ThreadId, &'static std::panic::Location<'static>>>,
> = OnceLock::new();
static ENFORCE_UI_THREAD: AtomicBool = AtomicBool::new(false);

fn ui_thread_registry() -> &'static Mutex<HashSet<ThreadId>> {
    UI_THREAD_REGISTRY.get_or_init(|| Mutex::new(HashSet::new()))
}

#[cfg(debug_assertions)]
fn ui_thread_locations() -> &'static Mutex<HashMap<ThreadId, &'static std::panic::Location<'static>>>
{
    UI_THREAD_SET_LOCATION.get_or_init(|| Mutex::new(HashMap::new()))
}

/// The internal reactive Runtime which stores all the reactive system states in a
/// thread local.
pub struct Runtime {
    pub(crate) current_effect: RefCell<Option<Rc<dyn EffectTrait>>>,
    pub(crate) current_scope: RefCell<Id>,
    pub(crate) children: RefCell<HashMap<Id, HashSet<Id>>>,
    pub(crate) parents: RefCell<HashMap<Id, Id>>,
    pub(crate) signals: RefCell<HashMap<Id, SignalState>>,
    pub(crate) effects: RefCell<HashMap<Id, Rc<dyn EffectTrait>>>,
    pub(crate) scope_contexts: RefCell<HashMap<Id, ScopeContexts>>,
    pub(crate) batching: Cell<bool>,
    pub(crate) pending_effects: RefCell<SmallVec<[Id; 10]>>,
    pub(crate) pending_effects_set: RefCell<HashSet<Id>>,
}

impl Default for Runtime {
    fn default() -> Self {
        Self::new()
    }
}

impl Runtime {
    pub(crate) fn new() -> Self {
        Self {
            current_effect: RefCell::new(None),
            current_scope: RefCell::new(Id::next()),
            children: RefCell::new(HashMap::new()),
            parents: RefCell::new(HashMap::new()),
            signals: Default::default(),
            effects: Default::default(),
            scope_contexts: Default::default(),
            batching: Cell::new(false),
            pending_effects: RefCell::new(SmallVec::new()),
            pending_effects_set: RefCell::new(HashSet::new()),
        }
    }

    /// Register the current thread as a UI thread.
    ///
    /// Backends should call this when a UI event loop/window starts on a thread.
    #[cfg_attr(debug_assertions, track_caller)]
    pub fn init_on_ui_thread() {
        let current = thread::current().id();
        {
            let mut registry = ui_thread_registry().lock().unwrap_or_else(|e| e.into_inner());
            registry.insert(current);
        }
        #[cfg(debug_assertions)]
        {
            let caller = std::panic::Location::caller();
            let mut locations = ui_thread_locations().lock().unwrap_or_else(|e| e.into_inner());
            locations.entry(current).or_insert(caller);
        }
        ENFORCE_UI_THREAD.store(true, Ordering::Relaxed);
    }

    /// Unregister the current thread as a UI thread.
    ///
    /// Backends should call this when a UI event loop/window stops on a thread.
    pub fn deinit_on_ui_thread() {
        let current = thread::current().id();
        let mut registry = ui_thread_registry().lock().unwrap_or_else(|e| e.into_inner());

        if registry.remove(&current) {
            #[cfg(debug_assertions)]
            {
                let mut locations = ui_thread_locations().lock().unwrap_or_else(|e| e.into_inner());
                locations.remove(&current);
            }
        }

        if registry.is_empty() {
            ENFORCE_UI_THREAD.store(false, Ordering::Relaxed);
        }
    }

    #[cfg_attr(debug_assertions, track_caller)]
    pub fn assert_ui_thread() {
        if !ENFORCE_UI_THREAD.load(Ordering::Relaxed) {
            return;
        }

        let current = thread::current().id();
        let registry = ui_thread_registry().lock().unwrap_or_else(|e| e.into_inner());
        if !registry.contains(&current) {
            #[cfg(debug_assertions)]
            {
                let caller = std::panic::Location::caller();
                let locations = ui_thread_locations().lock().unwrap_or_else(|e| e.into_inner());
                let expected = registry
                    .iter()
                    .map(|id| {
                        let set_info = locations
                            .get(id)
                            .map(|loc| format!(" (registered at {}:{})", loc.file(), loc.line()))
                            .unwrap_or_default();
                        format!("{:?}{}", id, set_info)
                    })
                    .collect::<Vec<_>>()
                    .join(", ");
                panic!(
                    "Unsync runtime access from non-UI thread\n  expected one of UI threads: [{}]\n  current thread: {:?}\n  caller: {}:{}",
                    expected,
                    current,
                    caller.file(),
                    caller.line(),
                );
            }
            #[cfg(not(debug_assertions))]
            {
                panic!(
                    "Unsync runtime access from non-UI thread: current {:?} is not a registered UI thread",
                    current
                );
            }
        }
    }

    pub fn is_ui_thread() -> bool {
        if !ENFORCE_UI_THREAD.load(Ordering::Relaxed) {
            true
        } else {
            ui_thread_registry()
                .lock()
                .unwrap_or_else(|e| e.into_inner())
                .contains(&thread::current().id())
        }
    }

    pub(crate) fn register_effect(&self, effect: &Rc<dyn EffectTrait>) {
        self.effects.borrow_mut().insert(effect.id(), effect.clone());
    }

    pub(crate) fn remove_effect(&self, id: Id) {
        self.effects.borrow_mut().remove(&id);
    }

    pub(crate) fn get_effect(&self, id: Id) -> Option<Rc<dyn EffectTrait>> {
        self.effects.borrow().get(&id).cloned()
    }

    pub(crate) fn add_pending_effect(&self, effect_id: Id) {
        let mut set = self.pending_effects_set.borrow_mut();
        if set.insert(effect_id) {
            self.pending_effects.borrow_mut().push(effect_id);
        }
    }

    pub(crate) fn run_pending_effects(&self) {
        loop {
            let mut pending_effects = self.pending_effects.take();
            if pending_effects.is_empty() {
                break;
            }
            pending_effects.sort_by_key(|id| {
                let priority = self
                    .get_effect(*id)
                    .map(|effect| effect.priority())
                    .unwrap_or(EffectPriority::Normal);
                (Reverse(priority), *id)
            });
            for effect_id in pending_effects {
                self.pending_effects_set.borrow_mut().remove(&effect_id);
                if let Some(effect) = self.get_effect(effect_id) {
                    run_effect(effect);
                }
            }
        }
    }

    /// Drain any queued work from the sync runtime and run pending UI effects.
    pub fn drain_pending_work() {
        Runtime::assert_ui_thread();
        let pending_effects = SYNC_RUNTIME.take_pending_effects();
        let pending_disposals = SYNC_RUNTIME.take_pending_disposals();
        RUNTIME.with(|runtime| {
            for id in pending_effects {
                runtime.add_pending_effect(id);
            }
            for id in pending_disposals {
                id.dispose();
            }
            runtime.run_pending_effects();
        });
    }

    /// Returns true if there is queued work for this runtime or the sync runtime.
    pub fn has_pending_work() -> bool {
        RUNTIME.with(|runtime| !runtime.pending_effects.borrow().is_empty())
            || SYNC_RUNTIME.has_pending_effects()
            || SYNC_RUNTIME.has_pending_disposals()
    }

    /// Set a waker that will be called when a sync signal is updated off the UI thread.
    /// The waker should nudge the UI event loop (e.g., by sending a proxy event).
    pub fn set_sync_effect_waker(waker: impl Fn() + Send + Sync + 'static) {
        SYNC_RUNTIME.set_waker(waker);
    }

    pub fn get_current_effect() -> Option<Rc<dyn EffectTrait>> {
        RUNTIME.with(|rt| rt.current_effect.borrow().clone())
    }

    pub fn set_current_effect(effect: Option<Rc<dyn EffectTrait>>) {
        RUNTIME.with(|rt| *rt.current_effect.borrow_mut() = effect);
    }

    pub fn with_effect<T>(effect: Option<Rc<dyn EffectTrait>>, f: impl FnOnce() -> T) -> T {
        struct EffectRestoreGuard(Option<Rc<dyn EffectTrait>>);

        impl Drop for EffectRestoreGuard {
            fn drop(&mut self) {
                Runtime::set_current_effect(self.0.clone());
            }
        }

        let saved_effect = Runtime::get_current_effect();
        let _restore = EffectRestoreGuard(saved_effect);
        Runtime::set_current_effect(effect);
        f()
    }
}
