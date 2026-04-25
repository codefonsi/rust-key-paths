//! Pin-project keypath support for `#[pin]` future fields.
//!
//! This module lets you compose:
//! - a synchronous keypath step (`Kp` or any `SyncKeyPathLike`)
//! - followed by a pinned future await step (`PinFutureAwaitLike`)
//!
//! # Pros
//! - Works with `pin_project`-generated helpers and macro-based keypaths.
//! - Keeps chaining ergonomic with `then_pin_future(...).get_mut(...).await`.
//! - Preserves zero-sized behavior when closures/segments are zero-sized.
//!
//! # Cons
//! - Read-only `get` cannot await and always returns `None`.
//! - Requires `Unpin` for the bridged mutable value target type.
//!
//! # Warnings
//! - The `get_mut` chain requires mutable access to the root and intermediate value.
//! - If an upstream keypath fails, the pinned await step is never executed.

use std::pin::Pin;

use crate::{KPWritable, Kp, KpReadable, PinFutureAwaitLike, SyncKeyPathLike};

/// Async keypath value that awaits a `#[pin]` future field.
#[derive(Clone)]
pub struct PinFutureAwaitKp<S, Output, L>
where
    L: Clone,
{
    inner: L,
    _p: std::marker::PhantomData<fn() -> (S, Output)>,
}

impl<S, Output, L> PinFutureAwaitKp<S, Output, L>
where
    L: PinFutureAwaitLike<S, Output> + Clone,
{
    #[inline]
    pub fn new(inner: L) -> Self {
        Self {
            inner,
            _p: std::marker::PhantomData,
        }
    }
}

#[async_trait::async_trait(?Send)]
impl<S, Output, L> PinFutureAwaitLike<S, Output> for PinFutureAwaitKp<S, Output, L>
where
    L: PinFutureAwaitLike<S, Output> + Sync + Clone,
{
    async fn get_await(&self, this: Pin<&mut S>) -> Option<Output> {
        self.inner.get_await(this).await
    }
}

/// Macro to create a [`PinFutureAwaitKp`] from a type's derived pin-await method.
#[macro_export]
macro_rules! pin_future_await_kp {
    ($ty:ty, $method:ident -> $output:ty) => {{
        #[derive(Clone, Copy)]
        struct KpImpl;
        #[::async_trait::async_trait(?Send)]
        impl $crate::PinFutureAwaitLike<$ty, $output> for KpImpl {
            async fn get_await(&self, this: std::pin::Pin<&mut $ty>) -> Option<$output> {
                <$ty>::$method(this).await
            }
        }
        $crate::pin::PinFutureAwaitKp::new(KpImpl)
    }};
}

/// Keypath chain that sync-navigates to `S` then awaits a pinned future from `S`.
#[derive(Clone)]
pub struct KpThenPinFuture<R, S, Output, First, Second> {
    pub(crate) first: First,
    pub(crate) second: Second,
    pub(crate) _p: std::marker::PhantomData<(R, S, Output)>,
}

impl<R, S, Output, First, Second> KpThenPinFuture<R, S, Output, First, Second> {
    #[inline]
    pub fn new(first: First, second: Second) -> Self {
        Self {
            first,
            second,
            _p: std::marker::PhantomData,
        }
    }
}

impl<R, S, Output, First, Second> KpThenPinFuture<R, S, Output, First, Second>
where
    S: Unpin,
    Output: 'static,
    First: SyncKeyPathLike<R, S>,
    Second: PinFutureAwaitLike<S, Output>,
{
    /// Immutable access cannot await pinned futures; returns `None`.
    pub async fn get(&self, _root: &R) -> Option<Output> {
        None
    }

    /// Mutable access chain: sync navigate then await pinned future.
    pub async fn get_mut(&self, root: &mut R) -> Option<Output> {
        let s: &mut S = self.first.sync_get_mut(root)?;
        self.second.get_await(Pin::new(s)).await
    }

    #[inline]
    pub async fn get_optional(&self, root: Option<&R>) -> Option<Output> {
        match root {
            Some(r) => self.get(r).await,
            None => None,
        }
    }

    #[inline]
    pub async fn get_mut_optional(&self, root: Option<&mut R>) -> Option<Output> {
        match root {
            Some(r) => self.get_mut(r).await,
            None => None,
        }
    }

    #[inline]
    pub async fn get_or_else<F>(&self, root: Option<&R>, f: F) -> Output
    where
        F: FnOnce() -> Output,
    {
        self.get_optional(root).await.unwrap_or_else(f)
    }

    #[inline]
    pub async fn get_mut_or_else<F>(&self, root: Option<&mut R>, f: F) -> Output
    where
        F: FnOnce() -> Output,
    {
        self.get_mut_optional(root).await.unwrap_or_else(f)
    }
}

impl<R, V, G, S> SyncKeyPathLike<R, V> for Kp<R, V, G, S>
where
    G: for<'r> Fn(&'r R) -> Option<&'r V>,
    S: for<'r> Fn(&'r mut R) -> Option<&'r mut V>,
{
    #[inline]
    fn sync_get<'a>(&self, root: &'a R) -> Option<&'a V> {
        self.get(root)
    }

    #[inline]
    fn sync_get_mut<'a>(&self, root: &'a mut R) -> Option<&'a mut V> {
        self.set(root)
    }
}
// //! Pin-project keypath support: #[pin] Future field await.
// //!
// //! Enables composing `fut_await` with [crate::Kp::then_pin_future], e.g.:
// //!   `kp.then_pin_future(WithPinnedBoxFuture::fut_pin_future_kp())`
// //!
// //! Requires: `#[pin_project]` `#[derive(Kp)]` struct with `#[pin] fut: Pin<Box<dyn Future>>`

// use async_trait::async_trait;
// use std::pin::Pin;

// use crate::async_lock::SyncKeyPathLike;

// /// Trait for awaiting a #[pin] Future field via Pin<&mut S>. Implemented by [pin_future_await_kp!]
// /// and by the Kp derive for `{field}_pin_future_kp()` methods.
// #[async_trait(?Send)]
// pub trait PinFutureAwaitLike<S, Output> {
//     /// Await the pinned future. Requires `Pin<&mut S>` (use via [crate::Kp::then_pin_future]).
//     async fn get_await(&self, this: Pin<&mut S>) -> Option<Output>;
// }

// /// Async keypath that awaits a #[pin] Future field. Use [pin_future_await_kp!] or the derived
// /// `{field}_pin_future_kp()` to construct.
// #[derive(Clone)]
// pub struct PinFutureAwaitKp<S, Output, L>
// where
//     L: Clone,
// {
//     inner: L,
//     _p: std::marker::PhantomData<fn() -> (S, Output)>,
// }

// impl<S, Output, L> PinFutureAwaitKp<S, Output, L>
// where
//     L: PinFutureAwaitLike<S, Output> + Clone,
// {
//     pub fn new(inner: L) -> Self {
//         Self {
//             inner,
//             _p: std::marker::PhantomData,
//         }
//     }
// }

// #[async_trait(?Send)]
// impl<S, Output, L> PinFutureAwaitLike<S, Output> for PinFutureAwaitKp<S, Output, L>
// where
//     L: PinFutureAwaitLike<S, Output> + Sync + Clone,
// {
//     async fn get_await(&self, this: Pin<&mut S>) -> Option<Output> {
//         self.inner.get_await(this).await
//     }
// }

// /// Macro to create a [PinFutureAwaitKp] from a derived `field_await` function.
// ///
// /// # Example
// /// ```ignore
// /// #[pin_project]
// /// #[derive(Kp)]
// /// struct WithPinnedBoxFuture {
// ///     #[pin] fut: Pin<Box<dyn Future<Output = i32> + Send>>,
// /// }
// ///
// /// let kp = Kp::identity::<WithPinnedBoxFuture>()
// ///     .then_pin_future(pin_future_await_kp!(WithPinnedBoxFuture, fut_await -> i32));
// /// let result = kp.get_mut(&mut data).await;  // Option<i32>
// /// ```
// #[macro_export]
// macro_rules! pin_future_await_kp {
//     ($ty:ty, $method:ident -> $output:ty) => {{
//         #[derive(Clone, Copy)]
//         struct KpImpl;
//         #[::async_trait::async_trait(?Send)]
//         impl $crate::pin::PinFutureAwaitLike<$ty, $output> for KpImpl {
//             async fn get_await(&self, this: std::pin::Pin<&mut $ty>) -> Option<$output> {
//                 <$ty>::$method(this).await
//             }
//         }
//         $crate::pin::PinFutureAwaitKp::new(KpImpl)
//     }};
// }

// /// Keypath that chains a sync [crate::Kp] with a [PinFutureAwaitKp]. Use [crate::Kp::then_pin_future] to create.
// /// Enables: `kp.then_pin_future(...).get_mut(&mut root).await` to await #[pin] Future fields.
// #[derive(Clone)]
// pub struct KpThenPinFuture<R, S, Output, Root, MutRoot, Value, MutValue, First, Second> {
//     pub(crate) first: First,
//     pub(crate) second: Second,
//     pub(crate) _p: std::marker::PhantomData<(R, S, Output, Root, MutRoot, Value, MutValue)>,
// }

// impl<R, S, Output, Root, MutRoot, Value, MutValue, First, Second>
//     KpThenPinFuture<R, S, Output, Root, MutRoot, Value, MutValue, First, Second>
// where
//     S: Unpin,
//     Output: 'static,
//     MutValue: std::borrow::BorrowMut<S>,
//     First: SyncKeyPathLike<Root, Value, MutRoot, MutValue>,
//     Second: PinFutureAwaitLike<S, Output>,
// {
//     /// Get through the chain. For pin futures, [get] returns `None` (await requires mutable access).
//     pub async fn get(&self, _root: Root) -> Option<Output> {
//         None
//     }

//     /// Get mutable through the chain: sync navigate to &mut S, then await the pinned future.
//     pub async fn get_mut(&self, root: MutRoot) -> Option<Output> {
//         let mut mut_value = self.first.sync_get_mut(root)?;
//         let s: &mut S = mut_value.borrow_mut();
//         self.second.get_await(Pin::new(s)).await
//     }

//     /// Like [get](KpThenPinFuture::get), but takes an optional root.
//     #[inline]
//     pub async fn get_optional(&self, root: Option<Root>) -> Option<Output> {
//         match root {
//             Some(r) => self.get(r).await,
//             None => None,
//         }
//     }

//     /// Like [get_mut](KpThenPinFuture::get_mut), but takes an optional root.
//     #[inline]
//     pub async fn get_mut_optional(&self, root: Option<MutRoot>) -> Option<Output> {
//         match root {
//             Some(r) => self.get_mut(r).await,
//             None => None,
//         }
//     }

//     /// Returns the value if the keypath succeeds, otherwise calls `f` and returns its result.
//     #[inline]
//     pub async fn get_or_else<F>(&self, root: Option<Root>, f: F) -> Output
//     where
//         F: FnOnce() -> Output,
//     {
//         self.get_optional(root).await.unwrap_or_else(f)
//     }

//     /// Returns the value (from get_mut) if the keypath succeeds, otherwise calls `f` and returns its result.
//     #[inline]
//     pub async fn get_mut_or_else<F>(&self, root: Option<MutRoot>, f: F) -> Output
//     where
//         F: FnOnce() -> Output,
//     {
//         self.get_mut_optional(root).await.unwrap_or_else(f)
//     }
// }
