mod shell;

use shell::repl;

fn main() {
    println!("Hello, world!");
    repl::run();
}