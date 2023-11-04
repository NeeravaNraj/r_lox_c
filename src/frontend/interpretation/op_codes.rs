use std::fmt::Display;

#[derive(Debug, Clone, Copy)]
pub enum OpCodes {
    Return,
    //       index
    Constant(usize),
    Negate,
    Add,
    Subtract,
    Multiply,
    Divide,
    None,
    True,
    False,
    Not,
    Equals,
    NotEquals,
    Greater,
    GreaterEquals,
    Less,
    LessEquals,
    Ternary,
    Print,
    Pop,
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
            Self::Divide => "OP_DIVIDE",
            Self::None => "OP_NONE",
            Self::True => "OP_TRUE",
            Self::False => "OP_FALSE",
            Self::Not => "OP_NOT",
            Self::Equals => "OP_EQAULS",
            Self::NotEquals => "OP_NOT_EQUAL",
            Self::Greater => "OP_GREATER",
            Self::GreaterEquals => "OP_GREATER_EQUAL",
            Self::Less => "OP_LESS",
            Self::LessEquals => "OP_LESS_EQUAL",
            Self::Ternary => "OP_TERNARY",
            Self::Print => "OP_PRINT",
            Self::Pop => "OP_POP",
        };
        write!(f, "{val}")
    }
}
