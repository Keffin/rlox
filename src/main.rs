mod lox;

use lox::Lox;
use std::env;
mod expr;
mod parser;
mod scanner;
mod token;
mod token_type;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut lox_instance = Lox::new();

    if args.len() == 1 {
        println!("Arg len 1 => Run prompt!");
        lox_instance.repl();
    } else if args.len() == 2 {
        println!("Arg len 2 => Run using file input");
        lox_instance.run_file(&args[1]);
    } else {
        println!("Arg len invalid => exiting program..");
    }
}
