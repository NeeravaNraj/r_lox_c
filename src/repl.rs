use std::{io::{stdout, Write, stdin}, rc::Rc};

use crate::{backend::vm::Vm, parse_args::Options};

pub struct Repl;

impl Repl {
    pub fn start(options: Options) {
        let mut code = String::new();
        let mut stdout = stdout();
        let name: Rc<str> = "repl".into();
        let mut vm = Vm::new(options);
        let mut commands: Vec<String> = Vec::new();
        loop {
            write!(stdout, "<Lox> ").expect("Unable to write to `stdout`");
            stdout.flush().expect("Unable to flush to `stdout`");
            stdin().read_line(&mut code).expect("Unable to read from `stdin`");

            match code.as_str().trim() {
                "quit" | "exit" => break,
                _ => () //interpret
            }
            vm.interpret(name.clone(), code.to_string());
            commands.push(code.to_string());
            code.clear();
        }
    }
}
