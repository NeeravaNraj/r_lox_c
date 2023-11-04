use super::{precedence::Precedence, tokenization::tokenkind::TokenKind};

pub type Rule = u16;
// Probably wont need postfix but whatever
// 4 bits postfix, 4 bits infix, 4 bits prefix, 4 bits precedence
pub const PRECEDENCE_MASK: u16 = 0x000F;
pub const PREFIX_MASK: u16 = 0x00F0;
pub const INFIX_MASK: u16 = 0x0F00;
pub const POSTFIX_MASK: u16 = 0xF000;

#[derive(Debug)]
pub enum RuleFn {
    None,

    // prefix
    Number,
    Literal,
    String,
    Grouping,
    Unary,
    Variable,

    // infix
    Binary,
    Ternary,
}

impl From<u16> for RuleFn {
    fn from(value: u16) -> Self {
        match value {
            0 => Self::None,

            _ if 0x10 == value => Self::Number,
            _ if 0x20 == value => Self::Literal,
            _ if 0x30 == value => Self::Grouping,
            _ if 0x40 == value => Self::Unary,
            _ if 0x50 == value => Self::String,
            _ if 0x60 == value => Self::Variable,

            _ if 0x100 == value => Self::Binary,
            _ if 0x200 == value => Self::Ternary,

            _ => panic!("Cannot convert {value} to rule."),
        }
    }
}

impl From<RuleFn> for u16 {
    fn from(value: RuleFn) -> Self {
        match value {
            RuleFn::None => 0,

            RuleFn::Number => 0x10,
            RuleFn::Literal => 0x20,
            RuleFn::Grouping => 0x30,
            RuleFn::Unary => 0x40,
            RuleFn::String => 0x50,
            RuleFn::Variable => 0x60,

            RuleFn::Binary => 0x100,
            RuleFn::Ternary => 0x200,
        }
    }
}

pub struct ParseRule;
impl ParseRule {
    pub fn rules(kind: TokenKind) -> Rule {
        /*
         * 0 - none
         * 1 - number
         * 2 - grouping
         * 3 - unary
         * 4 - binary
         * */
        match kind {
            TokenKind::LeftParen => Precedence::None as u16 | u16::from(RuleFn::Grouping),
            TokenKind::Minus => Precedence::Term as u16 | u16::from(RuleFn::Binary) | u16::from(RuleFn::Unary),
            TokenKind::Plus => Precedence::Term as u16 | u16::from(RuleFn::Binary),
            TokenKind::Slash => Precedence::Factor as u16 | u16::from(RuleFn::Binary),
            TokenKind::Star => Precedence::Factor as u16 | u16::from(RuleFn::Binary),
            TokenKind::Int => Precedence::None as u16 | u16::from(RuleFn::Number),
            TokenKind::Float => Precedence::None as u16 | u16::from(RuleFn::Number),
            TokenKind::None => Precedence::None as u16 | u16::from(RuleFn::Literal),
            TokenKind::True => Precedence::None as u16 | u16::from(RuleFn::Literal),
            TokenKind::False => Precedence::None as u16 | u16::from(RuleFn::Literal),
            TokenKind::Bang => Precedence::None as u16 | u16::from(RuleFn::Unary),
            TokenKind::BangEqual => Precedence::Equality as u16 | u16::from(RuleFn::Binary),
            TokenKind::Equals => Precedence::Equality as u16 | u16::from(RuleFn::Binary),
            TokenKind::Greater => Precedence::Comparison as u16 | u16::from(RuleFn::Binary),
            TokenKind::GreaterEqual => Precedence::Comparison as u16 | u16::from(RuleFn::Binary),
            TokenKind::Less => Precedence::Comparison as u16 | u16::from(RuleFn::Binary),
            TokenKind::LessEqual => Precedence::Comparison as u16 | u16::from(RuleFn::Binary),
            TokenKind::QuestionMark => Precedence::Ternary as u16 | u16::from(RuleFn::Ternary),
            TokenKind::String => Precedence::None as u16 | u16::from(RuleFn::String),
            TokenKind::Identifier => Precedence::None as u16 | u16::from(RuleFn::Variable),
            _ => Precedence::None as u16,
        }
    }

    pub fn get_precedence(rule: Rule) -> Precedence {
        (rule & PRECEDENCE_MASK).into()
    }

    pub fn get_prefix(rule: Rule) -> RuleFn {
        (rule & PREFIX_MASK).into()
    }

    pub fn get_infix(rule: Rule) -> RuleFn {
        (rule & INFIX_MASK).into()
    }

    pub fn get_postfix(rule: Rule) -> RuleFn {
        (rule & POSTFIX_MASK).into()
    }
}
