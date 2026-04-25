// pub type KpType<R, V, Root, Value, MutRoot, MutValue, G, S>
// where
//     Root: ,
//     Value:    Borrow<V>,
//     MutRoot:  BorrowMut<R>,
//     MutValue: std::borrow::BorrowMut<V>,
//     G:        Fn(Root) -> Option<Value>,
//     S:        Fn(MutRoot) -> Option<MutValue> = Kp<R, V, Root, Value, MutRoot, MutValue, G, S>;

// type Getter<R, V, Root, Value> where Root: std::borrow::Borrow<R>, Value: std::borrow::Borrow<V> = fn(Root) -> Option<Value>;
// type Setter<R, V> = fn(&'r mut R) -> Option<&'r mut V>;

use std::fmt;
use std::sync::Arc;

// Export the lock module
pub mod lock;
pub mod prelude;

// pub use lock::{
//     ArcMutexAccess, ArcRwLockAccess, LockAccess, LockKp, LockKpType, RcRefCellAccess,
//     StdMutexAccess, StdRwLockAccess,
// };

// #[cfg(feature = "parking_lot")]
// pub use lock::{
//     DirectParkingLotMutexAccess, DirectParkingLotRwLockAccess, ParkingLotMutexAccess,
//     ParkingLotRwLockAccess,
// };

// Export the async_lock module
pub mod async_lock;

type KpDynamic<R, V> = Kp<
    R,
    V,
    dyn for<'r> Fn(&'r R) -> Option<&'r V>,
    dyn for<'r> Fn(&'r mut R) -> Option<&'r mut V>,
>;

// pub struct KpStatic<R, V> {
//     pub get: fn(&R) -> Option<&V>,
//     pub set: fn(&mut R) -> Option<&mut V>,
// }
//
// // KpStatic holds only fn pointers; it is a functional component with no owned data.
// unsafe impl<R, V> Send for KpStatic<R, V> {}
// unsafe impl<R, V> Sync for KpStatic<R, V> {}
//
// impl<R, V> KpStatic<R, V> {
//     pub const fn new(
//         get: fn(&R) -> Option<&V>,
//         set: fn(&mut R) -> Option<&mut V>,
//     ) -> Self {
//         Self { get, set }
//     }
//
//     #[inline(always)]
//     pub fn get<'a>(&self, root: &'a R) -> Option<&'a V> {
//         (self.get)(root)
//     }
//
//     #[inline(always)]
//     pub fn set<'a>(&self, root: &'a mut R) -> Option<&'a mut V> {
//         (self.set)(root)
//     }
// }

// // Macro generates:
// #[inline(always)]
// fn __get_static_str_field(x: &AllContainersTest) -> Option<&'static str> {
//     Some(&x.static_str_field)
// }
//
// #[inline(always)]
// fn __set_static_str_field(x: &mut AllContainersTest) -> Option<&mut &'static str> {
//     Some(&mut x.static_str_field)
// }
//
// pub static STATIC_STR_FIELD_KP: KpStatic<AllContainersTest, &'static str> =
//     KpStatic::new(__get_static_str_field, __set_static_str_field);

#[cfg(feature = "pin_project")]
pub mod pin;

/// Used so that `then_async` can infer `V2` from `AsyncKp::Value` without ambiguity
/// (e.g. `&i32` has both `Borrow<i32>` and `Borrow<&i32>`; this picks the referent).
/// Implemented only for reference types so there is no overlap with the blanket impl.
pub trait KeyPathValueTarget {
    type Target: Sized;
}
impl<T> KeyPathValueTarget for &T {
    type Target = T;
}
impl<T> KeyPathValueTarget for &mut T {
    type Target = T;
}

// /// Build a keypath from `Type.field` segments. Use with types that have keypath accessors (e.g. `#[derive(Kp)]` from key-paths-derive).
// #[macro_export]
// macro_rules! keypath {
//     { $root:ident . $field:ident } => { $root::$field() };
//     { $root:ident . $field:ident . $($ty:ident . $f:ident).+ } => {
//         $root::$field() $(.then($ty::$f()))+
//     };
//     ($root:ident . $field:ident) => { $root::$field() };
//     ($root:ident . $field:ident . $($ty:ident . $f:ident).+) => {
//         $root::$field() $(.then($ty::$f()))+
//     };
// }

// /// Get value through a keypath or a default reference when the path returns `None`.
// /// Use with `KpType`: `get_or!(User::name(), &user, &default)` where `default` is `&T` (same type as the path value). Returns `&T`.
// /// Path syntax: `get_or!(&user => User.name, &default)`.
// #[macro_export]
// macro_rules! get_or {
//     ($kp:expr, $root:expr, $default:expr) => {
//         $kp.get($root).unwrap_or($default)
//     };
//     ($root:expr => $($path:tt)*, $default:expr) => {
//         $crate::get_or!($crate::keypath!($($path)*), $root, $default)
//     };
// }

// /// Get value through a keypath, or compute an owned fallback when the path returns `None`.
// /// Use with `KpType`: `get_or_else!(User::name(), &user, || "default".to_string())`.
// /// Returns `T` (owned). The keypath's value type must be `Clone`. The closure is only called when the path is `None`.
// /// Path syntax: `get_or_else!(&user => (User.name), || "default".to_string())` — path in parentheses.
// #[macro_export]
// macro_rules! get_or_else {
//     ($kp:expr, $root:expr, $closure:expr) => {
//         $kp.get($root).map(|r| r.clone()).unwrap_or_else($closure)
//     };
//     ($root:expr => ($($path:tt)*), $closure:expr) => {
//         $crate::get_or_else!($crate::keypath!($($path)*), $root, $closure)
//     };
// }

// /// Zip multiple keypaths on the same root and apply a closure to the tuple of values.
// /// Returns `Some(closure((v1, v2, ...)))` when all keypaths succeed, else `None`.
// ///
// /// # Example
// /// ```
// /// use rust_key_paths::{Kp, KpType, zip_with_kp};
// /// struct User { name: String, age: u32, city: String }
// /// let name_kp = KpType::new(|u: &User| Some(&u.name), |u: &mut User| Some(&mut u.name));
// /// let age_kp = KpType::new(|u: &User| Some(&u.age), |u: &mut User| Some(&mut u.age));
// /// let city_kp = KpType::new(|u: &User| Some(&u.city), |u: &mut User| Some(&mut u.city));
// /// let user = User { name: "Akash".into(), age: 30, city: "NYC".into() };
// /// let summary = zip_with_kp!(
// ///     &user,
// ///     |(name, age, city)| format!("{}, {} from {}", name, age, city) =>
// ///     name_kp,
// ///     age_kp,
// ///     city_kp
// /// );
// /// assert_eq!(summary, Some("Akash, 30 from NYC".to_string()));
// /// ```
// #[macro_export]
// macro_rules! zip_with_kp {
//     ($root:expr, $closure:expr => $kp1:expr, $kp2:expr) => {
//         match ($kp1.get($root), $kp2.get($root)) {
//             (Some(__a), Some(__b)) => Some($closure((__a, __b))),
//             _ => None,
//         }
//     };
//     ($root:expr, $closure:expr => $kp1:expr, $kp2:expr, $kp3:expr) => {
//         match ($kp1.get($root), $kp2.get($root), $kp3.get($root)) {
//             (Some(__a), Some(__b), Some(__c)) => Some($closure((__a, __b, __c))),
//             _ => None,
//         }
//     };
//     ($root:expr, $closure:expr => $kp1:expr, $kp2:expr, $kp3:expr, $kp4:expr) => {
//         match (
//             $kp1.get($root),
//             $kp2.get($root),
//             $kp3.get($root),
//             $kp4.get($root),
//         ) {
//             (Some(__a), Some(__b), Some(__c), Some(__d)) => Some($closure((__a, __b, __c, __d))),
//             _ => None,
//         }
//     };
//     ($root:expr, $closure:expr => $kp1:expr, $kp2:expr, $kp3:expr, $kp4:expr, $kp5:expr) => {
//         match (
//             $kp1.get($root),
//             $kp2.get($root),
//             $kp3.get($root),
//             $kp4.get($root),
//             $kp5.get($root),
//         ) {
//             (Some(__a), Some(__b), Some(__c), Some(__d), Some(__e)) => {
//                 Some($closure((__a, __b, __c, __d, __e)))
//             }
//             _ => None,
//         }
//     };
//     ($root:expr, $closure:expr => $kp1:expr, $kp2:expr, $kp3:expr, $kp4:expr, $kp5:expr, $kp6:expr) => {
//         match (
//             $kp1.get($root),
//             $kp2.get($root),
//             $kp3.get($root),
//             $kp4.get($root),
//             $kp5.get($root),
//             $kp6.get($root),
//         ) {
//             (Some(__a), Some(__b), Some(__c), Some(__d), Some(__e), Some(__f)) => {
//                 Some($closure((__a, __b, __c, __d, __e, __f)))
//             }
//             _ => None,
//         }
//     };
// }

// /// Kp will force dev to create get and set while value will be owned
// pub type KpValue<'a, R, V> = Kp<
//     R,
//     V,
//     &'a R,
//     V, // Returns owned V, not &V
//     &'a mut R,
//     V, // Returns owned V, not &mut V
//     for<'b> fn(&'b R) -> Option<V>,
//     for<'b> fn(&'b mut R) -> Option<V>,
// >;

// /// Kp will force dev to create get and set while root and value both will be owned
// pub type KpOwned<R, V> = Kp<
//     R,
//     V,
//     R,
//     V, // Returns owned V, not &V
//     R,
//     V, // Returns owned V, not &mut V
//     fn(R) -> Option<V>,
//     fn(R) -> Option<V>,
// >;

// /// Kp will force dev to create get and set while taking full ownership of the Root while returning Root as value.
// pub type KpRoot<R> = Kp<
//     R,
//     R,
//     R,
//     R, // Returns owned V, not &V
//     R,
//     R, // Returns owned V, not &mut V
//     fn(R) -> Option<R>,
//     fn(R) -> Option<R>,
// >;

// /// Kp for void - experimental
// pub type KpVoid = Kp<(), (), (), (), (), (), fn() -> Option<()>, fn() -> Option<()>>;

// pub type KpDynamic<R, V> = Kp<
//     R,
//     V,
//     &'static R,
//     &'static V,
//     &'static mut R,
//     &'static mut V,
//     Box<dyn for<'a> Fn(&'a R) -> Option<&'a V> + Send + Sync>,
//     Box<dyn for<'a> Fn(&'a mut R) -> Option<&'a mut V> + Send + Sync>,
// >;

// pub type KpBox<'a, R, V> = Kp<
//     R,
//     V,
//     &'a R,
//     &'a V,
//     &'a mut R,
//     &'a mut V,
//     Box<dyn Fn(&'a R) -> Option<&'a V> + 'a>,
//     Box<dyn Fn(&'a mut R) -> Option<&'a mut V> + 'a>,
// >;

// pub type KpArc<'a, R, V> = Kp<
//     R,
//     V,
//     &'a R,
//     &'a V,
//     &'a mut R,
//     &'a mut V,
//     Arc<dyn Fn(&'a R) -> Option<&'a V> + Send + Sync + 'a>,
//     Arc<dyn Fn(&'a mut R) -> Option<&'a mut V> + Send + Sync + 'a>,
// >;

// pub type KpType<'a, R, V> = Kp<
//     R,
//     V,
//     &'a R,
//     &'a V,
//     &'a mut R,
//     &'a mut V,
//     for<'b> fn(&'b R) -> Option<&'b V>,
//     for<'b> fn(&'b mut R) -> Option<&'b mut V>,
// >;

// pub type KpTraitType<'a, R, V> = dyn KpTrait<
//         R,
//         V,
//         &'a R,
//         &'a V,
//         &'a mut R,
//         &'a mut V,
//         for<'b> fn(&'b R) -> Option<&'b V>,
//         for<'b> fn(&'b mut R) -> Option<&'b mut V>,
//     >;

// /// Keypath for `Option<RefCell<T>>`: `get` returns `Option<Ref<V>>` so the caller holds the guard.
// /// Use `.get(root).as_ref().map(std::cell::Ref::deref)` to get `Option<&V>` while the `Ref` is in scope.
// pub type KpOptionRefCellType<'a, R, V> = Kp<
//     R,
//     V,
//     &'a R,
//     std::cell::Ref<'a, V>,
//     &'a mut R,
//     std::cell::RefMut<'a, V>,
//     for<'b> fn(&'b R) -> Option<std::cell::Ref<'b, V>>,
//     for<'b> fn(&'b mut R) -> Option<std::cell::RefMut<'b, V>>,
// >;

// impl<'a, R, V> KpType<'a, R, V> {
//     /// Converts this keypath to [KpDynamic] for dynamic dispatch and storage (e.g. in a struct field).
//     #[inline]
//     pub fn to_dynamic(self) -> KpDynamic<R, V> {
//         self.into()
//     }
// }

// impl<'a, R, V> From<KpType<'a, R, V>> for KpDynamic<R, V> {
//     #[inline]
//     fn from(kp: KpType<'a, R, V>) -> Self {
//         let get_fn = kp.get;
//         let set_fn = kp.set;
//         Kp::new(
//             Box::new(move |t: &R| get_fn(t)),
//             Box::new(move |t: &mut R| set_fn(t)),
//         )
//     }
// }

// impl<R, V, Root, Value, MutRoot, MutValue, G, S> Kp<R, V, Root, Value, MutRoot, MutValue, G, S>
// where
//     Root: std::borrow::Borrow<R>,
//     Value: std::borrow::Borrow<V>,
//     MutRoot: std::borrow::BorrowMut<R>,
//     MutValue: std::borrow::BorrowMut<V>,
//     G: Fn(Root) -> Option<Value> + Send + Sync + 'static,
//     S: Fn(MutRoot) -> Option<MutValue> + Send + Sync + 'static,
//     R: 'static,
//     V: 'static,
// {
//     /// Erases getter/setter type into [`KpDynamic`] so you can store composed paths (e.g. after [KpTrait::then]).
//     ///
//     /// `#[derive(Kp)]` methods return [`KpType`] (`fn` pointers); chaining with `.then()` produces opaque closures.
//     /// Neither matches a fixed `KpType<…>` field type—use `KpDynamic<R, V>` and `.into_dynamic()` (or
//     /// [KpType::to_dynamic] for a single segment).
//     ///
//     /// # Safety
//     ///
//     /// This uses a small amount of `unsafe` internally: it re-interprets `&R` / `&mut R` as `Root` / `MutRoot`.
//     /// That matches every [`Kp`] built from this crate’s public API ([`Kp::new`] on reference-shaped handles,
//     /// `#[derive(Kp)]`, and [KpTrait::then] / [Kp::then] on those paths). Do not call this on a custom [`Kp`]
//     /// whose `Root` / `MutRoot` are not layout-compatible with `&R` / `&mut R` or whose getters keep borrows
//     /// alive past the call.
//     #[inline]
//     pub fn into_dynamic(self) -> KpDynamic<R, V> {
//         let g = self.get;
//         let s = self.set;
//         Kp::new(
//             Box::new(move |t: &R| unsafe {
//                 // SAFETY: See `into_dynamic` rustdoc. `Root` is `&'_ R` for supported keypaths.
//                // debug_assert_eq!(std::mem::size_of::<Root>(), std::mem::size_of::<&R>());
//                 let root: Root = std::mem::transmute_copy(&t);
//                 match g(root) {
//                     None => None,
//                     Some(v) => {
//                         let r: &V = std::borrow::Borrow::borrow(&v);
//                         // Well-behaved getters return a view into `*t`; re-attach to this call's `&R`.
//                         Some(std::mem::transmute::<&V, &V>(r))
//                     }
//                 }
//             }),
//             Box::new(move |t: &mut R| unsafe {
//                 // debug_assert_eq!(std::mem::size_of::<MutRoot>(), std::mem::size_of::<&mut R>());
//                 let root: MutRoot = std::mem::transmute_copy(&t);
//                 match s(root) {
//                     None => None,
//                     Some(mut v) => {
//                         let r: &mut V = std::borrow::BorrowMut::borrow_mut(&mut v);
//                         Some(std::mem::transmute::<&mut V, &mut V>(r))
//                     }
//                 }
//             }),
//         )
//     }
// }

// pub type KpType<R, V> = Kp<
//     R,
//     V,
//     &'static R,
//     &'static V,
//     &'static mut R,
//     &'static mut V,
//     for<'a> fn(&'a R) -> Option<&'a V>,
//     for<'a> fn(&'a mut R) -> Option<&'a mut V>,
// >;

// struct A{
//     b: std::sync::Arc<std::sync::Mutex<B>>,
// }
// struct B{
//     c: C
// }
// struct C{
//     d: String
// }

// pub struct LockKp {
//     first: KpType<'static, A, B>,
//     mid: KpType<'static, std::sync::Mutex<B>, B>,
//     second: KpType<'static, B, C>,
// }
//
// impl LockKp {
//     fn then(&self, kp: KpType<'static, B, String>) {
//
//     }
//     fn then_lock() {}
// }

// // New type alias for composed/transformed keypaths
// pub type KpComposed<R, V> = Kp<
//     R,
//     V,
//     &'static R,
//     &'static V,
//     &'static mut R,
//     &'static mut V,
//     Box<dyn for<'b> Fn(&'b R) -> Option<&'b V> + Send + Sync>,
//     Box<dyn for<'b> Fn(&'b mut R) -> Option<&'b mut V> + Send + Sync>,
// >;

// impl<R, V>
//     Kp<
//         R,
//         V,
//         &'static R,
//         &'static V,
//         &'static mut R,
//         &'static mut V,
//         Box<dyn for<'b> Fn(&'b R) -> Option<&'b V> + Send + Sync>,
//         Box<dyn for<'b> Fn(&'b mut R) -> Option<&'b mut V> + Send + Sync>,
//     >
// {
//     /// Build a keypath from two closures (e.g. when they capture a variable like an index).
//     /// Same pattern as `Kp::new` in lock.rs; use this when the keypath captures variables.
//     pub fn from_closures<G, S>(get: G, set: S) -> Self
//     where
//         G: for<'b> Fn(&'b R) -> Option<&'b V> + Send + Sync + 'static,
//         S: for<'b> Fn(&'b mut R) -> Option<&'b mut V> + Send + Sync + 'static,
//     {
//         Self::new(Box::new(get), Box::new(set))
//     }
// }

// pub struct AKp {
//     getter: Rc<dyn for<'r> Fn(&'r dyn Any) -> Option<&'r dyn Any>>,
//     root_type_id: TypeId,
//     value_type_id: TypeId,
// }

// impl AKp {
//     /// Create a new AKp from a KpType (the common reference-based keypath)
//     pub fn new<'a, R, V>(keypath: KpType<'a, R, V>) -> Self
//     where
//         R: Any + 'static,
//         V: Any + 'static,
//     {
//         let root_type_id = TypeId::of::<R>();
//         let value_type_id = TypeId::of::<V>();
//         let getter_fn = keypath.get;

//         Self {
//             getter: Rc::new(move |any: &dyn Any| {
//                 if let Some(root) = any.downcast_ref::<R>() {
//                     getter_fn(root).map(|value: &V| value as &dyn Any)
//                 } else {
//                     None
//                 }
//             }),
//             root_type_id,
//             value_type_id,
//         }
//     }

//     /// Create an AKp from a KpType (alias for `new()`)
//     pub fn from<'a, R, V>(keypath: KpType<'a, R, V>) -> Self
//     where
//         R: Any + 'static,
//         V: Any + 'static,
//     {
//         Self::new(keypath)
//     }

//     /// Get the value as a trait object (with root type checking)
//     pub fn get<'r>(&self, root: &'r dyn Any) -> Option<&'r dyn Any> {
//         (self.getter)(root)
//     }

//     /// Get the TypeId of the Root type
//     pub fn root_type_id(&self) -> TypeId {
//         self.root_type_id
//     }

//     /// Get the TypeId of the Value type
//     pub fn value_type_id(&self) -> TypeId {
//         self.value_type_id
//     }

//     /// Try to get the value with full type checking
//     pub fn get_as<'a, Root: Any, Value: Any>(&self, root: &'a Root) -> Option<Option<&'a Value>> {
//         if self.root_type_id == TypeId::of::<Root>() && self.value_type_id == TypeId::of::<Value>()
//         {
//             Some(
//                 self.get(root as &dyn Any)
//                     .and_then(|any| any.downcast_ref::<Value>()),
//             )
//         } else {
//             None
//         }
//     }

//     /// Get a human-readable name for the value type
//     pub fn kind_name(&self) -> String {
//         format!("{:?}", self.value_type_id)
//     }

//     /// Get a human-readable name for the root type
//     pub fn root_kind_name(&self) -> String {
//         format!("{:?}", self.root_type_id)
//     }

//     /// Adapt this keypath to work with Arc<Root> instead of Root
//     pub fn for_arc<Root>(&self) -> AKp
//     where
//         Root: Any + 'static,
//     {
//         let value_type_id = self.value_type_id;
//         let getter = self.getter.clone();

//         AKp {
//             getter: Rc::new(move |any: &dyn Any| {
//                 if let Some(arc) = any.downcast_ref::<Arc<Root>>() {
//                     getter(arc.as_ref() as &dyn Any)
//                 } else {
//                     None
//                 }
//             }),
//             root_type_id: TypeId::of::<Arc<Root>>(),
//             value_type_id,
//         }
//     }

//     /// Adapt this keypath to work with Box<Root> instead of Root
//     pub fn for_box<Root>(&self) -> AKp
//     where
//         Root: Any + 'static,
//     {
//         let value_type_id = self.value_type_id;
//         let getter = self.getter.clone();

//         AKp {
//             getter: Rc::new(move |any: &dyn Any| {
//                 if let Some(boxed) = any.downcast_ref::<Box<Root>>() {
//                     getter(boxed.as_ref() as &dyn Any)
//                 } else {
//                     None
//                 }
//             }),
//             root_type_id: TypeId::of::<Box<Root>>(),
//             value_type_id,
//         }
//     }

//     /// Adapt this keypath to work with Rc<Root> instead of Root
//     pub fn for_rc<Root>(&self) -> AKp
//     where
//         Root: Any + 'static,
//     {
//         let value_type_id = self.value_type_id;
//         let getter = self.getter.clone();

//         AKp {
//             getter: Rc::new(move |any: &dyn Any| {
//                 if let Some(rc) = any.downcast_ref::<Rc<Root>>() {
//                     getter(rc.as_ref() as &dyn Any)
//                 } else {
//                     None
//                 }
//             }),
//             root_type_id: TypeId::of::<Rc<Root>>(),
//             value_type_id,
//         }
//     }

//     /// Adapt this keypath to work with Option<Root> instead of Root
//     pub fn for_option<Root>(&self) -> AKp
//     where
//         Root: Any + 'static,
//     {
//         let value_type_id = self.value_type_id;
//         let getter = self.getter.clone();

//         AKp {
//             getter: Rc::new(move |any: &dyn Any| {
//                 if let Some(opt) = any.downcast_ref::<Option<Root>>() {
//                     opt.as_ref().and_then(|root| getter(root as &dyn Any))
//                 } else {
//                     None
//                 }
//             }),
//             root_type_id: TypeId::of::<Option<Root>>(),
//             value_type_id,
//         }
//     }

//     /// Adapt this keypath to work with Result<Root, E> instead of Root
//     pub fn for_result<Root, E>(&self) -> AKp
//     where
//         Root: Any + 'static,
//         E: Any + 'static,
//     {
//         let value_type_id = self.value_type_id;
//         let getter = self.getter.clone();

//         AKp {
//             getter: Rc::new(move |any: &dyn Any| {
//                 if let Some(result) = any.downcast_ref::<Result<Root, E>>() {
//                     result
//                         .as_ref()
//                         .ok()
//                         .and_then(|root| getter(root as &dyn Any))
//                 } else {
//                     None
//                 }
//             }),
//             root_type_id: TypeId::of::<Result<Root, E>>(),
//             value_type_id,
//         }
//     }

//     /// Map the value through a transformation function with type checking
//     /// Both original and mapped values must implement Any
//     ///
//     /// # Example
//     /// ```
//     /// use rust_key_paths::{AKp, Kp, KpType};
//     /// struct User { name: String }
//     /// let user = User { name: "Akash".to_string() };
//     /// let name_kp = KpType::new(|u: &User| Some(&u.name), |_| None);
//     /// let name_akp = AKp::new(name_kp);
//     /// let len_akp = name_akp.map::<User, String, _, _>(|s| s.len());
//     /// ```
//     pub fn map<Root, OrigValue, MappedValue, F>(&self, mapper: F) -> AKp
//     where
//         Root: Any + 'static,
//         OrigValue: Any + 'static,
//         MappedValue: Any + 'static,
//         F: Fn(&OrigValue) -> MappedValue + 'static,
//     {
//         let orig_root_type_id = self.root_type_id;
//         let orig_value_type_id = self.value_type_id;
//         let getter = self.getter.clone();
//         let mapped_type_id = TypeId::of::<MappedValue>();

//         AKp {
//             getter: Rc::new(move |any_root: &dyn Any| {
//                 // Check root type matches
//                 if any_root.type_id() == orig_root_type_id {
//                     getter(any_root).and_then(|any_value| {
//                         // Verify the original value type matches
//                         if orig_value_type_id == TypeId::of::<OrigValue>() {
//                             any_value.downcast_ref::<OrigValue>().map(|orig_val| {
//                                 let mapped = mapper(orig_val);
//                                 // Box the mapped value and return as &dyn Any
//                                 Box::leak(Box::new(mapped)) as &dyn Any
//                             })
//                         } else {
//                             None
//                         }
//                     })
//                 } else {
//                     None
//                 }
//             }),
//             root_type_id: orig_root_type_id,
//             value_type_id: mapped_type_id,
//         }
//     }

//     /// Filter the value based on a predicate with full type checking
//     /// Returns None if types don't match or predicate fails
//     ///
//     /// # Example
//     /// ```
//     /// use rust_key_paths::{AKp, Kp, KpType};
//     /// struct User { age: i32 }
//     /// let user = User { age: 30 };
//     /// let age_kp = KpType::new(|u: &User| Some(&u.age), |_| None);
//     /// let age_akp = AKp::new(age_kp);
//     /// let adult_akp = age_akp.filter::<User, i32, _>(|age| *age >= 18);
//     /// ```
//     pub fn filter<Root, Value, F>(&self, predicate: F) -> AKp
//     where
//         Root: Any + 'static,
//         Value: Any + 'static,
//         F: Fn(&Value) -> bool + 'static,
//     {
//         let orig_root_type_id = self.root_type_id;
//         let orig_value_type_id = self.value_type_id;
//         let getter = self.getter.clone();

//         AKp {
//             getter: Rc::new(move |any_root: &dyn Any| {
//                 // Check root type matches
//                 if any_root.type_id() == orig_root_type_id {
//                     getter(any_root).filter(|any_value| {
//                         // Type check value and apply predicate
//                         if orig_value_type_id == TypeId::of::<Value>() {
//                             any_value
//                                 .downcast_ref::<Value>()
//                                 .map(|val| predicate(val))
//                                 .unwrap_or(false)
//                         } else {
//                             false
//                         }
//                     })
//                 } else {
//                     None
//                 }
//             }),
//             root_type_id: orig_root_type_id,
//             value_type_id: orig_value_type_id,
//         }
//     }
// }

// impl fmt::Debug for AKp {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         f.debug_struct("AKp")
//             .field("root_type_id", &self.root_type_id)
//             .field("value_type_id", &self.value_type_id)
//             .finish_non_exhaustive()
//     }
// }

// impl fmt::Display for AKp {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(
//             f,
//             "AKp(root_type_id={:?}, value_type_id={:?})",
//             self.root_type_id, self.value_type_id
//         )
//     }
// }

// pub struct PKp<Root> {
//     getter: Rc<dyn for<'r> Fn(&'r Root) -> Option<&'r dyn Any>>,
//     value_type_id: TypeId,
//     _phantom: std::marker::PhantomData<Root>,
// }

// impl<Root> PKp<Root>
// where
//     Root: 'static,
// {
//     /// Create a new PKp from a KpType (the common reference-based keypath)
//     pub fn new<'a, V>(keypath: KpType<'a, Root, V>) -> Self
//     where
//         V: Any + 'static,
//     {
//         let value_type_id = TypeId::of::<V>();
//         let getter_fn = keypath.get;

//         Self {
//             getter: Rc::new(move |root: &Root| getter_fn(root).map(|val: &V| val as &dyn Any)),
//             value_type_id,
//             _phantom: std::marker::PhantomData,
//         }
//     }

//     /// Create a PKp from a KpType (alias for `new()`)
//     pub fn from<'a, V>(keypath: KpType<'a, Root, V>) -> Self
//     where
//         V: Any + 'static,
//     {
//         Self::new(keypath)
//     }

//     /// Get the value as a trait object
//     pub fn get<'r>(&self, root: &'r Root) -> Option<&'r dyn Any> {
//         (self.getter)(root)
//     }

//     /// Get the TypeId of the Value type
//     pub fn value_type_id(&self) -> TypeId {
//         self.value_type_id
//     }

//     /// Try to downcast the result to a specific type
//     pub fn get_as<'a, Value: Any>(&self, root: &'a Root) -> Option<&'a Value> {
//         if self.value_type_id == TypeId::of::<Value>() {
//             self.get(root).and_then(|any| any.downcast_ref::<Value>())
//         } else {
//             None
//         }
//     }

//     /// Get a human-readable name for the value type
//     pub fn kind_name(&self) -> String {
//         format!("{:?}", self.value_type_id)
//     }

//     /// Adapt this keypath to work with Arc<Root> instead of Root
//     pub fn for_arc(&self) -> PKp<Arc<Root>> {
//         let getter = self.getter.clone();
//         let value_type_id = self.value_type_id;

//         PKp {
//             getter: Rc::new(move |arc: &Arc<Root>| getter(arc.as_ref())),
//             value_type_id,
//             _phantom: std::marker::PhantomData,
//         }
//     }

//     /// Adapt this keypath to work with Box<Root> instead of Root
//     pub fn for_box(&self) -> PKp<Box<Root>> {
//         let getter = self.getter.clone();
//         let value_type_id = self.value_type_id;

//         PKp {
//             getter: Rc::new(move |boxed: &Box<Root>| getter(boxed.as_ref())),
//             value_type_id,
//             _phantom: std::marker::PhantomData,
//         }
//     }

//     /// Adapt this keypath to work with Rc<Root> instead of Root
//     pub fn for_rc(&self) -> PKp<Rc<Root>> {
//         let getter = self.getter.clone();
//         let value_type_id = self.value_type_id;

//         PKp {
//             getter: Rc::new(move |rc: &Rc<Root>| getter(rc.as_ref())),
//             value_type_id,
//             _phantom: std::marker::PhantomData,
//         }
//     }

//     /// Adapt this keypath to work with Option<Root> instead of Root
//     pub fn for_option(&self) -> PKp<Option<Root>> {
//         let getter = self.getter.clone();
//         let value_type_id = self.value_type_id;

//         PKp {
//             getter: Rc::new(move |opt: &Option<Root>| opt.as_ref().and_then(|root| getter(root))),
//             value_type_id,
//             _phantom: std::marker::PhantomData,
//         }
//     }

//     /// Adapt this keypath to work with Result<Root, E> instead of Root
//     pub fn for_result<E>(&self) -> PKp<Result<Root, E>>
//     where
//         E: 'static,
//     {
//         let getter = self.getter.clone();
//         let value_type_id = self.value_type_id;

//         PKp {
//             getter: Rc::new(move |result: &Result<Root, E>| {
//                 result.as_ref().ok().and_then(|root| getter(root))
//             }),
//             value_type_id,
//             _phantom: std::marker::PhantomData,
//         }
//     }

//     /// Map the value through a transformation function
//     /// The mapped value must also implement Any for type erasure
//     ///
//     /// # Example
//     /// ```
//     /// use rust_key_paths::{Kp, KpType, PKp};
//     /// struct User { name: String }
//     /// let user = User { name: "Akash".to_string() };
//     /// let name_kp = KpType::new(|u: &User| Some(&u.name), |_| None);
//     /// let name_pkp = PKp::new(name_kp);
//     /// let len_pkp = name_pkp.map::<String, _, _>(|s| s.len());
//     /// assert_eq!(len_pkp.get_as::<usize>(&user), Some(&5));
//     /// ```
//     pub fn map<OrigValue, MappedValue, F>(&self, mapper: F) -> PKp<Root>
//     where
//         OrigValue: Any + 'static,
//         MappedValue: Any + 'static,
//         F: Fn(&OrigValue) -> MappedValue + 'static,
//     {
//         let orig_type_id = self.value_type_id;
//         let getter = self.getter.clone();
//         let mapped_type_id = TypeId::of::<MappedValue>();

//         PKp {
//             getter: Rc::new(move |root: &Root| {
//                 getter(root).and_then(|any_value| {
//                     // Verify the original type matches
//                     if orig_type_id == TypeId::of::<OrigValue>() {
//                         any_value.downcast_ref::<OrigValue>().map(|orig_val| {
//                             let mapped = mapper(orig_val);
//                             // Box the mapped value and return as &dyn Any
//                             // Note: This creates a new allocation
//                             Box::leak(Box::new(mapped)) as &dyn Any
//                         })
//                     } else {
//                         None
//                     }
//                 })
//             }),
//             value_type_id: mapped_type_id,
//             _phantom: std::marker::PhantomData,
//         }
//     }

//     /// Filter the value based on a predicate with type checking
//     /// Returns None if the type doesn't match or predicate fails
//     ///
//     /// # Example
//     /// ```
//     /// use rust_key_paths::{Kp, KpType, PKp};
//     /// struct User { age: i32 }
//     /// let user = User { age: 30 };
//     /// let age_kp = KpType::new(|u: &User| Some(&u.age), |_| None);
//     /// let age_pkp = PKp::new(age_kp);
//     /// let adult_pkp = age_pkp.filter::<i32, _>(|age| *age >= 18);
//     /// assert_eq!(adult_pkp.get_as::<i32>(&user), Some(&30));
//     /// ```
//     pub fn filter<Value, F>(&self, predicate: F) -> PKp<Root>
//     where
//         Value: Any + 'static,
//         F: Fn(&Value) -> bool + 'static,
//     {
//         let orig_type_id = self.value_type_id;
//         let getter = self.getter.clone();

//         PKp {
//             getter: Rc::new(move |root: &Root| {
//                 getter(root).filter(|any_value| {
//                     // Type check and apply predicate
//                     if orig_type_id == TypeId::of::<Value>() {
//                         any_value
//                             .downcast_ref::<Value>()
//                             .map(|val| predicate(val))
//                             .unwrap_or(false)
//                     } else {
//                         false
//                     }
//                 })
//             }),
//             value_type_id: orig_type_id,
//             _phantom: std::marker::PhantomData,
//         }
//     }
// }

// impl<Root> fmt::Debug for PKp<Root> {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         f.debug_struct("PKp")
//             .field("root_ty", &std::any::type_name::<Root>())
//             .field("value_type_id", &self.value_type_id)
//             .finish_non_exhaustive()
//     }
// }

// impl<Root> fmt::Display for PKp<Root> {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(
//             f,
//             "PKp<{}, value_type_id={:?}>",
//             std::any::type_name::<Root>(),
//             self.value_type_id
//         )
//     }
// }

// ========== ANY KEYPATHS (Hide Both Root and Value Types) ==========
pub trait KpTrait<R, V>: KpReadable<R, V> + KPWritable<R, V> {
    fn type_id_of_root() -> std::any::TypeId
    where
        R: 'static,
    {
        std::any::TypeId::of::<R>()
    }
    fn type_id_of_value() -> std::any::TypeId
    where
        V: 'static,
    {
        std::any::TypeId::of::<V>()
    }

    fn then<SV, G2, S2>(
        self,
        next: Kp<V, SV, G2, S2>,
    ) -> Kp<
        R,
        SV,
        impl for<'r> Fn(&'r R) -> Option<&'r SV>,
        impl for<'r> Fn(&'r mut R) -> Option<&'r mut SV>,
    >
    where
        G2: for<'r> Fn(&'r V) -> Option<&'r SV>,
        S2: for<'r> Fn(&'r mut V) -> Option<&'r mut SV>,
        for<'r> V: 'r;
}

pub trait KpReadable<R, V> {
    fn get<'a>(&self, root: &'a R) -> Option<&'a V>;
}
pub trait KPWritable<R, V> {
    fn set<'a>(&self, root: &'a mut R) -> Option<&'a mut V>;
}

// pub trait ChainExt<R, V, Root, Value, MutRoot, MutValue> {
//     /// Chain with a sync [crate::lock::LockKp]. Use `.get(root)` / `.get_mut(root)` on the returned keypath.
//     fn then_lock<
//         Lock,
//         Mid,
//         V2,
//         LockValue,
//         MidValue,
//         Value2,
//         MutLock,
//         MutMid,
//         MutValue2,
//         G1,
//         S1,
//         L,
//         G2,
//         S2,
//     >(
//         self,
//         lock_kp: crate::lock::LockKp<
//             V,
//             Lock,
//             Mid,
//             V2,
//             Value,
//             LockValue,
//             MidValue,
//             Value2,
//             MutValue,
//             MutLock,
//             MutMid,
//             MutValue2,
//             G1,
//             S1,
//             L,
//             G2,
//             S2,
//         >,
//     ) -> crate::lock::KpThenLockKp<
//         R,
//         V,
//         V2,
//         Root,
//         Value,
//         Value2,
//         MutRoot,
//         MutValue,
//         MutValue2,
//         Self,
//         crate::lock::LockKp<
//             V,
//             Lock,
//             Mid,
//             V2,
//             Value,
//             LockValue,
//             MidValue,
//             Value2,
//             MutValue,
//             MutLock,
//             MutMid,
//             MutValue2,
//             G1,
//             S1,
//             L,
//             G2,
//             S2,
//         >,
//     >
//     where
//         V: 'static + Clone,
//         V2: 'static,
//         Value: std::borrow::Borrow<V>,
//         Value2: std::borrow::Borrow<V2>,
//         MutValue: std::borrow::BorrowMut<V>,
//         MutValue2: std::borrow::BorrowMut<V2>,
//         LockValue: std::borrow::Borrow<Lock>,
//         MidValue: std::borrow::Borrow<Mid>,
//         MutLock: std::borrow::BorrowMut<Lock>,
//         MutMid: std::borrow::BorrowMut<Mid>,
//         G1: Fn(Value) -> Option<LockValue>,
//         S1: Fn(MutValue) -> Option<MutLock>,
//         L: crate::lock::LockAccess<Lock, MidValue> + crate::lock::LockAccess<Lock, MutMid>,
//         G2: Fn(MidValue) -> Option<Value2>,
//         S2: Fn(MutMid) -> Option<MutValue2>,
//         Self: Sized;

//     /// Chain with a #[pin] Future field await (pin_project pattern). Use `.get_mut(&mut root).await` on the returned keypath.
//     #[cfg(feature = "pin_project")]
//     fn then_pin_future<Struct, Output, L>(
//         self,
//         pin_fut: L,
//     ) -> crate::pin::KpThenPinFuture<R, Struct, Output, Root, MutRoot, Value, MutValue, Self, L>
//     where
//         Struct: Unpin + 'static,
//         Output: 'static,
//         Value: std::borrow::Borrow<Struct>,
//         MutValue: std::borrow::BorrowMut<Struct>,
//         L: crate::pin::PinFutureAwaitLike<Struct, Output> + Sync,
//         Self: Sized;

//     /// Chain with an async keypath (e.g. [crate::async_lock::AsyncLockKp]). Use `.get(&root).await` on the returned keypath.
//     fn then_async<AsyncKp>(
//         self,
//         async_kp: AsyncKp,
//     ) -> crate::async_lock::KpThenAsyncKeyPath<
//         R,
//         V,
//         <AsyncKp::Value as KeyPathValueTarget>::Target,
//         Root,
//         Value,
//         AsyncKp::Value,
//         MutRoot,
//         MutValue,
//         AsyncKp::MutValue,
//         Self,
//         AsyncKp,
//     >
//     where
//         Value: std::borrow::Borrow<V>,
//         MutValue: std::borrow::BorrowMut<V>,
//         AsyncKp: crate::async_lock::AsyncKeyPathLike<Value, MutValue>,
//         AsyncKp::Value: KeyPathValueTarget
//             + std::borrow::Borrow<<AsyncKp::Value as KeyPathValueTarget>::Target>,
//         AsyncKp::MutValue: std::borrow::BorrowMut<<AsyncKp::Value as KeyPathValueTarget>::Target>,
//         <AsyncKp::Value as KeyPathValueTarget>::Target: 'static,
//         Self: Sized;
// }

// impl<R, V, Root, Value, MutRoot, MutValue, G, S> ChainExt<R, V, Root, Value, MutRoot, MutValue>
//     for Kp<R, V, Root, Value, MutRoot, MutValue, G, S>
// where
//     Root: std::borrow::Borrow<R>,
//     Value: std::borrow::Borrow<V>,
//     MutRoot: std::borrow::BorrowMut<R>,
//     MutValue: std::borrow::BorrowMut<V>,
//     G: Fn(Root) -> Option<Value>,
//     S: Fn(MutRoot) -> Option<MutValue>,
// {
//     fn then_lock<
//         Lock,
//         Mid,
//         V2,
//         LockValue,
//         MidValue,
//         Value2,
//         MutLock,
//         MutMid,
//         MutValue2,
//         G1,
//         S1,
//         L,
//         G2,
//         S2,
//     >(
//         self,
//         lock_kp: crate::lock::LockKp<
//             V,
//             Lock,
//             Mid,
//             V2,
//             Value,
//             LockValue,
//             MidValue,
//             Value2,
//             MutValue,
//             MutLock,
//             MutMid,
//             MutValue2,
//             G1,
//             S1,
//             L,
//             G2,
//             S2,
//         >,
//     ) -> crate::lock::KpThenLockKp<
//         R,
//         V,
//         V2,
//         Root,
//         Value,
//         Value2,
//         MutRoot,
//         MutValue,
//         MutValue2,
//         Self,
//         crate::lock::LockKp<
//             V,
//             Lock,
//             Mid,
//             V2,
//             Value,
//             LockValue,
//             MidValue,
//             Value2,
//             MutValue,
//             MutLock,
//             MutMid,
//             MutValue2,
//             G1,
//             S1,
//             L,
//             G2,
//             S2,
//         >,
//     >
//     where
//         V: 'static + Clone,
//         V2: 'static,
//         Value: std::borrow::Borrow<V>,
//         Value2: std::borrow::Borrow<V2>,
//         MutValue: std::borrow::BorrowMut<V>,
//         MutValue2: std::borrow::BorrowMut<V2>,
//         LockValue: std::borrow::Borrow<Lock>,
//         MidValue: std::borrow::Borrow<Mid>,
//         MutLock: std::borrow::BorrowMut<Lock>,
//         MutMid: std::borrow::BorrowMut<Mid>,
//         G1: Fn(Value) -> Option<LockValue>,
//         S1: Fn(MutValue) -> Option<MutLock>,
//         L: crate::lock::LockAccess<Lock, MidValue> + crate::lock::LockAccess<Lock, MutMid>,
//         G2: Fn(MidValue) -> Option<Value2>,
//         S2: Fn(MutMid) -> Option<MutValue2>,
//     {
//         let first = self;
//         let second = lock_kp;

//         crate::lock::KpThenLockKp {
//             first: first,
//             second: second,
//             _p: std::marker::PhantomData,
//         }
//     }

//     #[cfg(feature = "pin_project")]
//     fn then_pin_future<Struct, Output, L>(
//         self,
//         pin_fut: L,
//     ) -> crate::pin::KpThenPinFuture<R, Struct, Output, Root, MutRoot, Value, MutValue, Self, L>
//     where
//         Struct: Unpin + 'static,
//         Output: 'static,
//         Value: std::borrow::Borrow<Struct>,
//         MutValue: std::borrow::BorrowMut<Struct>,
//         L: crate::pin::PinFutureAwaitLike<Struct, Output> + Sync,
//     {
//         let first = self;
//         let second = pin_fut;

//         crate::pin::KpThenPinFuture {
//             first: first,
//             second: second,
//             _p: std::marker::PhantomData,
//         }
//     }

//     fn then_async<AsyncKp>(
//         self,
//         async_kp: AsyncKp,
//     ) -> crate::async_lock::KpThenAsyncKeyPath<
//         R,
//         V,
//         <AsyncKp::Value as KeyPathValueTarget>::Target,
//         Root,
//         Value,
//         AsyncKp::Value,
//         MutRoot,
//         MutValue,
//         AsyncKp::MutValue,
//         Self,
//         AsyncKp,
//     >
//     where
//         Value: std::borrow::Borrow<V>,
//         MutValue: std::borrow::BorrowMut<V>,
//         AsyncKp: crate::async_lock::AsyncKeyPathLike<Value, MutValue>,
//         AsyncKp::Value: KeyPathValueTarget
//             + std::borrow::Borrow<<AsyncKp::Value as KeyPathValueTarget>::Target>,
//         AsyncKp::MutValue: std::borrow::BorrowMut<<AsyncKp::Value as KeyPathValueTarget>::Target>,
//         <AsyncKp::Value as KeyPathValueTarget>::Target: 'static,
//     {
//         let first = self;
//         let second = async_kp;

//         crate::async_lock::KpThenAsyncKeyPath {
//             first: first,
//             second: second,
//             _p: std::marker::PhantomData,
//         }
//     }
// }

// pub trait AccessorTrait<R, V, Root, Value, MutRoot, MutValue, G, S> {
//     /// Like [get](Kp::get), but takes an optional root: returns `None` if `root` is `None`, otherwise the result of the getter.
//     fn get_optional(&self, root: Option<Root>) -> Option<Value>;
//     //  {
//     //     root.and_then(|r| self.get(r))
//     // }

//     /// Like [get_mut](Kp::get_mut), but takes an optional root: returns `None` if `root` is `None`, otherwise the result of the setter.
//     fn get_mut_optional(&self, root: Option<MutRoot>) -> Option<MutValue>;
//     // {
//     //     root.and_then(|r| self.get_mut(r))
//     // }

//     /// Returns the value if the keypath succeeds, otherwise calls `f` and returns its result.
//     fn get_or_else<F>(&self, root: Root, f: F) -> Value
//     where
//         F: FnOnce() -> Value;
//     // {
//     //     self.get(root).unwrap_or_else(f)
//     // }

//     /// Returns the mutable value if the keypath succeeds, otherwise calls `f` and returns its result.
//     #[inline]
//     fn get_mut_or_else<F>(&self, root: MutRoot, f: F) -> MutValue
//     where
//         F: FnOnce() -> MutValue;
//     // {
//     //     self.get_mut(root).unwrap_or_else(f)
//     // }
// }

// pub trait CoercionTrait<R, V, Root, Value, MutRoot, MutValue, G, S>
// where
//     Root: std::borrow::Borrow<R>,
//     Value: std::borrow::Borrow<V>,
//     MutRoot: std::borrow::BorrowMut<R>,
//     MutValue: std::borrow::BorrowMut<V>,
//     G: Fn(Root) -> Option<Value>,
//     S: Fn(MutRoot) -> Option<MutValue>,
// {
//     fn for_arc<'b>(
//         &self,
//     ) -> Kp<
//         std::sync::Arc<R>,
//         V,
//         std::sync::Arc<R>,
//         Value,
//         std::sync::Arc<R>,
//         MutValue,
//         impl Fn(std::sync::Arc<R>) -> Option<Value>,
//         impl Fn(std::sync::Arc<R>) -> Option<MutValue>,
//     >
//     where
//         R: 'b,
//         V: 'b,
//         Root: for<'a> From<&'a R>,
//         MutRoot: for<'a> From<&'a mut R>;

//     fn for_box<'a>(
//         &self,
//     ) -> Kp<
//         Box<R>,
//         V,
//         Box<R>,
//         Value,
//         Box<R>,
//         MutValue,
//         impl Fn(Box<R>) -> Option<Value>,
//         impl Fn(Box<R>) -> Option<MutValue>,
//     >
//     where
//         R: 'a,
//         V: 'a,
//         Root: for<'b> From<&'b R>,
//         MutRoot: for<'b> From<&'b mut R>;

//     /// set fn is converting fn pointer to Fn closure
//     fn into_set(self) -> impl Fn(MutRoot) -> Option<MutValue>;

//     /// get fn is converting fn pointer to Fn closure
//     fn into_get(self) -> impl Fn(Root) -> Option<Value>;
// }

pub trait HofTrait<R, V, G, S>: KpTrait<R, V>
where
    G: for<'r> Fn(&'r R) -> Option<&'r V>,
    S: for<'r> Fn(&'r mut R) -> Option<&'r mut V>,
{
    /// Maps the keypath value into an owned transformed value.
    fn map<MappedValue, F>(&self, mapper: F) -> impl for<'r> Fn(&'r R) -> Option<MappedValue> + '_
    where
        F: Fn(&V) -> MappedValue + 'static,
    {
        move |root: &R| self.get(root).map(&mapper)
    }

    /// Filters values using a predicate and returns a new keypath.
    fn filter<F>(
        &self,
        predicate: F,
    ) -> Kp<
        R,
        V,
        impl for<'r> Fn(&'r R) -> Option<&'r V> + '_,
        impl for<'r> Fn(&'r mut R) -> Option<&'r mut V> + '_,
    >
    where
        F: Fn(&V) -> bool + Clone + 'static,
    {
        let predicate_for_get = predicate.clone();
        Kp::new(
            move |root: &R| self.get(root).filter(|value| predicate_for_get(value)),
            move |root: &mut R| self.set(root).filter(|value| predicate(value)),
        )
    }

    /// Maps and flattens the keypath value when mapper returns `Option`.
    fn filter_map<MappedValue, F>(
        &self,
        mapper: F,
    ) -> impl for<'r> Fn(&'r R) -> Option<MappedValue> + '_
    where
        F: Fn(&V) -> Option<MappedValue> + 'static,
    {
        move |root: &R| self.get(root).and_then(&mapper)
    }

    /// Runs `inspector` for side effects and returns a keypath for the same value.
    fn inspect<F>(
        &self,
        inspector: F,
    ) -> Kp<
        R,
        V,
        impl for<'r> Fn(&'r R) -> Option<&'r V> + '_,
        impl for<'r> Fn(&'r mut R) -> Option<&'r mut V> + '_,
    >
    where
        F: Fn(&V) + Clone + 'static,
    {
        let inspector_for_get = inspector.clone();
        Kp::new(
            move |root: &R| {
                self.get(root).inspect(|value| {
                    inspector_for_get(value);
                })
            },
            move |root: &mut R| {
                self.set(root).inspect(|value| {
                    inspector(value);
                })
            },
        )
    }

    /// Flat map - maps to an iterator and flattens.
    fn flat_map<I, Item, F>(&self, mapper: F) -> impl for<'r> Fn(&'r R) -> Vec<Item> + '_
    where
        F: Fn(&V) -> I + 'static,
        I: IntoIterator<Item = Item>,
    {
        move |root: &R| {
            self.get(root)
                .map(|value| mapper(value).into_iter().collect())
                .unwrap_or_else(Vec::new)
        }
    }

    /// Fold/reduce the value using an accumulator function.
    fn fold_value<Acc, F>(&self, init: Acc, folder: F) -> impl for<'r> Fn(&'r R) -> Acc + '_
    where
        F: Fn(Acc, &V) -> Acc + 'static,
        Acc: Copy + 'static,
    {
        move |root: &R| {
            self.get(root)
                .map(|value| folder(init, value))
                .unwrap_or(init)
        }
    }

    /// Check if the value satisfies a predicate.
    fn any<F>(&self, predicate: F) -> impl for<'r> Fn(&'r R) -> bool + '_
    where
        F: Fn(&V) -> bool + 'static,
    {
        move |root: &R| self.get(root).map(&predicate).unwrap_or(false)
    }

    /// Check if the value satisfies a predicate; returns true for missing values.
    fn all<F>(&self, predicate: F) -> impl for<'r> Fn(&'r R) -> bool + '_
    where
        F: Fn(&V) -> bool + 'static,
    {
        move |root: &R| self.get(root).map(&predicate).unwrap_or(true)
    }

    /// Count elements in a collection-like value.
    fn count_items<F>(&self, counter: F) -> impl for<'r> Fn(&'r R) -> Option<usize> + '_
    where
        F: Fn(&V) -> usize + 'static,
    {
        move |root: &R| self.get(root).map(&counter)
    }

    /// Find an item in a collection-like value.
    fn find_in<Item, F>(&self, finder: F) -> impl for<'r> Fn(&'r R) -> Option<Item> + '_
    where
        F: Fn(&V) -> Option<Item> + 'static,
    {
        move |root: &R| self.get(root).and_then(&finder)
    }

    /// Take first N elements from a collection-like value.
    fn take<Output, F>(&self, n: usize, taker: F) -> impl for<'r> Fn(&'r R) -> Option<Output> + '_
    where
        F: Fn(&V, usize) -> Output + 'static,
    {
        move |root: &R| self.get(root).map(|value| taker(value, n))
    }

    /// Skip first N elements from a collection-like value.
    fn skip<Output, F>(&self, n: usize, skipper: F) -> impl for<'r> Fn(&'r R) -> Option<Output> + '_
    where
        F: Fn(&V, usize) -> Output + 'static,
    {
        move |root: &R| self.get(root).map(|value| skipper(value, n))
    }

    /// Partition a collection-like value into two groups.
    fn partition_value<Output, F>(
        &self,
        partitioner: F,
    ) -> impl for<'r> Fn(&'r R) -> Option<Output> + '_
    where
        F: Fn(&V) -> Output + 'static,
    {
        move |root: &R| self.get(root).map(&partitioner)
    }

    /// Get min value from a collection-like value.
    fn min_value<Item, F>(&self, min_fn: F) -> impl for<'r> Fn(&'r R) -> Option<Item> + '_
    where
        F: Fn(&V) -> Option<Item> + 'static,
    {
        move |root: &R| self.get(root).and_then(&min_fn)
    }

    /// Get max value from a collection-like value.
    fn max_value<Item, F>(&self, max_fn: F) -> impl for<'r> Fn(&'r R) -> Option<Item> + '_
    where
        F: Fn(&V) -> Option<Item> + 'static,
    {
        move |root: &R| self.get(root).and_then(&max_fn)
    }

    /// Sum values from a collection-like value.
    fn sum_value<Sum, F>(&self, sum_fn: F) -> impl for<'r> Fn(&'r R) -> Option<Sum> + '_
    where
        F: Fn(&V) -> Sum + 'static,
    {
        move |root: &R| self.get(root).map(&sum_fn)
    }
}

impl<R, V, G, S> KPWritable<R, V> for Kp<R, V, G, S>
where
    G: for<'r> Fn(&'r R) -> Option<&'r V>,
    S: for<'r> Fn(&'r mut R) -> Option<&'r mut V>,
{
    fn set<'a>(&self, root: &'a mut R) -> Option<&'a mut V> {
        (self.set)(root)
    }
}

impl<R, V, G, S> KpReadable<R, V> for Kp<R, V, G, S>
where
    G: for<'r> Fn(&'r R) -> Option<&'r V>,
    S: for<'r> Fn(&'r mut R) -> Option<&'r mut V>,
{
    fn get<'a>(&self, root: &'a R) -> Option<&'a V> {
        (self.get)(root)
    }
}

impl<R, V, G, S> KpTrait<R, V> for Kp<R, V, G, S>
where
    G: for<'r> Fn(&'r R) -> Option<&'r V>,
    S: for<'r> Fn(&'r mut R) -> Option<&'r mut V>,
{
    #[inline]
    fn then<SV, G2, S2>(
        self,
        next: Kp<V, SV, G2, S2>,
    ) -> Kp<
        R,
        SV,
        impl for<'r> Fn(&'r R) -> Option<&'r SV>,
        impl for<'r> Fn(&'r mut R) -> Option<&'r mut SV>,
    >
    where
        G2: for<'r> Fn(&'r V) -> Option<&'r SV>,
        S2: for<'r> Fn(&'r mut V) -> Option<&'r mut SV>,
        for<'r> V: 'r,
    {
        let first_get = self.get;
        let first_set = self.set;
        let second_get = next.get;
        let second_set = next.set;

        Kp::new(
            move |root| first_get(root).and_then(|value| second_get(value)),
            move |root| first_set(root).and_then(|value| second_set(value)),
        )
    }

    fn type_id_of_root() -> std::any::TypeId
    where
        R: 'static,
    {
        std::any::TypeId::of::<R>()
    }

    fn type_id_of_value() -> std::any::TypeId
    where
        V: 'static,
    {
        std::any::TypeId::of::<V>()
    }

    // fn get(&self, root: Root) -> Option<Value> {
    //     (self.get)(root)
    // }

    // fn get_mut(&self, root: MutRoot) -> Option<MutValue> {
    //     (self.set)(root)
    // }
}

// impl<R, V, Root, Value, MutRoot, MutValue, G, S>
//     CoercionTrait<R, V, Root, Value, MutRoot, MutValue, G, S>
//     for Kp<R, V, Root, Value, MutRoot, MutValue, G, S>
// where
//     Root: std::borrow::Borrow<R>,
//     Value: std::borrow::Borrow<V>,
//     MutRoot: std::borrow::BorrowMut<R>,
//     MutValue: std::borrow::BorrowMut<V>,
//     G: Fn(Root) -> Option<Value>,
//     S: Fn(MutRoot) -> Option<MutValue>,
// {
//     fn for_arc<'b>(
//         &self,
//     ) -> Kp<
//         std::sync::Arc<R>,
//         V,
//         std::sync::Arc<R>,
//         Value,
//         std::sync::Arc<R>,
//         MutValue,
//         impl Fn(std::sync::Arc<R>) -> Option<Value>,
//         impl Fn(std::sync::Arc<R>) -> Option<MutValue>,
//     >
//     where
//         R: 'b,
//         V: 'b,
//         Root: for<'a> From<&'a R>,
//         MutRoot: for<'a> From<&'a mut R>,
//     {
//         Kp::new(
//             move |arc_root: std::sync::Arc<R>| {
//                 let r_ref: &R = &*arc_root;
//                 (self.get)(Root::from(r_ref))
//             },
//             move |mut arc_root: std::sync::Arc<R>| {
//                 // Get mutable reference only if we have exclusive ownership
//                 std::sync::Arc::get_mut(&mut arc_root)
//                     .and_then(|r_mut| (self.set)(MutRoot::from(r_mut)))
//             },
//         )
//     }

//     fn for_box<'a>(
//         &self,
//     ) -> Kp<
//         Box<R>,
//         V,
//         Box<R>,
//         Value,
//         Box<R>,
//         MutValue,
//         impl Fn(Box<R>) -> Option<Value>,
//         impl Fn(Box<R>) -> Option<MutValue>,
//     >
//     where
//         R: 'a,
//         V: 'a,
//         Root: for<'b> From<&'b R>,
//         MutRoot: for<'b> From<&'b mut R>,
//     {
//         Kp::new(
//             move |r: Box<R>| {
//                 let r_ref: &R = r.as_ref();
//                 (self.get)(Root::from(r_ref))
//             },
//             move |mut r: Box<R>| {
//                 // Get mutable reference only if we have exclusive ownership
//                 (self.set)(MutRoot::from(r.as_mut()))
//             },
//         )
//     }

//     /// set fn is converting fn pointer to Fn closure
//     #[inline]
//     fn into_set(self) -> impl Fn(MutRoot) -> Option<MutValue> {
//         self.set
//     }

//     /// get fn is converting fn pointer to Fn closure
//     #[inline]
//     fn into_get(self) -> impl Fn(Root) -> Option<Value> {
//         self.get
//     }
// }

impl<R, V, G, S> HofTrait<R, V, G, S> for Kp<R, V, G, S>
where
    G: for<'r> Fn(&'r R) -> Option<&'r V>,
    S: for<'r> Fn(&'r mut R) -> Option<&'r mut V>,
{
}

// impl<R, V, Root, Value, MutRoot, MutValue, G, S>
//     AccessorTrait<R, V, Root, Value, MutRoot, MutValue, G, S>
//     for Kp<R, V, Root, Value, MutRoot, MutValue, G, S>
// where
//     Root: std::borrow::Borrow<R>,
//     Value: std::borrow::Borrow<V>,
//     MutRoot: std::borrow::BorrowMut<R>,
//     MutValue: std::borrow::BorrowMut<V>,
//     G: Fn(Root) -> Option<Value>,
//     S: Fn(MutRoot) -> Option<MutValue>,
// {
//     /// Like [get](Kp::get), but takes an optional root: returns `None` if `root` is `None`, otherwise the result of the getter.
//     #[inline]
//     fn get_optional(&self, root: Option<Root>) -> Option<Value> {
//         root.and_then(|r| (self.get)(r))
//     }

//     /// Like [get_mut](Kp::get_mut), but takes an optional root: returns `None` if `root` is `None`, otherwise the result of the setter.
//     #[inline]
//     fn get_mut_optional(&self, root: Option<MutRoot>) -> Option<MutValue> {
//         root.and_then(|r| (self.set)(r))
//     }

//     /// Returns the value if the keypath succeeds, otherwise calls `f` and returns its result.
//     #[inline]
//     fn get_or_else<F>(&self, root: Root, f: F) -> Value
//     where
//         F: FnOnce() -> Value,
//     {
//         (self.get)(root).unwrap_or_else(f)
//     }

//     /// Returns the mutable value if the keypath succeeds, otherwise calls `f` and returns its result.
//     #[inline]
//     fn get_mut_or_else<F>(&self, root: MutRoot, f: F) -> MutValue
//     where
//         F: FnOnce() -> MutValue,
//     {
//         (self.set)(root).unwrap_or_else(f)
//     }
// }

/// AKp (AnyKeyPath) - Hides both Root and Value types
/// Most flexible keypath type for heterogeneous collections
/// Uses dynamic dispatch and type checking at runtime
///
/// # Mutation: get vs get_mut (setter path)
///
/// - **[get](Kp::get)** uses the `get` closure (getter): `Fn(Root) -> Option<Value>`
/// - **[get_mut](Kp::get_mut)** uses the `set` closure (setter): `Fn(MutRoot) -> Option<MutValue>`
///
/// When mutating through a Kp, the **setter path** is used—`get_mut` invokes the `set` closure,
/// not the `get` closure. The getter is for read-only access only.
#[derive(Clone, Debug)]
pub struct Kp<R, V, G, S>
where
    G: for<'r> Fn(&'r R) -> Option<&'r V>,
    S: for<'r> Fn(&'r mut R) -> Option<&'r mut V>,
{
    /// Getter closure: used by [Kp::get] for read-only access.
    pub get: G,
    /// Setter closure: used by [Kp::get_mut] for mutation.
    pub set: S,
    _p: std::marker::PhantomData<(R, V)>,
}

impl<R, V, G, S> Kp<R, V, G, S>
where
    G: for<'r> Fn(&'r R) -> Option<&'r V>,
    S: for<'r> Fn(&'r mut R) -> Option<&'r mut V>,
{
    pub fn new(get: G, set: S) -> Self {
        Self {
            get: get,
            set: set,
            _p: std::marker::PhantomData,
        }
    }

    // #[inline]
    // pub fn get(&self, root: Root) -> Option<Value> {
    //     (self.get)(root)
    // }

    // #[inline]
    // pub fn get_mut(&self, root: MutRoot) -> Option<MutValue> {
    //     (self.set)(root)
    // }

    #[inline]
    pub fn then<SV, G2, S2>(
        self,
        next: Kp<V, SV, G2, S2>,
    ) -> Kp<
        R,
        SV,
        impl for<'r> Fn(&'r R) -> Option<&'r SV>,
        impl for<'r> Fn(&'r mut R) -> Option<&'r mut SV>,
    >
    where
        G2: for<'r> Fn(&'r V) -> Option<&'r SV>,
        S2: for<'r> Fn(&'r mut V) -> Option<&'r mut SV>,
        for<'r> V: 'r,
    {
        let first_get = self.get;
        let first_set = self.set;
        let second_get = next.get;
        let second_set = next.set;

        Kp::new(
            move |root| first_get(root).and_then(|value| second_get(value)),
            move |root| first_set(root).and_then(|value| second_set(value)),
        )
    }

    // #[inline]
    // pub fn to_dynamic(self) -> KpDynamic<R, V> {
    //     self.into()
    // }

    pub fn identity() -> Kp<
        R,
        R,
        impl for<'r> Fn(&'r R) -> Option<&'r R>,
        impl for<'r> Fn(&'r mut R) -> Option<&'r mut R>,
    > {
        Kp::new(|r| Some(r), |r| Some(r))
    }
}

// impl<R, V, Root, Value, MutRoot, MutValue, G, S> fmt::Debug
//     for Kp<R, V, Root, Value, MutRoot, MutValue, G, S>
// where
//     Root: std::borrow::Borrow<R>,
//     Value: std::borrow::Borrow<V>,
//     MutRoot: std::borrow::BorrowMut<R>,
//     MutValue: std::borrow::BorrowMut<V>,
//     G: Fn(Root) -> Option<Value>,
//     S: Fn(MutRoot) -> Option<MutValue>,
// {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         f.debug_struct("Kp")
//             .field("root_ty", &std::any::type_name::<R>())
//             .field("value_ty", &std::any::type_name::<V>())
//             .finish_non_exhaustive()
//     }
// }

// impl<R, V, Root, Value, MutRoot, MutValue, G, S> fmt::Display
//     for Kp<R, V, Root, Value, MutRoot, MutValue, G, S>
// where
//     Root: std::borrow::Borrow<R>,
//     Value: std::borrow::Borrow<V>,
//     MutRoot: std::borrow::BorrowMut<R>,
//     MutValue: std::borrow::BorrowMut<V>,
//     G: Fn(Root) -> Option<Value>,
//     S: Fn(MutRoot) -> Option<MutValue>,
// {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(
//             f,
//             "Kp<{}, {}>",
//             std::any::type_name::<R>(),
//             std::any::type_name::<V>()
//         )
//     }
// }

// /// Zip two keypaths together to create a tuple
// /// Works only with KpType (reference-based keypaths)
// ///
// /// # Example
// /// ```
// /// use rust_key_paths::{KpType, zip_kps};
// /// struct User { name: String, age: i32 }
// /// let user = User { name: "Akash".to_string(), age: 30 };
// /// let name_kp = KpType::new(|u: &User| Some(&u.name), |_| None);
// /// let age_kp = KpType::new(|u: &User| Some(&u.age), |_| None);
// /// let zipped_fn = zip_kps(&name_kp, &age_kp);
// /// assert_eq!(zipped_fn(&user), Some((&"Akash".to_string(), &30)));
// /// ```
// pub fn zip_kps<'a, RootType, Value1, Value2>(
//     kp1: &'a KpType<'a, RootType, Value1>,
//     kp2: &'a KpType<'a, RootType, Value2>,
// ) -> impl Fn(&'a RootType) -> Option<(&'a Value1, &'a Value2)> + 'a
// where
//     RootType: 'a,
//     Value1: 'a,
//     Value2: 'a,
// {
//     move |root: &'a RootType| {
//         let val1 = (kp1.get)(root)?;
//         let val2 = (kp2.get)(root)?;
//         Some((val1, val2))
//     }
// }

// ========== ENUM KEYPATHS ==========

/// EnumKp - A keypath for enum variants that supports both extraction and embedding
/// Leverages the existing Kp architecture where optionals are built-in via Option<Value>
///
/// This struct serves dual purposes:
/// 1. As a concrete keypath instance for extracting and embedding enum variants
/// 2. As a namespace for static factory methods: `EnumKp::for_ok()`, `EnumKp::for_some()`, etc.
pub struct EnumKp<Enum, Variant, G, S, E>
where
    G: for<'r> Fn(&'r Enum) -> Option<&'r Variant>,
    S: for<'r> Fn(&'r mut Enum) -> Option<&'r mut Variant>,
    E: Fn(Variant) -> Enum,
{
    ex: Kp<Enum, Variant, G, S>,
    em: E,
}

// EnumKp is a functional component; Send/Sync follow from extractor and embedder.
// unsafe impl<Enum, Variant, G, S, E> Send
//     for EnumKp<Enum, Variant, G, S, E>
// where
//     G: for<'r> Fn(&'r Enum) -> Option<&'r Variant>,
//     S: for<'r> Fn(&'r mut Enum) -> Option<&'r mut Variant>,
//     E: Fn(Variant) -> Enum,
// {
// }

// unsafe impl<Enum, Variant, Root, Value, MutRoot, MutValue, G, S, E> Sync
//     for EnumKp<Enum, Variant, Root, Value, MutRoot, MutValue, G, S, E>
// where
//     Root: std::borrow::Borrow<Enum>,
//     Value: std::borrow::Borrow<Variant>,
//     MutRoot: std::borrow::BorrowMut<Enum>,
//     MutValue: std::borrow::BorrowMut<Variant>,
//     G: Fn(Root) -> Option<Value> + Sync,
//     S: Fn(MutRoot) -> Option<MutValue> + Sync,
//     E: Fn(Variant) -> Enum + Sync,
// {
// }

impl<Enum, Variant, G, S, E> EnumKp<Enum, Variant, G, S, E>
where
    G: for<'r> Fn(&'r Enum) -> Option<&'r Variant>,
    S: for<'r> Fn(&'r mut Enum) -> Option<&'r mut Variant>,
    E: Fn(Variant) -> Enum,
{
    /// Create a new EnumKp with extractor and embedder functions
    pub fn new(ex: Kp<Enum, Variant, G, S>, em: E) -> Self {
        Self { ex, em }
    }

    /// Extract the variant from an enum (returns None if wrong variant)
    pub fn get<'r>(&self, enum_value: &'r Enum) -> Option<&'r Variant> {
        (self.ex.get)(enum_value)
    }

    /// Extract the variant mutably from an enum (returns None if wrong variant)
    pub fn set<'r>(&self, enum_value: &'r mut Enum) -> Option<&'r mut Variant> {
        (self.ex.set)(enum_value)
    }

    /// Embed a value into the enum variant
    pub fn embed(&self, value: Variant) -> Enum {
        (self.em)(value)
    }

    /// Get the underlying Kp for composition with other keypaths
    pub fn as_kp(&self) -> &Kp<Enum, Variant, G, S> {
        &self.ex
    }

    /// Convert to Kp (loses embedding capability but gains composition)
    pub fn into_kp(self) -> Kp<Enum, Variant, G, S> {
        self.ex
    }

    // /// Map the variant value through a transformation function
    // ///
    // /// # Example
    // /// ```
    // /// use rust_key_paths::enum_ok;
    // /// let result: Result<String, i32> = Ok("hello".to_string());
    // /// let ok_kp = enum_ok();
    // /// let len_kp = ok_kp.map(|s: &String| s.len());
    // /// assert_eq!(len_kp.get(&result), Some(5));
    // /// ```
    // pub fn map<MappedValue, F>(
    //     &self,
    //     mapper: F,
    // ) -> EnumKp<
    //     Enum,
    //     MappedValue,
    //     impl for<'r> Fn(&'r Enum) -> Option<&'r MappedValue>,
    //     impl for<'r> Fn(&'r mut Enum) -> Option<&'r mut MappedValue>,
    //     impl for<'r> Fn(MappedValue) -> Enum,
    // >
    // where
    //     // Copy: Required because mapper is used via extractor.map() which needs it
    //     // 'static: Required because the returned EnumKp must own its closures
    //     F: Fn(&Variant) -> MappedValue + Copy + 'static,
    //     Variant: 'static,
    //     MappedValue: 'static,
    //     // Copy: Required for embedder to be captured in the panic closure
    //     E: Fn(Variant) -> Enum + Copy + 'static,
    // {
    //     let mapped_extractor = self.ex.map(mapper);

    //     // Create a new embedder that maps back
    //     // Note: This is a limitation - we can't reverse the map for embedding
    //     // So we create a placeholder that panics
    //     let new_embedder = move |_value: MappedValue| -> Enum {
    //         panic!(
    //             "Cannot embed mapped values back into enum. Use the original EnumKp for embedding."
    //         )
    //     };

    //     EnumKp::new(mapped_extractor, new_embedder)
    // }

    // /// Filter the variant value based on a predicate
    // /// Returns None if the predicate fails or if wrong variant
    // ///
    // /// # Example
    // /// ```
    // /// use rust_key_paths::enum_ok;
    // /// let result: Result<i32, String> = Ok(42);
    // /// let ok_kp = enum_ok();
    // /// let positive_kp = ok_kp.filter(|x: &i32| *x > 0);
    // /// assert_eq!(positive_kp.get(&result), Some(&42));
    // /// ```
    // pub fn filter<F>(
    //     &self,
    //     predicate: F,
    // ) -> EnumKp<
    //     Enum,
    //     Variant,
    //     Root,
    //     Value,
    //     MutRoot,
    //     MutValue,
    //     impl Fn(Root) -> Option<Value>,
    //     impl Fn(MutRoot) -> Option<MutValue>,
    //     E,
    // >
    // where
    //     // Copy: Required because predicate is used via extractor.filter() which needs it
    //     // 'static: Required because the returned EnumKp must own its closures
    //     F: Fn(&Variant) -> bool + Copy + 'static,
    //     Variant: 'static,
    //     // Copy: Required to clone embedder into the new EnumKp
    //     E: Copy,
    // {
    //     let filtered_extractor = self.extractor.filter(predicate);
    //     EnumKp::new(filtered_extractor, self.embedder)
    // }
}

// impl<Enum, Variant, Root, Value, MutRoot, MutValue, G, S, E> fmt::Debug
//     for EnumKp<Enum, Variant, Root, Value, MutRoot, MutValue, G, S, E>
// where
//     Root: std::borrow::Borrow<Enum>,
//     Value: std::borrow::Borrow<Variant>,
//     MutRoot: std::borrow::BorrowMut<Enum>,
//     MutValue: std::borrow::BorrowMut<Variant>,
//     G: Fn(Root) -> Option<Value>,
//     S: Fn(MutRoot) -> Option<MutValue>,
//     E: Fn(Variant) -> Enum,
// {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         f.debug_struct("EnumKp")
//             .field("enum_ty", &std::any::type_name::<Enum>())
//             .field("variant_ty", &std::any::type_name::<Variant>())
//             .finish_non_exhaustive()
//     }
// }

// impl<Enum, Variant, Root, Value, MutRoot, MutValue, G, S, E> fmt::Display
//     for EnumKp<Enum, Variant, Root, Value, MutRoot, MutValue, G, S, E>
// where
//     Root: std::borrow::Borrow<Enum>,
//     Value: std::borrow::Borrow<Variant>,
//     MutRoot: std::borrow::BorrowMut<Enum>,
//     MutValue: std::borrow::BorrowMut<Variant>,
//     G: Fn(Root) -> Option<Value>,
//     S: Fn(MutRoot) -> Option<MutValue>,
//     E: Fn(Variant) -> Enum,
// {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(
//             f,
//             "EnumKp<{}, {}>",
//             std::any::type_name::<Enum>(),
//             std::any::type_name::<Variant>()
//         )
//     }
// }

// // Type alias for the common case with references
// pub type EnumKpType<'a, Enum, Variant> = EnumKp<
//     Enum,
//     Variant,
//     &'a Enum,
//     &'a Variant,
//     &'a mut Enum,
//     &'a mut Variant,
//     for<'b> fn(&'b Enum) -> Option<&'b Variant>,
//     for<'b> fn(&'b mut Enum) -> Option<&'b mut Variant>,
//     fn(Variant) -> Enum,
// >;

// // Static factory functions for creating EnumKp instances
// /// Create an enum keypath with both extraction and embedding for a specific variant
// ///
// /// # Example
// /// ```
// /// use rust_key_paths::enum_variant;
// /// enum MyEnum {
// ///     A(String),
// ///     B(i32),
// /// }
// ///
// /// let kp = enum_variant(
// ///     |e: &MyEnum| match e { MyEnum::A(s) => Some(s), _ => None },
// ///     |e: &mut MyEnum| match e { MyEnum::A(s) => Some(s), _ => None },
// ///     |s: String| MyEnum::A(s)
// /// );
// /// ```
// pub fn enum_variant<'a, Enum, Variant>(
//     get: for<'b> fn(&'b Enum) -> Option<&'b Variant>,
//     set: for<'b> fn(&'b mut Enum) -> Option<&'b mut Variant>,
//     embedder: fn(Variant) -> Enum,
// ) -> EnumKpType<'a, Enum, Variant> {
//     EnumKp::new(Kp::new(get, set), embedder)
// }

// /// Extract from Result<T, E> - Ok variant
// ///
// /// # Example
// /// ```
// /// use rust_key_paths::enum_ok;
// /// let result: Result<String, i32> = Ok("success".to_string());
// /// let ok_kp = enum_ok();
// /// assert_eq!(ok_kp.get(&result), Some(&"success".to_string()));
// /// ```
// pub fn enum_ok<'a, T, E>() -> EnumKpType<'a, Result<T, E>, T> {
//     EnumKp::new(
//         Kp::new(
//             |r: &Result<T, E>| r.as_ref().ok(),
//             |r: &mut Result<T, E>| r.as_mut().ok(),
//         ),
//         |t: T| Ok(t),
//     )
// }

// /// Extract from Result<T, E> - Err variant
// ///
// /// # Example
// /// ```
// /// use rust_key_paths::enum_err;
// /// let result: Result<String, i32> = Err(42);
// /// let err_kp = enum_err();
// /// assert_eq!(err_kp.get(&result), Some(&42));
// /// ```
// pub fn enum_err<'a, T, E>() -> EnumKpType<'a, Result<T, E>, E> {
//     EnumKp::new(
//         Kp::new(
//             |r: &Result<T, E>| r.as_ref().err(),
//             |r: &mut Result<T, E>| r.as_mut().err(),
//         ),
//         |e: E| Err(e),
//     )
// }

// /// Extract from Option<T> - Some variant
// ///
// /// # Example
// /// ```
// /// use rust_key_paths::enum_some;
// /// let opt = Some("value".to_string());
// /// let some_kp = enum_some();
// /// assert_eq!(some_kp.get(&opt), Some(&"value".to_string()));
// /// ```
// pub fn enum_some<'a, T>() -> EnumKpType<'a, Option<T>, T> {
//     EnumKp::new(
//         Kp::new(|o: &Option<T>| o.as_ref(), |o: &mut Option<T>| o.as_mut()),
//         |t: T| Some(t),
//     )
// }

// // Helper functions for creating enum keypaths with type inference
// /// Create an enum keypath for a specific variant with type inference
// ///
// /// # Example
// /// ```
// /// use rust_key_paths::variant_of;
// /// enum MyEnum {
// ///     A(String),
// ///     B(i32),
// /// }
// ///
// /// let kp_a = variant_of(
// ///     |e: &MyEnum| match e { MyEnum::A(s) => Some(s), _ => None },
// ///     |e: &mut MyEnum| match e { MyEnum::A(s) => Some(s), _ => None },
// ///     |s: String| MyEnum::A(s)
// /// );
// /// ```
// pub fn variant_of<Enum, Variant, G, S>(
//     ex_get: G,
//     ex_set: S,
//     embedder: fn(Variant) -> Enum,
// ) -> EnumKpType<Enum, Variant> where
//     G: for<'r> Fn(&'r Enum) -> Option<&'r Variant>,
//     S: for<'r> Fn(&'r mut Enum) -> Option<&'r mut Variant>,
//  {
//     enum_variant(getter, setter, embedder)
// }

// // ========== CONTAINER KEYPATHS ==========

// // Helper functions for working with standard containers (Box, Arc, Rc)
// /// Create a keypath for unwrapping Box<T> -> T
// ///
// /// # Example
// /// ```
// /// use rust_key_paths::kp_box;
// /// let boxed = Box::new("value".to_string());
// /// let kp = kp_box();
// /// assert_eq!(kp.get(&boxed), Some(&"value".to_string()));
// /// ```
// pub fn kp_box<'a, T>() -> KpType<'a, Box<T>, T> {
//     Kp::new(
//         |b: &Box<T>| Some(b.as_ref()),
//         |b: &mut Box<T>| Some(b.as_mut()),
//     )
// }

// /// Create a keypath for unwrapping Arc<T> -> T (read-only)
// ///
// /// # Example
// /// ```
// /// use std::sync::Arc;
// /// use rust_key_paths::kp_arc;
// /// let arc = Arc::new("value".to_string());
// /// let kp = kp_arc();
// /// assert_eq!(kp.get(&arc), Some(&"value".to_string()));
// /// ```
// pub fn kp_arc<'a, T>() -> Kp<
//     Arc<T>,
//     T,
//     &'a Arc<T>,
//     &'a T,
//     &'a mut Arc<T>,
//     &'a mut T,
//     for<'b> fn(&'b Arc<T>) -> Option<&'b T>,
//     for<'b> fn(&'b mut Arc<T>) -> Option<&'b mut T>,
// > {
//     Kp::new(
//         |arc: &Arc<T>| Some(arc.as_ref()),
//         |arc: &mut Arc<T>| Arc::get_mut(arc),
//     )
// }

// /// Create a keypath for unwrapping Rc<T> -> T (read-only)
// ///
// /// # Example
// /// ```
// /// use std::rc::Rc;
// /// use rust_key_paths::kp_rc;
// /// let rc = Rc::new("value".to_string());
// /// let kp = kp_rc();
// /// assert_eq!(kp.get(&rc), Some(&"value".to_string()));
// /// ```
// pub fn kp_rc<'a, T>() -> Kp<
//     std::rc::Rc<T>,
//     T,
//     &'a std::rc::Rc<T>,
//     &'a T,
//     &'a mut std::rc::Rc<T>,
//     &'a mut T,
//     for<'b> fn(&'b std::rc::Rc<T>) -> Option<&'b T>,
//     for<'b> fn(&'b mut std::rc::Rc<T>) -> Option<&'b mut T>,
// > {
//     Kp::new(
//         |rc: &std::rc::Rc<T>| Some(rc.as_ref()),
//         |rc: &mut std::rc::Rc<T>| std::rc::Rc::get_mut(rc),
//     )
// }
