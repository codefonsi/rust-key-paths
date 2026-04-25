//! Async lock keypath utilities in modern `Kp` style.
//!
//! This module provides async composition over lock-backed data using:
//! - `AsyncLockKp` for one async lock hop
//! - `AsyncLockKpThenLockKp` for async-then-sync lock chaining
//!
//! # Pros
//! - Composes with existing `Kp`/`LockKp` keypaths.
//! - Preserves zero-sized behavior when callables are zero-sized.
//! - Uses update-closure APIs for safe in-lock mutation.
//!
//! # Cons
//! - `get` returns cloned terminal values (`V: Clone`) instead of lock guards.
//! - Deep async composition APIs from legacy module are intentionally reduced.
//!
//! # Warnings
//! - Hold locks briefly in updater closures to avoid contention.
//! - Upstream keypath misses short-circuit and skip lock operations.

use crate::{AsyncKeyPathLike, AsyncLockLike, Kp, KpReadable, KPWritable};
use std::marker::PhantomData;

#[cfg(feature = "tokio")]
pub use tokio::sync::{Mutex as TokioMutex, RwLock as TokioRwLock};

/// Async lock-aware keypath: `R -> Lock -> Mid -> V`.
#[derive(Clone)]
pub struct AsyncLockKp<R, Lock, Mid, V, G1, S1, L, G2, S2>
where
    G1: for<'r> Fn(&'r R) -> Option<&'r Lock>,
    S1: for<'r> Fn(&'r mut R) -> Option<&'r mut Lock>,
    G2: for<'r> Fn(&'r Mid) -> Option<&'r V>,
    S2: for<'r> Fn(&'r mut Mid) -> Option<&'r mut V>,
    L: AsyncLockLike<Lock, Mid> + Clone,
{
    pub(crate) prev: Kp<R, Lock, G1, S1>,
    pub(crate) mid: L,
    pub(crate) next: Kp<Mid, V, G2, S2>,
}

impl<R, Lock, Mid, V, G1, S1, L, G2, S2> AsyncLockKp<R, Lock, Mid, V, G1, S1, L, G2, S2>
where
    G1: for<'r> Fn(&'r R) -> Option<&'r Lock>,
    S1: for<'r> Fn(&'r mut R) -> Option<&'r mut Lock>,
    G2: for<'r> Fn(&'r Mid) -> Option<&'r V>,
    S2: for<'r> Fn(&'r mut Mid) -> Option<&'r mut V>,
    L: AsyncLockLike<Lock, Mid> + Clone,
{
    #[inline]
    pub fn new(prev: Kp<R, Lock, G1, S1>, mid: L, next: Kp<Mid, V, G2, S2>) -> Self {
        Self { prev, mid, next }
    }

    /// Read through async lock and clone terminal value.
    pub async fn get(&self, root: &R) -> Option<V>
    where
        V: Clone + Send,
    {
        let lock = (self.prev.get)(root)?;
        self.mid.with_read(lock, |mid| self.next.get(mid).cloned()).await
    }

    /// Alias for read-through behavior in this reduced API.
    pub async fn get_mut(&self, root: &mut R) -> Option<V>
    where
        V: Clone + Send,
    {
        self.get(root).await
    }

    /// Mutate the terminal value while holding async write lock.
    pub async fn update<F>(&self, root: &R, updater: F) -> bool
    where
        F: FnOnce(&mut V),
    {
        let Some(lock) = (self.prev.get)(root) else {
            return false;
        };
        self.mid
            .with_write(lock, |mid| {
                let value = self.next.set(mid)?;
                updater(value);
                Some(())
            })
            .await
            .is_some()
    }

    /// Chain with a sync lock keypath.
    pub fn then_lock<Lock2, Mid2, V2, G3, S3, L2, G4, S4>(
        self,
        other: crate::lock::LockKp<V, Lock2, Mid2, V2, G3, S3, L2, G4, S4>,
    ) -> AsyncLockKpThenLockKp<R, V, V2, Self, crate::lock::LockKp<V, Lock2, Mid2, V2, G3, S3, L2, G4, S4>>
    where
        G3: for<'r> Fn(&'r V) -> Option<&'r Lock2>,
        S3: for<'r> Fn(&'r mut V) -> Option<&'r mut Lock2>,
        G4: for<'r> Fn(&'r Mid2) -> Option<&'r V2>,
        S4: for<'r> Fn(&'r mut Mid2) -> Option<&'r mut V2>,
        L2: crate::LockAccess<Lock2, Mid2> + Clone,
    {
        AsyncLockKpThenLockKp::new(self, other)
    }
}

#[async_trait::async_trait(?Send)]
impl<R, Lock, Mid, V, G1, S1, L, G2, S2> AsyncKeyPathLike<R>
    for AsyncLockKp<R, Lock, Mid, V, G1, S1, L, G2, S2>
where
    G1: for<'r> Fn(&'r R) -> Option<&'r Lock> + Send + Sync,
    S1: for<'r> Fn(&'r mut R) -> Option<&'r mut Lock> + Send + Sync,
    G2: for<'r> Fn(&'r Mid) -> Option<&'r V> + Send + Sync,
    S2: for<'r> Fn(&'r mut Mid) -> Option<&'r mut V> + Send + Sync,
    L: AsyncLockLike<Lock, Mid> + Clone + Send + Sync,
    V: Clone + Send,
{
    type Value = V;
    async fn get(&self, root: &R) -> Option<Self::Value> {
        AsyncLockKp::get(self, root).await
    }
}

/// Async then sync-lock composition keypath.
#[derive(Clone)]
pub struct AsyncLockKpThenLockKp<R, V, V2, First, Second> {
    pub(crate) first: First,
    pub(crate) second: Second,
    pub(crate) _p: PhantomData<(R, V, V2)>,
}

impl<R, V, V2, First, Second> AsyncLockKpThenLockKp<R, V, V2, First, Second> {
    #[inline]
    pub fn new(first: First, second: Second) -> Self {
        Self {
            first,
            second,
            _p: PhantomData,
        }
    }
}

impl<R, V, V2, First, Second> AsyncLockKpThenLockKp<R, V, V2, First, Second>
where
    First: AsyncKeyPathLike<R, Value = V>,
    Second: for<'r> SyncLockLike<&'r V, V2>,
{
    pub async fn get(&self, root: &R) -> Option<V2>
    where
        V2: Clone,
    {
        let v = self.first.get(root).await?;
        self.second.sync_lock_get(&v)
    }
}

/// Internal sync lock-like adapter for composing over owned `V`.
pub trait SyncLockLike<Root, V> {
    fn sync_lock_get(&self, root: Root) -> Option<V>;
}

impl<'a, V, Lock2, Mid2, V2, G3, S3, L2, G4, S4>
    SyncLockLike<&'a V, V2> for crate::lock::LockKp<V, Lock2, Mid2, V2, G3, S3, L2, G4, S4>
where
    G3: for<'r> Fn(&'r V) -> Option<&'r Lock2>,
    S3: for<'r> Fn(&'r mut V) -> Option<&'r mut Lock2>,
    G4: for<'r> Fn(&'r Mid2) -> Option<&'r V2>,
    S4: for<'r> Fn(&'r mut Mid2) -> Option<&'r mut V2>,
    L2: crate::LockAccess<Lock2, Mid2> + Clone,
    V2: Clone,
{
    fn sync_lock_get(&self, root: &'a V) -> Option<V2> {
        self.get(root)
    }
}

/// Async lock accessor for `Arc<tokio::sync::Mutex<T>>`.
#[cfg(feature = "tokio")]
#[derive(Default)]
pub struct TokioMutexAccess<T>(PhantomData<T>);
#[cfg(feature = "tokio")]
impl<T> Clone for TokioMutexAccess<T> {
    fn clone(&self) -> Self {
        Self(PhantomData)
    }
}
#[cfg(feature = "tokio")]
impl<T> TokioMutexAccess<T> {
    pub fn new() -> Self {
        Self(PhantomData)
    }
}
#[cfg(feature = "tokio")]
#[async_trait::async_trait(?Send)]
impl<T: Send + Sync> AsyncLockLike<std::sync::Arc<tokio::sync::Mutex<T>>, T> for TokioMutexAccess<T> {
    async fn with_read<Rv, F>(
        &self,
        lock: &std::sync::Arc<tokio::sync::Mutex<T>>,
        f: F,
    ) -> Option<Rv>
    where
        F: FnOnce(&T) -> Option<Rv>,
    {
        let guard = lock.lock().await;
        f(&guard)
    }

    async fn with_write<Rv, F>(
        &self,
        lock: &std::sync::Arc<tokio::sync::Mutex<T>>,
        f: F,
    ) -> Option<Rv>
    where
        F: FnOnce(&mut T) -> Option<Rv>,
    {
        let mut guard = lock.lock().await;
        f(&mut guard)
    }
}

/// Async lock accessor for `Arc<tokio::sync::RwLock<T>>`.
#[cfg(feature = "tokio")]
#[derive(Default)]
pub struct TokioRwLockAccess<T>(PhantomData<T>);
#[cfg(feature = "tokio")]
impl<T> Clone for TokioRwLockAccess<T> {
    fn clone(&self) -> Self {
        Self(PhantomData)
    }
}
#[cfg(feature = "tokio")]
impl<T> TokioRwLockAccess<T> {
    pub fn new() -> Self {
        Self(PhantomData)
    }
}
#[cfg(feature = "tokio")]
#[async_trait::async_trait(?Send)]
impl<T: Send + Sync> AsyncLockLike<std::sync::Arc<tokio::sync::RwLock<T>>, T> for TokioRwLockAccess<T> {
    async fn with_read<Rv, F>(
        &self,
        lock: &std::sync::Arc<tokio::sync::RwLock<T>>,
        f: F,
    ) -> Option<Rv>
    where
        F: FnOnce(&T) -> Option<Rv>,
    {
        let guard = lock.read().await;
        f(&guard)
    }

    async fn with_write<Rv, F>(
        &self,
        lock: &std::sync::Arc<tokio::sync::RwLock<T>>,
        f: F,
    ) -> Option<Rv>
    where
        F: FnOnce(&mut T) -> Option<Rv>,
    {
        let mut guard = lock.write().await;
        f(&mut guard)
    }
}
