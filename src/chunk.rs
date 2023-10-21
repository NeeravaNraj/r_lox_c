use crate::op_codes::OpCodes;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Line {
    pub line: usize,
    start: usize,
    end: Option<usize>,
}

#[derive(Debug)]
pub struct Chunk {
    pub code: Vec<OpCodes>,
    pub constants: Vec<f64>,
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

    pub fn write(&mut self, op_code: OpCodes, line: usize) {
        self.code.push(op_code);
        if let Some(index) = self.lines.last() {
            if line > index.line {
                self.lines.push(Line {
                    line,
                    start: self.lines.len(),
                    end: None,
                })
            }
            let new_len = self.lines.len();

            if let Some(prev) = self.lines.get_mut(new_len - 1) {
                prev.end = Some(self.code.len());
            }
        } else {
            self.lines.push(Line {
                line,
                start: self.lines.len(),
                end: None,
            })
        }
    }

    pub fn add_constant(&mut self, constant: f64, line: usize) {
        self.constants.push(constant);
        self.write(OpCodes::Constant(self.constants.len() - 1), line);
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
