use std::{path::Path, rc::Rc, process};

use crate::{repl::Repl, runner::Runner};

const HELP: &str = r#"
USAGE: lox [OPTIONS] [FILE] 

Options:
  -h, --help    Displays this screen.
  -d, --debug   Displays the opcodes and stack values.
  -t, --tokens  Displays lexed tokens.
"#;

#[derive(Clone)]
pub struct Options {
    pub file_path: Rc<Path>,
    pub debug: bool,
    pub print_tokens: bool,
}

impl Default for Options {
    fn default() -> Self {
        Options {
            file_path: Path::new("").into(),
            debug: false,
            print_tokens: false
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
                "-h" | "--help" => {
                    print!("{HELP}");
                    process::exit(0)
                },
                "-d" | "--debug" => options.debug = true,
                "-t" | "--tokens" => options.print_tokens = true,
                _ => options.file_path = Path::new(arg.as_str()).into(),
            }
        }

        options
    }
}
