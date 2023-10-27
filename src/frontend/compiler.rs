use std::rc::Rc;

use super::{
    interpretation::{literal::Literal, op_codes::OpCodes},
    parse_rule::{ParseRule, Rule, RuleFn},
    precedence::Precedence,
    tokenization::{token::Token, tokenkind::TokenKind},
};
use crate::{common::chunk::Chunk, error_at, prelude::CompilerResult};

pub struct Compiler<'tokens> {
    file_path: Rc<str>,
    tokens: &'tokens [Token],
    chunk: Chunk,
    current: usize,
    had_error: bool,
}

impl<'tokens> Compiler<'tokens> {
    pub fn new(file_path: Rc<str>, tokens: &'tokens Vec<Token>) -> Self {
        Self {
            tokens,
            chunk: Chunk::new(),
            file_path,
            had_error: false,
            current: 0,
        }
    }

    pub fn compile<'chunk>(&'chunk mut self) -> CompilerResult<'chunk> {
        self.expression();
        self.consume(TokenKind::EOF, "expected end of expression.");
        self.end();
        if self.had_error {
            return Err(());
        }
        Ok(&self.chunk)
    }

    fn rule_fn(&mut self, f: RuleFn) {
        match f {
            RuleFn::None => panic!("Required fn but got `None`"),

            RuleFn::PrefixNumber => self.number(),
            RuleFn::PrefixGrouping => self.grouping(),
            RuleFn::PrefixUnary => self.unary(),
            RuleFn::PrefixLiteral => self.literal(),

            RuleFn::InfixBinary => self.binary(),
            RuleFn::InfixTernary => self.ternary(),
        }
    }

    fn parse_precedence(&mut self, precedence: Precedence) {
        self.advance();
        let kind = self.previous().kind;
        let prefix_rule = self.get_rule(kind);
        let prefix_rule = ParseRule::get_prefix(prefix_rule);

        match prefix_rule {
            RuleFn::None => {
                self.error_occured();
                let token = self.previous();
                error_at!(&token.span, "expected expression");
                return;
            }
            _ => self.rule_fn(prefix_rule),
        }

        while !self.is_at_end() {
            let kind = self.current().kind;
            let rule = self.get_rule(kind);
            let next_prece = ParseRule::get_precedence(rule);
            if precedence <= next_prece {
                self.advance();
                let kind = self.previous().kind;
                let infix_rule = ParseRule::get_infix(self.get_rule(kind));
                self.rule_fn(infix_rule);
            } else {
                break;
            }
        }
    }

    fn get_rule(&self, kind: TokenKind) -> Rule {
        ParseRule::rules(kind)
    }

    fn number(&mut self) {
        let token = self.previous();
        match token.kind {
            TokenKind::Int => {
                let value = token.lexeme.parse::<isize>().expect("Failed to parse int.");
                self.emit_constant(Literal::Int(value))
            }
            TokenKind::Float => {
                let value = token.lexeme.parse::<f64>().expect("Failed to parse float.");
                self.emit_constant(Literal::Float(value))
            }
            _ => (),
        }
    }

    fn literal(&mut self) {
        let token = self.previous();
        match token.kind {
            TokenKind::False => self.emit_byte(OpCodes::False),
            TokenKind::True => self.emit_byte(OpCodes::True),
            TokenKind::None => self.emit_byte(OpCodes::None),
            _ => (),
        }
    }

    fn grouping(&mut self) {
        self.expression();
        self.consume(TokenKind::RightParen, "expected ')' after expression");
    }

    fn unary(&mut self) {
        let token = self.previous();
        let kind = token.kind;
        self.parse_precedence(Precedence::Unary);

        match kind {
            TokenKind::Minus => self.emit_byte(OpCodes::Negate),
            TokenKind::Bang => self.emit_byte(OpCodes::Not),
            _ => (),
        }
    }

    fn binary(&mut self) {
        let operator = self.previous();
        let kind = operator.kind;
        let rule = self.get_rule(kind);
        self.parse_precedence(ParseRule::get_precedence(rule));

        match kind {
            TokenKind::Minus => self.emit_byte(OpCodes::Subtract),
            TokenKind::Plus => self.emit_byte(OpCodes::Add),
            TokenKind::Star => self.emit_byte(OpCodes::Multiply),
            TokenKind::Slash => self.emit_byte(OpCodes::Divide),
            TokenKind::Equals => self.emit_byte(OpCodes::Equals),
            TokenKind::BangEqual => self.emit_byte(OpCodes::NotEquals),
            TokenKind::Greater => self.emit_byte(OpCodes::Greater),
            TokenKind::GreaterEqual => self.emit_byte(OpCodes::GreaterEquals),
            TokenKind::Less => self.emit_byte(OpCodes::Less),
            TokenKind::LessEqual => self.emit_byte(OpCodes::LessEquals),
            _ => (),
        }
    }

    fn ternary(&mut self) {
        self.expression();
        self.consume(TokenKind::Colon, "expected `:` after expression");
        self.expression();
        self.emit_byte(OpCodes::Ternary);
    }

    fn expression(&mut self) {
        self.parse_precedence(Precedence::Assignment);
    }

    fn emit_constant(&mut self, constant: Literal) {
        self.chunk
            .add_constant(constant, self.current().span.location.line)
    }

    fn emit_byte(&mut self, code: OpCodes) {
        self.chunk.write(code, self.previous().span.location.line)
    }

    fn emit_bytes(&mut self, a: OpCodes, b: OpCodes) {
        self.chunk.write(a, self.previous().span.location.line);
        self.chunk.write(b, self.previous().span.location.line);
    }

    fn advance(&mut self) {
        self.current += 1;
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.tokens.len()
    }

    fn consume(&mut self, kind: TokenKind, message: &str) {
        let token = self.current();
        if token.kind == kind {
            self.advance();
            return;
        }
        error_at!(&token.span, "{message}");
        self.error_occured();
    }

    fn end(&mut self) {
        self.emit_byte(OpCodes::Return);
    }

    fn current(&self) -> &'tokens Token {
        &self.tokens[self.current]
    }

    fn previous(&self) -> &'tokens Token {
        &self.tokens[self.current - 1]
    }

    fn error_occured(&mut self) {
        self.had_error = true;
    }
}
