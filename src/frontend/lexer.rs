use super::tokenization::{location::Location, span::Span, token::Token, tokenkind::TokenKind};
use crate::{error_at, prelude::LexerResult};
use std::rc::Rc;

pub struct Lexer {
    file_path: Rc<str>,
    source: Vec<char>,
    location: Location,
    start: usize,
    current: usize,
}

impl Lexer {
    pub fn new(file_path: Rc<str>, source: String) -> Self {
        Self {
            file_path,
            source: source.chars().collect(),
            location: Location::new(1, 0, 0),
            start: 0,
            current: 0,
        }
    }

    pub fn tokens(&mut self) -> Result<Vec<Token>, ()> {
        let mut stream: Vec<Token> = Vec::new();
        loop {
            if self.is_at_end() {
                break;
            }
            stream.push(self.token()?);
        }

        Ok(stream)
    }

    pub fn token(&mut self) -> LexerResult {
        self.skip_whitespace();
        self.start = self.current;
        self.location.start = self.location.end;

        if self.is_at_end() {
            return Ok(Token::eof(Span::new(self.file_path.clone(), self.location)));
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
                if self.is_match('=') {
                    return Ok(self.make_token(TokenKind::SlashEqual));
                } else {
                    return Ok(self.make_token(TokenKind::Slash));
                }
            }
            '"' | '\'' => return self.string(c),

            '0'..='9' => return self.number(),

            _ if self.is_alphanum(c) => return self.identifier(),
            _ => return Err(()),
        }
    }

    fn make_token(&mut self, kind: TokenKind) -> Token {
        let lexeme: String = self.source[self.start..self.current].iter().collect();
        Token::new(kind, Span::new(self.file_path.clone(), self.location), lexeme)
    }

    fn skip_whitespace(&mut self) {
        loop {
            match self.peek() {
                Some(' ') | Some('\r') | Some('\t') => self.advance(),
                Some('\n') => {
                    self.reset_loc();
                    self.location.line += 1;
                    self.advance();
                    break;
                }
                Some('/') => {
                    if self.peek_next() == Some('/') {
                        while self.peek() != Some('\n') && !self.is_at_end() {
                            self.advance();
                        }
                    } else if self.peek_next() == Some('*') {
                        // self.block_comment()?;
                    }
                    break;
                }
                _ => break,
            };
        }
    }

    fn identifier(&mut self) -> LexerResult {
        while let Some(ch) = self.peek() {
            if self.is_alphanum(ch) || ch.is_ascii_digit() {
                self.advance();
            } else {
                break;
            }
        }

        let Some(kind) = self.keywords() else {
            return Ok(self.make_token(TokenKind::Identifier))
        };

        Ok(self.make_token(kind))
    }

    fn string(&mut self, starter: char) -> LexerResult {
        while self.peek() != Some(starter) && !self.is_at_end() {
            if self.peek() == Some('\n') {
                self.location.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            self.location.end = self.location.start + 1;
            error_at!(
                &self.create_span(),
                "unterminated string"
            );
            return Err(());
        }
        self.advance();

        Ok(self.make_token(TokenKind::String))
    }

    // fn format_string(&mut self) -> LexerResult {
    //     while self.peek() != Some('`') && !self.is_at_end() {
    //         match self.peek() {
    //             Some('\n') => self.location.line += 1,
    //             Some('{') => (),
    //             _ => ()
    //         };
    //         self.advance();
    //     }
    //
    //     if self.is_at_end() {
    //         self.location.end = self.location.start + 1;
    //         error!(Span::new(self.file_path, self.location),"unterminated string");
    //         return Err(())
    //     }
    //     self.advance();
    //
    //     Ok(self.make_token(TokenKind::FormatString))
    // }

    fn number(&mut self) -> LexerResult {
        while let Some(ch) = self.peek() {
            if !ch.is_ascii_digit() {
                break;
            }
            self.advance();
        }

        if self.peek() == Some('.') {
            self.advance();
            while let Some(ch) = self.peek() {
                if !ch.is_ascii_digit() {
                    return Ok(self.make_token(TokenKind::Float));
                }
                self.advance();
            }
        }

        Ok(self.make_token(TokenKind::Int))
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        self.location.end += 1;
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
            return None;
        }

        if let Some(ch) = self.source.get(self.current) {
            return Some(*ch);
        }

        None
    }

    fn peek_next(&self) -> Option<char> {
        if self.is_at_end() {
            return None;
        }

        if let Some(ch) = self.source.get(self.current + 1) {
            return Some(*ch);
        }

        None
    }

    fn create_span(&self) -> Span {
        Span::new(self.file_path.clone(), self.location)
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn is_alphanum(&self, c: char) -> bool {
        c.is_alphanumeric() || c == '_'
    }

    fn keywords(&self) -> Option<TokenKind> {
        let source: String = self.source[self.start..self.current].iter().collect();
        match source.as_str() {
            "if" => Some(TokenKind::If),
            "else" => Some(TokenKind::Else),
            "elif" => Some(TokenKind::Elif),
            "this" => Some(TokenKind::This),
            "true" => Some(TokenKind::True),
            "false" => Some(TokenKind::False),
            "class" => Some(TokenKind::Class),
            "continue" => Some(TokenKind::Continue),
            "break" => Some(TokenKind::Break),
            "for" => Some(TokenKind::For),
            "while" => Some(TokenKind::While),
            "fn" => Some(TokenKind::DefFn),
            "let" => Some(TokenKind::Let),
            "pub" => Some(TokenKind::Public),
            "static" => Some(TokenKind::Static),
            "lm" => Some(TokenKind::DefLambda),
            "none" => Some(TokenKind::None),
            "return" => Some(TokenKind::Return),
            "or" => Some(TokenKind::Or),
            "and" => Some(TokenKind::And),
            _ => None,
        }
    }

    fn reset_loc(&mut self) {
        self.location.start = 0;
        self.location.end = 0;
    }
}
