use std::fmt::{Display, Debug};

#[derive(Clone, Copy)]
pub struct Location {
    pub line: u32,
    pub start: usize,
    pub end: usize 
}

impl Location {
    pub fn new(line: u32, start: usize, end: usize) -> Self {
        Self {
            line,
            start,
            end
        }
    }
}

impl Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}:{}", self.line, self.start, self.end)
    }
}

impl Debug for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

