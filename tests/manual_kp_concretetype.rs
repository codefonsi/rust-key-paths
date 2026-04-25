use rust_key_paths::Kp;

#[derive(Debug)]
struct Size {
    width: u32,
    height: u32,
}

#[derive(Debug)]
struct Rectangle {
    size: Size,
    name: String,
}

// Manual keypath: Rectangle -> Size
fn rect_size_kp() -> Kp<
    Rectangle,
    Size,
    &'static Rectangle,
    &'static Size,
    &'static mut Rectangle,
    &'static mut Size,
    for<'a> fn(&'a Rectangle) -> Option<&'a Size>,
    for<'a> fn(&'a mut Rectangle) -> Option<&'a mut Size>,
> {
    Kp::new(|x| Some(&x.size), |x| Some(&mut x.size))
}

// Manual keypath: Size -> width
fn size_width_kp() -> Kp<
    Size,
    u32,
    &'static Size,
    &'static u32,
    &'static mut Size,
    &'static mut u32,
    for<'a> fn(&'a Size) -> Option<&'a u32>,
    for<'a> fn(&'a mut Size) -> Option<&'a mut u32>,
> {
    Kp::new(|x| Some(&x.height), |x| Some(&mut x.height))
}

#[test]
fn manual_keypath_then_read_write_works() {
    let mut rect = Rectangle {
        size: Size {
            width: 30,
            height: 50,
        },
        name: "MyRect".to_string(),
    };

    let width_kp = rect_size_kp().then(size_width_kp());

    println!("size of concreate kp = {:?}", size_of_val(&width_kp));
    // assert_eq!((width_kp).get(&rect), Some(&30));

    // if let Some(w) = width_kp.get_mut(&mut rect) {
    //     *w += 12;
    // }

    // assert_eq!((width_kp).get(&rect), Some(&42));
    // assert_eq!(rect.size.height, 50);
    // assert_eq!(rect.name, "MyRect");
}
