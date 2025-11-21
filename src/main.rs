mod utils;
mod shell;

use clap::Parser;
use utils::args::Args;
use shell::repl;

fn main() {
    let _args= Args::parse(); // Arguments parsing
    repl::run();
}