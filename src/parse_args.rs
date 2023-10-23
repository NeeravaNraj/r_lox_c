use std::path::Path;

use crate::{repl::Repl, runner::Runner};

const HELP: &str = r#"
USAGE: lox [OPTIONS]* [FILE] 

*No arguments will start the REPL.

-h, --help    Shows this screen.
"#;

pub struct ParseArgs {
    args: Vec<String>,
}

impl ParseArgs {
    pub fn new(args: Vec<String>) -> Self {
        Self { args }
    }

    pub fn parse(&self) {
        if self.args.len() == 0 {
            Repl::start();
        } else {
            for arg in self.args.iter() {
                if let Some(path) = self.parse_option(arg) {
                    let path = Path::new(path);
                    let mut runner = Runner::new(path);
                    runner.run();
                }
            }
        }
    }

    pub fn parse_option<'a>(&'a self, arg: &'a String) -> Option<&String> {
        match arg.as_str() {
            "-h" | "--help" =>  println!("{HELP}"),
            _ => return Some(arg),
        }
        None
    }
}
