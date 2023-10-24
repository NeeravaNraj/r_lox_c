use std::rc::Rc;
use std::fmt::Display;
use super::location::Location;

pub struct Span {
    pub file: Rc<str>,
    pub location: Location,
}

impl Display for Span {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.file, self.location)
    }
}

impl Span {
    pub fn new(file: Rc<str>, location: Location) -> Self {
        Self {
            file,
            location,
        }
    }

    pub fn dup(&self) -> Self {
        Self {
            file: self.file.clone(),
            location: self.location
        }
    }
}
