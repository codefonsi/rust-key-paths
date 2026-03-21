use key_paths_derive::Kp;
use rust_key_paths::{AccessorTrait, KpTrait, KpType};

pub struct RuleBuilder<'a, R, V> {
    root: Option<R>,
    kp: KpType<'a, R, V>,
    rules: Vec<fn(Option<&V>) -> Option<&'static str>>,
}

impl<'a, R, V> RuleBuilder<'a, R, V> {
    pub fn new(kp: KpType<'a, R, V>) -> Self {
        Self {
            root: None,
            kp,
            rules: vec![],
        }
    }

    pub fn with_root(mut self, root: R) -> Self {
        self.root = Some(root);
        self
    }

    pub fn rule(mut self, f: fn(Option<&V>) -> Option<&'static str>) -> Self {
        self.rules.push(f);
        self
    }

    pub fn apply(&self) -> Vec<&'static str> {
        let val = self.kp.get_optional(self.root.as_ref());
        self.rules
            .iter()
            .filter_map(|f| f(val))
            .collect()
    }
}

fn iso123rule(r: Option<&str>) -> Option<&'static str> {
    if r.map_or(true, |s| s.trim().is_empty()) {
        Some("iso123rule: field is required and must not be blank")
    } else {
        None
    }
}

#[derive(Kp)]
struct Test {
    a: String,
}

fn main() {
    let t = Test { a: "  ".to_string() };

    let errors = RuleBuilder::new(Test::a())
        .with_root(t)
        .rule(iso123rule)
        .rule(iso123rule)
        .apply();

    for e in &errors {
        println!("{}", e);
    }
}