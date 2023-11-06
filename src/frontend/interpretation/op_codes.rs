use std::fmt::Display;

#[derive(Debug, Clone)]
pub enum OpCodes {
    Return,
    //       index
    Constant(usize),
    DefGlobal(usize),
    GetGlobal(usize),
    SetGlobal(usize),
    SetLocal(usize),
    GetLocal(usize),
    JumpFalse(usize),
    Jump(usize),
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

impl OpCodes {
    pub fn patch_jump(&self, offset: usize) -> Self {
        match self {
            Self::JumpFalse(_) => Self::JumpFalse(offset),
            Self::Jump(_) => Self::Jump(offset),
            _ => unreachable!("bad"),
        }
    }
}

impl Display for OpCodes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let val = match self {
            Self::Return => "OP_RETURN",
            Self::Constant(_) => "OP_CONSTANT",
            Self::DefGlobal(_) => "OP_GLOBAL_DEF",
            Self::GetGlobal(_) => "OP_GLOBAL_GET",
            Self::SetGlobal(_) => "OP_GLOBAL_SET",
            Self::GetLocal(_) => "OP_LOCAL_GET",
            Self::SetLocal(_) => "OP_LOCAL_SET",
            Self::JumpFalse(_) => "OP_JUMP_FALSE",
            Self::Jump(_) => "OP_JUMP",
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
