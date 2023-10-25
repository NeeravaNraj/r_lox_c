use std::rc::Rc;

use crate::{
    common::chunk::Chunk,
    error,
    frontend::{
        compiler::Compiler,
        interpretation::{interpret_result::InterpretResult, literal::Literal, op_codes::OpCodes},
        lexer::Lexer,
    },
    utils::debug::Debugger, parse_args::Options,
};

pub struct Vm {
    debugger: Debugger,
    stack: Vec<Literal>,
    ip: usize,
    options: Options
}

impl Vm {
    pub fn new(options: Options) -> Self {
        Self {
            ip: 0,
            debugger: Debugger::new("debug_vm"),
            stack: Vec::new(),
            options
        }
    }

    pub fn interpret(&mut self, file_path: Rc<str>, source: String) -> InterpretResult {
        let mut lexer = Lexer::new(file_path.clone(), source);
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
        macro_rules! vm_binary_op {
            // Only use for binary operations
            // WARNING: `tt` will match almost anything
            ($op:tt) => {
                {
                    let rhs = self.stack.pop().unwrap();
                    let lhs = self.stack.pop().unwrap();
                    match (lhs $op rhs) {
                        Ok(value) => self.stack.push(value),
                        Err(err) => {
                            error!("{err}");
                            return InterpretResult::RuntimeError
                        }
                    };
                }
            };
        }
        loop {
            if self.options.debug {
                self.print_stack_slots();
                self.debugger.disassemble_instruction(chunk, self.ip);
            }
            let instruction: OpCodes = chunk.code[self.ip].into();
            match instruction {
                OpCodes::Return => {
                    if let Some(c) = self.stack.pop() {
                        println!("{c}");
                    }
                    return InterpretResult::Ok;
                }
                OpCodes::Constant(index) => {
                    let constant = chunk.constants[index];
                    self.stack.push(constant);
                }
                OpCodes::Negate => {
                    if let Some(c) = self.stack.pop() {
                        self.stack.push(c.negate());
                    }
                }
                OpCodes::Add => vm_binary_op!(+),
                OpCodes::Subtract => vm_binary_op!(-),
                OpCodes::Multiply => vm_binary_op!(*),
                OpCodes::Divide => vm_binary_op!(/),
            }
            self.bump();
        }
    }

    fn bump(&mut self) {
        self.ip += 1
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
}
