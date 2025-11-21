use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about = "A simple Rust shell")]
pub struct Args{
    #[arg(short, long)]
    pub debug: bool,
}