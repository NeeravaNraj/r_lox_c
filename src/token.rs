use crate::{location::Span, tokenkind::TokenKind};

pub struct Token<'a> {
    pub kind: TokenKind,
    pub span: Span<'a>,
    pub lexeme: String,
}

impl<'a> Token<'a> {
    pub fn new(kind: TokenKind, span: Span<'a>, lexeme: impl Into<String>) -> Self {
        let lexeme: String = lexeme.into();
        Self { kind, span, lexeme }
    }

    pub fn eof(span: Span<'a>) -> Self {
        Self {
            kind: TokenKind::EOF,
            span,
            lexeme: String::from(""),
        }
    }
}
