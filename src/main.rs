use std::{env, fs};

use elden::run_lexer;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("No file found. Usage: elden [script]");
    }

    let program_file = &args[1];
    run_program(program_file);
}

fn run_program(program_file: &str) {
    let contents =
        fs::read_to_string(program_file).expect("Should have been able to read the file");
    let (tokens, rest) = run_lexer(&contents).unwrap();
    println!("Tokens: {:?}", tokens);
    println!("{}", rest);
}
