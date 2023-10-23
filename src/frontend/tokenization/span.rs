use std::fmt::Display;
use super::location::Location;


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
