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
        write!(f, "{}:{}", self.line, self.start)
    }
}

impl Debug for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Location ( {} )", self)
    }
}

pub struct Span<'a> {
    pub file: &'a str,
    pub location: Location,
}

impl<'a> Display for Span<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.file, self.location)
    }
}

impl<'a> Span<'a> {
    pub fn new(file: &'a str, location: Location) -> Self {
        Self {
            file,
            location,
        }
    }

    pub fn dup(&self) -> Self {
        Self {
            file: self.file,
            location: self.location
        }
    }
}