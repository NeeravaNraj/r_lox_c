use super::{precedence::Precedence, tokenization::tokenkind::TokenKind};

pub type Rule = u16;

// Probably wont need postfix but whatever
// 4 bits postfix, 4 bits infix, 4 bits prefix, 4 bits precedence
const PREFIX_SHIFT: u16 = 4;
const INFIX_SHIFT: u16 = 8;
const POSTFIX_SHIFT: u16 = 12;
pub const PRECEDENCE_MASK: u16 = 0xFFF0;
pub const PREFIX_MASK: u16 = 0xFF0F;
pub const INFIX_MASK: u16 = 0xF0FF;
pub const POSTFIX_MASK: u16 = 0x0FFF;

pub enum RuleFn {
    None,

    PrefixNumber,
    PrefixGrouping,
    PrefixUnary,
    PrefixBinary,

    InfixNumber,
    InfixGrouping,
    InfixUnary,
    InfixBinary,

    PostfixNumber,
    PostfixGrouping,
    PostfixUnary,
    PostfixBinary,
}

impl From<u16> for RuleFn {
    fn from(value: u16) -> Self {
        match value {
            0 => Self::None,
            _ if 1 << PREFIX_SHIFT == value => Self::PrefixNumber,
            _ if 2 << PREFIX_SHIFT == value => Self::PrefixGrouping,
            _ if 3 << PREFIX_SHIFT == value => Self::PrefixUnary,
            _ if 4 << PREFIX_SHIFT == value => Self::PrefixBinary,

            _ if 1 << INFIX_SHIFT == value => Self::InfixNumber,
            _ if 2 << INFIX_SHIFT == value => Self::InfixGrouping,
            _ if 3 << INFIX_SHIFT == value => Self::InfixUnary,
            _ if 4 << INFIX_SHIFT == value => Self::InfixBinary,

            _ if 1 << POSTFIX_SHIFT == value => Self::PostfixNumber,
            _ if 2 << POSTFIX_SHIFT == value => Self::PostfixGrouping,
            _ if 3 << POSTFIX_SHIFT == value => Self::PostfixUnary,
            _ if 4 << POSTFIX_SHIFT == value => Self::PostfixBinary,

            _ => panic!("Cannot convert {value} to rule."),
        }
    }
}

impl From<RuleFn> for u16 {
    fn from(value: RuleFn) -> Self {
        match value {
            RuleFn::None => 0,
            RuleFn::PrefixNumber => 1 << PREFIX_SHIFT,
            RuleFn::PrefixGrouping => 2 << PREFIX_SHIFT,
            RuleFn::PrefixUnary => 3 << PREFIX_SHIFT,
            RuleFn::PrefixBinary => 4 << PREFIX_SHIFT,

            RuleFn::InfixNumber => 1 << INFIX_SHIFT,
            RuleFn::InfixGrouping => 2 << INFIX_SHIFT,
            RuleFn::InfixUnary => 3 << INFIX_SHIFT,
            RuleFn::InfixBinary => 4 << INFIX_SHIFT,

            RuleFn::PostfixNumber => 1 << POSTFIX_SHIFT,
            RuleFn::PostfixGrouping => 2 << POSTFIX_SHIFT,
            RuleFn::PostfixUnary => 3 << POSTFIX_SHIFT,
            RuleFn::PostfixBinary => 4 << POSTFIX_SHIFT,
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
            TokenKind::LeftParen => Precedence::None as u16 | RuleFn::PrefixGrouping as u16,
            TokenKind::Minus => Precedence::Term as u16 | RuleFn::InfixBinary as u16 | RuleFn::PrefixUnary as u16,
            TokenKind::Plus => Precedence::Term as u16 | RuleFn::InfixBinary as u16,
            TokenKind::Slash => Precedence::Factor as u16 | RuleFn::InfixBinary as u16,
            TokenKind::Star => Precedence::Factor as u16 | RuleFn::InfixBinary as u16,
            TokenKind::Int => Precedence::None as u16,
            TokenKind::Float => Precedence::None as u16,
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
