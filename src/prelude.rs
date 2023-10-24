use crate::frontend::tokenization::token::Token;

pub type LexerResult = Result<Token, ()>;
pub type CompilerResult = Result<(), ()>;
