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
            _ if 0x10 == value => Self::PrefixNumber,
            _ if 0x20 == value => Self::PrefixGrouping,
            _ if 0x30 == value => Self::PrefixUnary,
            _ if 0x40 == value => Self::PrefixBinary,

            _ if 0x100 == value => Self::InfixNumber,
            _ if 0x200 == value => Self::InfixGrouping,
            _ if 0x300 == value => Self::InfixUnary,
            _ if 0x400 == value => Self::InfixBinary,

            _ if 0x1000 == value => Self::PostfixNumber,
            _ if 0x2000 == value => Self::PostfixGrouping,
            _ if 0x3000 == value => Self::PostfixUnary,
            _ if 0x4000 == value => Self::PostfixBinary,

            _ => panic!("Cannot convert {value} to rule."),
        }
    }
}

impl From<RuleFn> for u16 {
    fn from(value: RuleFn) -> Self {
        match value {
            RuleFn::None => 0,
            RuleFn::PrefixNumber => 0x10,
            RuleFn::PrefixGrouping => 0x20,
            RuleFn::PrefixUnary => 0x30,
            RuleFn::PrefixBinary => 0x40,

            RuleFn::InfixNumber => 0x100,
            RuleFn::InfixGrouping => 0x200,
            RuleFn::InfixUnary => 0x300,
            RuleFn::InfixBinary => 0x400,

            RuleFn::PostfixNumber => 0x1000,
            RuleFn::PostfixGrouping => 0x2000,
            RuleFn::PostfixUnary => 0x3000,
            RuleFn::PostfixBinary => 0x4000,
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
            TokenKind::LeftParen => Precedence::None as u16 | u16::from(RuleFn::PrefixGrouping),
            TokenKind::Minus => Precedence::Term as u16 | u16::from(RuleFn::InfixBinary) | u16::from(RuleFn::PrefixUnary),
            TokenKind::Plus => Precedence::Term as u16 | u16::from(RuleFn::InfixBinary),
            TokenKind::Slash => Precedence::Factor as u16 | u16::from(RuleFn::InfixBinary),
            TokenKind::Star => Precedence::Factor as u16 | u16::from(RuleFn::InfixBinary),
            TokenKind::Int => Precedence::None as u16 | u16::from(RuleFn::PrefixNumber),
            TokenKind::Float => Precedence::None as u16 | u16::from(RuleFn::PrefixNumber),
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
