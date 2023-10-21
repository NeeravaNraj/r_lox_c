use crate::{
    location::{Location, Span},
    prelude::LexerResult,
    token::Token,
    tokenkind::TokenKind,
};

pub struct Lexer<'a> {
    file_path: &'a str,
    source: Vec<char>,
    location: Location,
    current: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(file_path: &'a str, source: String) -> Self {
        Self {
            file_path,
            source: source.chars().collect(),
            location: Location::new(1, 0, 0),
            current: 0,
        }
    }

    pub fn token(&mut self) -> LexerResult {
        self.skip_whitespace();
        self.location.start = self.current;

        if self.is_at_end() {
            return Ok(Token::eof(Span::new(self.file_path, self.location)));
        }

        let c = self.advance();

        match c {
            '(' => return Ok(self.make_token(TokenKind::LeftParen)),
            ')' => return Ok(self.make_token(TokenKind::RightParen)),
            '{' => return Ok(self.make_token(TokenKind::LeftBrace)),
            '}' => return Ok(self.make_token(TokenKind::RightBrace)),
            '[' => return Ok(self.make_token(TokenKind::LeftBracket)),
            ']' => return Ok(self.make_token(TokenKind::RightBracket)),
            ',' => return Ok(self.make_token(TokenKind::Comma)),
            '.' => return Ok(self.make_token(TokenKind::Dot)),
            '-' => {
                let token = if self.is_match('=') {
                    TokenKind::MinusEqual
                } else if self.is_match('-') {
                    TokenKind::MinusMinus
                } else {
                    TokenKind::Minus
                };
                return Ok(self.make_token(token));
            }
            '+' => {
                let token = if self.is_match('=') {
                    TokenKind::PlusEqual
                } else if self.is_match('+') {
                    TokenKind::PlusPlus
                } else {
                    TokenKind::Plus
                };
                return Ok(self.make_token(token));
            }
            '%' => {
                let token = if self.is_match('=') {
                    TokenKind::ModEqual
                } else {
                    TokenKind::Modulus
                };
                return Ok(self.make_token(token));
            }
            '?' => return Ok(self.make_token(TokenKind::QuestionMark)),
            ':' => return Ok(self.make_token(TokenKind::Colon)),
            '*' => {
                let token = if self.is_match('=') {
                    TokenKind::StarEqual
                } else {
                    TokenKind::Star
                };
                return Ok(self.make_token(token));
            }
            ';' => return Ok(self.make_token(TokenKind::Semicolon)),
            '!' => {
                let token = if self.is_match('=') {
                    TokenKind::BangEqual
                } else {
                    TokenKind::Bang
                };
                return Ok(self.make_token(token));
            }
            '=' => {
                let token = if self.is_match('=') {
                    TokenKind::Equals
                } else {
                    TokenKind::Assign
                };
                return Ok(self.make_token(token));
            }
            '<' => {
                let token = if self.is_match('=') {
                    TokenKind::LessEqual
                } else {
                    TokenKind::Less
                };
                return Ok(self.make_token(token));
            }
            '>' => {
                let token = if self.is_match('=') {
                    TokenKind::GreaterEqual
                } else {
                    TokenKind::Greater
                };
                return Ok(self.make_token(token));
            }
            '/' => {
                if self.is_match('/') {
                    while self.peek() != Some('\n') && !self.is_at_end() {
                        self.advance();
                    }
                } else if self.is_match('*') {
                    // self.block_comment()?;
                } else if self.is_match('=') {
                    return Ok(self.make_token(TokenKind::SlashEqual));
                } else {
                    return Ok(self.make_token(TokenKind::Slash));
                }
            }
        }

        Err(())
    }

    fn make_token(&mut self, kind: TokenKind) -> Token<'_> {
        let lexeme: String = self.source[self.location.start..self.current]
            .iter()
            .collect();
        self.location.end = self.current;
        Token::new(kind, Span::new(self.file_path, self.location), lexeme)
    }

    fn skip_whitespace(&mut self) {
        loop {
            match self.peek() {
                Some(' ') | Some('\r') | Some('\t') => self.advance(),
                Some('\n') => {
                    self.location.line += 1;
                    self.advance();
                    break
                },
                _ => break,
            };
        }
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        self.source[self.current - 1]
    }

    fn is_match(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }

        if let Some(ch) = self.source.get(self.current) {
            if *ch != expected {
                return false;
            }
        }

        self.current += 1;
        true
    }

    fn peek(&self) -> Option<char> {
        if self.is_at_end() {
            return Some('\0');
        }

        if let Some(ch) = self.source.get(self.current) {
            return Some(*ch);
        }

        None
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }
}
