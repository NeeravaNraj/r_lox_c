use crate::{frontend::tokenization::token::Token, common::chunk::Chunk};

pub type LexerResult = Result<Token, ()>;
pub type CompilerResult<'chunk> = Result<&'chunk Chunk, ()>;
