use crate::frontend::interpretation::{literal::Literal, op_codes::OpCodes};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Line {
    pub line: u32,
    start: usize,
    end: Option<usize>,
}

#[derive(Debug)]
pub struct Chunk {
    pub code: Vec<OpCodes>,
    pub constants: Vec<Literal>,
    pub lines: Vec<Line>,
}

impl Chunk {
    pub fn new() -> Self {
        Self {
            code: Vec::new(),
            constants: Vec::new(),
            lines: Vec::new(),
        }
    }

    pub fn write(&mut self, op_code: OpCodes, line: u32) {
        self.code.push(op_code);
        if let Some(index) = self.lines.last() {
            if line > index.line {
                self.lines.push(Line {
                    line,
                    start: self.code.len() - 1,
                    end: None,
                })
            }
            let new_len = self.lines.len();

            if let Some(prev) = self.lines.get_mut(new_len - 1) {
                prev.end = Some(self.code.len() - 1);
            }
        } else {
            self.lines.push(Line {
                line,
                start: self.code.len() - 1,
                end: None,
            })
        }
    }

    pub fn add_constant(&mut self, constant: Literal, line: u32) {
        self.constants.push(constant);
        self.write(OpCodes::Constant(self.constants.len() - 1), line);
    }

    pub fn add_constant_manual(&mut self, constant: Literal) -> usize {
        self.constants.push(constant);
        self.constants.len() - 1
    }

    pub fn get_line(&self, index: usize) -> Option<&Line> {
        self.lines.iter().find(|&line| {
            if let Some(end) = line.end {
                return index >= line.start && index < end;
            }
            index >= line.start
        })
    }

    pub fn check_previous(&self, offset: usize) -> bool {
        let Some(current) = self.get_line(offset) else {
            return false;
        };

        let Some(previous) = self.get_line(offset - 1) else {
            return false;
        };

        if current.line == previous.line {
            return true;
        }

        false
    }
}
