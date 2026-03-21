use std::borrow::Cow;

use key_paths_derive::Kp;
use rust_key_paths::{AccessorTrait, KpTrait, KpType};

#[derive(Debug)]
pub enum ErrorCode {
    ISOError,
    Some1Error(Cow<'static, String>),
    Some2Error(Cow<'static, String>),
    Some3Error(Cow<'static, String>),
    Success
}
pub struct RuleBuilder<'a, R, V> {
    root: Option<&'a R>,
    kp: KpType<'a, R, V>,
    rules: Vec<fn(Option<&'a V>) -> &'a ErrorCode>,
}

impl<'a, R, V> RuleBuilder<'a, R, V> {
    pub fn new(kp: KpType<'a, R, V>) -> Self {
        Self {
            root: None,
            kp,
            rules: vec![],
        }
    }

    pub fn with_root(mut self, root: &'a R) -> Self {
        self.root = Some(root);
        self
    }

    pub fn rule(mut self, f: fn(Option<&'a V>) -> &'a ErrorCode) -> Self {
        self.rules.push(f);
        self
    }

    pub fn apply(&self) -> Vec<&'a ErrorCode> {
        let val = self.kp.get_optional(self.root);
        self.rules.iter().map(|f| f(val)).collect()
    }
}


mod iso_pain {
    // For raw rule — still receives Option<&String>
    pub fn iso123rule<'a>(r: Option<&'a String>) -> &'a crate::ErrorCode {
        if r.map_or(true, |s| s.trim().is_empty()) {
            &crate::ErrorCode::ISOError
        } else {
            &crate::ErrorCode::Success
        }
    }

    // For mandatory/optional — receives &String directly, None already handled
    pub fn not_blank<'a>(s: &'a String) -> &'a crate::ErrorCode {
        if s.trim().is_empty() {
            &crate::ErrorCode::ISOError
        } else {
            &crate::ErrorCode::Success
        }
    }

    pub fn max_len_35<'a>(s: &'a String) -> &'a crate::ErrorCode {
        if s.len() > 35 {
            &crate::ErrorCode::ISOError
        } else {
            &crate::ErrorCode::Success
        }
    }
}

#[derive(Kp)]
struct Test {
    a: String,
    b: String,

}

fn main() {
    let t = Test {
        a: "  ".to_string(),
        b: "asdf ".to_string(),
    };

    let errors = RuleBuilder::new(Test::a())
        .with_root(&t)
        .rule(iso_pain::iso123rule)
        .rule(iso_pain::iso123rule)
        .apply();

    for e in &errors {
        println!("{:?}", e);
    }
}
