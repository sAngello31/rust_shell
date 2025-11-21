use std::io::{self, Write};

pub fn run(){
    println!("Welcome to the Toy Rust Shell!");

    loop{
        print!("> ");
        std::io::stdout().flush().unwrap();
        let mut input: String = String::new();
        if io::stdin().read_line(&mut input).is_err(){
            continue;
        }

        let command = input.trim();
        if command == "exit"{
            break;
        }

        println!("{}", command);

    }
}