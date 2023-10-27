use std::io::{stdout, Stdout, Write};

use crate::{
    common::chunk::Chunk,
    frontend::interpretation::op_codes::OpCodes
};

pub struct Debugger {
    name: String,
    stdout: Stdout,
}

impl Debugger {
    pub fn new(name: impl Into<String>) -> Self {
        let name: String = name.into();
        Self {
            name,
            stdout: stdout(),
        }
    }

    pub fn disassemble(&mut self, chunk: &Chunk) {
        writeln!(
            self.stdout,
            "\x1b[1m\x1b[4m\x1b[38;2;255;170;50m==== {} ====\x1b[0m",
            self.name
        )
        .unwrap();

        let mut offset: usize = 0;
        while offset < chunk.code.len() {
            offset = self.disassemble_instruction(chunk, offset)
        }

        self.stdout.flush().unwrap();
    }

    fn constant_instruction(&mut self, chunk: &Chunk, value: usize, offset: usize) -> usize {
        write!(self.stdout, "{:-16} {offset:4} '", chunk.code[offset]).unwrap();
        writeln!(self.stdout, "{}", chunk.constants[value]).unwrap();
        offset + 1
    }

    fn simple_instruction(&mut self, chunk: &Chunk, offset: usize) -> usize {
        writeln!(self.stdout, "{}", chunk.code[offset]).unwrap();
        offset + 1
    }

    pub fn disassemble_instruction(&mut self, chunk: &Chunk, offset: usize) -> usize {
        let instruction = chunk.code[offset];
        write!(self.stdout, "{:04} ", offset).unwrap();

        if offset > 0 && chunk.check_previous(offset) {
            write!(self.stdout, "   |  ").unwrap();
        } else {
            if let Some(line) = chunk.get_line(offset) {
                write!(self.stdout, "{:>4}  ", line.line).unwrap();
            }
        }

        match instruction.into() {
            OpCodes::Constant(value) => self.constant_instruction(chunk, value, offset),
            OpCodes::Return |
            OpCodes::Negate |
            OpCodes::Add |
            OpCodes::Subtract |
            OpCodes::Multiply |
            OpCodes::Divide |
            OpCodes::True |
            OpCodes::False |
            OpCodes::Not |
            OpCodes::Less |
            OpCodes::Greater |
            OpCodes::LessEquals |
            OpCodes::GreaterEquals |
            OpCodes::NotEquals |
            OpCodes::Equals |
            OpCodes::Ternary |
            OpCodes::None => self.simple_instruction(chunk, offset),
        }
    }
}
