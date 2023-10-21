use crate::{lexer::Lexer, tokenkind::TokenKind};

pub struct Compiler<'a> {
    lexer: Lexer<'a>
}

impl<'a> Compiler<'a> {
    pub fn new(file_path: &str, source: String) -> Self {
        Self {
            lexer: Lexer::new(file_path, source)
        }
    }

    pub fn compile(&self) {
        loop {
            let mut line = 0;
            let token = self.lexer.token();
            if token.span.location.line != line {
                print!("{:4}", token.span.location.line);
                line = token.span.location.line;
            } else {
                print!("   | ");
            }
            println!("{:?} '{}'", token.kind, token.lexeme);
            
            if token.kind == TokenKind::EOF { break; }
        }
    }
}
