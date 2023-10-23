use crate::frontend::tokenization::token::Token;

pub type LexerResult<'a> = Result<Token<'a>, ()>;
pub type CompilerResult = Result<(), ()>;
