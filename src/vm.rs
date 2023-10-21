use crate::{
    chunk::Chunk, comiler::Compiler, debug::Debugger, interpret_result::InterpretResult,
    op_codes::OpCodes,
};

const STACK_MAX: usize = 256;

pub struct Vm {
    debugger: Debugger,
    debug_trace: bool,
    stack: Vec<f64>,
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

    pub fn interpret(&mut self, file_path: &str, source: String) -> InterpretResult {
        let compiler = Compiler::new(file_path, source).compile();
        // self.run(chunk)
        return InterpretResult::Ok;
    }

    pub fn run(&mut self, chunk: &Chunk) -> InterpretResult {
        macro_rules! vm_binary_op {
            // Only use for binary operations
            // WARNING: `tt` will match almost anything
            ($op:tt) => {
                {
                    let rhs = self.stack.pop().unwrap();
                    let lhs = self.stack.pop().unwrap();
                    self.stack.push(lhs $op rhs);
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
                        self.stack.push(-c);
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

    fn reset_stack(&mut self) {
        self.stack.clear()
    }

    pub fn set_debug_mode(&mut self) {
        self.debug_trace = true
    }

    pub fn disable_debug_mode(&mut self) {
        self.debug_trace = false
    }
}
