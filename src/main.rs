use std::{env, fs};

use elden::{get_program, print_ast, run_lexer};

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
    let tokens = match run_lexer(&contents) {
        Ok(output) => output.0,
        Err(err) => {
            println!("{}", err);
            return;
        }
    };
    println!("Tokens: {:?}", tokens);
    let program = match get_program(&tokens) {
        Ok(output) => output,
        Err(err) => {
            println!("{}", err);
            return;
        }
    };
    print_ast(&program);
}
