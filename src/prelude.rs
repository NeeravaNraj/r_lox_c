use crate::token::Token;

pub type LexerResult<'a> = Result<Token<'a>, ()>;
