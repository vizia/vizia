use std::{fmt, marker::PhantomData};

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

/// A thread-safe read-write signal.
pub struct SyncSignal<T> {
    pub(crate) id: Id,
    pub(crate) ty: PhantomData<T>,
}

pub struct SyncReadSignal<T> {
    pub(crate) id: Id,
    pub(crate) ty: PhantomData<T>,
}

pub struct SyncWriteSignal<T> {
    pub(crate) id: Id,
    pub(crate) ty: PhantomData<T>,
}

impl<T> SyncSignal<T> {
    pub fn id(&self) -> Id {
        self.id
    }

    /// Create a Getter of this Signal
    pub fn read_only(&self) -> SyncReadSignal<T> {
        SyncReadSignal { id: self.id, ty: PhantomData }
    }

    /// Create a Setter of this Signal
    pub fn write_only(&self) -> SyncWriteSignal<T> {
        SyncWriteSignal { id: self.id, ty: PhantomData }
    }
}

impl<T> SyncReadSignal<T> {
    pub fn id(&self) -> Id {
        self.id
    }
}

impl<T> SyncWriteSignal<T> {
    pub fn id(&self) -> Id {
        self.id
    }
}

impl<T> Copy for SyncSignal<T> {}

impl<T> Clone for SyncSignal<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Eq for SyncSignal<T> {}

impl<T> PartialEq for SyncSignal<T> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl<T> fmt::Debug for SyncSignal<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = f.debug_struct("SyncSignal");
        s.field("id", &self.id);
        s.field("ty", &self.ty);
        s.finish()
    }
}

impl<T> Copy for SyncReadSignal<T> {}

impl<T> Clone for SyncReadSignal<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Eq for SyncReadSignal<T> {}

impl<T> PartialEq for SyncReadSignal<T> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl<T> fmt::Debug for SyncReadSignal<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = f.debug_struct("SyncReadSignal");
        s.field("id", &self.id);
        s.field("ty", &self.ty);
        s.finish()
    }
}

impl<T> Copy for SyncWriteSignal<T> {}

impl<T> Clone for SyncWriteSignal<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Eq for SyncWriteSignal<T> {}

impl<T> PartialEq for SyncWriteSignal<T> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl<T> fmt::Debug for SyncWriteSignal<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = f.debug_struct("SyncWriteSignal");
        s.field("id", &self.id);
        s.field("ty", &self.ty);
        s.finish()
    }
}

impl<T: Send + Sync + 'static> SyncSignal<T> {
    /// Creates a sync signal. When called off the UI thread, the signal is left
    /// unscoped, so callers must ensure it is disposed manually.
    pub fn new(value: T) -> Self {
        let id = SignalState::new_sync(value);
        if Runtime::is_ui_thread() {
            id.set_scope();
        }
        Self { id, ty: PhantomData }
    }

    /// Creates a sync signal with separate read/write handles. Off-UI calls
    /// leave the signal unscoped; the caller is responsible for disposal.
    pub fn new_split(value: T) -> (SyncReadSignal<T>, SyncWriteSignal<T>) {
        let sig = Self::new(value);
        (sig.read_only(), sig.write_only())
    }
}

impl<T: Send + Sync + 'static> From<T> for SyncSignal<T> {
    fn from(value: T) -> Self {
        SyncSignal::new(value)
    }
}

impl<T> SignalTrack<T> for SyncSignal<T> {
    fn id(&self) -> Id {
        self.id
    }
}

impl<T> SignalTrack<T> for SyncReadSignal<T> {
    fn id(&self) -> Id {
        self.id
    }
}

#[cfg(feature = "serde")]
impl<T> Serialize for SyncSignal<T>
where
    T: Serialize + Send + Sync + 'static,
{
    fn serialize<Se>(&self, serializer: Se) -> Result<Se::Ok, Se::Error>
    where
        Se: Serializer,
    {
        self.with_untracked(|value| value.serialize(serializer))
    }
}

#[cfg(feature = "serde")]
impl<'de, T> Deserialize<'de> for SyncSignal<T>
where
    T: Deserialize<'de> + Send + Sync + 'static,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = T::deserialize(deserializer)?;
        Ok(SyncSignal::new(value))
    }
}

impl<T: Default + Send + Sync + 'static> Default for SyncSignal<T> {
    fn default() -> Self {
        SyncSignal::new(T::default())
    }
}

impl<T: Clone + Send + Sync> SignalGet<T> for SyncSignal<T> {
    fn id(&self) -> Id {
        self.id
    }
}

impl<T: Send + Sync> SignalWith<T> for SyncSignal<T> {
    fn id(&self) -> Id {
        self.id
    }
}

impl<T: Send + Sync> SignalRead<T> for SyncSignal<T> {
    fn id(&self) -> Id {
        self.id
    }

    fn try_read(&self) -> Option<crate::read::ReadRef<'_, T>>
    where
        T: 'static,
    {
        self.id().signal().map(|signal| {
            signal.subscribe();
            crate::read::ReadRef::Sync(crate::read::SyncReadRef::new(signal.as_sync::<T>()))
        })
    }

    fn try_read_untracked(&self) -> Option<crate::read::ReadRef<'_, T>>
    where
        T: 'static,
    {
        self.id().signal().map(|signal| {
            crate::read::ReadRef::Sync(crate::read::SyncReadRef::new(signal.as_sync::<T>()))
        })
    }
}

impl<T: Send + Sync> SignalUpdate<T> for SyncSignal<T> {
    fn id(&self) -> Id {
        self.id
    }

    fn set(&self, new_value: T)
    where
        T: 'static,
    {
        if let Some(signal) = self.id().signal() {
            signal.update_value_sync(|v| *v = new_value);
        }
    }

    fn update(&self, f: impl FnOnce(&mut T))
    where
        T: 'static,
    {
        if let Some(signal) = self.id().signal() {
            signal.update_value_sync(f);
        }
    }

    fn try_update<O>(&self, f: impl FnOnce(&mut T) -> O) -> Option<O>
    where
        T: 'static,
    {
        self.id().signal().map(|signal| signal.update_value_sync(f))
    }
}

impl<T: Send + Sync> SignalWrite<T> for SyncSignal<T> {
    fn id(&self) -> Id {
        self.id
    }

    fn write(&self) -> crate::write::WriteRef<'_, T>
    where
        T: 'static,
    {
        self.try_write().unwrap()
    }

    fn try_write(&self) -> Option<crate::write::WriteRef<'_, T>>
    where
        T: 'static,
    {
        self.id().signal().map(|signal| {
            crate::write::WriteRef::Sync(crate::write::SyncWriteRef::new(
                signal.id,
                signal.as_sync::<T>(),
            ))
        })
    }
}

impl<T: Default + Send + Sync + 'static> Default for SyncReadSignal<T> {
    fn default() -> Self {
        SyncSignal::new(T::default()).read_only()
    }
}

#[cfg(feature = "serde")]
impl<T> Serialize for SyncReadSignal<T>
where
    T: Serialize + Send + Sync + 'static,
{
    fn serialize<Se>(&self, serializer: Se) -> Result<Se::Ok, Se::Error>
    where
        Se: Serializer,
    {
        self.with_untracked(|value| value.serialize(serializer))
    }
}

#[cfg(feature = "serde")]
impl<'de, T> Deserialize<'de> for SyncReadSignal<T>
where
    T: Deserialize<'de> + Send + Sync + 'static,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = T::deserialize(deserializer)?;
        Ok(SyncSignal::new(value).read_only())
    }
}

impl<T: Clone + Send + Sync> SignalGet<T> for SyncReadSignal<T> {
    fn id(&self) -> Id {
        self.id
    }
}

impl<T: Send + Sync> SignalWith<T> for SyncReadSignal<T> {
    fn id(&self) -> Id {
        self.id
    }
}

impl<T: Send + Sync> SignalRead<T> for SyncReadSignal<T> {
    fn id(&self) -> Id {
        self.id
    }

    fn try_read(&self) -> Option<crate::read::ReadRef<'_, T>>
    where
        T: 'static,
    {
        self.id().signal().map(|signal| {
            signal.subscribe();
            crate::read::ReadRef::Sync(crate::read::SyncReadRef::new(signal.as_sync::<T>()))
        })
    }

    fn try_read_untracked(&self) -> Option<crate::read::ReadRef<'_, T>>
    where
        T: 'static,
    {
        self.id().signal().map(|signal| {
            crate::read::ReadRef::Sync(crate::read::SyncReadRef::new(signal.as_sync::<T>()))
        })
    }
}

impl<T: Default + Send + Sync + 'static> Default for SyncWriteSignal<T> {
    fn default() -> Self {
        SyncSignal::new(T::default()).write_only()
    }
}

#[cfg(feature = "serde")]
impl<T> Serialize for SyncWriteSignal<T>
where
    T: Serialize + Send + Sync + 'static,
{
    fn serialize<Se>(&self, serializer: Se) -> Result<Se::Ok, Se::Error>
    where
        Se: Serializer,
    {
        self.id().signal().unwrap().with_untracked::<_, T>(|value| value.serialize(serializer))
    }
}

#[cfg(feature = "serde")]
impl<'de, T> Deserialize<'de> for SyncWriteSignal<T>
where
    T: Deserialize<'de> + Send + Sync + 'static,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = T::deserialize(deserializer)?;
        Ok(SyncSignal::new(value).write_only())
    }
}

impl<T: Send + Sync> SignalUpdate<T> for SyncWriteSignal<T> {
    fn id(&self) -> Id {
        self.id
    }

    fn set(&self, new_value: T)
    where
        T: 'static,
    {
        if let Some(signal) = self.id().signal() {
            signal.update_value_sync(|v| *v = new_value);
        }
    }

    fn update(&self, f: impl FnOnce(&mut T))
    where
        T: 'static,
    {
        if let Some(signal) = self.id().signal() {
            signal.update_value_sync(f);
        }
    }

    fn try_update<O>(&self, f: impl FnOnce(&mut T) -> O) -> Option<O>
    where
        T: 'static,
    {
        self.id().signal().map(|signal| signal.update_value_sync(f))
    }
}

impl<T: Send + Sync> SignalWrite<T> for SyncWriteSignal<T> {
    fn id(&self) -> Id {
        self.id
    }

    fn try_write(&self) -> Option<crate::write::WriteRef<'_, T>>
    where
        T: 'static,
    {
        self.id().signal().map(|signal| {
            crate::write::WriteRef::Sync(crate::write::SyncWriteRef::new(
                signal.id,
                signal.as_sync::<T>(),
            ))
        })
    }
}
