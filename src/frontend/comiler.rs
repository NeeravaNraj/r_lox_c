use std::rc::Rc;

use super::{
    interpretation::{literal::Literal, op_codes::OpCodes},
    parse_rule::{ParseRule, Rule},
    tokenization::{token::Token, tokenkind::TokenKind},
};
use crate::{common::chunk::Chunk, error_at, prelude::CompilerResult};

pub struct Compiler<'tokens> {
    file_path: Rc<str>,
    had_error: bool,
    tokens: &'tokens [Token],
    current: usize,
}

impl<'tokens> Compiler<'tokens> {
    pub fn new(file_path: Rc<str>, tokens: &'tokens Vec<Token>) -> Self {
        Self {
            tokens,
            file_path,
            had_error: false,
            current: 0,
        }
    }

    pub fn compile(&mut self, chunk: &mut Chunk) -> CompilerResult {
        Ok(())
    }

    fn parse_precedence(&self, rule: Rule) {
        let precedence = ParseRule::get_precedence(rule);
        let prefix_rule = self.get_rule(self.previous().as_ref().unwrap().kind);
    }

    fn get_rule(&self, kind: TokenKind) -> Rule {
        ParseRule::rules(kind)
    }

    fn number(&self, chunk: &mut Chunk) {
        if let Some(token) = self.previous() {
            match token.kind {
                TokenKind::Int => {
                    let value = token.lexeme.parse::<i32>().expect("Failed to parse int.");
                    self.emit_constant(Literal::Int(value), chunk)
                }
                TokenKind::Float => {
                    let value = token.lexeme.parse::<f64>().expect("Failed to parse float.");
                    self.emit_constant(Literal::Float(value), chunk)
                }
                _ => (),
            }
        }
    }

    fn grouping(&mut self) {
        self.expression();
        self.consume(TokenKind::RightParen, "expected ')' after expression");
    }

    fn unary(&self, chunk: &mut Chunk) {
        if let Some(token) = self.previous() {
            self.expression();

            match token.kind {
                TokenKind::Minus => self.emit_byte(OpCodes::Negate, chunk),
                _ => return,
            }
        }
    }

    fn binary(&mut self, chunk: &mut Chunk) {
        if let Some(operator) = self.previous() {
            let kind = operator.kind;
            let rule = self.get_rule(kind);
            self.parse_precedence(rule);
            self.advance();

            match kind {
                TokenKind::Minus => self.emit_byte(OpCodes::Subtract, chunk),
                TokenKind::Plus => self.emit_byte(OpCodes::Add, chunk),
                TokenKind::Star => self.emit_byte(OpCodes::Multiply, chunk),
                TokenKind::Slash => self.emit_byte(OpCodes::Divide, chunk),
                _ => return,
            }
        }
    }

    fn expression(&self) {}

    fn emit_constant(&self, constant: Literal, chunk: &mut Chunk) {
        if self.current().is_none() {
            panic!("Current is none?");
        }
        chunk.add_constant(constant, self.current().unwrap().span.location.line)
    }

    fn emit_byte(&self, code: OpCodes, chunk: &mut Chunk) {
        if self.current().is_none() {
            panic!("Current is none?");
        }
        chunk.write(code, self.current().unwrap().span.location.line)
    }

    fn advance(&mut self) {
        self.current += 1;
    }

    fn consume(&mut self, kind: TokenKind, message: &str) {
        if let Some(token) = self.current() {
            if token.kind == kind {
                self.advance();
                return;
            }
            error_at!(token.span.dup(), "{message}");
            self.had_error = true;
        }
    }

    fn end(&self, chunk: &mut Chunk) {
        self.emit_byte(OpCodes::Return, chunk);
    }

    fn current(&self) -> Option<&Token> {
        self.tokens.get(self.current)
    }

    fn previous(&self) -> Option<&Token> {
        self.tokens.get(self.current - 1)
    }
}
