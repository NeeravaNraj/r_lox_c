use std::{path::Path, rc::Rc};

use crate::{repl::Repl, runner::Runner};

const HELP: &str = r#"
USAGE: lox [OPTIONS]* [FILE] 

*No arguments will start the REPL.

-h, --help    Shows this screen.
"#;

#[derive(Clone)]
pub struct Options {
    pub file_path: Rc<Path>,
    pub debug: bool,
}

impl Default for Options {
    fn default() -> Self {
        Options {
            file_path: Path::new("").into(),
            debug: false,
        }
    }
}

pub struct ParseArgs {
    args: Vec<String>,
}

impl ParseArgs {
    pub fn new(args: Vec<String>) -> Self {
        Self { args }
    }

    pub fn parse(&self) {
        let options = self.parse_option(&self.args);
        if !options.file_path.exists() {
            Repl::start(options);
        } else {
            let mut runner = Runner::new(options);
            runner.run();
        }
    }

    pub fn parse_option<'a>(&'a self, args: &'a Vec<String>) -> Options {
        let mut options = Options::default();
        for arg in args.iter() {
            match arg.as_str() {
                "-h" | "--help" => println!("{HELP}"),
                "-d" | "--debug" => options.debug = true,
                _ => options.file_path = Path::new(arg.as_str()).into(),
            }
        }

        options
    }
}
