#[derive(PartialEq, PartialOrd)]
pub enum Precedence {
    None,
    Assignment,
    Ternary,
    Or,
    And,
    Equality,
    Comparison,
    Term,
    Factor,
    Unary,
    Call,
    Primary,
}

impl From<u16> for Precedence {
    fn from(value: u16) -> Self {
        match value {
            0 => Precedence::None,
            1 => Precedence::Assignment,
            2 => Precedence::Ternary,
            3 => Precedence::Or,
            4 => Precedence::And,
            5 => Precedence::Equality,
            6 => Precedence::Comparison,
            7 => Precedence::Term,
            8 => Precedence::Factor,
            9 => Precedence::Unary,
            10 => Precedence::Call,
            11 => Precedence::Primary,
            _ => panic!("Precedence for {value} does not exist"),
        }
    }
}
