mod chunk;
mod debug;
mod interpret_result;
mod op_codes;
mod repl;
mod vm;
mod parse_args;
mod runner;
mod lexer;
mod comiler;
mod logger;
mod token;
mod tokenkind;
mod location;
mod prelude;

use parse_args::ParseArgs;

fn main() {
    let args: Vec<_> = std::env::args().skip(1).collect();
    ParseArgs::new(args).parse();
}
