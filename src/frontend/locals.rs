use super::tokenization::token::Token;

pub struct Local<'token> {
    pub name: &'token Token,
    pub depth: usize,
    pub initialized: bool,
}

impl<'token> Local<'token> {
    pub fn new(token: &'token Token, depth: usize) -> Self {
        Local {
            name: token,
            initialized: false,
            depth,
        }
    }
}
