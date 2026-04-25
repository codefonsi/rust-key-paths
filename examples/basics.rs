//! Basic keypath example: derive Kp, compose with then(), read and write.
//!
//! Run with: `cargo run --example basics`

use std::process::Output;

use key_paths_derive::Kp;
use rust_key_paths::{CoercionTrait, HofTrait, KpDynamic, KpTrait};

pub struct Service {
    rect_to_width_kp: KpDynamic<Rectangle, u32>,
}

// impl Service {
//     pub fn new() -> Self {
//         Self {
//             rect_to_width_kp: Rectangle::test().into(),
//         }
//     }
// }

#[derive(Debug, Kp)]
struct Size {
    width: u32,
    height: u32,
}

#[derive(Debug, Kp)]
struct Rectangle {
    size: Size,
    name: String,
}

impl Rectangle {
    // fn kp() -> KpType<'static, Rectangle, String> {
    //     KpType::new(
    //         |root| {
    //             let x = root.name.borrow();
    //             let y = &*x as *const String;
    //             Some(unsafe { &*y })}
    //         ,|root| {
    //             let mut x = root.name.borrow_mut();
    //             let y = &mut *x as *mut String;
    //             Some(unsafe {&mut *y})
    //         }
    //     )
    // }

    // fn kp() -> KpType<'static, Rectangle, std::cell::Ref<'static, String>> {
    //     KpType::new(
    //         |root| { Some(&'static root.name.borrow()) }
    //         ,|_| { None }
    //     )
    // }
}
// Standalone fn pointers for keypath (reference: lib.rs identity_typed / Kp with fn types)

impl Rectangle {
    // /// Keypath to `size.width`, built with fn pointers (same pattern as lib.rs `identity_typed`).
    // pub const fn size() -> KpType<'static, Rectangle, Size> {
    //     const fn g(r: &Rectangle) -> Option<&Size> {
    //         Some(&r.size)
    //     }
    //     const fn s(r: &mut Rectangle) -> Option<&mut Size> {
    //         Some(&mut r.size)
    //     }

    //     Kp::new_const(g, s)
    // }
}

fn main() {
    let mut rect = Rectangle {
        size: Size {
            width: 30,
            height: 50,
        },
        name: "MyRect".into(),
    };

    // Read: compose keypaths with then()
    {
        let width_path = Rectangle::size().then(Size::width());
        if let Some(w) = (width_path.get)(&rect) {
            println!("Width: {}", w);
        }
        println!("Width (direct): {:?}", width_path.get(&rect));
    }

    // Writable: get_mut and modify
    {
        let width_mut_kp = Rectangle::size().then(Size::width());
        if let Some(w) = width_mut_kp.get_mut(&mut rect) {
            *w += 50;
        }
    }

    // let kp = Rectangle::size().then(Size::width()).get;
    // let kp = |root: &mut Rectangle| {(Rectangle::size().set)(root)};
    // let x:fn() = || {};

    let kp = Rectangle::size();
    println!("==={:?}", size_of_val(&kp));
    let kp = Rectangle::size().set;

    println!("{:?}", size_of_val(&kp));

    let kp = Rectangle::size().then(Size::width());
    let kp= kp.get;

    println!("size of kp = {}", size_of_val(&kp));
    // let x: fn(&Rectangle) -> Option<&Size> = Rectangle::size().get;
    // let y = that_takes(x);

    // let x: fn() = || {};

    // let x = Rectangle::kp().get(todo!());
    // let x = Rectangle::kp().get_mut(todo!());
    println!("Updated rectangle: {:?}", rect);
}

fn that_takes(f: fn(&Rectangle) -> Option<&Size>) -> for<'a> fn(&'a Rectangle) -> String {
    |_root| "working".to_string()
}

// fn
// impl Fn
// Fn, FnMut, FnOnce
// Box<dyn Fn>
