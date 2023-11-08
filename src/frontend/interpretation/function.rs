use std::fmt::Display;

use crate::common::chunk::Chunk;

pub struct Function {
    arity: u32,
    chunk: Chunk,
    name: String,
}

impl Function {}

impl Default for Function {
    fn default() -> Self {
        Self {
            arity: 0,
            name: "".into(),
            chunk: Chunk::new(),
        }
    }
}

impl Display for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<fn {}>", self.name)
    }
}
