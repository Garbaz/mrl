use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

#[derive(Hash)]
pub enum Value {
    List(Vec<Value>),
    Symbol(String),
}

impl Value {
    fn hash(&self) -> u64 {
        let mut h = DefaultHasher::new();
        Hash::hash(&self, &mut h);
        h.finish()
    }
}

struct F6464([f64;64]);

impl From<u64> for F6464 {
    fn from(mut value: u64) -> Self {
        let mut f = F6464([0.;64]);
        for i in 0..64 {
            f.0[i] = (value & 1) as f64;
            value >>= 1;
        }
        f
    }
}

pub fn main() {
    use Value::*;

    println!(
        "{}",
        List(vec![
            Symbol("a".to_string()),
            Symbol("b".to_string()),
            Symbol("c".to_string()),
        ])
        .hash()
    );
}