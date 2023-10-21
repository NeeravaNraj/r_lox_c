use std::fmt::Display;

#[derive(Debug, Clone, Copy)]
pub enum OpCodes {
    Return,
    Constant(usize),
    Negate,
    Add,
    Subtract,
    Multiply,
    Divide,
}

impl Display for OpCodes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let val = match self {
            Self::Return => "OP_RETURN",
            Self::Constant(_) => "OP_CONSTANT",
            Self::Negate => "OP_NEGATE",
            Self::Add => "OP_ADD",
            Self::Subtract => "OP_SUBTRACT",
            Self::Multiply => "OP_MULTIPLY",
            Self::Divide => "OP_DIVIDE"
        };
        write!(f, "{val}")
    }
}
