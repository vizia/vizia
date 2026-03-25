//! # Vizia Reactive
//!
//! [`Signal::new_split`](Signal::new_split) returns a separated [`ReadSignal`] and [`WriteSignal`] for a variable.
//! An existing `Signal` may be converted using [`Signal::read_only`](Signal::read_only)
//! and [`Signal::write_only`](Signal::write_only) where necessary, but the reverse is not possible.

mod derived;
mod effect;
mod id;
mod impls;
mod map;
mod memo;
mod read;
mod runtime;
mod scope;
mod signal;
mod storage;
mod sync_runtime;
mod write;

pub use derived::{
    DerivedSignal, SyncDerivedSignal, create_derived_signal, create_sync_derived_signal,
};
pub use effect::{Effect, EffectTrait, SignalTracker, UpdaterEffect};
pub use id::Id as ReactiveId;
pub use map::SignalMap;
pub use memo::Memo;
pub use read::{ReadRef, SignalGet, SignalRead, SignalTrack, SignalWith};
pub use runtime::Runtime;
pub use scope::Scope;
pub use signal::{ReadSignal, Signal, SyncReadSignal, SyncSignal, SyncWriteSignal, WriteSignal};
pub use write::{SignalUpdate, SignalWrite, WriteRef};
