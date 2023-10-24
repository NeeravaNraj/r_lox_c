use super::{span::Span, tokenkind::TokenKind};

pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
    pub lexeme: String,
}

impl Token {
    pub fn new(kind: TokenKind, span: Span, lexeme: impl Into<String>) -> Self {
        let lexeme: String = lexeme.into();
        Self { kind, span, lexeme }
    }

    pub fn eof(span: Span) -> Self {
        Self {
            kind: TokenKind::EOF,
            span,
            lexeme: String::from(""),
        }
    }

    pub fn dup(&self) -> Self {
        Self {
            kind: self.kind,
            span: self.span.dup(),
            lexeme: self.lexeme.to_string(),
        }
    }
}
