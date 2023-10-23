use crate::{common::chunk::Chunk, prelude::CompilerResult, error_at};

use super::{
    interpretation::{literal::Literal, op_codes::OpCodes},
    lexer::Lexer,
    tokenization::{token::Token, tokenkind::TokenKind},
    precedence::Precedence
};

pub struct Compiler<'a> {
    lexer: Lexer<'a>,
    file_path: &'a str,
    tokens: Vec<Token<'a>>,
    had_error: bool,
}

impl<'a> Compiler<'a> {
    pub fn new(file_path: &'a str, source: String) -> Self {
        Self {
            lexer: Lexer::new(file_path, source),
            file_path,
            tokens: Vec::new(),
            had_error: false,
        }
    }

    pub fn compile(&'a mut self, chunk: &mut Chunk) -> CompilerResult {
        Ok(())
    }

    fn parse_precedence(&self, precedence: Precedence) {}

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

    fn grouping(&'a mut self) {
        self.expression();
        self.consume(TokenKind::RightParen, "expected ')' after expression");
    }

    fn unary(&mut self, chunk: &mut Chunk) {
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
            // let rule = self.get_rule(operator.kind);
            // self.parse_precedence(rule.precedence);

            match operator.kind {
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

    fn advance(&'a mut self) {
        if let Some(token) = self.tokens.pop() {
            self.tokens[0] = token;
        }

        if let Ok(token) = self.lexer.token() {
            self.tokens.push(token);
        }
    }

    fn consume(&'a mut self, kind: TokenKind, message: &str) {
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

    fn current(&self) -> Option<&Token<'a>> {
        self.tokens.get(1)
    }

    fn previous(&self) -> Option<&Token<'a>> {
        self.tokens.get(0)
    }
}
