use std::io::{stdout, Write, stdin};

pub struct Repl;

impl Repl {
    pub fn start() {
        let mut code = String::new();
        let mut stdout = stdout();
        loop {
            write!(stdout, "Lox> ").expect("Unable to write to `stdout`");
            stdout.flush().expect("Unable to flush to `stdout`");
            stdin().read_line(&mut code).expect("Unable to read from `stdin`");

            match code.as_str().trim() {
                "quit" | "exit" => break,
                _ => () //interpret
            }
            code.clear();
        }
    }
}
