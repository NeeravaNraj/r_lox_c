#![deny(rust_2018_idioms)]

mod frontend;
mod backend;
mod utils;
mod common;
mod repl;
mod parse_args;
mod runner;
mod prelude;

use parse_args::ParseArgs;

fn main() {
    let args: Vec<_> = std::env::args().skip(1).collect();
    ParseArgs::new(args).parse();
}
