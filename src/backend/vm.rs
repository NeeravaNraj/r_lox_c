use std::rc::Rc;

use crate::{
    common::chunk::Chunk,
    error,
    frontend::{
        comiler::Compiler,
        interpretation::{interpret_result::InterpretResult, literal::Literal, op_codes::OpCodes},
        lexer::Lexer,
    },
    utils::debug::Debugger,
};

pub struct Vm {
    debugger: Debugger,
    debug_trace: bool,
    stack: Vec<Literal>,
    ip: usize,
}

impl Vm {
    pub fn new() -> Self {
        Self {
            ip: 0,
            debug_trace: false,
            debugger: Debugger::new("debug_vm"),
            stack: Vec::new(),
        }
    }

    pub fn interpret(&mut self, file_path: Rc<str>, source: String) -> InterpretResult {
        let mut chunk = Chunk::new();
        let mut lexer = Lexer::new(file_path.clone(), source);
        let Ok(tokens) = lexer.tokens() else {
            return InterpretResult::CompileError;
        };
        let mut compiler = Compiler::new(file_path.clone(), &tokens);
        let Ok(()) = compiler.compile(&mut chunk) else {
            error!("Couldn't run file due to error(s).");
            return InterpretResult::CompileError;
        };
        return self.run(&chunk);
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
            if self.debug_trace {
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
                    println!("{constant}");
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
        print!("        ");
        for slot in self.stack.iter() {
            println!("[{}]", slot)
        }
    }

    #[allow(dead_code)]
    fn reset_stack(&mut self) {
        self.stack.clear()
    }

    #[allow(dead_code)]
    pub fn set_debug_mode(&mut self) {
        self.debug_trace = true
    }

    #[allow(dead_code)]
    pub fn disable_debug_mode(&mut self) {
        self.debug_trace = false
    }
}
