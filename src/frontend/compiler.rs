use std::{cell::RefCell, rc::Rc};

use super::{
    interpretation::{literal::Literal, op_codes::OpCodes},
    locals::Local,
    parse_rule::{ParseRule, Rule, RuleFn},
    precedence::Precedence,
    tokenization::{location::Location, span::Span, token::Token, tokenkind::TokenKind},
};
use crate::{common::chunk::Chunk, error_at, prelude::CompilerResult};

pub struct Compiler<'tokens> {
    file_path: Rc<str>,
    source_map: Rc<RefCell<Vec<Span>>>,
    locals: Vec<Local<'tokens>>,
    depth: usize,
    tokens: &'tokens [Token],
    chunk: Chunk,
    current: usize,
    had_error: bool,
    panic_mode: bool,
}

impl<'tokens> Compiler<'tokens> {
    pub fn new(
        file_path: Rc<str>,
        tokens: &'tokens Vec<Token>,
        source_map: Rc<RefCell<Vec<Span>>>,
    ) -> Self {
        Self {
            file_path,
            tokens,
            source_map,
            locals: Vec::new(),
            chunk: Chunk::new(),
            had_error: false,
            panic_mode: false,
            depth: 0,
            current: 0,
        }
    }

    pub fn compile<'chunk>(&'chunk mut self) -> CompilerResult<'chunk> {
        while !self.is_match(TokenKind::EOF) {
            self.statement();
        }
        self.end();

        if self.had_error {
            return Err(());
        }
        Ok(&self.chunk)
    }

    fn statement(&mut self) {
        let token = self.current();
        match token.kind {
            TokenKind::Print => self.print_statement(),
            TokenKind::Let => self.var_decl(),
            TokenKind::LeftBrace => self.block(),
            TokenKind::If => self.if_statement(),
            TokenKind::While => self.while_statement(),
            _ => self.expression_statement(),
        };

        if self.panic_mode {
            self.synchronize();
        }
    }

    fn while_statement(&mut self) {
        self.advance();
        let loop_start = self.chunk.code.len();
        let Ok(_) = self.expression() else {
            return;
        };

        let exit = self.emit_jump(OpCodes::JumpFalse(0));
        self.emit_byte(OpCodes::Pop);
        self.statement();
        self.emit_loop(loop_start);

        self.patch_jump(exit);
        self.emit_byte(OpCodes::Pop);
    }

    fn if_statement(&mut self) {
        self.advance();
        let Ok(_) = self.expression() else {
            return;
        };

        let offset = self.emit_jump(OpCodes::JumpFalse(69));
        self.statement();
        let else_offset = self.emit_jump(OpCodes::Jump(42069));
        self.patch_jump(offset);

        if self.current().kind == TokenKind::Elif {
            self.if_statement();
        }

        if self.is_match(TokenKind::Else) {
            self.statement();
        }
        self.patch_jump(else_offset);
    }

    fn block(&mut self) {
        self.advance();
        self.begin_scope();
        while !self.check(TokenKind::RightBrace) && !self.check(TokenKind::EOF) {
            self.statement();
        }
        let Ok(_) = self.consume(TokenKind::RightBrace, "expected '}' after block") else {
            return;
        };
        self.end_scope();
    }

    fn begin_scope(&mut self) {
        self.depth += 1
    }

    fn end_scope(&mut self) {
        self.depth -= 1;

        while self.locals.len() > 0 && self.locals[self.locals.len() - 1].depth > self.depth {
            self.emit_byte(OpCodes::Pop);
            self.locals.pop();
        }
    }

    fn var_decl(&mut self) {
        self.advance();
        let Ok(global) = self.parse_var("expected variable name") else {
            return;
        };

        if self.is_match(TokenKind::Assign) {
            let _ = self.expression();
        } else {
            self.emit_byte(OpCodes::None);
        }

        let Ok(_) = self.consume(TokenKind::Semicolon, "expected ';' after expression") else {
            return;
        };

        self.define_var(global);
    }

    fn declare_local(&mut self) {
        if self.depth == 0 {
            return;
        }

        let name = self.previous();

        for local in self.locals.iter().rev() {
            if local.depth < self.depth {
                break;
            }

            if name.lexeme == local.name.lexeme {
                self.error(format!("cannot redefine variable '{}'", name.lexeme).as_str());
                return;
            }
        }

        self.add_local(name)
    }

    fn add_local(&mut self, token: &'tokens Token) {
        self.locals.push(Local::new(token, self.depth))
    }

    fn parse_var(&mut self, error_msg: &str) -> Result<usize, ()> {
        let Ok(_) = self.consume(TokenKind::Identifier, error_msg) else {
            return Err(());
        };

        self.declare_local();
        if self.depth > 0 {
            return Ok(0);
        }

        Ok(self.identifier_constant(self.previous()))
    }

    fn identifier_constant(&mut self, token: &Token) -> usize {
        self.chunk
            .add_constant_manual(Literal::Variable(token.lexeme.clone()))
    }

    fn define_var(&mut self, index: usize) {
        if self.depth > 0 {
            self.mark_initialized();
            return;
        }
        self.emit_byte(OpCodes::DefGlobal(index));
    }

    fn mark_initialized(&mut self) {
        self.locals
            .last_mut()
            .expect("could not unwrap last")
            .initialized = true;
    }

    fn print_statement(&mut self) {
        self.advance();
        let Ok(_) = self.expression() else {
            let token = self.previous();
            error_at!(&token.span, "expected expression after '{}'", token.lexeme);
            return;
        };

        let Ok(_) = self.consume(TokenKind::Semicolon, "expected ';' after expression") else {
            return;
        };

        self.emit_byte(OpCodes::Print)
    }

    fn expression_statement(&mut self) {
        let Ok(_) = self.expression() else {
            return;
        };

        let Ok(_) = self.consume(TokenKind::Semicolon, "expected ';' after expression") else {
            return;
        };

        self.emit_byte(OpCodes::Pop)
    }

    fn synchronize(&mut self) {
        self.panic_mode = false;
        while self.current().kind != TokenKind::EOF {
            if self.previous().kind == TokenKind::Semicolon {
                return;
            }

            match self.current().kind {
                TokenKind::Class
                | TokenKind::DefFn
                | TokenKind::Let
                | TokenKind::For
                | TokenKind::While
                | TokenKind::If
                | TokenKind::Else
                | TokenKind::Elif
                | TokenKind::Print
                | TokenKind::Return => break,
                _ => (),
            };
            self.advance();
        }
    }

    fn rule_fn(&mut self, f: RuleFn, can_assign: bool) {
        match f {
            RuleFn::None => panic!("Required fn but got `None`"),

            // prefix
            RuleFn::Number => self.number(),
            RuleFn::Grouping => self.grouping(),
            RuleFn::Unary => self.unary(),
            RuleFn::Literal => self.literal(),
            RuleFn::String => self.string(),
            RuleFn::Variable => self.variable(can_assign),

            // infix
            RuleFn::Binary => self.binary(),
            RuleFn::Ternary => self.ternary(),
            RuleFn::And => self.and(),
            RuleFn::Or => self.or(),
        }
    }

    fn parse_precedence(&mut self, precedence: Precedence) -> Result<(), ()> {
        // expect caller to handle the error
        if self.is_next_end() {
            self.error_occured();
            return Err(());
        }
        self.advance();
        let kind = self.previous().kind;
        let prefix_rule = self.get_rule(kind);
        let prefix_rule = ParseRule::get_prefix(prefix_rule);
        let can_assign = precedence <= Precedence::Assignment;

        match prefix_rule {
            RuleFn::None => {
                self.error_occured();
                let token = self.previous();
                error_at!(&token.span, "expected expression");
                return Ok(()); // returning `Err` is not required because error is being handled
            }
            _ => self.rule_fn(prefix_rule, can_assign),
        }

        while !self.is_at_end() {
            let kind = self.current().kind;
            let rule = self.get_rule(kind);
            let next_prece = ParseRule::get_precedence(rule);
            if precedence <= next_prece {
                self.advance();
                let kind = self.previous().kind;
                let infix_rule = ParseRule::get_infix(self.get_rule(kind));
                self.rule_fn(infix_rule, can_assign);
            } else {
                break;
            }
        }

        if can_assign && self.is_match(TokenKind::Assign) {
            error_at!(&self.previous().span, "invalid assignment target");
            return Err(());
        }
        Ok(())
    }

    fn and(&mut self) {
        let end_jump = self.emit_jump(OpCodes::JumpFalse(0));

        self.emit_byte(OpCodes::Pop);
        let Ok(_) = self.parse_precedence(Precedence::And) else {
            self.error(format!("expected expression after '{}'", self.previous().lexeme).as_str());
            return;
        };

        self.patch_jump(end_jump);
    }

    fn or(&mut self) {
        let else_jump = self.emit_jump(OpCodes::JumpFalse(0));
        let end_jump = self.emit_jump(OpCodes::Jump(0));

        self.patch_jump(else_jump);
        self.emit_byte(OpCodes::Pop);
        let Ok(_) = self.parse_precedence(Precedence::Or) else {
            self.error(format!("expected expression after '{}'", self.previous().lexeme).as_str());
            return;
        };

        self.patch_jump(end_jump);
    }

    fn get_rule(&self, kind: TokenKind) -> Rule {
        ParseRule::rules(kind)
    }

    fn variable(&mut self, can_assign: bool) {
        self.named_var(self.previous(), can_assign);
    }

    fn resolve_local(&mut self, token: &Token) -> Option<usize> {
        for (i, local) in self.locals.iter().rev().enumerate() {
            if local.name.lexeme == token.lexeme {
                if !local.initialized {
                    self.error("cannot read local variable in its own initializer");
                }
                return Some(self.locals.len() - i - 1);
            }
        }
        None
    }

    fn named_var(&mut self, token: &Token, can_assign: bool) {
        let index = self.identifier_constant(token);
        let mut get_op = OpCodes::GetGlobal(index);
        let mut set_op = OpCodes::SetGlobal(index);

        if let Some(local_index) = self.resolve_local(token) {
            get_op = OpCodes::GetLocal(local_index);
            set_op = OpCodes::SetLocal(local_index);
        }

        if can_assign && self.is_match(TokenKind::Assign) {
            let _ = self.expression();
            self.emit_byte(set_op);
        } else {
            self.emit_byte(get_op);
        }
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

    fn string(&mut self) {
        let token = self.previous();
        let len = token.lexeme.len() - 1;
        let literal = token.lexeme[1..len].chars().collect();
        self.emit_constant(Literal::String(literal))
    }

    fn grouping(&mut self) {
        let left_paren = self.previous();
        let Ok(()) = self.expression() else {
            error_at!(
                &left_paren.span,
                "expected expression after '{}'",
                left_paren.lexeme
            );
            return;
        };
        let token = self.current();
        if token.kind == TokenKind::RightParen {
            self.advance();
            return;
        }
        error_at!(&left_paren.span, "expected ')' after expression");
        self.error_occured();
    }

    fn unary(&mut self) {
        let token = self.previous();
        let kind = token.kind;
        let Ok(()) = self.parse_precedence(Precedence::Unary) else {
            error_at!(&token.span, "expected expression after `{}`", token.lexeme);
            return;
        };

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
        let Ok(()) = self.parse_precedence(ParseRule::get_precedence(rule)) else {
            error_at!(
                &operator.span,
                "expected expression after '{}'",
                operator.lexeme
            );
            return;
        };

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
        let Ok(()) = self.expression() else {
            let token = self.previous();
            error_at!(&token.span, "expected expression after `{}`", token.lexeme);
            return;
        };

        let Ok(()) = self.consume(TokenKind::Colon, "expected `:` after expression") else {
            return;
        };

        let Ok(()) = self.expression() else {
            let token = self.previous();
            error_at!(&token.span, "expected expression after `{}`", token.lexeme);
            return;
        };
        self.emit_byte(OpCodes::Ternary);
    }

    fn expression(&mut self) -> Result<(), ()> {
        let line = self.current().span.location.line;
        let start = self.current().span.location.start;
        self.parse_precedence(Precedence::Assignment)?;
        let end = self.previous().span.location.end; // ignore semicolon
        self.map_source(line, start, end);
        Ok(())
    }

    fn map_source(&self, line: u32, start: usize, end: usize) {
        self.source_map.borrow_mut().push(Span::new(
            self.file_path.clone(),
            Location::new(line, start, end),
        ));
    }

    // makeConstant
    fn emit_constant(&mut self, constant: Literal) {
        self.chunk
            .add_constant(constant, self.current().span.location.line)
    }

    fn emit_byte(&mut self, code: OpCodes) {
        self.chunk.write(code, self.previous().span.location.line)
    }

    fn emit_jump(&mut self, code: OpCodes) -> usize {
        self.emit_byte(code);
        self.chunk.code.len() - 1
    }

    fn patch_jump(&mut self, offset: usize) {
        let index = self.chunk.code.len() - offset - 1;
        self.chunk.code[offset] = self
            .chunk
            .code
            .get(offset)
            .expect("no jump 2")
            .patch_jump(index);
    }

    fn emit_loop(&mut self, start: usize) {
        let offset = self.chunk.code.len() - start + 1;
        self.emit_byte(OpCodes::Loop(offset));
    }

    fn advance(&mut self) {
        self.current += 1;
    }

    fn peek_match(&self, kind: TokenKind) -> bool {
        if self.current + 1 >= self.tokens.len() {
            return false;
        }

        if self.tokens[self.current + 1].kind == kind {
            return true;
        }

        return false;
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.tokens.len()
    }

    fn consume(&mut self, kind: TokenKind, message: &str) -> Result<(), ()> {
        let token = self.current();
        if token.kind == kind {
            self.advance();
            return Ok(());
        }
        error_at!(&token.span, "{message}");
        self.error_occured();
        Err(())
    }

    fn error(&mut self, msg: &str) {
        let token = self.previous();
        self.error_occured();
        error_at!(&token.span, "{msg}");
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
        self.panic_mode = true;
    }

    fn is_next_end(&self) -> bool {
        self.current + 1 == self.tokens.len()
    }

    fn is_next_end_error(&mut self, token: &Token, message: String) -> Result<(), ()> {
        if self.is_next_end() {
            self.error_occured();
            error_at!(&token.span, "{}", message);
            return Err(());
        }
        Ok(())
    }

    #[inline]
    fn check(&self, kind: TokenKind) -> bool {
        self.current().kind == kind
    }

    #[inline]
    fn is_match(&mut self, kind: TokenKind) -> bool {
        if !self.check(kind) {
            return false;
        }
        self.advance();
        true
    }
}
