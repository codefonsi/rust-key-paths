use key_paths_derive::Kp;
use rust_key_paths::{Kp, KpType};

#[derive(Debug, Kp, Default, Clone)]
struct SomeComplexStruct {
    scsf: Box<SomeOtherStruct>,
}

#[derive(Debug, Kp, Default, Clone)]
struct SomeOtherStruct {
    sosf: OneMoreStruct,
}

#[derive(Debug, Kp, Clone)]
enum SomeEnum {
    A(String),
    B(DarkStruct),
}

impl Default for SomeEnum {
    fn default() -> Self {
        SomeEnum::A(String::new())
    }
}

#[derive(Debug, Kp, Default, Clone)]
struct OneMoreStruct {
    omsf: String,
    omse: SomeEnum,
}

#[derive(Debug, Kp, Default, Clone)]
struct DarkStruct {
    dsf: String,
}

/// Keypath builder
pub struct KpBuilder<T> {
    value: T,
}

impl<T: Default> KpBuilder<T> {
    pub fn new() -> Self {
        Self {
            value: T::default(),
        }
    }

    pub fn set<V>(mut self, kp: impl Fn(&mut T) -> Option<&mut V>, value: V) -> Self
    where
        V: Clone,
    {
        if let Some(field) = kp(&mut self.value) {
            *field = value;
        }
        self
    }

    pub fn build(self) -> T {
        self.value
    }
}

fn main() {
    // Method 1: Direct builder
    let instance = KpBuilder::new()
        .set(
            |root: &mut SomeComplexStruct| {
                SomeComplexStruct::scsf()
                    .then(SomeOtherStruct::sosf())
                    .then(OneMoreStruct::omsf())
                    .get_mut(root)
            },
            "Hello World".to_string(),
        )
        .set(
            |root: &mut SomeComplexStruct| {
                SomeComplexStruct::scsf()
                    .then(SomeOtherStruct::sosf())
                    .then(OneMoreStruct::omse())
                    .then(SomeEnum::b())
                    .then(DarkStruct::dsf())
                    .get_mut(root)
            },
            "🖖🏿🖖🏿🖖🏿".to_string(),
        )
        .build();

    println!("{:#?}", instance);
}
