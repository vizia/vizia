use std::{
    cell::{Ref, RefCell, RefMut},
    fmt,
    marker::PhantomData,
};

#[cfg(debug_assertions)]
use std::cell::Cell;
#[cfg(debug_assertions)]
use std::panic::Location;

#[cfg(feature = "serde")]
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::{
    SignalGet, SignalUpdate,
    id::Id,
    read::{SignalRead, SignalTrack, SignalWith},
    runtime::Runtime,
    state::SignalState,
    write::SignalWrite,
};

#[derive(Debug)]
pub(crate) struct TrackedRefCell<T> {
    inner: RefCell<T>,
    #[cfg(debug_assertions)]
    shared_borrows: Cell<usize>,
    #[cfg(debug_assertions)]
    has_mut_borrow: Cell<bool>,
    #[cfg(debug_assertions)]
    holder: Cell<Option<&'static Location<'static>>>,
}

impl<T> TrackedRefCell<T> {
    #[cfg_attr(debug_assertions, track_caller)]
    pub(crate) fn new(value: T) -> Self {
        Self {
            inner: RefCell::new(value),
            #[cfg(debug_assertions)]
            shared_borrows: Cell::new(0),
            #[cfg(debug_assertions)]
            has_mut_borrow: Cell::new(false),
            #[cfg(debug_assertions)]
            holder: Cell::new(None),
        }
    }

    #[cfg_attr(debug_assertions, track_caller)]
    pub(crate) fn borrow(&self) -> TrackedRef<'_, T> {
        #[cfg(debug_assertions)]
        return self.borrow_at(Location::caller());
        #[cfg(not(debug_assertions))]
        return TrackedRef { inner: self.inner.borrow() };
    }

    #[cfg_attr(debug_assertions, track_caller)]
    pub(crate) fn borrow_mut(&self) -> TrackedRefMut<'_, T> {
        #[cfg(debug_assertions)]
        return self.borrow_mut_at(Location::caller());
        #[cfg(not(debug_assertions))]
        return TrackedRefMut { inner: self.inner.borrow_mut() };
    }

    #[cfg(debug_assertions)]
    pub(crate) fn borrow_at(&self, caller: &'static Location<'static>) -> TrackedRef<'_, T> {
        let inner = self.inner.try_borrow().unwrap_or_else(|_| self.panic_conflict(caller));
        let shared = self.shared_borrows.get();
        if shared == 0 && !self.has_mut_borrow.get() {
            self.holder.set(Some(caller));
        }
        self.shared_borrows.set(shared + 1);
        TrackedRef { inner, cell: self }
    }

    #[cfg(debug_assertions)]
    pub(crate) fn borrow_mut_at(&self, caller: &'static Location<'static>) -> TrackedRefMut<'_, T> {
        let inner = self.inner.try_borrow_mut().unwrap_or_else(|_| self.panic_conflict(caller));
        if self.shared_borrows.get() == 0 && !self.has_mut_borrow.get() {
            self.holder.set(Some(caller));
        }
        self.has_mut_borrow.set(true);
        TrackedRefMut { inner, cell: self }
    }

    #[cfg(debug_assertions)]
    fn release_shared(&self) {
        let shared = self.shared_borrows.get().saturating_sub(1);
        self.shared_borrows.set(shared);
        if shared == 0 && !self.has_mut_borrow.get() {
            self.holder.set(None);
        }
    }

    #[cfg(debug_assertions)]
    fn release_mut(&self) {
        self.has_mut_borrow.set(false);
        if self.shared_borrows.get() == 0 {
            self.holder.set(None);
        }
    }

    #[cfg(debug_assertions)]
    fn panic_conflict(&self, caller: &'static Location<'static>) -> ! {
        match self.holder.get() {
            Some(loc) => panic!(
                "signal value already borrowed at {}:{} (attempted at {}:{})",
                loc.file(),
                loc.line(),
                caller.file(),
                caller.line()
            ),
            None => panic!(
                "signal value already borrowed (attempted at {}:{})",
                caller.file(),
                caller.line()
            ),
        }
    }
}

pub struct TrackedRef<'a, T> {
    inner: Ref<'a, T>,
    #[cfg(debug_assertions)]
    cell: &'a TrackedRefCell<T>,
}

impl<'a, T> Drop for TrackedRef<'a, T> {
    fn drop(&mut self) {
        #[cfg(debug_assertions)]
        self.cell.release_shared();
    }
}

impl<'a, T> std::ops::Deref for TrackedRef<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

pub struct TrackedRefMut<'a, T> {
    inner: RefMut<'a, T>,
    #[cfg(debug_assertions)]
    cell: &'a TrackedRefCell<T>,
}

impl<'a, T> Drop for TrackedRefMut<'a, T> {
    fn drop(&mut self) {
        #[cfg(debug_assertions)]
        self.cell.release_mut();
    }
}

impl<'a, T> std::ops::Deref for TrackedRefMut<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<'a, T> std::ops::DerefMut for TrackedRefMut<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

/// A local (UI-thread) read write Signal.
pub struct Signal<T> {
    pub(crate) id: Id,
    pub(crate) ty: PhantomData<T>,
}

impl<T> Signal<T> {
    pub fn id(&self) -> Id {
        self.id
    }

    /// Create a Getter of this Signal
    pub fn read_only(&self) -> ReadSignal<T> {
        ReadSignal { id: self.id, ty: PhantomData }
    }

    /// Create a Setter of this Signal
    pub fn write_only(&self) -> WriteSignal<T> {
        WriteSignal { id: self.id, ty: PhantomData }
    }
}

impl<T> Copy for Signal<T> {}

impl<T> Clone for Signal<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Eq for Signal<T> {}

impl<T> PartialEq for Signal<T> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl<T> fmt::Debug for Signal<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = f.debug_struct("Signal");
        s.field("id", &self.id);
        s.field("ty", &self.ty);
        s.finish()
    }
}

impl<T> SignalTrack<T> for Signal<T> {
    fn id(&self) -> Id {
        self.id
    }
}

#[cfg(feature = "serde")]
impl<T> Serialize for Signal<T>
where
    T: Serialize + 'static,
{
    fn serialize<Se>(&self, serializer: Se) -> Result<Se::Ok, Se::Error>
    where
        Se: Serializer,
    {
        self.with_untracked(|value| value.serialize(serializer))
    }
}

#[cfg(feature = "serde")]
impl<'de, T> Deserialize<'de> for Signal<T>
where
    T: Deserialize<'de> + 'static,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = T::deserialize(deserializer)?;
        Ok(Signal::new(value))
    }
}

impl<T: Default + 'static> Default for Signal<T> {
    fn default() -> Self {
        Signal::new(T::default())
    }
}

impl<T: 'static> From<T> for Signal<T> {
    fn from(value: T) -> Self {
        Signal::new(value)
    }
}

impl<T: 'static> Signal<T> {
    #[cfg_attr(debug_assertions, track_caller)]
    pub fn new(value: T) -> Self {
        Runtime::assert_ui_thread();
        let id = SignalState::new(value);
        id.set_scope();
        Signal { id, ty: PhantomData }
    }

    #[cfg_attr(debug_assertions, track_caller)]
    pub fn new_split(value: T) -> (ReadSignal<T>, WriteSignal<T>) {
        let sig = Self::new(value);
        (sig.read_only(), sig.write_only())
    }
}

/// A getter only Signal
pub struct ReadSignal<T> {
    pub(crate) id: Id,
    pub(crate) ty: PhantomData<T>,
}

impl<T> ReadSignal<T> {
    pub fn id(&self) -> Id {
        self.id
    }
}

impl<T> Copy for ReadSignal<T> {}

impl<T> Clone for ReadSignal<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Eq for ReadSignal<T> {}

impl<T> PartialEq for ReadSignal<T> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl<T> SignalTrack<T> for ReadSignal<T> {
    fn id(&self) -> Id {
        self.id
    }
}

impl<T: Default + 'static> Default for ReadSignal<T> {
    fn default() -> Self {
        Signal::new(T::default()).read_only()
    }
}

#[cfg(feature = "serde")]
impl<T> Serialize for ReadSignal<T>
where
    T: Serialize + 'static,
{
    fn serialize<Se>(&self, serializer: Se) -> Result<Se::Ok, Se::Error>
    where
        Se: Serializer,
    {
        self.with_untracked(|value| value.serialize(serializer))
    }
}

#[cfg(feature = "serde")]
impl<'de, T> Deserialize<'de> for ReadSignal<T>
where
    T: Deserialize<'de> + 'static,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = T::deserialize(deserializer)?;
        Ok(Signal::new(value).read_only())
    }
}

/// A setter only Signal
pub struct WriteSignal<T> {
    pub(crate) id: Id,
    pub(crate) ty: PhantomData<T>,
}

impl<T> WriteSignal<T> {
    pub fn id(&self) -> Id {
        self.id
    }
}

impl<T> Copy for WriteSignal<T> {}

impl<T> Clone for WriteSignal<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Eq for WriteSignal<T> {}

impl<T> PartialEq for WriteSignal<T> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl<T: Default + 'static> Default for WriteSignal<T> {
    fn default() -> Self {
        Signal::new(T::default()).write_only()
    }
}

#[cfg(feature = "serde")]
impl<T> Serialize for WriteSignal<T>
where
    T: Serialize + 'static,
{
    fn serialize<Se>(&self, serializer: Se) -> Result<Se::Ok, Se::Error>
    where
        Se: Serializer,
    {
        self.id().signal().unwrap().with_untracked::<_, T>(|value| value.serialize(serializer))
    }
}

#[cfg(feature = "serde")]
impl<'de, T> Deserialize<'de> for WriteSignal<T>
where
    T: Deserialize<'de> + 'static,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = T::deserialize(deserializer)?;
        Ok(Signal::new(value).write_only())
    }
}

impl<T: Clone> SignalGet<T> for Signal<T> {
    fn id(&self) -> Id {
        self.id
    }
}

impl<T> SignalWith<T> for Signal<T> {
    fn id(&self) -> Id {
        self.id
    }
}

impl<T> SignalRead<T> for Signal<T> {
    fn id(&self) -> Id {
        self.id
    }

    #[cfg_attr(debug_assertions, track_caller)]
    fn try_read(&self) -> Option<crate::read::ReadRef<'_, T>>
    where
        T: 'static,
    {
        Runtime::assert_ui_thread();
        self.id().signal().map(|signal| {
            signal.subscribe();
            crate::read::ReadRef::Local(crate::read::LocalReadRef::new(signal.as_local::<T>()))
        })
    }

    fn try_read_untracked(&self) -> Option<crate::read::ReadRef<'_, T>>
    where
        T: 'static,
    {
        Runtime::assert_ui_thread();
        self.id().signal().map(|signal| {
            crate::read::ReadRef::Local(crate::read::LocalReadRef::new(signal.as_local::<T>()))
        })
    }
}

impl<T> SignalUpdate<T> for Signal<T> {
    fn id(&self) -> Id {
        self.id
    }

    #[cfg_attr(debug_assertions, track_caller)]
    fn set(&self, new_value: T)
    where
        T: 'static,
    {
        Runtime::assert_ui_thread();
        if let Some(signal) = self.id().signal() {
            signal.update_value_local(|v| *v = new_value);
        }
    }

    #[cfg_attr(debug_assertions, track_caller)]
    fn update(&self, f: impl FnOnce(&mut T))
    where
        T: 'static,
    {
        Runtime::assert_ui_thread();
        if let Some(signal) = self.id().signal() {
            signal.update_value_local(f);
        }
    }

    #[cfg_attr(debug_assertions, track_caller)]
    fn try_update<O>(&self, f: impl FnOnce(&mut T) -> O) -> Option<O>
    where
        T: 'static,
    {
        Runtime::assert_ui_thread();
        self.id().signal().map(|signal| signal.update_value_local(f))
    }
}

impl<T> SignalWrite<T> for Signal<T> {
    fn id(&self) -> Id {
        self.id
    }

    #[cfg_attr(debug_assertions, track_caller)]
    fn write(&self) -> crate::write::WriteRef<'_, T>
    where
        T: 'static,
    {
        Runtime::assert_ui_thread();
        self.try_write().unwrap()
    }

    #[cfg_attr(debug_assertions, track_caller)]
    fn try_write(&self) -> Option<crate::write::WriteRef<'_, T>>
    where
        T: 'static,
    {
        Runtime::assert_ui_thread();
        self.id().signal().map(|signal| {
            crate::write::WriteRef::Local(crate::write::LocalWriteRef::new(
                signal.id,
                signal.as_local::<T>(),
            ))
        })
    }
}

impl<T> SignalUpdate<T> for WriteSignal<T> {
    fn id(&self) -> Id {
        self.id
    }

    #[cfg_attr(debug_assertions, track_caller)]
    fn set(&self, new_value: T)
    where
        T: 'static,
    {
        Runtime::assert_ui_thread();
        if let Some(signal) = self.id().signal() {
            signal.update_value_local(|v| *v = new_value);
        }
    }

    #[cfg_attr(debug_assertions, track_caller)]
    fn update(&self, f: impl FnOnce(&mut T))
    where
        T: 'static,
    {
        Runtime::assert_ui_thread();
        if let Some(signal) = self.id().signal() {
            signal.update_value_local(f);
        }
    }

    #[cfg_attr(debug_assertions, track_caller)]
    fn try_update<O>(&self, f: impl FnOnce(&mut T) -> O) -> Option<O>
    where
        T: 'static,
    {
        Runtime::assert_ui_thread();
        self.id().signal().map(|signal| signal.update_value_local(f))
    }
}

impl<T> SignalWrite<T> for WriteSignal<T> {
    fn id(&self) -> Id {
        self.id
    }

    #[cfg_attr(debug_assertions, track_caller)]
    fn try_write(&self) -> Option<crate::write::WriteRef<'_, T>>
    where
        T: 'static,
    {
        Runtime::assert_ui_thread();
        self.id().signal().map(|signal| {
            crate::write::WriteRef::Local(crate::write::LocalWriteRef::new(
                signal.id,
                signal.as_local::<T>(),
            ))
        })
    }
}

impl<T: Clone> SignalGet<T> for ReadSignal<T> {
    fn id(&self) -> Id {
        self.id
    }
}

impl<T> SignalWith<T> for ReadSignal<T> {
    fn id(&self) -> Id {
        self.id
    }
}

impl<T> SignalRead<T> for ReadSignal<T> {
    fn id(&self) -> Id {
        self.id
    }

    #[cfg_attr(debug_assertions, track_caller)]
    fn try_read(&self) -> Option<crate::read::ReadRef<'_, T>>
    where
        T: 'static,
    {
        Runtime::assert_ui_thread();
        self.id().signal().map(|signal| {
            signal.subscribe();
            crate::read::ReadRef::Local(crate::read::LocalReadRef::new(signal.as_local::<T>()))
        })
    }

    fn try_read_untracked(&self) -> Option<crate::read::ReadRef<'_, T>>
    where
        T: 'static,
    {
        Runtime::assert_ui_thread();
        self.id().signal().map(|signal| {
            crate::read::ReadRef::Local(crate::read::LocalReadRef::new(signal.as_local::<T>()))
        })
    }
}
