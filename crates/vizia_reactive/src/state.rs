use std::{any::Any, collections::HashSet, rc::Rc, sync::Arc};

#[cfg(debug_assertions)]
use std::panic::Location;

use parking_lot::{Mutex, MutexGuard};

use crate::{
    id::Id,
    runtime::{RUNTIME, Runtime},
    signal::{TrackedRef, TrackedRefCell},
    sync_runtime::{SYNC_RUNTIME, SyncSignal as RuntimeSyncSignal},
};

/// Internal state for a signal; stores the value and subscriber set.
#[derive(Clone)]
pub(crate) struct SignalState {
    pub(crate) id: Id,
    pub(crate) value: SignalValue,
    pub(crate) subscribers: Arc<Mutex<HashSet<Id>>>,
}

#[derive(Clone)]
pub(crate) enum SignalValue {
    Sync(Arc<dyn Any + Send + Sync>),
    Local(Rc<dyn Any>),
}

#[allow(dead_code)]
pub enum SignalBorrow<'a, T> {
    Sync(MutexGuard<'a, T>),
    Local(TrackedRef<'a, T>),
}

impl SignalState {
    #[allow(clippy::new_ret_no_self)]
    #[cfg_attr(debug_assertions, track_caller)]
    pub fn new<T>(value: T) -> Id
    where
        T: Any + 'static,
    {
        Runtime::assert_ui_thread();
        let id = Id::next();
        let value = TrackedRefCell::new(value);
        let signal = SignalState {
            id,
            subscribers: Arc::new(Mutex::new(HashSet::new())),
            value: SignalValue::Local(Rc::new(value)),
        };
        id.add_signal(signal);
        id
    }

    pub fn new_sync<T>(value: T) -> Id
    where
        T: Any + Send + Sync + 'static,
    {
        let id = Id::next();
        let value = Arc::new(Mutex::new(value));
        let subscribers = Arc::new(Mutex::new(HashSet::new()));
        // Sync signals live in the global sync runtime; we do not store them in the TLS runtime.
        SYNC_RUNTIME.insert_signal(id, RuntimeSyncSignal { id, value, subscribers });
        id
    }

    #[allow(dead_code)]
    #[cfg_attr(debug_assertions, track_caller)]
    pub fn borrow<T: 'static>(&self) -> SignalBorrow<'_, T> {
        match &self.value {
            SignalValue::Sync(v) => {
                let v = v.as_ref().downcast_ref::<Mutex<T>>().expect("to downcast signal type");
                SignalBorrow::Sync(v.lock())
            }
            SignalValue::Local(v) => {
                let v = v
                    .as_ref()
                    .downcast_ref::<TrackedRefCell<T>>()
                    .expect("to downcast signal type");
                #[cfg(debug_assertions)]
                {
                    SignalBorrow::Local(v.borrow_at(Location::caller()))
                }
                #[cfg(not(debug_assertions))]
                {
                    SignalBorrow::Local(v.borrow())
                }
            }
        }
    }

    #[cfg_attr(debug_assertions, track_caller)]
    pub(crate) fn get_untracked<T: Clone + 'static>(&self) -> T {
        match &self.value {
            SignalValue::Sync(v) => {
                let v = v.as_ref().downcast_ref::<Mutex<T>>().expect("to downcast signal type");
                v.lock().clone()
            }
            SignalValue::Local(v) => {
                let v = v
                    .as_ref()
                    .downcast_ref::<TrackedRefCell<T>>()
                    .expect("to downcast signal type");
                #[cfg(debug_assertions)]
                {
                    v.borrow_at(Location::caller()).clone()
                }
                #[cfg(not(debug_assertions))]
                {
                    v.borrow().clone()
                }
            }
        }
    }

    #[cfg_attr(debug_assertions, track_caller)]
    pub(crate) fn get<T: Clone + 'static>(&self) -> T {
        self.subscribe();
        self.get_untracked()
    }

    #[cfg_attr(debug_assertions, track_caller)]
    pub(crate) fn with_untracked<O, T: 'static>(&self, f: impl FnOnce(&T) -> O) -> O {
        match &self.value {
            SignalValue::Sync(v) => {
                let v = v.as_ref().downcast_ref::<Mutex<T>>().expect("to downcast signal type");
                f(&v.lock())
            }
            SignalValue::Local(v) => {
                let v = v
                    .as_ref()
                    .downcast_ref::<TrackedRefCell<T>>()
                    .expect("to downcast signal type");
                #[cfg(debug_assertions)]
                {
                    f(&v.borrow_at(Location::caller()))
                }
                #[cfg(not(debug_assertions))]
                {
                    f(&v.borrow())
                }
            }
        }
    }

    #[cfg_attr(debug_assertions, track_caller)]
    pub(crate) fn with<O, T: 'static>(&self, f: impl FnOnce(&T) -> O) -> O {
        self.subscribe();
        self.with_untracked(f)
    }

    pub(crate) fn update_value_sync<U, T: Send + Sync + 'static>(
        &self,
        f: impl FnOnce(&mut T) -> U,
    ) -> U {
        let value = self.as_sync::<T>();
        let mut guard = value.lock();
        let result = f(&mut *guard);
        drop(guard);
        self.run_effects();
        result
    }

    #[cfg_attr(debug_assertions, track_caller)]
    pub(crate) fn update_value_local<U, T: 'static>(&self, f: impl FnOnce(&mut T) -> U) -> U {
        let value = self.as_local::<T>();
        #[cfg(debug_assertions)]
        let mut guard = value.borrow_mut_at(Location::caller());
        #[cfg(not(debug_assertions))]
        let mut guard = value.borrow_mut();
        let result = f(&mut *guard);
        drop(guard);
        self.run_effects();
        result
    }

    pub(crate) fn subscriber_ids(&self) -> HashSet<Id> {
        self.subscribers.lock().iter().copied().collect()
    }

    pub(crate) fn run_effects(&self) {
        let ids: smallvec::SmallVec<[_; 3]> = self.subscriber_ids().into_iter().collect();
        let on_ui_thread = Runtime::is_ui_thread();

        if !on_ui_thread {
            SYNC_RUNTIME.enqueue_effects(ids);
            return;
        }

        RUNTIME.with(|r| {
            for id in &ids {
                r.add_pending_effect(*id);
            }
            if !r.batching.get() {
                r.run_pending_effects();
            }
        });
    }

    pub(crate) fn subscribe(&self) {
        RUNTIME.with(|runtime| {
            if let Some(effect) = runtime.current_effect.borrow().as_ref() {
                self.subscribers.lock().insert(effect.id());
                effect.add_observer(self.id);
            }
        });
    }

    pub(crate) fn as_sync<T: Send + Sync + 'static>(&self) -> Arc<Mutex<T>> {
        match &self.value {
            SignalValue::Sync(v) => {
                v.clone().downcast::<Mutex<T>>().expect("to downcast signal type")
            }
            SignalValue::Local(_) => unreachable!("expected sync signal storage"),
        }
    }

    pub(crate) fn as_local<T: 'static>(&self) -> Rc<TrackedRefCell<T>> {
        match &self.value {
            SignalValue::Local(v) => {
                v.clone().downcast::<TrackedRefCell<T>>().expect("to downcast signal type")
            }
            SignalValue::Sync(_) => unreachable!("expected local signal storage"),
        }
    }
}
