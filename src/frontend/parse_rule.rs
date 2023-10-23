use super::{precedence::Precedence, tokenization::tokenkind::TokenKind};

// 4 bits postfix, 4 bits infix, 4 bits prefix, 4 bits precedence
const PREFIX_NUMBER: u16 = 0b0000_0000_0000_0000;
const PREFIX_GROUPING: u16 = 0b0000_0000_0001_0000;
const PREFIX_UNARY: u16 = 0b0000_0000_0010_0000;
const PREFIX_BINARY: u16 = 0b0000_0000_0011_0000;

const INFIX_NUMBER: u16 = 0b0000_0000_0000_0000;
const INFIX_GROUPING: u16 = 0b0000_0001_0000_0000;
const INFIX_UNARY: u16 = 0b0000_0010_0000_0000;
const INFIX_BINARY: u16 = 0b0000_0011_0000_0000;

const POSTFIX_NUMBER: u16 = 0b0000_0000_0000_0000;
const POSTFIX_GROUPING: u16 = 0b0001_0000_0000_0000;
const POSTFIX_UNARY: u16 = 0b0010_0000_0000_0000;
const POSTFIX_BINARY: u16 = 0b0011_0000_0000_0000;

struct ParseRule;
impl ParseRule {
    fn rules(kind: TokenKind) -> u16 {
        /*
         * 0 - number
         * 1 - grouping
         * 2 - unary
         * 3 - binary
         * */
        match kind {
            TokenKind::LeftParen => PREFIX_GROUPING | Precedence::None as u16,
            TokenKind::Minus => INFIX_BINARY | PREFIX_UNARY | Precedence::Term as u16,
            TokenKind::Plus => INFIX_BINARY | Precedence::Term as u16,
            TokenKind::Slash => INFIX_BINARY | Precedence::Factor as u16,
            TokenKind::Star => INFIX_BINARY | Precedence::Factor as u16,
            TokenKind::Int => Precedence::None as u16,
            TokenKind::Float => Precedence::None as u16,
            _ => Precedence::None as u16,
        }
    }
}
