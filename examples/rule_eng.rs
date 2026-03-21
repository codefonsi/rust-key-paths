use key_paths_derive::Kp;
use rust_key_paths::{AccessorTrait, KpTrait, KpType};

struct RuleBuilder<'a, R, V> {
    root: Option<R>, 
    value: Option<V>, 
    kp: KpType<'a, R, V>,
    rules: Vec<fn(Option<&'a V>) -> Option<&'a str>>,
    errors: Vec<&'a str>
}

impl<'a, R, V> RuleBuilder<'a, R, V> {
    pub fn new(kp: KpType<'a, R, V>) -> Self {
        todo!()
    }
}

pub trait ISOabc123 {
    fn a<'a>() -> &'a str;
}

impl<'a, R, V> RuleBuilder<'a, R, V>  {
    fn rule(mut self, f: fn(Option<&V>) -> Option<&'a str>) -> Self {
        self.rules.push(f);
        self
    }

    fn apply(&'a self) {
        for i in &self.rules {
        let x = (i)(self.kp.get_optional(self.root.as_ref()));
        }
    }
}



fn rule<'a, R, V>(kp: KpType<'a, R, V>) -> RuleBuilder<'a, R, V> {
    RuleBuilder::new(kp)
}


fn iso123rule<'a>(r:Option<&str>) -> Option<&'a str> {
    if r.is_none() && r.unwrap().trim().len() > 0 {
        None
    } else {
        Some("iso123rule vailated")
    }
}

#[derive(Kp)]
struct Test {
    a: String
}
fn main() {
    let mut builder = RuleBuilder::new(Test::a());
    let rule = builder
    .rule(iso123rule)
    .rule(iso123rule)
    .apply();
}