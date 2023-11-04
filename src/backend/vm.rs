use std::{collections::HashMap, rc::Rc};

use crate::{
    common::chunk::Chunk,
    error, error_line,
    frontend::{
        compiler::Compiler,
        interpretation::{interpret_result::InterpretResult, literal::Literal, op_codes::OpCodes},
        lexer::Lexer,
        tokenization::{location::Location, span::Span},
    },
    parse_args::Options,
    utils::debug::Debugger,
};

pub struct Vm {
    debugger: Debugger,
    stack: Vec<Literal>,
    globals: HashMap<String, Literal>,
    ip: usize,
    options: Options,
}

impl Vm {
    pub fn new(options: Options) -> Self {
        Self {
            ip: 0,
            debugger: Debugger::new("debug_vm"),
            stack: Vec::new(),
            globals: HashMap::new(),
            options,
        }
    }

    pub fn interpret(&mut self, file_path: Rc<str>, source: String) -> InterpretResult {
        let mut lexer = Lexer::new(file_path.clone(), source, self.options.clone());
        let Ok(tokens) = lexer.tokens() else {
            return InterpretResult::CompileError;
        };
        let mut compiler = Compiler::new(file_path.clone(), &tokens);
        let Ok(chunk) = compiler.compile() else {
            error!("Couldn't run file due to error(s).");
            return InterpretResult::CompileError;
        };
        self.reset_stack();
        return self.run(chunk);
    }

    pub fn run(&mut self, chunk: &Chunk) -> InterpretResult {
        loop {
            if self.options.debug {
                self.print_stack_slots();
                self.debugger.disassemble_instruction(chunk, self.ip);
            }
            let instruction = &chunk.code[self.ip];
            match instruction {
                OpCodes::Return => {
                    return InterpretResult::Ok;
                }
                OpCodes::Constant(index) => {
                    let constant = chunk.constants[*index].clone();
                    self.stack.push(constant);
                }
                OpCodes::Negate => {
                    let Ok(c) = self.negate(&chunk) else {
                        return InterpretResult::RuntimeError
                    };
                    self.stack.push(c)
                }
                OpCodes::Add => {
                    let Ok(res) = self.binary(|l, r| l + r, chunk) else {
                        return InterpretResult::RuntimeError;
                    };
                    self.stack.push(res)
                }
                OpCodes::Subtract => {
                    let Ok(res) = self.binary(|l, r| l - r, chunk) else {
                        return InterpretResult::RuntimeError;
                    };
                    self.stack.push(res)
                }
                OpCodes::Multiply => {
                    let Ok(res) = self.binary(|l, r| l * r, chunk) else {
                        return InterpretResult::RuntimeError;
                    };
                    self.stack.push(res)
                }
                OpCodes::Divide => {
                    let Ok(res) = self.binary(|l, r| l / r, chunk) else {
                        return InterpretResult::RuntimeError;
                    };
                    self.stack.push(res)
                }
                OpCodes::True => self.stack.push(Literal::Bool(true)),
                OpCodes::False => self.stack.push(Literal::Bool(false)),
                OpCodes::None => self.stack.push(Literal::None),
                OpCodes::Not => {
                    let Ok(res) = self.not(chunk) else {
                        return InterpretResult::RuntimeError;
                    };
                    self.stack.push(res)
                }
                OpCodes::Equals => {
                    let Ok(res) = self.binary(|a, b| {
                        a.equatable(&b)?;
                        Ok(Literal::Bool(a == b))
                    }, chunk) else {
                        return InterpretResult::RuntimeError;
                    };
                    self.stack.push(res)
                }

                OpCodes::NotEquals => {
                    let Ok(res) = self.binary(|a, b| {
                        a.equatable(&b)?;
                        Ok(Literal::Bool(a != b))
                    }, chunk) else {
                        return InterpretResult::RuntimeError;
                    };
                    self.stack.push(res)
                }

                OpCodes::Greater => {
                    let Ok(res) = self.binary(|a, b| {
                        a.comparable(&b)?;
                        Ok(Literal::Bool(a > b))
                    }, chunk) else {
                        return InterpretResult::RuntimeError;
                    };
                    self.stack.push(res)
                }

                OpCodes::GreaterEquals => {
                    let Ok(res) = self.binary(|a, b| {
                        a.equatable(&b)?;
                        Ok(Literal::Bool(a >= b))
                    }, chunk) else {
                        return InterpretResult::RuntimeError;
                    };
                    self.stack.push(res)
                }

                OpCodes::Less => {
                    let Ok(res) = self.binary(|a, b| {
                        a.equatable(&b)?;
                        Ok(Literal::Bool(a < b))
                    }, chunk) else {
                        return InterpretResult::RuntimeError;
                    };
                    self.stack.push(res)
                }

                OpCodes::LessEquals => {
                    let Ok(res) = self.binary(|a, b| {
                        a.equatable(&b)?;
                        Ok(Literal::Bool(a <= b))
                    }, chunk) else {
                        return InterpretResult::RuntimeError;
                    };
                    self.stack.push(res)
                }
                OpCodes::Ternary => {
                    let Ok(res) = self.ternary(chunk) else {
                        return InterpretResult::RuntimeError;
                    };
                    self.stack.push(res)
                }
                OpCodes::Print => {
                    if let Some(literal) = self.stack.pop() {
                        println!("{}", literal);
                    } else {
                        self.try_error_line("no literal to print", chunk)
                    }
                }
                OpCodes::Pop => {
                    self.stack.pop();
                }

                OpCodes::SetGlobal(index) => {
                    let Some(value) = self.peek(0) else {
                        self.try_error_line("could not get variable value", chunk);
                        return InterpretResult::RuntimeError;
                    };

                    let Some(Literal::Variable(name))= chunk.constants.get(*index) else {
                        self.try_error_line("could not get variable name", chunk);
                        return InterpretResult::RuntimeError;
                    };

                    self.globals.insert(name.clone(), value.clone());
                    self.stack.pop();
                }

                OpCodes::GetGlobal(index) => {
                    let Some(Literal::Variable(name))= chunk.constants.get(*index) else {
                        self.try_error_line("could not get variable name", chunk);
                        return InterpretResult::RuntimeError;
                    };

                    let Some(value) = self.globals.get(name) else {
                        self.try_error_line(format!("undefined variable '{}'", name).as_str(), chunk);
                        return InterpretResult::RuntimeError;
                    };

                    self.stack.push(value.clone());
                }
            }
            self.bump();
        }
    }

    fn negate(&mut self, chunk: &Chunk) -> Result<Literal, ()> {
        let Some(c) = self.stack.pop() else {
            self.try_error_line(format!("no operand for `-` expression").as_str(), chunk);
            return Err(())
        };
        if !c.is_number() {
            self.try_error_line(
                format!("cannot negate type {}", c.type_name()).as_str(),
                chunk,
            );
            return Err(());
        }
        return Ok(c.negate());
    }

    fn not(&mut self, chunk: &Chunk) -> Result<Literal, ()> {
        let Some(c) = self.stack.pop() else {
            self.try_error_line(format!("no operand for `!` expression").as_str(), chunk);
            return Err(())
        };

        return Ok(c.not());
    }

    fn binary(
        &mut self,
        f: fn(Literal, Literal) -> Result<Literal, String>,
        chunk: &Chunk,
    ) -> Result<Literal, ()> {
        let Some(rhs) = self.stack.pop() else {
            self.try_error_line(format!("no rhs operand for binary expression").as_str(), chunk);
            return Err(())
        };
        let Some(lhs) = self.stack.pop() else {
            self.try_error_line(format!("no lhs operand for binary expression").as_str(), chunk);
            return Err(())
        };
        match f(lhs, rhs) {
            Ok(value) => return Ok(value),
            Err(err) => {
                error!("{err}");
                return Err(());
            }
        };
    }

    fn ternary(&mut self, chunk: &Chunk) -> Result<Literal, ()> {
        let Some(falsey) = self.stack.pop() else {
            self.try_error_line(format!("no `falsey` expression for ternary operator").as_str(), chunk);
            return Err(())
        };

        let Some(truthy) = self.stack.pop() else {
            self.try_error_line(format!("no `truthy` expression for ternary operator").as_str(), chunk);
            return Err(())
        };

        let Some(condition) = self.stack.pop() else {
            self.try_error_line(format!("no condition for ternary operator").as_str(), chunk);
            return Err(())
        };

        if let Literal::Bool(v) = condition {
            if v {
                return Ok(truthy);
            }
        }
        Ok(falsey)
    }

    fn bump(&mut self) {
        self.ip += 1
    }

    fn peek(&self, distance: usize) -> Option<&Literal> {
        self.stack.get(self.stack.len() - 1 - distance)
    }

    fn print_stack_slots(&self) {
        println!("stack trace:");
        let mut stack = String::from("[");
        for slot in self.stack.iter() {
            stack.push_str(slot.to_string().as_str());
            stack.push_str(", ");
        }
        stack.push(']');
        println!("{stack}");
    }

    fn reset_stack(&mut self) {
        self.ip = 0;
        self.stack.clear()
    }

    fn try_error_line(&self, message: &str, chunk: &Chunk) {
        if let Some(line) = chunk.get_line(self.ip - 1) {
            let line = line.line;
            error_line!(&self.create_span(line), "{}", message)
        } else {
            // probably unreachable
            error!("{}", message)
        }
    }

    fn create_span(&self, line: u32) -> Span {
        let file = self.options.file_path.to_str().unwrap();
        Span::new(file.into(), Location::new(line, 0, 0))
    }
}
